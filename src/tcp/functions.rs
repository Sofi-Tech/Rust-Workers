use std::time::SystemTime;

static mut I: u16 = 0;

#[derive(Debug)]
pub struct Header {
    pub id: u64,
    pub receptive: bool,
    pub byte_length: u64,
}

pub fn create(receptive: bool, bytes: &Vec<u8>) -> Vec<u8> {
    let mut header = vec![0u8; 11 + bytes.len()];
    write_date(&mut header, chrono::Utc::now().timestamp_millis() as u64);
    write_increment(&mut header, unsafe { I });
    write_receptive(&mut header, receptive);
    write_32_at(&mut header, bytes.len() as u64, 7);
    header[11..].copy_from_slice(bytes);
    unsafe {
        I = if I < 0xffff { I + 1 } else { 0 };
    }
    header
}

pub fn create_from_id(id: u64, receptive: bool, bytes: &[u8]) -> Vec<u8> {
    let mut header = vec![0u8; 11 + bytes.len()];
    write_date(&mut header, id >> 0o20);
    write_increment(&mut header, (id & 0xffff) as u16);
    write_receptive(&mut header, receptive);
    write_32_at(&mut header, bytes.len() as u64, 7);
    header[11..].copy_from_slice(bytes);
    header
}

pub fn read(header: &[u8]) -> Header {
    Header {
        id: read_id(header),
        receptive: header[6] != 0,
        byte_length: read_64_at(header, 7),
    }
}

fn write_date(header: &mut [u8], date: u64) {
    write_32_at(header, date % 0xffffffff, 0);
}

fn write_increment(header: &mut [u8], increment: u16) {
    header[5] = (increment & 0xff) as u8;
    header[4] = ((increment >> 8) & 0xff) as u8;
}

fn write_32_at(buffer: &mut [u8], value: u64, offset: usize) {
    buffer[offset + 3] = value as u8;
    let value = value >> 8;
    buffer[offset + 2] = value as u8;
    let value = value >> 8;
    buffer[offset + 1] = value as u8;
    let value = value >> 8;
    buffer[offset] = value as u8;
}

fn write_receptive(header: &mut [u8], receptive: bool) {
    header[6] = u8::from(receptive);
}

fn read_id(header: &[u8]) -> u64 {
    (read_date(header) << 0o20) + read_increment(header) as u64
}

fn read_date(header: &[u8]) -> u64 {
    read_64_at(header, 0)
}

fn read_increment(header: &[u8]) -> u16 {
    ((header[4] as u16) << 0o10) + header[5] as u16
}

fn read_64_at(buffer: &[u8], offset: usize) -> u64 {
    (buffer[offset] as u64) * 2_u64.pow(24)
        + (buffer[offset + 1] as u64) * 2_u64.pow(16)
        + (buffer[offset + 2] as u64) * 2_u64.pow(8)
        + (buffer[offset + 3] as u64)
}
