use crate::conf::{BUF_SIZE, POOL_SIZE};
use crossbeam_queue::SegQueue;

pub struct BufferPool {
    pool: SegQueue<Box<[u8; BUF_SIZE]>>,
}

impl BufferPool {
    pub fn new() -> Self {
        let pool = SegQueue::new();
        for _ in 0..POOL_SIZE {
            pool.push(Box::new([0u8; BUF_SIZE]));
        }
        Self { pool }
    }

    pub fn get(&self) -> Box<[u8; BUF_SIZE]> {
        self.pool.pop().unwrap_or_else(|| Box::new([0u8; BUF_SIZE]))
    }

    pub fn put(&self, buf: Box<[u8; BUF_SIZE]>) {
        self.pool.push(buf);
    }
}
