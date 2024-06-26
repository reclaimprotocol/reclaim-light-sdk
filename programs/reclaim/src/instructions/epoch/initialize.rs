use crate::constants::*;
use crate::errors::*;
use crate::state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeEpochConfig<'info> {
    #[account(
        init,
        payer = deployer,
        space = EpochConfig::size(&[]),
        seeds = [
            SEED_PREFIX,
            SEED_EPOCH_CONFIG,
            create_key.key().as_ref()
        ],
        bump
    )]
    pub epoch_config: Account<'info, EpochConfig>,
    pub create_key: Signer<'info>,

    #[account(mut)]
    pub deployer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<InitializeEpochConfig>,
    args: InitializeEpochConfigArgs,
) -> Result<()> {
    let InitializeEpochConfig {
        epoch_config,
        deployer,
        create_key,
        ..
    } = ctx.accounts;

    require!(
        args.epoch_duration_seconds > 0,
        ReclaimError::InvalidEpochDuration
    );

    epoch_config.set_inner(EpochConfig {
        bump: ctx.bumps.epoch_config,
        create_key: create_key.key(),
        deployer: deployer.key(),
        epoch_duration_seconds: args.epoch_duration_seconds,
        epoch_index: 0,
        epochs: vec![],
    });

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeEpochConfigArgs {
    pub epoch_duration_seconds: u64,
}
