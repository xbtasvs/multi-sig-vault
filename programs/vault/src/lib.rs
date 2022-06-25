use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;
use anchor_lang::{AnchorSerialize, AnchorDeserialize, Account};
use borsh::{BorshSerialize, BorshDeserialize};
use crate::constants::*;

declare_id!("4zCBXCuFuP2TCAwJPXELhd54LEKa5Nnfv17CUMBX7zah");

mod constants {
    use solana_program::{pubkey, pubkey::Pubkey};
    pub const ADMIN_WALLET1: Pubkey = pubkey!("7EGWwj35r6sd4ERZMU2CGoTFL1ZuoUNup8DhxFyr6UPf");
    pub const ADMIN_WALLET2: Pubkey = pubkey!("3rgWEviKxXxEjbLnSQCreVNRhgx2QgHtVnVbh8ZjPgix");
    pub const ADMIN_WALLET3: Pubkey = pubkey!("EvKcFuJ63k2AVdg6fjee36JtPsq7RzQpvgb2wyX3gjrh");
    pub const ADMIN_WALLET4: Pubkey = pubkey!("5wQ4XdFbzFbRppW8if8iJwaK1qUkjhpxmTq7WJrWMYjh");
    pub const THRESHOLD: u8 = 3;
}
 
#[program]
pub mod vault {
    use super::*;

    pub fn init_vault(_ctx: Context<InitContext>, _bump: u8) -> ProgramResult {
        Ok(())
    }

    pub fn init_proposals(ctx: Context<InitProposalsContext>, _bump: u8) -> ProgramResult {
        let proposals = &mut ctx.accounts.proposals;
        proposals.created = Vec::new();
        Ok(())
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, recipient: Pubkey, amount1: u32, amount2: u32, _bump: u8, id: u8) -> ProgramResult {
        let signer = ctx.accounts.signer.to_account_info();
        let proposals = &mut ctx.accounts.proposals;
        let proposal = &mut ctx.accounts.proposal;

        for x in &proposals.created {
            if *x == id {
                return Err(ErrorCode::AlreadyExists.into());
            }
        }

        let index;
        if signer.key().to_bytes() == ADMIN_WALLET1.to_bytes() {
            index = 0;
        } else if signer.key().to_bytes() == ADMIN_WALLET2.to_bytes() {
            index = 1;
        } else if signer.key().to_bytes() == ADMIN_WALLET3.to_bytes() {
            index = 2;
        } else if signer.key().to_bytes() == ADMIN_WALLET4.to_bytes() {
            index = 3;
        } else {
            return Err(ErrorCode::NotAdmin.into());
        }

        proposal.creator = signer.key();
        let mut signed = [false, false, false, false];
        signed[index] = true;
        proposal.id = id;
        proposal.signed = signed;
        proposal.recipient = recipient;
        proposal.amount = 0xffffffff * amount1 as u64 + amount2 as u64;
        proposals.created.push(id);
        Ok(())
    }

    pub fn cancel_proposal(ctx: Context<CancelProposal>, _bump: u8) -> ProgramResult {
        let signer = ctx.accounts.signer.to_account_info();
        let proposal = &mut ctx.accounts.proposal;
        let proposals = &mut ctx.accounts.proposals;
        if signer.key().to_bytes() != proposal.creator.to_bytes() {
            return Err(ErrorCode::NotProposalOwner.into());
        }
        for i in 0..proposals.created.len() {
            if proposals.created[i] == proposal.id {
                proposals.created[i] = proposals.created[proposals.created.len() - 1];
                proposals.created.pop();
            }
        }
        proposal.close(signer.to_account_info());
        Ok(())
    }

    pub fn claim(ctx: Context<ClaimContext>, _bump: u8) -> ProgramResult {
        let vault_account = &mut ctx.accounts.vault_account;
        let recipient = &mut ctx.accounts.recipient;
        let proposal = &mut ctx.accounts.proposal;
        let proposals = &mut ctx.accounts.proposals;
        let signer = ctx.accounts.signer.to_account_info();

        let index;
        if signer.key().to_bytes() == ADMIN_WALLET1.to_bytes() {
            index = 0;
        } else if signer.key().to_bytes() == ADMIN_WALLET2.to_bytes() {
            index = 1;
        } else if signer.key().to_bytes() == ADMIN_WALLET3.to_bytes() {
            index = 2;
        } else if signer.key().to_bytes() == ADMIN_WALLET4.to_bytes() {
            index = 3;
        } else {
            return Err(ErrorCode::NotAdmin.into());
        }

        if proposal.signed[index] == true {
            return Err(ErrorCode::AlreadySigned.into());
        }

        let balance = **vault_account.lamports.borrow();
        if balance <= proposal.amount {
            return Err(ErrorCode::InsufficientSolToWithdraw.into());
        }

        proposal.signed[index] = true;
        let mut approved = 0;
        for i in 0..4 {
            if proposal.signed[i] == true {
                approved += 1;
            }
        }
        if approved == THRESHOLD {
            **vault_account.lamports.borrow_mut() -= proposal.amount;
            **recipient.try_borrow_mut_lamports()? += proposal.amount;
            for i in 0..proposals.created.len() {
                if proposals.created[i] == proposal.id {
                    proposals.created[i] = proposals.created[proposals.created.len() - 1];
                    proposals.created.pop();
                }
            }

            proposal.close(signer.to_account_info());
        }
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitContext<'info> {
    #[account(init, seeds = [b"vault".as_ref()], payer = user, space = 0, bump)]
    /// CHECK:
    vault: AccountInfo<'info>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitProposalsContext<'info> {
    #[account(init, seeds = [b"proposals".as_ref()], payer = user, space = 1024, bump)]
    /// CHECK:
    proposals: Account<'info, Proposals>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8, id: u8)]
pub struct CreateProposal<'info> {
    #[account(init, seeds = [b"proposal".as_ref(), format!("{}", id).as_ref()], payer = signer, space = 8 + 69 + 4 + 5 + 8, bump)]
    pub proposal: Account<'info, Proposal>,
    /// CHECK:
    pub vault_account: AccountInfo<'info>,
    pub proposals: Account<'info, Proposals>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CancelProposal<'info> {
    #[account(mut, constraint = (signer.key() == ADMIN_WALLET1 || signer.key() == ADMIN_WALLET2 || signer.key() == ADMIN_WALLET3 || signer.key() == ADMIN_WALLET4))]
    pub signer: Signer<'info>,
    #[account(mut)]
    /// CHECK:
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposals: Account<'info, Proposals>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimContext<'info> {
    #[account(mut, constraint = (signer.key() == ADMIN_WALLET1 || signer.key() == ADMIN_WALLET2 || signer.key() == ADMIN_WALLET3 || signer.key() == ADMIN_WALLET4))]
    pub signer: Signer<'info>,
    #[account(mut)]
    /// CHECK:
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposals: Account<'info, Proposals>,
    #[account(mut)]
    /// CHECK:
    pub vault_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    /// CHECK:
    pub recipient: AccountInfo<'info>,
}

#[error]
pub enum ErrorCode {
    #[msg("Only Admin is allowed to withdraw.")]
    NotAdmin,
    #[msg("Insufficient sol to withdraw.")]
    InsufficientSolToWithdraw,
    #[msg("You have already signed")]
    AlreadySigned,
    #[msg("Invalid Claiming Accounts")]
    InvalidAccounts,
    #[msg("Not Owner of Proposal")]
    NotProposalOwner,
    #[msg("Already exists")]
    AlreadyExists,
}

#[account]
pub struct Proposals {
    pub created: Vec<u8>,
}

#[account]
pub struct Proposal {
    pub id: u8,
    pub creator: Pubkey,
    pub signed: [bool;4],
    pub recipient: Pubkey,
    pub amount: u64,
}