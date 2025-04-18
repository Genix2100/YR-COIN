use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{ProgramResult, entrypoint},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    msg,
};
use solana_program::program_error::ProgramError;

#[inline(always)]
fn create_token_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("Creating YR Coin Token account with initial amount {}", amount);
    
    let rent_lamports = rent.minimum_balance(0);
    let instruction = system_instruction::create_account(
        payer_account.key,
        token_account.key,
        rent_lamports,
        0,
        &program_id,
    );
    solana_program::program::invoke_signed(
        &instruction,
        &[payer_account.clone(), token_account.clone()],
        &[&[&payer_account.key.as_ref()]],
    )?;

    Ok(())
}

#[inline(always)]
fn mint_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;

    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("Minting {} YR Coins to account {}", amount, token_account.key);
    // Minting logic here, the details depend on token implementation
    Ok(())
}

#[inline(always)]
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction_type = instruction_data[0];
    match instruction_type {
        0 => create_token_account(program_id, accounts, 69_000_000_000), // Example amount
        1 => mint_token(program_id, accounts, 100), // Example mint amount
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

entrypoint!(process_instruction);
