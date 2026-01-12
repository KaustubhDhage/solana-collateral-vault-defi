use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

// *** CRITICAL: PROGRAM ID ***
declare_id!("H3hQCZw1NHn8uW1mTkk9q3GMJp2KcyaSgtWxJm9DDpd9");

#[program]
pub mod collateral_vault {
    use super::*;

    // Added vault_id param to allow multiple test runs
    pub fn initialize_vault(ctx: Context<InitializeVault>, vault_id: u8) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.signer.key();
        vault.token_account = ctx.accounts.vault_token_account.key();
        vault.vault_id = vault_id;
        vault.created_at = Clock::get()?.unix_timestamp;
        vault.bump = ctx.bumps.vault;
        
        msg!("✅ Vault #{} initialized. Owner: {}", vault_id, vault.owner);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, vault_id: u8, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        let vault = &mut ctx.accounts.vault;
        vault.total_balance = vault.total_balance.checked_add(amount).ok_or(ErrorCode::MathError)?;
        vault.available_balance = vault.available_balance.checked_add(amount).ok_or(ErrorCode::MathError)?;

        msg!("✅ Deposited {}. Vault Balance: {}", amount, vault.total_balance);
        emit!(DepositEvent {
            user: vault.owner,
            amount,
            new_balance: vault.total_balance,
        });

        Ok(())
    }

    pub fn lock_collateral(ctx: Context<LockCollateral>, vault_id: u8, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        require!(
            vault.available_balance >= amount,
            ErrorCode::InsufficientAvailableCollateral
        );

        vault.available_balance = vault.available_balance.checked_sub(amount).ok_or(ErrorCode::MathError)?;
        vault.locked_balance = vault.locked_balance.checked_add(amount).ok_or(ErrorCode::MathError)?;

        msg!("✅ Locked {}. Available: {}", amount, vault.available_balance);
        emit!(LockEvent {
            user: vault.owner,
            amount,
            locked_balance: vault.locked_balance,
        });

        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct CollateralVault {
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub total_balance: u64,
    pub locked_balance: u64,
    pub available_balance: u64,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub created_at: i64,
    pub bump: u8,
    pub vault_id: u8, // Added ID
}

#[derive(Accounts)]
#[instruction(vault_id: u8)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + 200, // Safe space allocation
        // Seed now includes vault_id for uniqueness
        seeds = [b"vault", signer.key().as_ref(), &[vault_id]],
        bump
    )]
    pub vault: Account<'info, CollateralVault>,

    // Standard PDA Token Account derived from the Vault PDA
    #[account(
        init,
        payer = signer,
        seeds = [b"token_account", vault.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(vault_id: u8)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut, 
        seeds = [b"vault", signer.key().as_ref(), &[vault_id]], 
        bump = vault.bump
    )]
    pub vault: Account<'info, CollateralVault>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = vault.token_account)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(vault_id: u8)]
pub struct LockCollateral<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut, 
        seeds = [b"vault", signer.key().as_ref(), &[vault_id]], 
        bump = vault.bump
    )]
    pub vault: Account<'info, CollateralVault>,
}

#[event]
pub struct DepositEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub new_balance: u64,
}

#[event]
pub struct LockEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub locked_balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Amount")] InvalidAmount,
    #[msg("Insufficient Collateral")] InsufficientAvailableCollateral,
    #[msg("Math Error")] MathError,
    #[msg("Invalid Token Account")] InvalidVaultTokenAccount,
}