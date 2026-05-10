use anchor_lang::prelude::*;
use crate::state::Market;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MarketParams {
    pub max_ltv_bps: u16,
    pub liquidation_threshold_bps: u16,
    pub supply_apy_bps: u16,
    pub borrow_apy_bps: u16,
    pub min_supply: u64,
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Market::LEN,
        seeds = [b"market", asset_mint.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    pub asset_mint: Account<'info, anchor_spl::token::Mint>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitMarket>,
    params: MarketParams,
) -> Result<()> {
    let market = &mut ctx.accounts.market;
    market.authority = ctx.accounts.authority.key();
    market.asset_mint = ctx.accounts.asset_mint.key();
    market.max_ltv_bps = params.max_ltv_bps;
    market.liquidation_threshold_bps = params.liquidation_threshold_bps;
    market.supply_apy_bps = params.supply_apy_bps;
    market.borrow_apy_bps = params.borrow_apy_bps;
    market.min_supply = params.min_supply;
    market.is_active = true;
    market.bump = ctx.bumps.market;

    emit!(MarketInitialized {
        authority: market.authority,
        asset_mint: market.asset_mint,
        max_ltv_bps: market.max_ltv_bps,
    });

    Ok(())
}

#[event]
pub struct MarketInitialized {
    pub authority: Pubkey,
    pub asset_mint: Pubkey,
    pub max_ltv_bps: u16,
}
