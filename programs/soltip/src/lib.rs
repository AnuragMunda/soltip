use anchor_lang::prelude::*;

declare_id!("BJy8cEtQ6PoXQUCgcCM1vGbi2PHWzn64YiMb3gnW92VC");

#[program]
pub mod soltip {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
