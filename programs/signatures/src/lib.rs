use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod signatures {

    use super::*;
    pub fn create_user(ctx: Context<CreateUser>, user_name:String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.user_name = user_name;
        user_account.key = ctx.accounts.user.key();
        Ok(())
    }

    pub fn create_message_account(ctx: Context<CreateMessageAccount>, message: String) -> Result<()> {
        let message_account = &mut ctx.accounts.message_account;
        let user_1_obj = &mut ctx.accounts.user_1;
        message_account.user_1 = *user_1_obj.to_account_info().unsigned_key();
        message_account.message = message;
        Ok(())
    }

    pub fn update_message(ctx: Context<UpdateMessage>,new_message: String) -> Result<()> {
        let message_account = &mut ctx.accounts.message_account;
        if *ctx.accounts.user_1.to_account_info().key != message_account.user_1 {
            msg!("Inavild signature!")
        }
        message_account.message = new_message;
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    #[account(mut)]
    pub message_account: Account<'info, Message>,
    #[account(mut)]
    pub user_1: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMessageAccount<'info> {
    #[account(init, payer = owner, space = 8 + 64 + 64 + 64 + 64)]
    pub message_account: Account<'info, Message>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub user_1: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct User {
    pub user_name: String,
    pub key: Pubkey
}

#[account]
pub struct Message {
    pub user_1: Pubkey,
    pub message: String
}