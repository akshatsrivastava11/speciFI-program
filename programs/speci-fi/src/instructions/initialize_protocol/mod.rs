use anchor_lang::prelude::*;

use crate::{protocol, Protocol};

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub authority:Signer<'info>,
    #[account(
        init,
        payer=authority,
        space=8+Protocol::INIT_SPACE,
        seeds=[b"protocol"],
        bump
    )]
    pub protocol:Account<'info,Protocol>,
    pub system_program:Program<'info,System>
}

impl<'info>Initialize<'info>{
    pub fn initialize(&mut self,fee_bps:u64,bumps:InitializeBumps)->Result<()>{
        self.protocol.set_inner(Protocol { 
            authority: self.authority.key(), 
            fee_bps,
             total_loans:0,
              total_volume:0,
               is_paused:false,
                bump:bumps.protocol
             });
             Ok(())
    }
}
