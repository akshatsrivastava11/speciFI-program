use anchor_lang::prelude::*;

#[account]
pub struct KaminoPosition{
    pub authority:Pubkey,
    pub kamino_lending_market:Pubkey,

    //token reserves
    pub usdc_reserve:Pubkey,
    pub sol_reserve:Pubkey,


    pub total_borrowed:u64,
    pub total_interest_owed:u64,
    pub collateral_deposit:u64,

    //position health
    pub health_factor:u64
}