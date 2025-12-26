# LOCAL UNIVERSE

Local Universe is a dimension mining protocol on Solana.

## API

- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Event`](api/src/event.rs) – Custom program events.
- [`Instruction`](api/src/instruction.rs) – Declared instructions and arguments.
- [`SDK`](api/src/sdk.rs) – Client-side instruction builders.

## Instructions

#### Dimension

- [`Scan`](program/src/scan.rs) – Scans a new dimension, revealing its richness score.

#### Drill

- [`Tick`](program/src/tick.rs) – Advances the global tick.
- [`Excavate`](program/src/excavate.rs) – Processes an excavation for the previous tick, determining hit or miss.

#### Mining

- [`Deploy`](program/src/deploy.rs) – Deploys SOL to a dimension's excavation.
- [`Checkpoint`](program/src/checkpoint.rs) – Claims rewards from a processed excavation.
- [`ClaimSOL`](program/src/claim_sol.rs) – Claims pending SOL rewards.
- [`ClaimLUXITE`](program/src/claim_luxite.rs) – Claims pending LUXITE mining rewards.
- [`Close`](program/src/close.rs) – Closes an expired excavation and reclaims rent.

#### Automation

- [`Automate`](program/src/automate.rs) – Sets up automation for hands-free mining.
- [`ReloadSOL`](program/src/reload_sol.rs) – Reloads SOL winnings back into automation balance.

#### Staking

- [`Deposit`](program/src/deposit.rs) – Deposits LUXITE into a stake account.
- [`Withdraw`](program/src/withdraw.rs) – Withdraws LUXITE from a stake account.
- [`ClaimYield`](program/src/claim_yield.rs) – Claims staking yield.
- [`CompoundYield`](program/src/compound_yield.rs) – Auto-compounds staking yield (bot callable).

#### Admin

- [`Initialize`](program/src/initialize.rs) – Initializes program accounts.
- [`SetAdmin`](program/src/set_admin.rs) – Re-assigns the admin authority.
- [`FundTreasury`](program/src/fund_treasury.rs) – Funds the treasury with LUXITE for emissions.
- [`Wrap`](program/src/wrap.rs) – Wraps SOL in the treasury for swap transactions.
- [`Buyback`](program/src/buyback.rs) – Swaps WSOL for LUXITE, burns 90%, distributes 10% to stakers.

## State

- [`Config`](api/src/state/config.rs) – Global program configuration.
- [`Grid`](api/src/state/grid.rs) – Tracks the current tick and timestamps.
- [`Dimension`](api/src/state/dimension.rs) – A discovered dimension with its richness score.
- [`Drill`](api/src/state/drill.rs) – Global stats for a dimension's mining activity.
- [`Excavation`](api/src/state/excavation.rs) – A single tick's mining event on a dimension.
- [`Navigator`](api/src/state/navigator.rs) – A user's global profile across all dimensions.
- [`Miner`](api/src/state/miner.rs) – A user's mining position on a specific dimension.
- [`Automation`](api/src/state/automation.rs) – Automation settings for hands-free mining.
- [`Stake`](api/src/state/stake.rs) – Manages a user's staking activity.
- [`Treasury`](api/src/state/treasury.rs) – Manages LUXITE emissions, buybacks, and burns.

## Tests

To run the test suite, use the Solana toolchain:
```
cargo test-sbf
```

For line coverage, use llvm-cov:
```
cargo llvm-cov
```
