use anchor_lang::prelude::*;

declare_id!("D2ddLjQTShwD1QDAuiUEeEjxpLie2tKs32VxzTNDBsmk");

#[program]
pub mod new_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
