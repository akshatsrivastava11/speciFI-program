use anchor_lang::{
    prelude::*,
    solana_program::{
        clock::{self},
    },
};
use anchor_spl::token::{Token, TokenAccount};

use crate::{CollateralType, Loan, LoanStatus, Protocol};

const SECONDS_PER_DAY: i64 = 86400;

#[derive(Accounts)]
pub struct CreateLoan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,
    #[account(
        seeds=[b"protocol"],
        bump
    )]
    pub protocol: Account<'info, Protocol>,

    #[account(
        init,
        payer=borrower,
        space=8+Loan::INIT_SPACE,
        seeds=[b"loan",protocol.key().as_ref(),borrower.key().as_ref()],
        bump
    )]
    pub loan: Account<'info, Loan>,

    pub Ata_Asset: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateLoan<'info> {
    pub fn initialize_loan_accounts(
        &mut self,
        total_amount_of_loan: u64,
        payment_per_period: u64,
        collateral: CollateralType,
        bumps: CreateLoanBumps,
    ) -> Result<()> {
        let clock = Clock::get()?;
        self.loan.set_inner(Loan {
            borrower: self.borrower.key(),
            total_amount: total_amount_of_loan,
            protocol: self.protocol.key(),
            bump_loan: bumps.loan,
            payment_per_period: payment_per_period,
            amount_paid: 0,
            due_data: clock.unix_timestamp + (SECONDS_PER_DAY * 30),
            collateral: collateral,
            status: LoanStatus::Active,
            asset: self.Ata_Asset.key(),
            kamino_obligation: None,
        });
        Ok(())
    }

    // pub fn create_loan(&mut self)->Result<()>{
    //     const 
    // }

}
