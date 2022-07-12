use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer, TokenAccount, Token};

declare_id!("7Hqh6QVDRomntKJYwvQNH2xxwG1AfRxsyeK3E1r1yxjC");

#[program]
pub mod mymoneydapp {
    use super::*;

    pub fn proxy_transfer(ctx: Context<ProxyTransfer>, amount: u64) -> ProgramResult {
        token::transfer(ctx.accounts.into(), amount)
    }
    
    pub fn proxy_mint_to(ctx: Context<ProxyMintTo>, amount: u64) -> ProgramResult {
        token::mint_to(ctx.accounts.into(), amount)
    }
    
    pub fn proxy_burn(ctx: Context<ProxyBurn>, amount: u64) -> ProgramResult {
        token::burn(ctx.accounts.into(), amount)
    }
    
    pub fn proxy_set_authority(
        ctx: Context<ProxySetAuthority>,
        authority_type: AuthorityType,
        new_authority: Option<Pubkey>,
    ) -> ProgramResult {
        token::set_authority(ctx.accounts.into(), authority_type.into(), new_authority)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType {
    MintTokens,     // Authority to mint new tokens
    FreezeAccount, // Authority to freeze any account associated with the Mint
    AccountOwner, // Owner of a given token account
    CloseAccount, // Authority to close a token account
}

#[derive(Accounts)]
pub struct ProxyTransfer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub from:Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to: Box<Account<'info, TokenAccount>>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyMintTo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ProxyBurn<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ProxySetAuthority<'info> {
    #[account(mut)]
    pub current_authority: Signer<'info>,
    #[account(mut)]
    pub account_or_mint: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyTransfer<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut ProxyTransfer<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.from.to_account_info().clone(),
            to: accounts.to.to_account_info().clone(),
            authority: accounts.authority.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.to_account_info().clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyMintTo<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(accounts: &mut ProxyMintTo<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.mint.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.to_account_info().clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyBurn<'info>> for CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
    fn from(accounts: &mut ProxyBurn<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: accounts.mint.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.to_account_info().clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxySetAuthority<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>>
{
    fn from(
        accounts: &mut ProxySetAuthority<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts.account_or_mint.clone(),
            current_authority: accounts.current_authority.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.to_account_info().clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl From<AuthorityType> for spl_token::instruction::AuthorityType {
    fn from(authority_ty: AuthorityType) -> spl_token::instruction::AuthorityType {
        match authority_ty {
            AuthorityType::MintTokens => spl_token::instruction::AuthorityType::MintTokens,
            AuthorityType::FreezeAccount => spl_token::instruction::AuthorityType::FreezeAccount,
            AuthorityType::AccountOwner => spl_token::instruction::AuthorityType::AccountOwner,
            AuthorityType::CloseAccount => spl_token::instruction::AuthorityType::CloseAccount,
        }
    }
}
