use subwire::runtime::dev::runtime;

fn main() {
    runtime(
        "[::]:5322",
        true,
        |buf, size, peer_addr, buffer_pool, _socket| {
            /// ini harus disini
            buffer_pool.put(buf);
        },
    )
    .unwrap();
}
