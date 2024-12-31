API_DEFINITION := api/openapi.yaml
FE_API_DIR := frontend/src/api
FE_API_CLIENT := typescript-axios

.PHONY: up down build clean_build logs exec-backend exec-frontend generate-client clean

up:
	docker compose up -d

down:
	docker compose down

build: generate-client
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

generate-client:
	@echo "Generating client code..."
	docker run --rm \
		--platform linux/amd64 \
		-v $(shell pwd):/local \
		openapitools/openapi-generator-cli generate \
		-i /local/$(API_DEFINITION) \
		-g $(FE_API_CLIENT) \
		-o /local/$(FE_API_DIR) \
		--api-package api \
		--model-package model \
		--additional-properties withInterfaces=true,withSeparateModelsAndApi=true
