use std::error::Error;
use std::io::Cursor;
use tokio::io::AsyncWriteExt;

/// Convert an integer to varint bytes
pub fn to_varint(int: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut int = (int as u64) & 0xFFFF_FFFF;
    let mut written = 0;
    let mut buffer = [0; 5];

    loop {
        let temp = (int & 0b0111_1111) as u8;
        int >>= 7;
        if int != 0 {
            buffer[written] = temp | 0b1000_0000;
        } else {
            buffer[written] = temp;
        }
        written += 1;
        if int == 0 {
            break;
        }
    }

    Ok(buffer[0..written].to_vec())
}

pub async fn build_packet(
    protocol: isize,
    description: &str,
    version: &str,
    favicon: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let payload = format!("{{\"version\": {{\"name\": \"{}\", \"protocol\": {}}}, \"players\": {{\"max\": 1, \"online\": 0}}, \"favicon\": \"{}\", \"description\": {{\"text\": \"{}\"}}}}", version, protocol, favicon, description);

    let mut packet = Cursor::new(Vec::<u8>::new());

    // Packet ID (0x00)
    packet.write_all(to_varint(0)?.as_slice()).await?;

    // Payload length
    packet
        .write_all(to_varint(payload.len())?.as_slice())
        .await?;

    // Payload
    packet.write_all(payload.as_bytes()).await?;

    Ok(packet.get_ref().clone())
}
