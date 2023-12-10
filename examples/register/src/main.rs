use std::io::stdin;

use qmulti::Registration;

#[tokio::main]
async fn main() {
    println!("Starting service...");
    let registration = qmulti::publish_service("test-service", qmulti::Protocol::Tcp, 7779).await.unwrap();
    println!("'{:?}' - '{:?}'", registration.info().name(), registration.info().domain());
    println!("Service started successfully. Hit enter to stop");

    let mut input = "".to_owned();
    stdin().read_line(&mut input).unwrap();

    println!("Stopping...");
}