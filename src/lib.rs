use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{AccountInfo, next_account_info},
     msg,
    entrypoint,
    pubkey::Pubkey,
entrypoint::{ProgramResult},
};



entrypoint!(process_instruction);

#[derive(BorshDeserialize,BorshSerialize)]
struct OnChainData {
    count : u32
}

fn process_instruction(
    program_id : &Pubkey,
    accounts: &[AccountInfo],
    instruction_data : &[u8]
)-> ProgramResult {
  let  data_account =  next_account_info(&mut accounts.iter())?;


  let mut counter = OnChainData::try_from_slice(&data_account.data.borrow_mut())?; // bytes u8

  if counter.count == 0 {
    counter.count = 1;
  } else {
    counter.count *= 2;
  }

  counter.serialize(&mut *data_account.data.borrow_mut())?;
  msg!("everything went well");
    Ok(())
}