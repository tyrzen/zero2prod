ENV := .env
include $(ENV)

DATABASE_URL := "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_HOST_PORT}/${POSTGRES_DB}"

install:
	cargo install --version=0.7 sqlx-cli --no-default-features --features postgres
	cargo install cargo-udeps
	cargo install bunyan

db-create:
	export export DATABASE_URL=$(DATABASE_URL) && sqlx database create

db-migrate:
	export export DATABASE_URL=$(DATABASE_URL) && sqlx migrate run

test-verbose:
	TEST_LOG=true cargo test | bunyan