use minecraft_protocol::ServerAddress;
use tokio::runtime::Runtime;

async fn bot() {
    let address = ServerAddress::parse(&"mc.hypixel.net".to_string()).unwrap();
    minecraft_protocol::server_status_pinger::ping_server(&address)
        .await
        .unwrap();
}

fn main() {
    println!("Hello, world!");

    let io_loop = Runtime::new().unwrap();
    io_loop.block_on(bot());
}
