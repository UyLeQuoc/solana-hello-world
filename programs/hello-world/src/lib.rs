use anchor_lang::prelude::*;

declare_id!("EFcB24iGSL68x6HRb17veK5ff7dga61Tc9g4egeEuC36");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
