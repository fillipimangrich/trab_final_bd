DB_DOCKER_CONTAINER=trab_final_postgres_container

install:
	cargo install sqlx-cli

build:
	cargo build

create_migrations:
	sqlx migrate add -r init

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert
	
stop_containers:
	docker stop $$(docker ps -q)


create_docker_container:
	docker run --name ${DB_DOCKER_CONTAINER} -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:12-alpine

create_postgres_db:
	docker exec -it ${DB_DOCKER_CONTAINER} createdb --username=root --owner=root trabfinalbddb

start_docker_db:
	docker start ${DB_DOCKER_CONTAINER}

run:
	cargo run



