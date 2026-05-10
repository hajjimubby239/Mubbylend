use anchor_lang::prelude::*;

#[account]
pub struct Position {
    pub owner: Pubkey,              // 32
    pub market: Pubkey,             // 32
    pub supplied_commitment: [u8; 32], // 32 — Pedersen commitment to supply amount
    pub borrowed_commitment: [u8; 32], // 32 — Pedersen commitment to borrow amount
    pub arcium_job_id: [u8; 32],    // 32 — MPC computation reference
    pub open_slot: u64,             // 8
    pub is_active: bool,            // 1
    pub bump: u8,                   // 1
}

impl Position {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 32 + 8 + 1 + 1;
}
