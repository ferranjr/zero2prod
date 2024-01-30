use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use secrecy::ExposeSecret;
use zero2prod::configuration::get_configuration;
use zero2prod::{startup, telemetry};
use zero2prod::domain::SubscriberEmail;
use zero2prod::email_client::EmailClient;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    let sender_email = configuration.email_client.sender()
        .expect("Invalid sender email address.");

    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token
    );

    startup::run(
        listener,
        connection_pool,
        email_client,
    )?.await
}
