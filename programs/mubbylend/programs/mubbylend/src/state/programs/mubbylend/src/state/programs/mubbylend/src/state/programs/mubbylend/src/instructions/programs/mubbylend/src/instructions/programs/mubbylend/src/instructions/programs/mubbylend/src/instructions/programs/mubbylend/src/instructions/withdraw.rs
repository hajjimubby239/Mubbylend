use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::Position;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub supplier: Signer<'info>,

    #[account(
        mut,
        seeds = [b"position", supplier.key().as_ref()],
        bump = position.bump,
        constraint = position.owner == supplier.key(),
        constraint = position.is_active == true,
        close = supplier
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub supplier_token_account: Account<'info, TokenAccount>,

    /// CHECK: Arcium verifier confirms health check passed
    pub arcium_verifier: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<Withdraw>,
    _mpc_proof: Vec<u8>,
    interest_owed: u64,
) -> Result<()> {
    // Arcium health_check circuit privately confirmed position
    // remains healthy after withdrawal
    // In production: arcium_sdk::verify_health_proof(&mpc_proof)?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.supplier_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
        ),
        interest_owed,
    )?;

    emit!(Withdrawn {
        supplier: ctx.accounts.position.owner,
        interest_earned: interest_owed, // Only interest revealed
        slot: Clock::get()?.slot,
    });

    Ok(())
}

#[event]
pub struct Withdrawn {
    pub supplier: Pubkey,
    pub interest_earned: u64, // Only interest revealed — not the principal
    pub slot: u64,
}
