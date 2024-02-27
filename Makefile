ENV := .env
include $(ENV)

DATABASE_URL := "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_HOST_PORT}/${POSTGRES_DB}"
#DATABASE_URL := "postgresql://newsletter:AVNS_6LUD6MprriwPu54_bvj@app-aa418121-9a11-441c-a7ab-8723cc69a6fc-do-user-12139927-0.c.db.ondigitalocean.com:25060/newsletter?sslmode=require"
DOCKER_IMAGE_NAME :="zero2prod"

install:
	cargo install --version=0.7 sqlx-cli --no-default-features --features postgres
	cargo install cargo-udeps
	cargo install bunyan

db-url:
	export DATABASE_URL=$(DATABASE_URL)

db-create:
	DATABASE_URL=$(DATABASE_URL) && sqlx database create

db-migrate:
	export DATABASE_URL=$(DATABASE_URL) && ./scripts/init_db.sh

test-verbose:
	TEST_LOG=true cargo test | bunyan


docker-build:
	docker build --tag $(DOCKER_IMAGE_NAME) --file Dockerfile .

docker-run:
	docker run -p 8000:8000 $(DOCKER_IMAGE_NAME)

docker-compose:
	docker-compose up --detach

sqlx-prepare:
	sqlx prepare -- --lib

APP_ID := $$(doctl apps list | awk 'NR==2 {print $1}')
app-create:
	doctl apps create --spec spec.yaml

app-delete:
	doctl apps delete $(APP_ID)

app-status:
	doctl apps list

app-update:
	doctl apps update $(APP_ID) --spec spec.yaml
