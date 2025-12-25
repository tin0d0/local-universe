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
- [`Excavate`](program/src/excavate.rs) – Processes a drill for the current tick, determining if it hit LUXITE.

#### Mining

- [`Deploy`](program/src/deploy.rs) – Deploys SOL to a drill on a dimension.
- [`ClaimLUXITE`](program/src/claim_luxite.rs) – Claims pending LUXITE mining rewards.

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
- [`Drill`](api/src/state/drill.rs) – A drill deployed on a dimension, tracking mining activity.
- [`Navigator`](api/src/state/navigator.rs) – A user's global profile across all dimensions.
- [`Miner`](api/src/state/miner.rs) – A user's mining position on a specific drill.
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
