.PHONY: up-client up-server

up-client:
	@echo "Starting client..."
	@$(MAKE) -C client up-dev

up-server:
	@echo "Starting server..."
	@$(MAKE) -C server up-dev
