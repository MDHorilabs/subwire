use crate::conf::BUF_SIZE;
use crate::queue::buffer_poll::BufferPool;
use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};
use socket2::{Domain, Protocol, Socket, Type};
use std::{io::ErrorKind, net::SocketAddr, time::Duration};

pub fn runtime<F>(addr: &str, ipv6_only: bool, mut handler: F) -> std::io::Result<()>
where
    F: FnMut(Box<[u8; BUF_SIZE]>, usize, SocketAddr, &BufferPool, &UdpSocket),
{
    let addr: SocketAddr = addr.parse().expect("Address must IPv6");

    let socket =
        Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP)).expect("fail create socket");

    if ipv6_only {
        socket.set_only_v6(true).expect("fail set only v6");
    }
    socket.set_nonblocking(true).expect("fail set nonblocking");

    socket.bind(&addr.into()).expect("failed bind");

    let socket_std: std::net::UdpSocket = socket.into();
    let mut socket = UdpSocket::from_std(socket_std);

    let mut event_poll = Poll::new().expect("fail create event poll");
    let token = Token(0);
    event_poll
        .registry()
        .register(&mut socket, token, Interest::READABLE)
        .expect("fail regist event poll");

    let mut events = Events::with_capacity(128);
    let buf_poll = BufferPool::new();

    loop {
        event_poll.poll(&mut events, Some(Duration::from_millis(100)))?;
        for event in events.iter() {
            if event.token() == token && event.is_readable() {
                let mut buf = buf_poll.get();
                match socket.recv_from(&mut buf[..]) {
                    Ok((n, peer_addr)) => {
                        println!("recv buffer size: {} from: {:?}", n, peer_addr);
                        handler(buf, n, peer_addr, &buf_poll, &socket);
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        buf_poll.put(buf);
                        println!("[WouldBlock]");
                        break;
                    }
                    Err(err) => {
                        eprintln!("[ERROR] recv_from: {:?}", err);
                        buf_poll.put(buf);
                        break;
                    }
                }
            }
        }
    }
}
