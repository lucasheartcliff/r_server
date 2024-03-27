use server::Server;

mod http;
mod server;

fn main() {
    let address = "0.0.0.0:8080".to_string();
    let server = Server::new(address);
    server.run();
}
