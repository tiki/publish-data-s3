THIS_FILE := $(lastword $(MAKEFILE_LIST))

build:
	@$(MAKE) -f $(THIS_FILE) clean
	@mkdir out && cp -r src out/src
	@sed -E '/tiki-private-ingest/s/features = \[[^]]*\]/features = \["$(schema)"\]/' < Cargo.toml > out/Cargo.toml
	@cp -r infra/lambda out/infra
	@sed -E 's/__QUEUE_ARN__/$(queue)/g; s/__V1__/$(v1)/g' < infra/lambda/samconfig.toml > out/infra/samconfig.toml
	@cargo build --manifest-path out/Cargo.toml
	@cd out/infra && sam build

clean:
	rm -rf out

deploy:
	@cd out/infra && sam build && sam package && sam deploy --stack-name ingest-s3-$(schema) --parameter-overrides QueueArn=$(queue) KeyPrefix=$(filter) BucketName=$(bucket) TableName=$(table) FileType=$(type) Compression=$(compression)
