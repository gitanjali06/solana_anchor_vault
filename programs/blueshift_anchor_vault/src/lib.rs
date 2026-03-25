use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222222222222222222");

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
}

// declaring accounts
#[derive(Accounts)]
pub struct VaultAction<'info> {
    //user wallet
    #[account(mut)]
    pub signer: Signer<'info>,
    //pda vault
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

   pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // Check if vault is empty
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);
        // Ensure amount exceeds rent-exempt minimum
        require_gt!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);

        use anchor_lang::system_program::{transfer, Transfer};
        // transfer from wallet to pda deposit
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
      pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        // check vault has lamports 
        require_neq!(ctx.accounts.vault.lamports(), 0, VaultError::InvalidAmount);

        let signer_key = ctx.accounts.signer.key();
        //seeds for verifying pda as pda has no keys
        let signer_seeds = &[b"vault", signer_key.as_ref(), &[ctx.bumps.vault]];
        use anchor_lang::system_program::{transfer, Transfer};
         transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[&signer_seeds[..]]
            ),
            ctx.accounts.vault.lamports()
)?;

        Ok(())
    }
}


