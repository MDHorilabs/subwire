use mio::{Events, Interest, Poll, Token};
use mio::net::UdpSocket;
use crossbeam_queue::SegQueue;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

const BUF_SIZE: usize = 1500;
const POOL_SIZE: usize = 1024;
const SOCKET: Token = Token(0);

/// Buffer pool: reuse memory instead of reallocating
struct BufferPool {
    pool: SegQueue<Box<[u8; BUF_SIZE]>>,
}

impl BufferPool {
    fn new() -> Self {
        let pool = SegQueue::new();
        for _ in 0..POOL_SIZE {
            pool.push(Box::new([0u8; BUF_SIZE]));
        }
        Self { pool }
    }

    fn get(&self) -> Box<[u8; BUF_SIZE]> {
        self.pool.pop().unwrap_or_else(|| Box::new([0u8; BUF_SIZE]))
    }

    fn put(&self, buf: Box<[u8; BUF_SIZE]>) {
        self.pool.push(buf);
    }
}

fn main() -> std::io::Result<()> {
    let addr: SocketAddr = "0.0.0.0:9000".parse().unwrap();
    let mut socket = UdpSocket::bind(addr)?;
    println!("📡 Listening on {}", addr);

    let mut poll = Poll::new()?;
    poll.registry().register(&mut socket, SOCKET, Interest::READABLE)?;

    let mut events = Events::with_capacity(128);
    let buffer_pool = Arc::new(BufferPool::new());

    loop {
        // Wait for up to 100ms for an event
        poll.poll(&mut events, Some(Duration::from_millis(100)))?;

        for event in events.iter() {
            if event.token() == SOCKET && event.is_readable() {
                let buf = buffer_pool.get();
                match socket.recv_from(&mut buf[..]) {
                    Ok((n, addr)) => {
                        println!("📨 [{} bytes] from {}: {:?}", n, addr, &buf[..n]);

                        // TODO: handle message / decode / ACK logic

                        // return buffer to pool
                        buffer_pool.put(buf);
                    }
                    Err(e) => {
                        eprintln!("❌ recv_from error: {:?}", e);
                        buffer_pool.put(buf); // return buffer even on error
                    }
                }
            }
        }
    }
}
