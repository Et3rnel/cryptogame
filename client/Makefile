.PHONY: build-dev build-prod run-dev run-prod up-dev up-prod

TAG ?= latest

# Build the development Docker image
build-dev:
	docker buildx bake client-development --set client-development.tags=svelte-client:dev-${TAG}

# Build the production Docker image
build-prod:
	docker buildx bake client-production --set client-production.tags=svelte-client:prod-${TAG}

# Run the development container with volume mapping for hot reloading
run-dev:
	docker run --rm \
		-v "$(PWD):/app" \
		-v "/app/node_modules" \
		-p 5173:5173 \
		svelte-client:dev-${TAG}

# Run the production container
run-prod:
	docker run --rm -d -p 80:80 svelte-client:prod-${TAG}

# Shortcuts to build and run environments
up-dev: build-dev run-dev
up-prod: build-prod run-prod
