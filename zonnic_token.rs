// Sealana Token
// =============

// Import necessary dependencies
use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    spl_token::{
        error::TokenError,
        state::{Account, Mint},
    },
};

// Define the Sealana token struct
pub struct SealanaToken;

// Define the token's mint and decimals
const MINT: Pubkey = Pubkey::new_unique();
const DECIMALS: u8 = 9;

// Define the token's metadata
const NAME: &str = "Sealana";
const SYMBOL: &str = "SEAL";

// Implement the Sealana token's logic
impl SealanaToken {
    // Initialize the token mint
    fn init_mint(accounts: &[AccountInfo]) -> ProgramResult {
        if let Err(error) = spl_token::state::Mint::create_cpi(
            accounts,
            MINT,
            DECIMALS,
            &spl_token::state::MintTo {
                mint: MINT,
                to: accounts[0].key,
            },
        ) {
            msg!("Error creating mint: {}", error);
            return Err(ProgramError::Custom(error as u32));
        }
        Ok(())
    }

    // Mint tokens to a user
    fn mint_to(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        if let Err(error) = spl_token::instruction::mint_to(
            accounts,
            MINT,
            amount,
            &spl_token::state::MintTo {
                mint: MINT,
                to: accounts[0].key,
            },
        ) {
            msg!("Error minting tokens: {}", error);
            return Err(ProgramError::Custom(error as u32));
        }
        Ok(())
    }

    // Burn tokens from a user
    fn burn(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        if let Err(error) = spl_token::instruction::burn(
            accounts,
            MINT,
            amount,
            &spl_token::state::Burn {
                mint: MINT,
                from: accounts[0].key,
            },
        ) {
            msg!("Error burning tokens: {}", error);
            return Err(ProgramError::Custom(error as u32));
        }
        Ok(())
    }
}

// Define the entry point for the Sealana token program
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() != 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let instruction = instruction_data[0];

    match instruction {
        0 => {
            // Initialize mint
            SealanaToken::init_mint(accounts)
        }
        1 => {
            // Mint tokens
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            SealanaToken::mint_to(accounts, amount)
        }
        2 => {
            // Burn tokens
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            SealanaToken::burn(accounts, amount)
        }
        _ => Err(ProgramError::InvalidInstruction),
    }
}
