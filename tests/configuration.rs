use secrecy::ExposeSecret;
use zero2prod::configuration::get_configuration;

#[test]
fn database_url() {
    let config = get_configuration();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    assert_eq!(
        "postgres://postgres:password@localhost:4325/newsletter",
        url
    );
    assert_eq!(
        "postgres://postgres:password@localhost:4325/newsletter",
        config.database.connection_string().expose_secret()
    );
    assert_eq!(
        "postgres://postgres:password@localhost:4325",
        config.database.connection_string_without_db()
    );
}
