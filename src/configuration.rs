use crate::domain::SubscriberEmail;
use crate::email_client;
use clap::Parser;
use config;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use std::convert::{TryFrom, TryInto};

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        return match self {
            Environment::Local => "local",
            Environment::Production => "production",
        };
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        return match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        };
    }
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub database: Database,
    pub application: Application,
    pub email_client: EmailClient,
    pub redis_uri: Secret<String>,
}

#[derive(Deserialize, Parser, Debug, Clone)]
pub struct Database {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl Database {
    pub fn with_db(&self) -> PgConnectOptions {
        return self
            .without_db()
            .database(&self.database_name)
            .log_statements(tracing::log::LevelFilter::Trace);
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        return PgConnectOptions::new()
            .host(self.host.as_str())
            .username(self.username.as_str())
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode);
    }
}

#[derive(Deserialize, Parser, Debug, Clone)]
pub struct Application {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub hmac_secret: Secret<String>,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    return settings.try_deserialize::<Settings>();
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailClient {
    pub base_url: String,
    pub sender_email: String,
    pub authorization_token: Secret<String>,
    pub timeout_milliseconds: u64,
}

impl EmailClient {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        return SubscriberEmail::parse(self.sender_email.clone());
    }

    pub fn timeout(&self) -> std::time::Duration {
        return std::time::Duration::from_millis(self.timeout_milliseconds);
    }

    pub fn client(self) -> email_client::EmailClient {
        let sender_email = self.sender().expect("Invalid sender email address");
        let timeout = self.timeout();

        return email_client::EmailClient::new(
            self.base_url,
            sender_email,
            self.authorization_token,
            timeout,
        );
    }
}
