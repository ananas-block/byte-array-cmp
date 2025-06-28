mod comparisons;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use comparisons::{
    benchmark_default_comparison,
    benchmark_manual_loop,
    benchmark_unrolled_comparison,
    benchmark_unsafe_pointer,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Test data - 32-byte arrays
    let array1: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    ];
    
    let array2: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33 // Different last byte
    ];

    let array3: [u8; 32] = [
        2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, // Different first byte
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    ];

    match instruction_data[0] {
        1 => benchmark_default_comparison(&array1, &array2, &array3),
        2 => benchmark_manual_loop(&array1, &array2, &array3),
        3 => benchmark_unrolled_comparison(&array1, &array2, &array3),
        4 => benchmark_unsafe_pointer(&array1, &array2, &array3),
        _ => {
            msg!("Invalid test number. Use 1-4");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}