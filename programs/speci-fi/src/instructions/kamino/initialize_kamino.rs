use anchor_lang::prelude::*;
use crate::state::kamino_position::*;
#[derive(Accounts)]
pub struct initialize_kamino<'info>{
    #[account(mut)]
    pub authority:Signer<'info>,
    #[account(
        init,
        payer=authority,
        space=8,
        seeds=[b"kamino_position",authority.key().as_ref()],
        bump
    )]
    pub kamino_position:Account<'info,KaminoPosition>,

    /// CHECK: Kamino program
    pub kamino_program: AccountInfo<'info>,
    
    /// CHECK: Kamino lending market
    pub kamino_lending_market: AccountInfo<'info>,
    
    /// CHECK: USDC reserve in Kamino
    pub usdc_reserve: AccountInfo<'info>,
    
    /// CHECK: SOL reserve in Kamino
    pub sol_reserve: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,

}
