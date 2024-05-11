
run_ch0_server_v1:
	cargo build && cargo run -- ch0-server  --port 3000 v1
run_ch0_server_v2:
	cargo build && cargo run -- ch0-server  --port 3000 v2


run_ch0_client_v1:
	cargo build && cargo run -- ch0-client  --port 3000 v1
run_ch0_client_v2:
	cargo build && cargo run -- ch0-client  --port 3000 v2