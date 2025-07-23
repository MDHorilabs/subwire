use mio::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("[::]:0".parse().unwrap()).unwrap();
    socket
        .send_to(b"hello", "[::]:5322".parse().unwrap())
        .unwrap();
}
