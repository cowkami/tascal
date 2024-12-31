API_DEFINITION := api/openapi.yaml
FE_API_DIR := frontend/src/api
FE_API_CLIENT := typescript-axios

.PHONY: up down build clean_build logs exec-backend exec-frontend generate-client clean openapi-validate

up:
	docker compose up -d --build

down:
	docker compose down

build:
	docker compose build

clean-build:
	docker compose build --no-cache

logs:
	docker compose logs -f

exec-backend:
	docker compose exec backend bash #バックエンドコンテナ内でbashを実行

exec-frontend:
	docker compose exec frontend bash #フロントエンドコンテナ内でbashを実行

clean:
	@echo "Cleaning output directory..."
	rm -rf $(FE_API_DIR)
	rm -rf frontend/dist

generate-client:
	@echo "Generating client code..."
	docker run --rm \
		-v $(shell pwd):/local \
		openapitools/openapi-generator-cli generate \
		-i /local/$(API_DEFINITION) \
		-g $(FE_API_CLIENT) \
		-o /local/$(FE_API_DIR) \
		--api-package api \
		--model-package model \
		--additional-properties withInterfaces=true,withSeparateModelsAndApi=true

openapi-validate:
	@echo "Validating OpenAPI spec..."
	docker run --rm \
		-v $(shell pwd):/local \
		stoplight/spectral lint /local/$(API_DEFINITION)