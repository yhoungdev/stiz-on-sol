use anchor_lang::prelude::*;

declare_id!("BzUEJ5Ky8Qetj9UhWR1tzQCtso1P8qe7R3YKX9GjkTY");

#[program]
pub mod wardrope {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
