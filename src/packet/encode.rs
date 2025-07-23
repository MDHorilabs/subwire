use crate::conf::{BUF_SIZE, PACKET_SIZE, SUBWIRE_MAGIC};

pub fn encode<'a>(
    data: &[u8; BUF_SIZE],
    is_encrypt: bool,
    is_compressed: bool,
    out: &'a mut [u8; PACKET_SIZE],
) {
    out[..4].copy_from_slice(&SUBWIRE_MAGIC);
    out[4] = if is_encrypt { 1 } else { 0 };
    out[5] = if is_compressed { 1 } else { 0 };
    out[6..].copy_from_slice(data);
}
