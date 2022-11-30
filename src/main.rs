use clap::Parser;

mod server;

#[derive(clap::Parser)]
struct Arguments {
    host: String,
    port: u16,

    #[arg(short, long, default_value_t = String::from("Hello, world!"))]
    description: String,

    #[arg(short, long, default_value_t = String::from(""))]
    favicon: String,

    #[arg(short, long, default_value_t = String::from("PaperSpigot 1.8.8"))]
    version: String,
}

#[tokio::main]
async fn main() {
    println!(
        r"
pwncraft
+--
deliver leet payloads through
Minecraft SLP protocol.
+--
twitter.com/t0x00ast
        "
    );

    let args = Arguments::parse();

    server::Server::new(
        &args.host,
        args.port,
        &args.description,
        &args.favicon,
        &args.version,
    )
    .run()
    .await
    .unwrap()
}
