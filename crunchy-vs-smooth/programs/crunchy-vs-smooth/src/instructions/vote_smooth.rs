use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct VoteSmooth<'info> {
    /// Merking accounts as mut persits any changes made upon
    /// existring the program, allowing our votes to be recorded
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

pub fn handler(ctx: Context<VoteSmooth>) -> ProgramResult {
    let vote_account = &mut ctx.accounts.vote_account;
    vote_account.smooth += 1;
    Ok(())
}
