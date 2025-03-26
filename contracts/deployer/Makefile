.PHONY: all build_deployer build_contract deploy_deployer deploy_contract

all: deploy_contract

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

# Step 3: Deploy contract deployer
deploy_deployer: build_contract
	@echo "----------------------------------------"
	@echo "Deploying Stellar Contract Deployer"
	@echo "----------------------------------------"
	@cd ../../ && stellar contract deploy \
		--wasm target/wasm32-unknown-unknown/release/deployer.wasm \
		--source deployer_admin \
        --network testnet \
        --alias stellar_hp_deployer \
        -- \
        --admin GBOG7YYS2NSOFBAQPDX4T3HLFO4LIOPB3VVP3BWWIMRANQND2HS4HCJJ
	@if [ $$? -ne 0 ]; then \
        echo "Stellar Deploy Contract Deployer failed"; \
        exit 1; \
    fi

# Step 4: Deploy contract
deploy_contract: deploy_deployer
	@echo "----------------------------------------"
	@echo "Deploying Stellar Contract"
	@echo "----------------------------------------"
	@cd ../../.stellar/contract-ids && rm -rf stellar_hp.json
	@cd ../../ && WASM_HASH=$$(stellar contract upload \
  		--source deployer_admin \
		--network testnet \
	  	--wasm target/wasm32-unknown-unknown/release/stellar_hp.wasm) \
	&& RANDOM_SALT=$$(openssl rand -base64 12 | tr -dc '0-9' | head -c 10) \
	&& CONTRACT_ID=$$(stellar contract invoke \
    		--id stellar_hp_deployer \
      		--source deployer_admin \
    		--network testnet \
    	  	-- \
    	  	deploy \
    		--salt $$RANDOM_SALT \
    		--wasm_hash $$WASM_HASH \
    	  	--constructor_args '[{"address":"GB5Z2MAR3DQFTTDPKQEVR3KSVR22UQXHVOOG2MPZ7OS6FCZ4ZRX4SVGL"}]') \
    && UNQUOTED_ID=$$(echo "$$CONTRACT_ID" | tr -d '"') \
	&& stellar contract alias add --id $$UNQUOTED_ID stellar_hp