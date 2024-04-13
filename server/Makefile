.PHONY: server ws build build-dev build-prod run-dev run-prod up-dev up-prod

ws:
	@echo "Connecting to the WebSocket server..."
	@websocat ws://127.0.0.1:8080

# Build all targets defined in docker-bake.hcl
build:
	docker buildx bake --set *.tags=cryptogame:${TAG}

# Build the environment image
build-dev:
	docker buildx bake development --set development.tags=cryptogame:dev-${TAG}
build-prod:
	docker buildx bake production --set production.tags=cryptogame:prod-${TAG}

# Run the environment container
run-dev:
	docker run --rm -v "$(PWD):/usr/src/cryptogame" -p 8080:8080 cryptogame:dev-${TAG}
run-prod:
	docker run --rm -p 8080:8080 cryptogame:prod-${TAG}

# Shortcuts to build and run environments
up-dev: build-dev run-dev
up-prod: build-prod run-prod
