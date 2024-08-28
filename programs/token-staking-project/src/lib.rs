use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer, MintTo, Burn};
use anchor_spl::token::Token;

declare_id!("bosuUbACQoWaF6Zj8A57tJedjFbYUU4aJkrgutcCYE5");

#[program]
pub mod token_staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        staking_account.reward_rate = 1;
        staking_account.reward_pool = 0;
        Ok(())
    }

    pub fn distribute_tokens(ctx: Context<DistributeTokens>, amount: u64) -> Result<()> {
        for i in 0..10 {
            let seeds = &[
                b"user".as_ref(),
                &[i as u8],
                &[ctx.bumps["user_account"]],
            ];
            let signer = &[&seeds[..]];
            token::transfer(ctx.accounts.transfer_ctx().with_signer(signer), amount)?;
        }
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        staking_account.total_staked += amount;
        staking_account.reward_pool += (amount as f64 * staking_account.reward_rate as f64 / 100.0) as u64;
        token::transfer(ctx.accounts.transfer_ctx(), amount)?;
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        let reward = (staking_account.reward_pool as f64 * staking_account.reward_rate as f64 / 100.0) as u64;
        staking_account.reward_pool -= reward;
        token::mint_to(ctx.accounts.mint_to_ctx(), reward)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        staking_account.total_staked -= amount;
        token::transfer(ctx.accounts.transfer_ctx(), amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeTokens<'info> {
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub staking_account: Account<'info, StakingAccount>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakingAccount {
    pub total_staked: u64,
    pub reward_pool: u64,
    pub reward_rate: u8, // Percentage rate for rewards
}
