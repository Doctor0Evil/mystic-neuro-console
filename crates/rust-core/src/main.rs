mod command;
mod role;
mod scope;
mod backend;
mod server;

#[tokio::main]
async fn main() {
    if let Err(e) = server::run_server("127.0.0.1:8080").await {
        eprintln!("server error: {e}");
    }
}
