use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};

// PLACEHOLDER ONLY: replace before local-validator testing or deployment.
declare_id!("11111111111111111111111111111111");

#[program]
pub mod astro_bux_rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, decimals: u8) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.admin = ctx.accounts.admin.key();
        config.reward_mint = ctx.accounts.reward_mint.key();
        config.paused = false;
        config.bump = ctx.bumps.config;
        config.mint_authority_bump = ctx.bumps.mint_authority;
        Ok(())
    }

    pub fn create_campaign(ctx: Context<CreateCampaign>, campaign_id: u64, cap: u64) -> Result<()> {
        require!(!ctx.accounts.config.paused, RewardsError::ProgramPaused);
        require!(cap > 0, RewardsError::InvalidAmount);
        let campaign = &mut ctx.accounts.campaign;
        campaign.config = ctx.accounts.config.key();
        campaign.campaign_id = campaign_id;
        campaign.cap = cap;
        campaign.minted = 0;
        campaign.active = true;
        campaign.bump = ctx.bumps.campaign;
        emit!(CampaignCreated { campaign: campaign.key(), campaign_id, cap });
        Ok(())
    }

    pub fn award_reward(ctx: Context<AwardReward>, activity_id: [u8; 32], amount: u64) -> Result<()> {
        require!(!ctx.accounts.config.paused, RewardsError::ProgramPaused);
        require!(ctx.accounts.campaign.active, RewardsError::CampaignInactive);
        require!(amount > 0, RewardsError::InvalidAmount);
        let new_total = ctx.accounts.campaign.minted.checked_add(amount)
            .ok_or(RewardsError::MathOverflow)?;
        require!(new_total <= ctx.accounts.campaign.cap, RewardsError::CampaignCapExceeded);

        let config_key = ctx.accounts.config.key();
        let bump = [ctx.accounts.config.mint_authority_bump];
        let signer_seeds: &[&[u8]] = &[b"mint_authority", config_key.as_ref(), &bump];
        let signer = &[signer_seeds];
        let cpi_accounts = MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        token::mint_to(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer),
            amount,
        )?;

        ctx.accounts.campaign.minted = new_total;
        ctx.accounts.receipt.campaign = ctx.accounts.campaign.key();
        ctx.accounts.receipt.activity_id = activity_id;
        ctx.accounts.receipt.recipient = ctx.accounts.recipient.key();
        ctx.accounts.receipt.amount = amount;
        ctx.accounts.receipt.bump = ctx.bumps.receipt;
        emit!(RewardAwarded {
            campaign: ctx.accounts.campaign.key(),
            recipient: ctx.accounts.recipient.key(),
            amount,
            activity_id,
        });
        Ok(())
    }

    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
        ctx.accounts.config.paused = paused;
        emit!(PauseChanged { paused });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = 8 + RewardsConfig::INIT_SPACE, seeds = [b"config"], bump)]
    pub config: Account<'info, RewardsConfig>,
    /// CHECK: PDA used only as the SPL mint authority.
    #[account(seeds = [b"mint_authority", config.key().as_ref()], bump)]
    pub mint_authority: UncheckedAccount<'info>,
    #[account(
        init, payer = admin, mint::decimals = decimals,
        mint::authority = mint_authority, mint::freeze_authority = mint_authority,
        seeds = [b"reward_mint", config.key().as_ref()], bump
    )]
    pub reward_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(campaign_id: u64)]
pub struct CreateCampaign<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, RewardsConfig>,
    pub admin: Signer<'info>,
    #[account(
        init, payer = admin, space = 8 + RewardCampaign::INIT_SPACE,
        seeds = [b"campaign", config.key().as_ref(), campaign_id.to_le_bytes().as_ref()], bump
    )]
    pub campaign: Account<'info, RewardCampaign>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AwardReward<'info> {
    #[account(mut, has_one = admin, has_one = reward_mint)]
    pub config: Account<'info, RewardsConfig>,
    pub admin: Signer<'info>,
    /// CHECK: Recipient owns the destination associated token account.
    pub recipient: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"campaign", config.key().as_ref(), campaign.campaign_id.to_le_bytes().as_ref()],
        bump = campaign.bump,
        has_one = config
    )]
    pub campaign: Account<'info, RewardCampaign>,
    /// CHECK: PDA signing authority for the mint_to CPI.
    #[account(seeds = [b"mint_authority", config.key().as_ref()], bump = config.mint_authority_bump)]
    pub mint_authority: UncheckedAccount<'info>,
    #[account(mut, address = config.reward_mint)]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init, payer = admin, space = 8 + AwardReceipt::INIT_SPACE,
        seeds = [b"receipt", campaign.key().as_ref(), activity_id.as_ref()], bump
    )]
    pub receipt: Account<'info, AwardReceipt>,
    #[account(
        init_if_needed, payer = admin,
        associated_token::mint = reward_mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPaused<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, RewardsConfig>,
    pub admin: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct RewardsConfig {
    pub admin: Pubkey,
    pub reward_mint: Pubkey,
    pub paused: bool,
    pub bump: u8,
    pub mint_authority_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct RewardCampaign {
    pub config: Pubkey,
    pub campaign_id: u64,
    pub cap: u64,
    pub minted: u64,
    pub active: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct AwardReceipt {
    pub campaign: Pubkey,
    pub activity_id: [u8; 32],
    pub recipient: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

#[event]
pub struct CampaignCreated { pub campaign: Pubkey, pub campaign_id: u64, pub cap: u64 }

#[event]
pub struct RewardAwarded {
    pub campaign: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub activity_id: [u8; 32],
}

#[event]
pub struct PauseChanged { pub paused: bool }

#[error_code]
pub enum RewardsError {
    #[msg("The rewards program is paused.")] ProgramPaused,
    #[msg("The campaign is inactive.")] CampaignInactive,
    #[msg("Amount must be greater than zero.")] InvalidAmount,
    #[msg("The campaign reward cap would be exceeded.")] CampaignCapExceeded,
    #[msg("Arithmetic overflow.")] MathOverflow,
}
