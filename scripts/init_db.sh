#!/usr/bin/env bash

source .env
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.7 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi
>&2 echo "Postgres is up and running on port ${POSTGRES_HOST_PORT} - running migrations now!"

export DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_HOST_PORT}/${POSTGRES_DB}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
