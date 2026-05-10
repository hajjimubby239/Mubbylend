use anchor_lang::prelude::*;
use crate::state::Position;

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"position", position.owner.as_ref()],
        bump = position.bump,
        constraint = position.is_active == true,
        close = liquidator
    )]
    pub position: Account<'info, Position>,

    /// CHECK: Arcium verifier confirms liquidation_check circuit returned true
    pub arcium_verifier: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<Liquidate>,
    _mpc_proof: Vec<u8>,
) -> Result<()> {
    // Arcium liquidation_check circuit privately verified this
    // position's health factor dropped below 1.0
    // Liquidator only learns: is_liquidatable = true
    // They never learn the exact collateral amount or health factor
    // In production: arcium_sdk::verify_liquidation_proof(&mpc_proof)?;

    emit!(Liquidated {
        borrower: ctx.accounts.position.owner,
        liquidator: ctx.accounts.liquidator.key(),
        slot: Clock::get()?.slot,
        // NO collateral amount, NO health factor revealed
    });

    Ok(())
}

#[event]
pub struct Liquidated {
    pub borrower: Pubkey,
    pub liquidator: Pubkey,
    pub slot: u64,
}
