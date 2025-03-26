include Makefile.invoke

default: deploy

all: test

test: deploy
	cargo test

build:
	@echo "------------------------------"
	@echo "Building Stellar Contract"
	@echo "------------------------------"
	@stellar contract build
	@if [ $$? -ne 0 ]; then \
        echo "Stellar Contract Build failed!"; \
        exit 1; \
    fi
	@cd ../../ && ls -l target/wasm32-unknown-unknown/release/*.wasm

deploy: build
	@echo "------------------------------"
	@echo "Deploying Stellar Contract"
	@echo "------------------------------"
	@cd ../../ && stellar contract deploy \
		--wasm target/wasm32-unknown-unknown/release/stellar_hp.wasm \
		--source plato \
        --network testnet \
        --alias stellar_hp
	@if [ $$? -ne 0 ]; then \
        echo "Stellar Contract Deploy failed"; \
        exit 1; \
    fi

fmt:
	cargo fmt --all

clean:
	cargo clean