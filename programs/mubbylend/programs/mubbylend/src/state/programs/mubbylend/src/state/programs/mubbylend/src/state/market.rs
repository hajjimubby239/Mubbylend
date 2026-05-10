use anchor_lang::prelude::*;

#[account]
pub struct Market {
    pub authority: Pubkey,          // 32
    pub asset_mint: Pubkey,         // 32
    pub vault: Pubkey,              // 32
    pub total_supplied: u64,        // 8 — floor amount only
    pub total_borrowed: u64,        // 8 — floor amount only
    pub min_supply: u64,            // 8
    pub max_ltv_bps: u16,           // 2 — public parameter
    pub liquidation_threshold_bps: u16, // 2 — public parameter
    pub supply_apy_bps: u16,        // 2
    pub borrow_apy_bps: u16,        // 2
    pub is_active: bool,            // 1
    pub bump: u8,                   // 1
}

impl Market {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 2 + 2 + 2 + 2 + 1 + 1;
}
