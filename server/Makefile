.PHONY: server websocket

server:
	@echo "Launching Rust server..."
	@cargo run

websocket:
	@echo "Connecting to the WebSocket server..."
	@websocat ws://127.0.0.1:8080
