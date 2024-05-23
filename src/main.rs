use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration file.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let error_msg = format!(
        "Failed to bind to port {}",
        configuration.application_port
    );
    let listener = TcpListener::bind(address).expect(&error_msg);

    run(listener)?.await
}
