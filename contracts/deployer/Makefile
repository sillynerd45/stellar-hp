.PHONY: all build_deployer build_contract deploy_deployer invoke_deployer

all: deploy_deployer

# Step 1: Build contract deployer
build_deployer:
	@echo "----------------------------------------"
	@echo "Building Stellar Contract Deployer"
	@echo "----------------------------------------"
	@stellar contract build
	@if [ $$? -ne 0 ]; then \
        echo "Stellar Build Contract Deployer failed!"; \
        exit 1; \
    fi

# Step 2: Build contract
build_contract: build_deployer
	@echo "----------------------------------------"
	@echo "Building Stellar Contract"
	@echo "----------------------------------------"
	@cd ../stellar-hp/ && stellar contract build
	@if [ $$? -ne 0 ]; then \
        echo "Stellar Build Contract failed!"; \
        exit 1; \
    fi
	@cd ../../ && ls -l target/wasm32-unknown-unknown/release/*.wasm

# Step 3: Deploy the contract deployer
deploy_deployer: build_contract
	@echo "----------------------------------------"
	@echo "Deploying Stellar Contract Deployer"
	@echo "----------------------------------------"
	@cd ../../ && stellar contract deploy \
		--wasm target/wasm32-unknown-unknown/release/deployer.wasm \
		--source plato \
        --network testnet \
        --alias stellar_hp_deployer \
        -- \
        --admin GB3NWHOV6POCYEVH6B74V5KM6A22T4J36Z5EMEHHGP6SGTMCD7SEVQGM
	@if [ $$? -ne 0 ]; then \
        echo "Stellar Deploy Contract Deployer failed"; \
        exit 1; \
    fi

invoke_deployer:
	@echo "----------------------------------------"
	@echo "Invoking Contract Deployer"
	@echo "----------------------------------------"
	@stellar contract alias remove stellar_hp
	@RANDOM_SALT=$$(openssl rand -base64 12 | tr -dc '0-9' | head -c 10) && WASM_HASH=${INPUT_VAR} && \
	CONTRACT_ID=$$(stellar contract invoke \
		--id stellar_hp_deployer \
  		--source plato \
		--network testnet \
	  	-- \
	  	deploy \
		--salt $$RANDOM_SALT \
		--wasm_hash $$WASM_HASH \
	  	--constructor_args '[{"address":"GAN2QRLHTKDIFZYU3GSQ3AYSJDGF6H6KUWCUPH2PXJGDVVKONGZXPPNG"}]') \
	&& UNQUOTED_ID=$$(echo "$$CONTRACT_ID" | tr -d '"') && stellar contract alias add --id $$UNQUOTED_ID stellar_hp