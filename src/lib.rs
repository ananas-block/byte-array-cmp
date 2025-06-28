pub mod changelog;
mod comparisons;

use solana_program::{
    account_info::AccountInfo,
    entrypoint::{entrypoint, ProgramResult},
    log::sol_log_compute_units,
    //msg,
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

        // Changelog benchmarks (10-15), optimizations (20-26), p-token optimizations (27-33), and SIMD iterations (34-38)
        10..=15 | 20..=38 => {
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
                    //msg!("=== Changelog Find After 10 Iterations (Built-in) ===");
                    let result = changelog.find_latest::<false>(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                11 => {
                    //msg!("=== Changelog Find After 100 Iterations (Built-in) ===");
                    let result = changelog.find_latest::<false>(target_key, Some(100));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                12 => {
                    //msg!("=== Changelog Find Not Found (Built-in) ===");
                    let result = changelog.find_latest::<false>(target_key, None);
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                13 => {
                    //msg!("=== Changelog Find After 10 Iterations (Manual) ===");
                    let result = changelog.find_latest::<true>(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                14 => {
                    //msg!("=== Changelog Find After 100 Iterations (Manual) ===");
                    let result = changelog.find_latest::<true>(target_key, Some(100));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                15 => {
                    //msg!("=== Changelog Find Not Found (Manual) ===");
                    let result = changelog.find_latest::<true>(target_key, None);
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                20 => {
                    //msg!("=== Optimization: Unrolled Comparison ===");
                    let result = changelog.find_latest_unrolled(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                21 => {
                    //msg!("=== Optimization: SIMD-style Comparison ===");
                    let result = changelog.find_latest_simd(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                22 => {
                    //msg!("=== Optimization: Branchless Comparison ===");
                    let result = changelog.find_latest_branchless(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                23 => {
                    //msg!("=== Optimization: Unsafe Fast Comparison ===");
                    let result = changelog.find_latest_unsafe(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                24 => {
                    //msg!("=== Optimization: Unrolled Not Found Test ===");
                    let result = changelog.find_latest_unrolled(target_key, None);
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                25 => {
                    //msg!("=== Optimization: SIMD 100 Iterations ===");
                    let result = changelog.find_latest_simd(target_key, Some(100));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                26 => {
                    //msg!("=== Optimization: SIMD 1000 Iterations (Not Found) ===");
                    let result = changelog.find_latest_simd(target_key, None);
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                27 => {
                    //msg!("=== P-Token: sol_memcmp Optimization ===");
                    let result = changelog.find_latest_sol_memcmp(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                28 => {
                    //msg!("=== P-Token: u128 Cast Optimization ===");
                    let result = changelog.find_latest_u128_cast(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                29 => {
                    //msg!("=== P-Token: Pointer Equality Optimization ===");
                    let result = changelog.find_latest_pointer_equality(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                30 => {
                    //msg!("=== P-Token: Combined Fast Optimization ===");
                    let result = changelog.find_latest_combined_fast(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                31 => {
                    //msg!("=== P-Token: u128 Cast 100 Iterations ===");
                    let result = changelog.find_latest_u128_cast(target_key, Some(100));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                32 => {
                    //msg!("=== P-Token: u128 Cast 1000 Iterations (Not Found) ===");
                    let result = changelog.find_latest_u128_cast(target_key, None);
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                33 => {
                    //msg!("=== Reference Comparison Test ===");
                    // Test reference vs value comparison
                    let array1: [u8; 32] = [1; 32];
                    let array2: [u8; 32] = [1; 32];
                    let array3: [u8; 32] = [2; 32];

                    // Test various comparison methods
                    let ref1 = &array1;
                    let ref2 = &array2;
                    let ref3 = &array3;

                    // Reference comparison (what you're asking about)
                    let result1 = ref1 == ref2; // Same content, different memory locations
                    let result2 = ref1 == ref3; // Different content
                    let result3 = ref1 == &array1; // Same content, same memory location

                    //msg!("Ref comparison same content: {}", result1);
                    //msg!("Ref comparison diff content: {}", result2);
                    //msg!("Ref vs original: {}", result3);

                    // Pointer comparison for reference
                    let result4 = std::ptr::eq(ref1, ref2);
                    let result5 = std::ptr::eq(ref1, &array1);

                    //msg!("Pointer eq same content: {}", result4);
                    //msg!("Pointer eq same memory: {}", result5);
                }
                34 => {
                    //msg!("=== SIMD Iterator Optimization ===");
                    let result = changelog.find_latest_simd_iterator(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                35 => {
                    //msg!("=== SIMD Zip Iterator Optimization ===");
                    let result = changelog.find_latest_simd_zip(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                36 => {
                    //msg!("=== SIMD Slice Comparison Optimization ===");
                    let result = changelog.find_latest_simd_slice(target_key, Some(10));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                37 => {
                    //msg!("=== SIMD Iterator 100 Iterations ===");
                    let result = changelog.find_latest_simd_iterator(target_key, Some(100));
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                38 => {
                    //msg!("=== SIMD Iterator 1000 Iterations (Not Found) ===");
                    let result = changelog.find_latest_simd_iterator(target_key, None);
                    //msg!("Found: {:?}", result.is_some());
                    if let Some(value) = result {
                        //msg!("Value: {}", value);
                    }
                }
                _ => unreachable!(),
            }
        }

        _ => {
            //msg!("Invalid instruction. Use 1-4 for comparisons, 10-15 for changelog, 20-26 for optimizations, 27-38 for p-token/SIMD optimizations");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}
