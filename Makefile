API_DEFINITION := api/openapi.yaml
FE_API_DIR := frontend/src/api

.PHONY: up down build clean_build logs exec-backend exec-frontend generate-fe-api generate-be-api clean

up:
	docker compose up --build

down:
	docker compose down

build: generate-fe-api generate-be-api
	docker compose build

clean-build:
	docker compose build --no-cache

logs:
	docker compose logs -f

exec-backend:
	docker compose exec tascal-backend bash #バックエンドコンテナ内でbashを実行

exec-frontend:
	docker compose exec tascal-frontend bash #フロントエンドコンテナ内でbashを実行

clean:
	@echo "Cleaning output directory..."
	rm -rf $(FE_API_DIR)
	rm -rf frontend/dist
	rm -rf backend/target

generate-fe-api:
	@echo "Generating frontend API code..."
	docker run --rm \
		--platform linux/amd64 \
		-v $(shell pwd):/local \
		openapitools/openapi-generator-cli generate \
		-i /local/$(API_DEFINITION) \
		-g typescript-axios \
		-o /local/$(FE_API_DIR) \
		--api-package api \
		--model-package model \
		--additional-properties withInterfaces=true,withSeparateModelsAndApi=true

generate-be-api:
	@echo "Generating backend API code..."
	docker run --rm \
		--platform linux/amd64 \
		-v $(shell pwd):/local \
		openapitools/openapi-generator-cli generate \
		-i /local/$(API_DEFINITION) \
		-g rust-axum \
		-o /local/backend/api-server/openapi \
		--api-package api \
		--model-package model \
		--additional-properties withInterfaces=true,withSeparateModelsAndApi=true
