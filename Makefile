
run_ch0_server:
	cargo build && cargo run -- ch0-server  --port 3000

run_ch0_client:
	cargo build && cargo run -- ch0-client  --port 3000