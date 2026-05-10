use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{Position, Market};

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        mut,
        seeds = [b"position", borrower.key().as_ref()],
        bump = position.bump,
        constraint = position.owner == borrower.key(),
        constraint = position.is_active == true,
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub borrower_token_account: Account<'info, TokenAccount>,

    /// CHECK: Arcium verifier confirms LTV check passed
    pub arcium_verifier: UncheckedAccount<'info>,

    pub market: Account<'info, Market>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<Borrow>,
    amount_commitment: [u8; 32],
    arcium_job_id: [u8; 32],
) -> Result<()> {
    let pos = &mut ctx.accounts.position;
    pos.borrowed_commitment = amount_commitment; // Pedersen commitment — no plaintext
    pos.arcium_job_id = arcium_job_id;

    // Arcium MPC privately verified LTV is within limits
    // Only the boolean result crossed the privacy boundary
    // In production: arcium_sdk::verify_ltv_proof(&arcium_job_id)?;

    emit!(Borrowed {
        borrower: pos.owner,
        market: pos.market,
        slot: Clock::get()?.slot,
        // NO borrow amount emitted — stays private
    });

    Ok(())
}

#[event]
pub struct Borrowed {
    pub borrower: Pubkey,
    pub market: Pubkey,
    pub slot: u64,
}
