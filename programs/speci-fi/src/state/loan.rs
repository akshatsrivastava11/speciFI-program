use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Loan{
    pub borrower:Pubkey,
    pub total_amount:u64,
    pub protocol:Pubkey,
    pub bump_loan:u8,
    pub asset:Pubkey,
    pub payment_per_period:u64,
    pub amount_paid:u64,
    pub due_data:i64,
    pub collateral:CollateralType,
    pub status:LoanStatus,
    pub kamino_obligation:Option<Pubkey>
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone)]
pub enum  CollateralType{
    NFT,
    TOKEN
}
impl Space for CollateralType{
    const INIT_SPACE: usize = 32;
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone)]
pub enum LoanStatus {
    Active,
    Completed,
    Liquidated
}
impl Space for LoanStatus{
    const INIT_SPACE: usize = 1;
}