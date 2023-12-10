use std::io::stdin;

use waver::Registration;

#[tokio::main]
async fn main() {
    println!("Starting service...");
    let registration = waver::publish_service("test-service", waver::Protocol::Tcp, 7779).await.unwrap();
    println!("'{:?}' - '{:?}'", registration.info().name(), registration.info().domain());
    println!("Service started successfully. Hit enter to stop");

    let mut input = "".to_owned();
    stdin().read_line(&mut input).unwrap();

    println!("Stopping...");
}