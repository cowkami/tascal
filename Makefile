.PHONY: up down build clean_build logs exec-backend exec-frontend

up:
	docker compose up -d --build

down:
	docker compose down

build:
	docker compose build

clean_build:
	docker compose build --no-cache

logs:
	docker compose logs -f

exec-backend:
	docker compose exec backend bash #バックエンドコンテナ内でbashを実行

exec-frontend:
	docker compose exec frontend bash #フロントエンドコンテナ内でbashを実行