SERVER_PORT := 3000

# Run Server 
ch0_server_v1:
	cargo build && cargo run -- ch0-server  --port $(SERVER_PORT) echo-v1
ch0_server_v2:
	cargo build && cargo run -- ch0-server  --port $(SERVER_PORT) echo-v2


# Run Client 
ch0_client_v1:
	cargo build && cargo run -- ch0-client  --port $(SERVER_PORT) v1
ch0_client_v2:
	cargo build && cargo run -- ch0-client  --port $(SERVER_PORT) v2