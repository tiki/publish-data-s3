build:
	@sed -i -E '/tiki-private-ingest/s/features = \[[^]]*\]/features = \["$(schema)"\]/' Cargo.toml
	@cargo build
	mv Cargo.toml-E Cargo.toml

test:
	@sed -i -E '/tiki-private-ingest/s/features = \[[^]]*\]/features = \["$(schema)"\]/' Cargo.toml
	@cargo test
	mv Cargo.toml-E Cargo.toml

clean:
	rm Cargo.toml-E
	rm -rf target
