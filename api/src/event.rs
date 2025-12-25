use serde::{Deserialize, Serialize};
use steel::*;

pub enum LocalUniverseEvent {
    Scan = 0,
    Deploy = 1,
    Tick = 2,
    Excavate = 3,
    Buyback = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct ScanEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The dimension ID.
    pub dimension_id: u64,

    /// The wallet that scanned.
    pub scanner: Pubkey,

    /// The richness score generated.
    pub richness: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct DeployEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The dimension ID.
    pub dimension_id: u64,

    /// The authority of the deployer.
    pub authority: Pubkey,

    /// The signer of the deployer.
    pub signer: Pubkey,

    /// The amount of SOL deployed.
    pub amount: u64,

    /// The tick ID.
    pub tick_id: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct TickEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The new tick ID.
    pub tick_id: u64,

    /// The start slot of this tick.
    pub start_slot: u64,

    /// The end slot of this tick.
    pub end_slot: u64,

    /// The epoch ID.
    pub epoch_id: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct ExcavateEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The dimension ID.
    pub dimension_id: u64,

    /// The tick ID this result is for.
    pub tick_id: u64,

    /// The richness score of the dimension.
    pub richness: u64,

    /// The total LUXITE distributed.
    pub luxite_distributed: u64,

    /// The total SOL deployed on this drill.
    pub total_deployed: u64,

    /// The number of miners on this drill.
    pub miner_count: u64,

    /// The new depth of the drill.
    pub depth: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct BuybackEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The amount of LUXITE burned.
    pub luxite_burned: u64,

    /// The amount of LUXITE shared with stakers.
    pub luxite_shared: u64,

    /// The amount of SOL swapped.
    pub sol_amount: u64,

    /// The new circulating supply of LUXITE.
    pub new_circulating_supply: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

event!(ScanEvent);
event!(DeployEvent);
event!(TickEvent);
event!(ExcavateEvent);
event!(BuybackEvent);
