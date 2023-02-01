use zero2prod::serve;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    serve()?.await
}
