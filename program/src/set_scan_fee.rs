use steel::*;
use localuniverse_api::{
    consts::*,
    instruction::*,
    state::*,
    error::*,
};

/// Sets the scan fee (admin only).
pub fn process_set_scan_fee(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = SetScanFee::try_from_bytes(data)?;
    let new_fee = u64::from_le_bytes(args.scan_fee);

    let [signer_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    let config = config_info
        .as_account_mut::<Config>(&localuniverse_api::ID)?
        .assert_mut_err(
            |c| c.admin == *signer_info.key,
            LocalUniverseError::NotAuthorized.into(),
        )?;

    config.scan_fee = new_fee;

    Ok(())
}
