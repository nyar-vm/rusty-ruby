//! Ruby LSP server entry point

use ruby_lsp::start_server;

#[tokio::main]
async fn main() {
    println!("Starting Ruby LSP server...");
    start_server().await;
}
