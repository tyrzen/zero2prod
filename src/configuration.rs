use clap::Parser;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub database: Database,
    pub app: App,
}

#[derive(Deserialize, Parser, Debug)]
pub struct Database {
    #[clap(env = "POSTGRES_USER", default_value = "postgres")]
    pub username: String,

    #[clap(env = "POSTGRES_PASSWORD", default_value = "postgres")]
    pub password: Secret<String>,

    #[clap(env = "POSTGRES_HOST_PORT", default_value = "4325")]
    pub port: u16,

    #[clap(env = "POSTGRES_HOST", default_value = "0.0.0.0")]
    pub host: String,

    #[clap(env = "POSTGRES_DB", default_value = "newsletter")]
    pub database_name: String,
}

impl Database {
    pub fn connection_string(&self) -> Secret<String> {
        return Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ));
    }

    pub fn connection_string_without_db(&self) -> String {
        return format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        );
    }
}

#[derive(Deserialize, Parser, Debug)]
pub struct App {
    #[clap(env = "APP_PORT", default_value = "8000")]
    pub application_port: u16,
}

pub fn get_configuration() -> Settings {
    dotenv::dotenv().ok();
    let database = Database::parse();
    let app = App::parse();

    return Settings { database, app };
}

pub fn set_database_url() {
    dotenv::dotenv().ok();
    let database = Database::parse();
    std::env::set_var("DATABASE_URL", database.connection_string().expose_secret());
}
