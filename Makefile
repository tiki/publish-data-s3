THIS_FILE := $(lastword $(MAKEFILE_LIST))

build:
	@$(MAKE) -f $(THIS_FILE) clean
	@mkdir out && cp -r src out/src
	@sed -E '/tiki-private-ingest/s/features = \[[^]]*\]/features = \["$(schema)"\]/' < Cargo.toml > out/Cargo.toml
	@cargo build --manifest-path out/Cargo.toml

clean:
	rm -rf out
