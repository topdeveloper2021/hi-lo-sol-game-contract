
use anchor_lang::prelude::*;


declare_id!("47zoveBTgbydhERDjqYuoAuJRffCWSYAQgv5M7eDNqSA");

#[program]
pub mod myepicproject {
    use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};
    use anchor_lang::solana_program::{clock::Clock};
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        Ok(())
    }

    pub fn generate_rand(_ctx: Context<GenerateRand>) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        let now = Clock::get().unwrap().unix_timestamp as u64;
        let card_rand = (now%44 + now%33 + now%22 + now%11 + now%7 + now%5)%52 +1;
        base_account.current_rand = card_rand;
        Ok(())
    }

    pub fn compare_bet(_ctx: Context<CompareBet>, bet_string: String) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        let now = Clock::get().unwrap().unix_timestamp as u64;
        let new_rand = (now%44 + now%33 + now%22 + now%11 + now%7 + now%5)%52 +1;
        let mut new_rand_rest = 0;
        let mut start_rand_rest = 0;
        if (base_account.current_rand%13)==0 {
            start_rand_rest = 13;
        } else {
            start_rand_rest = base_account.current_rand%13;
        }

        if (new_rand%13)==0 {
            new_rand_rest = 13;
        } else {
            new_rand_rest = new_rand%13;
        }

        if bet_string == "up" {
            if new_rand_rest >= start_rand_rest {
                    let multiple1 = 2000-(14-start_rand_rest)*1000/13;
                    base_account.bet_amount = base_account.bet_amount * multiple1 /1000;
            } else {
                base_account.bet_amount = 0;
            }
        }
        else {
            if new_rand_rest <= start_rand_rest {
                    let multiple2 = 2000-start_rand_rest*1000/13;
                    base_account.bet_amount = base_account.bet_amount * multiple2/1000;
            } 
            else {
                base_account.bet_amount = 0;
            }
        }
        
        base_account.current_rand = new_rand;
        Ok(())
    }


    pub fn place_bet(_ctx: Context<PlaceBet>, bet_amount: String) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        let pool_wallet = &mut _ctx.accounts.pool_wallet;
        let company_wallet = &mut _ctx.accounts.company_wallet;
        let stake_bal: u64 = bet_amount.parse().unwrap();
        base_account.bet_amount = stake_bal as u64;
        let user = &mut _ctx.accounts.user;
        let game_amount = stake_bal*24/25;
        let fee_amount = stake_bal/25;        
        invoke(
                &transfer(
                    user.to_account_info().key,
                    pool_wallet.to_account_info().key,
                    game_amount,
                ),
                &[
                    user.to_account_info(),
                    pool_wallet.to_account_info(),
                    _ctx.accounts.system_program.to_account_info()
                ],
        )?; 
        
        invoke(
            &transfer(
                user.to_account_info().key,
                company_wallet.to_account_info().key,
                fee_amount,
            ),
            &[
                user.to_account_info(),
                company_wallet.to_account_info(),
                _ctx.accounts.system_program.to_account_info()
            ],
        )?; // send fee to company wallet
        let now = Clock::get().unwrap().unix_timestamp as u64;
        let card_rand = (now%44 + now%33 + now%22 + now%11 + now%7 + now%5)%52 +1;
        base_account.current_rand = card_rand;
        Ok(())

    }

    pub fn cash_out(_ctx: Context<CashOut>) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        let pool_wallet = &mut _ctx.accounts.pool_wallet;
        let user = &mut _ctx.accounts.user;
        let transfer_amount = base_account.bet_amount as u64;
        if transfer_amount > 0 {
            invoke(
                &transfer(
                    pool_wallet.to_account_info().key,
                    user.to_account_info().key,
                    transfer_amount,
                ),
                &[
                    pool_wallet.to_account_info(),
                    user.to_account_info(),
                    _ctx.accounts.system_program.to_account_info()
                ],
           )?;
        }
       base_account.bet_amount = 0;
       
      
       Ok(())   

    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer= user, space = 1000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct GenerateRand<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct CompareBet<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub pool_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub company_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct CashOut<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub pool_wallet: Signer<'info>,
    #[account(mut)]
    pub user: AccountInfo<'info>,
    pub system_program: Program <'info, System>,
}

#[account]
pub struct BaseAccount {
    pub bet_amount: u64,
    pub current_rand: u64
}

#[account]
pub struct PoolWallet{
    pub balance: u64 
} 
#[account]
pub struct CompanyWallet{
    pub balance: u64 
} 
