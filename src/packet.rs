use tokio::io::Result;

pub const CONTINUE_BITS: u64 = 0x7F;
pub const SEG_BITS: u64 = 0x80;

/// Convert an integer to varint bytes
pub fn to_varint(int: usize) -> Result<Vec<u8>> {
    let mut buffer = [0; 5]; // varint size is 5 bytes
    let mut written = 0;

    let mut int = (int as u64) & 0xFFFF_FFFF; // 2^32

    loop {
        let temp = (int & CONTINUE_BITS) as u8;

        int >>= 7;

        buffer[written] = {
            if int != 0 {
                temp | SEG_BITS as u8
            } else {
                temp
            }
        };

        written += 1;

        if int == 0 {
            break;
        }
    }

    Ok(buffer[0..written].to_vec())
}

/// Craft the SLP packet
///
/// See:
///     - wiki.vg/Server_List_Ping
///     - wiki.vg/Protocol
///
pub fn build_packet(
    protocol: isize,
    description: &str,
    version: &str,
    favicon: &str,
) -> Result<Vec<u8>> {
    let payload = format!(
        "{{\"version\": {{\"name\": \"{}\", \"protocol\": {}}}, \"players\": {{\"max\": 1, \"online\": 0}}, \"favicon\": \"{}\", \"description\": {{\"text\": \"{}\"}}}}",
        version,
        protocol,
        favicon,
        description
    );

    if payload.len() > 32767 {
        panic!("Payload to big")
    }

    let mut pkt = Vec::<u8>::new();

    // Packet ID (0x00)
    pkt.append(&mut to_varint(0)?);

    // Payload length
    pkt.append(&mut to_varint(payload.len())?);

    // Payload
    pkt.append(&mut payload.as_bytes().to_vec());

    Ok(pkt)
}
