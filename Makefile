.PHONY: up down build run

up:
	docker-compose up -d

down:
	docker-compose down

build:
	cargo build

run:
	cargo run
