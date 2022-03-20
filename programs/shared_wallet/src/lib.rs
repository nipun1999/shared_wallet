use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("EuAv4GbnaREmLgyd6QFqYSmM2ow9v125MHRjmyqoMjrc");

#[program]
pub mod shared_wallet {

    use anchor_lang::solana_program::{
        program::{invoke},
        system_instruction::{transfer}
    };

    use super::*;
    pub fn create_shared_wallet(ctx: Context<CreateSharedWallet>,user_1_contribution:u64,user_2_contribution:u64) -> ProgramResult {
        let shared_wallet = &mut ctx.accounts.shared_wallet;
        let user_1_obj = &mut ctx.accounts.user_1;
        let user_2_obj = &mut ctx.accounts.user_2;

        let user_1_balance = user_1_obj.to_account_info().lamports();
        let user_2_balance = user_2_obj.to_account_info().lamports();

        if user_1_contribution > user_1_balance {
            return Err(ErrorCode::NotEnoughLamports.into());
        }

        if user_2_contribution > user_2_balance {
            return Err(ErrorCode::NotEnoughLamports.into());
        }

        shared_wallet.user_1 = *user_1_obj.to_account_info().unsigned_key();
        shared_wallet.user_2 = *user_2_obj.to_account_info().unsigned_key();

        shared_wallet.user_1_balance = user_1_contribution;
        shared_wallet.user_2_balance = user_2_contribution;

        let transfer_instruction_user_1 = &transfer(
            &shared_wallet.user_1,
            &ctx.accounts.owner.to_account_info().key,
            user_1_contribution,
        );

        invoke(
            transfer_instruction_user_1,
            &[
                ctx.accounts.user_1.to_account_info(),
                ctx.accounts.owner.to_account_info(),       
            ]
        )?;

        let transfer_instruction_user_2 = &transfer(
            &shared_wallet.user_2,
            &ctx.accounts.owner.to_account_info().key,
            user_2_contribution,
        );

        invoke(
            transfer_instruction_user_2,
            &[
                ctx.accounts.user_2.to_account_info(),
                ctx.accounts.owner.to_account_info(),       
            ]
        )

    }

    pub fn execute_transaction(ctx: Context<ExecuteTransaction>,new_user_1_balance: u64, new_user_2_balance: u64, total_transaction_amount: u64) -> ProgramResult {
        let shared_wallet = &mut ctx.accounts.shared_wallet;
        let previous_balance = shared_wallet.user_1_balance + shared_wallet.user_2_balance;
        if  total_transaction_amount > previous_balance {
            return Err(ErrorCode::InvalidBalances.into());
        }
        if previous_balance - new_user_1_balance - new_user_2_balance != total_transaction_amount {
            return Err(ErrorCode::InvalidTransaction.into());
        }
        if *ctx.accounts.user_1.to_account_info().key != shared_wallet.user_1 &&  *ctx.accounts.user_1.to_account_info().key != shared_wallet.user_2{
            return Err(ErrorCode::InvalidSigner.into());
        }
        if *ctx.accounts.user_2.to_account_info().key != shared_wallet.user_1 &&  *ctx.accounts.user_2.to_account_info().key != shared_wallet.user_2{
            return Err(ErrorCode::InvalidSigner.into());
        }
        shared_wallet.user_1_balance = new_user_1_balance;
        shared_wallet.user_2_balance = new_user_2_balance;

        let transfer_instruction = &transfer(
            &ctx.accounts.owner.to_account_info().key,
            &ctx.accounts.recipient.to_account_info().key,
            total_transaction_amount,
        );

        invoke(
            transfer_instruction,
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.recipient.to_account_info()
            ]
        )?;
        Ok(())
    }

    pub fn withdraws_balance(ctx: Context<WithdrawBalance>) -> ProgramResult {
        let shared_wallet = &mut ctx.accounts.shared_wallet;

        if *ctx.accounts.signer.to_account_info().key != shared_wallet.user_1 &&  *ctx.accounts.signer.to_account_info().key != shared_wallet.user_2{
            return Err(ErrorCode::InvalidSigner.into());
        }

        let user_1_balance = shared_wallet.user_1_balance;
        let user_2_balance = shared_wallet.user_2_balance;

        let transfer_instruction_user_1 = &transfer(
            &ctx.accounts.owner.to_account_info().key,
            &shared_wallet.user_1,
            user_1_balance,
        );

        invoke(
            transfer_instruction_user_1,
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.user_1.to_account_info()
            ]
        )?;


        let transfer_instruction_user_2 = &transfer(
            &ctx.accounts.owner.to_account_info().key,
            &shared_wallet.user_2,
            user_2_balance,
        );

        invoke(
            transfer_instruction_user_2,
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.user_2.to_account_info()
            ]
        )?;

        shared_wallet.user_1_balance = 0;
        shared_wallet.user_2_balance = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut)]
    pub shared_wallet: Account<'info, SharedWallet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub user_1: Signer<'info>,
    #[account(mut)]
    pub user_2: Signer<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct WithdrawBalance<'info> {
    #[account(mut)]
    pub shared_wallet: Account<'info, SharedWallet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub user_1: AccountInfo<'info>,
    #[account(mut)]
    pub user_2: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateSharedWallet<'info> {
    #[account(init, payer = owner, space = 8 + 64 + 64 + 64 + 64)]
    pub shared_wallet: Account<'info, SharedWallet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub user_1: Signer<'info>,
    #[account(mut)]
    pub user_2: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}


#[account]
pub struct SharedWallet {
    pub user_1: Pubkey,
    pub user_2: Pubkey,
    pub user_1_balance: u64,
    pub user_2_balance: u64
}

#[error]
pub enum ErrorCode {
    #[msg("Not enough lamports in wallet")]
    NotEnoughLamports,
    #[msg("Total transaction amount is greater than total money present in wallet")]
    InvalidBalances,
    #[msg("Invalid split of users balance w.r.t total transaction value")]
    InvalidTransaction,
    #[msg("Not a valid signer")]
    InvalidSigner,
}