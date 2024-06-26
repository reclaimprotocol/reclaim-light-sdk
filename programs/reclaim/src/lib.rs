#![allow(unknown_lints)]
#![allow(ambiguous_glob_reexports)]

use anchor_lang::prelude::*;
use instructions::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("HBLDx6SUJdUxHp2bHKM7Nxi28y3bJpQSxoddE79HeXNZ");

#[program]
pub mod reclaim {
    use super::*;

    pub fn initialize_epoch_config(
        ctx: Context<InitializeEpochConfig>,
        args: InitializeEpochConfigArgs,
    ) -> Result<()> {
        epoch::initialize(ctx, args)
    }

    pub fn change_epoch_index_epoch_config(
        ctx: Context<ChangeEpochIndexEpochConfig>,
        args: ChangeEpochIndexEpochConfigArgs,
    ) -> Result<()> {
        epoch::change_epoch_index(ctx, args)
    }

    pub fn add_epoch(ctx: Context<AddEpoch>, args: AddEpochArgs) -> Result<()> {
        epoch::add(ctx, args)
    }

    pub fn create_group(ctx: Context<CreateGroup>, args: CreateGroupArgs) -> Result<()> {
        group::create(ctx, args)
    }

    pub fn verify_proof(ctx: Context<VerifyProof>, args: VerifyProofArgs) -> Result<()> {
        group::verify_proof(ctx, args)
    }

    pub fn create_dapp(ctx: Context<CreateDapp>, args: CreateDappArgs) -> Result<()> {
        dapp::create(ctx, args)
    }
}
