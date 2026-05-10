use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod mubbylend {
    use super::*;

    pub fn initialize_market(
        ctx: Context<InitMarket>,
        params: MarketParams,
    ) -> Result<()> {
        instructions::init_market::handler(ctx, params)
    }

    pub fn supply(
        ctx: Context<Supply>,
        amount_commitment: [u8; 32],
        arcium_job_id: [u8; 32],
    ) -> Result<()> {
        instructions::supply::handler(ctx, amount_commitment, arcium_job_id)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        mpc_proof: Vec<u8>,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw::handler(ctx, mpc_proof, amount)
    }

    pub fn borrow(
        ctx: Context<Borrow>,
        amount_commitment: [u8; 32],
        arcium_job_id: [u8; 32],
    ) -> Result<()> {
        instructions::borrow::handler(ctx, amount_commitment, arcium_job_id)
    }

    pub fn repay(
        ctx: Context<Repay>,
        mpc_proof: Vec<u8>,
        interest_owed: u64,
    ) -> Result<()> {
        instructions::repay::handler(ctx, mpc_proof, interest_owed)
    }

    pub fn liquidate(
        ctx: Context<Liquidate>,
        mpc_proof: Vec<u8>,
    ) -> Result<()> {
        instructions::liquidate::handler(ctx, mpc_proof)
    }
}

pub mod instructions;
pub mod state;
