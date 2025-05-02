use bacon_repro::run_server;

#[tokio::test]
async fn test_server() {
    println!("Starting server");
    run_server().await;
}
