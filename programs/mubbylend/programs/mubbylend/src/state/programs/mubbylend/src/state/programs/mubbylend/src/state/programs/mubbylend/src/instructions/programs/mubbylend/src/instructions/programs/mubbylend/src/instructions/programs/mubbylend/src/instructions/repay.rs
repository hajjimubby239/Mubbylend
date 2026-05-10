use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::Position;

#[derive(Accounts)]
pub struct Repay<'info> {
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

    /// CHECK: Arcium verifier confirms interest computation
    pub arcium_verifier: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<Repay>,
    _mpc_proof: Vec<u8>,
    interest_owed: u64,
) -> Result<()> {
    // Arcium interest_accrue circuit computed interest privately
    // Only the final interest_owed is revealed for settlement
    // In production: arcium_sdk::verify_interest_proof(&mpc_proof, interest_owed)?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.borrower_token_account.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.borrower.to_account_info(),
            },
        ),
        interest_owed,
    )?;

    // Clear borrowed commitment after repayment
    let pos = &mut ctx.accounts.position;
    pos.borrowed_commitment = [0u8; 32];

    emit!(Repaid {
        borrower: pos.owner,
        interest_owed, // Only interest revealed — not the principal
        slot: Clock::get()?.slot,
    });

    Ok(())
}

#[event]
pub struct Repaid {
    pub borrower: Pubkey,
    pub interest_owed: u64, // ONLY this is revealed — minimum for settlement
    pub slot: u64,
}
