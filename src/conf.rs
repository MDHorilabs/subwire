pub const SUBWIRE_MAGIC: [u8; 4] = [0x53, 0x57, 0x49, 0x52]; // ASCII: S W I R
pub const PACKET_HEADING_SIZE: usize = 3;
pub const PACKET_SIZE: usize = 1472;
pub const BUF_SIZE: usize = PACKET_SIZE - (4 + PACKET_HEADING_SIZE);
pub const POOL_SIZE: usize = 1024;
