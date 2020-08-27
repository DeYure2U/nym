mod connection;
mod controller;
mod core;
mod websocket;

#[tokio::main]
async fn main() {
    println!("Starting socks5 service provider:");
    let mut server = core::ServiceProvider::new();
    server.run().await;
}
