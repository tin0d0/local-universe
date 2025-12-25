use steel::*;
use localuniverse_api::{
    consts::*,
    instruction::*,
    state::*,
    error::*,
};

/// Sets the admin.
pub fn process_set_admin(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = SetAdmin::try_from_bytes(data)?;
    let new_admin = Pubkey::new_from_array(args.admin);

    let [
        signer_info,
        config_info,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer
    signer_info.is_signer()?;

    // Validate config and check admin
    let config = config_info
        .as_account_mut::<Config>(&localuniverse_api::ID)?
        .assert_mut_err(
            |c| c.admin == *signer_info.key,
            LocalUniverseError::NotAuthorized.into(),
        )?;

    // Set new admin
    config.admin = new_admin;

    Ok(())
}
