pub mod protos;
pub mod server;

fn main() -> std::io::Result<()> {
    println!(
        "spec: {}",
        "JAoKCggKBghkEGQgBAoOCgoSCAhkEGQYMiAyEAEKBgoCGgAQAg"
    );
    server::start_server()
}
