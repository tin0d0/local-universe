use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum LocalUniverseInstruction {
    // Dimension
    Scan = 0,

    // Drill (global clock + processing)
    Tick = 10,
    Excavate = 11,

    // Miner
    Deploy = 20,
    Checkpoint = 21,
    ClaimLUXITE = 22,
    ClaimSOL = 23,
    Close = 24,

    // Staker
    Deposit = 30,
    Withdraw = 31,
    ClaimYield = 32,
    CompoundYield = 33,

    // Automation
    Automate = 40,
    ReloadSOL = 41,

    // Admin
    Initialize = 100,
    SetAdmin = 101,
    Buyback = 102,
    Wrap = 103,
    FundTreasury = 104,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Scan {
    pub dimension_id: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Tick {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Excavate {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Deploy {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Checkpoint {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ClaimLUXITE {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ClaimSOL {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Close {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Deposit {
    pub amount: [u8; 8],
    pub compound_fee: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Withdraw {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ClaimYield {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CompoundYield {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Automate {
    pub dimension_id: [u8; 8],
    pub amount: [u8; 8],
    pub deposit: [u8; 8],
    pub fee: [u8; 8],
    pub reload: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ReloadSOL {
    pub dimension_id: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetAdmin {
    pub admin: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Buyback {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Wrap {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct FundTreasury {
    pub amount: [u8; 8],
}

instruction!(LocalUniverseInstruction, Scan);
instruction!(LocalUniverseInstruction, Tick);
instruction!(LocalUniverseInstruction, Excavate);
instruction!(LocalUniverseInstruction, Deploy);
instruction!(LocalUniverseInstruction, Checkpoint);
instruction!(LocalUniverseInstruction, ClaimLUXITE);
instruction!(LocalUniverseInstruction, ClaimSOL);
instruction!(LocalUniverseInstruction, Close);
instruction!(LocalUniverseInstruction, Deposit);
instruction!(LocalUniverseInstruction, Withdraw);
instruction!(LocalUniverseInstruction, ClaimYield);
instruction!(LocalUniverseInstruction, CompoundYield);
instruction!(LocalUniverseInstruction, Automate);
instruction!(LocalUniverseInstruction, ReloadSOL);
instruction!(LocalUniverseInstruction, Initialize);
instruction!(LocalUniverseInstruction, SetAdmin);
instruction!(LocalUniverseInstruction, Wrap);
instruction!(LocalUniverseInstruction, Buyback);
instruction!(LocalUniverseInstruction, FundTreasury);
