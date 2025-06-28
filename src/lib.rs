pub mod changelog;
mod comparisons;

use solana_program::{
    account_info::AccountInfo,
    entrypoint::{entrypoint, ProgramResult},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use comparisons::{
    benchmark_default_comparison, benchmark_manual_loop, benchmark_unrolled_comparison,
    benchmark_unsafe_pointer,
};

use changelog::{Entry, GenericChangelog};
use zerocopy::IntoBytes;

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    match instruction_data[0] {
        // Comparison benchmarks (1-4) and reference test (33)
        1..=4 | 33 => {
            // Test data - 32-byte arrays
            let array1: [u8; 32] = [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32,
            ];

            let array2: [u8; 32] = [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 33, // Different last byte
            ];

            let array3: [u8; 32] = [
                2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, // Different first byte
                17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
            ];

            match instruction_data[0] {
                1 => benchmark_default_comparison(&array1, &array2, &array3),
                2 => benchmark_manual_loop(&array1, &array2, &array3),
                3 => benchmark_unrolled_comparison(&array1, &array2, &array3),
                4 => benchmark_unsafe_pointer(&array1, &array2, &array3),
                _ => unreachable!(),
            }
        }

        // Changelog benchmarks (10-15), optimizations (20-26), p-token optimizations (27-33), and SIMD iterations (34-39)
        10..=15 | 20..=39 => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }

            if instruction_data.len() < 33 {
                // 1 byte instruction + 32 bytes key
                return Err(ProgramError::InvalidInstructionData);
            }

            let changelog_account = &accounts[0];
            let target_key: [u8; 32] = instruction_data[1..33]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            // Deserialize changelog from account data
            let mut data = changelog_account.data.borrow_mut();
            let changelog: GenericChangelog<'_, Entry> =
                GenericChangelog::from_bytes(data.as_mut_bytes())
                    .map_err(|_| ProgramError::InvalidAccountData)?;
            match instruction_data[0] {
                10 => {
                    let result = changelog.find_latest::<false>(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                11 => {
                    let result = changelog.find_latest::<false>(target_key, Some(100));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                12 => {
                    let result = changelog.find_latest::<false>(target_key, None);
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                13 => {
                    let result = changelog.find_latest::<true>(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                14 => {
                    let result = changelog.find_latest::<true>(target_key, Some(100));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                15 => {
                    let result = changelog.find_latest::<true>(target_key, None);
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                20 => {
                    let result = changelog.find_latest_unrolled(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                21 => {
                    let result = changelog.find_latest_simd(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                22 => {
                    let result = changelog.find_latest_branchless(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                23 => {
                    let result = changelog.find_latest_unsafe(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                24 => {
                    let result = changelog.find_latest_unrolled(target_key, None);
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                25 => {
                    let result = changelog.find_latest_simd(target_key, Some(100));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                26 => {
                    let result = changelog.find_latest_simd(target_key, None);
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                27 => {
                    let result = changelog.find_latest_sol_memcmp(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                28 => {
                    let result = changelog.find_latest_u128_cast(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                29 => {
                    let result = changelog.find_latest_pointer_equality(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                30 => {
                    let result = changelog.find_latest_combined_fast(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                31 => {
                    let result = changelog.find_latest_u128_cast(target_key, Some(100));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                32 => {
                    let result = changelog.find_latest_u128_cast(target_key, None);
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                33 => {
                    // Test reference vs value comparison
                    let array1: [u8; 32] = [1; 32];
                    let array2: [u8; 32] = [1; 32];
                    let array3: [u8; 32] = [2; 32];

                    // Test various comparison methods
                    let ref1 = &array1;
                    let ref2 = &array2;
                    let ref3 = &array3;

                    // Reference comparison
                    let _result1 = ref1 == ref2; // Same content, different memory locations
                    let _result2 = ref1 == ref3; // Different content
                    let _result3 = ref1 == &array1; // Same content, same memory location

                    // Pointer comparison for reference
                    let _result4 = std::ptr::eq(ref1, ref2);
                    let _result5 = std::ptr::eq(ref1, &array1);
                }
                34 => {
                    let result = changelog.find_latest_simd_iterator(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                35 => {
                    let result = changelog.find_latest_simd_zip(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                36 => {
                    let result = changelog.find_latest_simd_slice(target_key, Some(10));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                37 => {
                    let result = changelog.find_latest_simd_iterator(target_key, Some(100));
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                38 => {
                    let result = changelog.find_latest_simd_iterator(target_key, None);
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                39 => {
                    msg!("start");
                    let result = changelog.find_latest_direct_field_access(target_key);
                    msg!("end");
                    if let Some(_value) = result {
                        // Found value, using it for computation
                    }
                }
                _ => unreachable!(),
            }
        }

        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}
