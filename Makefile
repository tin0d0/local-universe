.PHONY: build test clean

# ============================================================================
# Config
# ============================================================================

KEYPAIR ?= ~/.config/solana/id.json
RPC_DEVNET = https://api.devnet.solana.com
RPC_MAINNET = https://api.mainnet-beta.solana.com

# Default to devnet
RPC ?= $(RPC_DEVNET)

CLI = KEYPAIR=$(KEYPAIR) RPC=$(RPC) ./target/release/localuniverse-cli

# ============================================================================
# Build
# ============================================================================

build:
	cargo build-sbf
	cargo build --release --package localuniverse-cli

build-cli:
	cargo build --release --package localuniverse-cli

build-program:
	cargo build-sbf

# ============================================================================
# Deploy
# ============================================================================

deploy-devnet:
	solana program deploy \
		--url devnet \
		target/deploy/localuniverse_program.so \
		--program-id LUXcEf35hqyZQEkUkSWjxk19YfsoeRf1AduPPyQvRBm.json

deploy-mainnet:
	solana program deploy \
		--url mainnet-beta \
		target/deploy/localuniverse_program.so \
		--program-id LUXcEf35hqyZQEkUkSWjxk19YfsoeRf1AduPPyQvRBm.json

# ============================================================================
# Devnet Commands
# ============================================================================

devnet-clock:
	@RPC=$(RPC_DEVNET) COMMAND=clock $(CLI)

devnet-config:
	@RPC=$(RPC_DEVNET) COMMAND=config $(CLI)

devnet-grid:
	@RPC=$(RPC_DEVNET) COMMAND=grid $(CLI)

devnet-treasury:
	@RPC=$(RPC_DEVNET) COMMAND=treasury $(CLI)

devnet-keys:
	@RPC=$(RPC_DEVNET) COMMAND=keys $(CLI)

devnet-dimension:
	@RPC=$(RPC_DEVNET) COMMAND=dimension ID=$(ID) $(CLI)

devnet-drill:
	@RPC=$(RPC_DEVNET) COMMAND=drill ID=$(ID) $(CLI)

devnet-excavation:
	@RPC=$(RPC_DEVNET) COMMAND=excavation ID=$(ID) TICK=$(TICK) $(CLI)

devnet-miner:
	@RPC=$(RPC_DEVNET) COMMAND=miner ID=$(ID) $(CLI)

devnet-navigator:
	@RPC=$(RPC_DEVNET) COMMAND=navigator $(CLI)

devnet-stake:
	@RPC=$(RPC_DEVNET) COMMAND=stake $(CLI)

devnet-initialize:
	@RPC=$(RPC_DEVNET) COMMAND=initialize $(CLI)

devnet-scan:
	@RPC=$(RPC_DEVNET) COMMAND=scan ID=$(ID) $(CLI)

devnet-deploy:
	@RPC=$(RPC_DEVNET) COMMAND=deploy ID=$(ID) AMOUNT=$(AMOUNT) $(CLI)

devnet-tick:
	@RPC=$(RPC_DEVNET) COMMAND=tick $(CLI)

devnet-excavate:
	@RPC=$(RPC_DEVNET) COMMAND=excavate ID=$(ID) $(CLI)

devnet-checkpoint:
	@RPC=$(RPC_DEVNET) COMMAND=checkpoint ID=$(ID) $(CLI)

devnet-close:
	@RPC=$(RPC_DEVNET) COMMAND=close ID=$(ID) TICK=$(TICK) $(CLI)

devnet-claim-luxite:
	@RPC=$(RPC_DEVNET) COMMAND=claim_luxite ID=$(ID) $(CLI)

devnet-claim-sol:
	@RPC=$(RPC_DEVNET) COMMAND=claim_sol ID=$(ID) $(CLI)

devnet-fund-treasury:
	@RPC=$(RPC_DEVNET) COMMAND=fund_treasury AMOUNT=$(AMOUNT) $(CLI)

devnet-automation:
	@RPC=$(RPC_DEVNET) COMMAND=automation ID=$(ID) $(CLI)

devnet-automate:
	@RPC=$(RPC_DEVNET) COMMAND=automate ID=$(ID) EXECUTOR=$(EXECUTOR) AMOUNT=$(AMOUNT) DEPOSIT=$(DEPOSIT) FEE=$(FEE) RELOAD=$(RELOAD) $(CLI)

devnet-automate-close:
	@RPC=$(RPC_DEVNET) COMMAND=automate ID=$(ID) EXECUTOR=11111111111111111111111111111111 $(CLI)

devnet-reload-sol:
	@RPC=$(RPC_DEVNET) COMMAND=reload-sol ID=$(ID) AUTHORITY=$(AUTHORITY) $(CLI)

# ============================================================================
# Mainnet Commands
# ============================================================================

mainnet-clock:
	@RPC=$(RPC_MAINNET) COMMAND=clock $(CLI)

mainnet-config:
	@RPC=$(RPC_MAINNET) COMMAND=config $(CLI)

mainnet-grid:
	@RPC=$(RPC_MAINNET) COMMAND=grid $(CLI)

mainnet-treasury:
	@RPC=$(RPC_MAINNET) COMMAND=treasury $(CLI)

mainnet-keys:
	@RPC=$(RPC_MAINNET) COMMAND=keys $(CLI)

mainnet-dimension:
	@RPC=$(RPC_MAINNET) COMMAND=dimension ID=$(ID) $(CLI)

mainnet-drill:
	@RPC=$(RPC_MAINNET) COMMAND=drill ID=$(ID) $(CLI)

mainnet-excavation:
	@RPC=$(RPC_MAINNET) COMMAND=excavation ID=$(ID) TICK=$(TICK) $(CLI)

mainnet-miner:
	@RPC=$(RPC_MAINNET) COMMAND=miner ID=$(ID) $(CLI)

mainnet-navigator:
	@RPC=$(RPC_MAINNET) COMMAND=navigator $(CLI)

mainnet-stake:
	@RPC=$(RPC_MAINNET) COMMAND=stake $(CLI)

mainnet-initialize:
	@RPC=$(RPC_MAINNET) COMMAND=initialize $(CLI)

mainnet-scan:
	@RPC=$(RPC_MAINNET) COMMAND=scan ID=$(ID) $(CLI)

mainnet-deploy:
	@RPC=$(RPC_MAINNET) COMMAND=deploy ID=$(ID) AMOUNT=$(AMOUNT) $(CLI)

mainnet-tick:
	@RPC=$(RPC_MAINNET) COMMAND=tick $(CLI)

mainnet-excavate:
	@RPC=$(RPC_MAINNET) COMMAND=excavate ID=$(ID) $(CLI)

mainnet-checkpoint:
	@RPC=$(RPC_MAINNET) COMMAND=checkpoint ID=$(ID) $(CLI)

mainnet-close:
	@RPC=$(RPC_MAINNET) COMMAND=close ID=$(ID) TICK=$(TICK) $(CLI)

mainnet-claim-luxite:
	@RPC=$(RPC_MAINNET) COMMAND=claim_luxite ID=$(ID) $(CLI)

mainnet-claim-sol:
	@RPC=$(RPC_MAINNET) COMMAND=claim_sol ID=$(ID) $(CLI)

mainnet-fund-treasury:
	@RPC=$(RPC_MAINNET) COMMAND=fund_treasury AMOUNT=$(AMOUNT) $(CLI)

mainnet-automation:
	@RPC=$(RPC_MAINNET) COMMAND=automation ID=$(ID) $(CLI)

mainnet-automate:
	@RPC=$(RPC_MAINNET) COMMAND=automate ID=$(ID) EXECUTOR=$(EXECUTOR) AMOUNT=$(AMOUNT) DEPOSIT=$(DEPOSIT) FEE=$(FEE) RELOAD=$(RELOAD) $(CLI)

mainnet-automate-close:
	@RPC=$(RPC_MAINNET) COMMAND=automate ID=$(ID) EXECUTOR=11111111111111111111111111111111 $(CLI)

mainnet-reload-sol:
	@RPC=$(RPC_MAINNET) COMMAND=reload-sol ID=$(ID) AUTHORITY=$(AUTHORITY) $(CLI)

# ============================================================================
# Test Flow (Devnet)
# ============================================================================

test-flow:
	@echo "=== 1. Initialize ==="
	@RPC=$(RPC_DEVNET) COMMAND=initialize $(CLI) || true
	@echo ""
	@echo "=== 2. Check Grid ==="
	@RPC=$(RPC_DEVNET) COMMAND=grid $(CLI)
	@echo ""
	@echo "=== 3. Scan Dimension 0 ==="
	@RPC=$(RPC_DEVNET) COMMAND=scan ID=0 $(CLI) || true
	@echo ""
	@echo "=== 4. Check Dimension ==="
	@RPC=$(RPC_DEVNET) COMMAND=dimension ID=0 $(CLI)
	@echo ""
	@echo "=== 5. Check Drill ==="
	@RPC=$(RPC_DEVNET) COMMAND=drill ID=0 $(CLI)

# ============================================================================
# Help
# ============================================================================

help:
	@echo "Local Universe CLI"
	@echo ""
	@echo "Build:"
	@echo "  make build          - Build program and CLI"
	@echo "  make build-cli      - Build CLI only"
	@echo "  make build-program  - Build program only"
	@echo ""
	@echo "Deploy:"
	@echo "  make deploy-devnet  - Deploy to devnet"
	@echo "  make deploy-mainnet - Deploy to mainnet"
	@echo ""
	@echo "Devnet Commands:"
	@echo "  make devnet-grid"
	@echo "  make devnet-treasury"
	@echo "  make devnet-dimension ID=0"
	@echo "  make devnet-drill ID=0"
	@echo "  make devnet-excavation ID=0 TICK=1"
	@echo "  make devnet-miner ID=0"
	@echo "  make devnet-scan ID=0"
	@echo "  make devnet-deploy ID=0 AMOUNT=100000000"
	@echo "  make devnet-tick"
	@echo "  make devnet-excavate ID=0"
	@echo "  make devnet-checkpoint ID=0"
	@echo "  make devnet-claim-sol ID=0"
	@echo "  make devnet-claim-luxite ID=0"
	@echo "  make devnet-close ID=0 TICK=1"
	@echo ""
	@echo "Mainnet Commands:"
	@echo "  (same as devnet, replace devnet- with mainnet-)"
	@echo ""
	@echo "Test:"
	@echo "  make test-flow      - Run test flow on devnet"
