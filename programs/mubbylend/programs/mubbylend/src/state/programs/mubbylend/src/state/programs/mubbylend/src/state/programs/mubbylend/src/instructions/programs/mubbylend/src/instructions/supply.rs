use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{Position, Market};

#[derive(Accounts)]
pub struct Supply<'info> {
    #[account(mut)]
    pub supplier: Signer<'info>,

    #[account(
        init,
        payer = supplier,
        space = Position::LEN,
        seeds = [b"position", supplier.key().as_ref()],
        bump
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub supplier_token_account: Account<'info, TokenAccount>,

    pub market: Account<'info, Market>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<Supply>,
    amount_commitment: [u8; 32],
    arcium_job_id: [u8; 32],
) -> Result<()> {
    let pos = &mut ctx.accounts.position;
    pos.owner = ctx.accounts.supplier.key();
    pos.market = ctx.accounts.market.key();
    pos.supplied_commitment = amount_commitment; // Pedersen commitment — no plaintext
    pos.arcium_job_id = arcium_job_id;
    pos.open_slot = Clock::get()?.slot;
    pos.is_active = true;
    pos.bump = ctx.bumps.position;

    // Transfer floor collateral to vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.supplier_token_account.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.supplier.to_account_info(),
            },
        ),
        ctx.accounts.market.min_supply,
    )?;

    emit!(Supplied {
        supplier: pos.owner,
        market: pos.market,
        slot: pos.open_slot,
        // NO amount emitted — stays private
    });

    Ok(())
}

#[event]
pub struct Supplied {
    pub supplier: Pubkey,
    pub market: Pubkey,
    pub slot: u64,
}
