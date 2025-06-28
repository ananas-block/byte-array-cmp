use {
    light_zero_copy::cyclic_vec::ZeroCopyCyclicVecU64,
    mollusk_svm::Mollusk,
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    optimize_cmp::changelog::{Entry, GenericChangelog},
    rand::{Rng, SeedableRng},
    rand::rngs::StdRng,
    solana_account::Account,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
};

const PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12,
    0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
]);

// Test program instruction types - Built-in comparison
const FIND_AFTER_10_ITERATIONS_BUILTIN: u8 = 10;
const FIND_AFTER_100_ITERATIONS_BUILTIN: u8 = 11;
const FIND_NOT_FOUND_BUILTIN: u8 = 12;

// Test program instruction types - Manual comparison  
const FIND_AFTER_10_ITERATIONS_MANUAL: u8 = 13;
const FIND_AFTER_100_ITERATIONS_MANUAL: u8 = 14;
const FIND_NOT_FOUND_MANUAL: u8 = 15;

// Optimization instruction types
const OPTIMIZATION_UNROLLED: u8 = 20;
const OPTIMIZATION_SIMD: u8 = 21;
const OPTIMIZATION_BRANCHLESS: u8 = 22;
const OPTIMIZATION_UNSAFE: u8 = 23;
const OPTIMIZATION_UNROLLED_NOT_FOUND: u8 = 24;
const OPTIMIZATION_SIMD_100: u8 = 25;
const OPTIMIZATION_SIMD_1000_NOT_FOUND: u8 = 26;

// P-Token inspired optimization instruction types  
const PTOKEN_SOL_MEMCMP: u8 = 27;
const PTOKEN_U128_CAST: u8 = 28;
const PTOKEN_POINTER_EQUALITY: u8 = 29;
const PTOKEN_COMBINED_FAST: u8 = 30;
const PTOKEN_U128_CAST_100: u8 = 31;
const PTOKEN_U128_CAST_1000_NOT_FOUND: u8 = 32;

// SIMD iteration variants
const SIMD_ITERATOR: u8 = 34;
const SIMD_ZIP: u8 = 35;
const SIMD_SLICE: u8 = 36;
const SIMD_ITERATOR_100: u8 = 37;
const SIMD_ITERATOR_1000_NOT_FOUND: u8 = 38;

// Deterministic seed for consistent benchmark results
const BENCHMARK_SEED: u64 = 9876543210987654321;

fn create_random_mint(rng: &mut StdRng) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    bytes
}

fn create_changelog_account_data() -> (Vec<u8>, [u8; 32], [u8; 32], [u8; 32]) {
    let capacity = 1000u64;
    let mut backing_store =
        vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
    let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

    let mut rng = StdRng::seed_from_u64(BENCHMARK_SEED);

    // Create target keys that we'll search for
    let target_key_10 = create_random_mint(&mut rng); // Will be found after 10 iterations
    let target_key_100 = create_random_mint(&mut rng); // Will be found after 100 iterations
    let target_key_not_found = create_random_mint(&mut rng); // Will not be found

    // Fill with 1000 random entries
    for _i in 0..1000 {
        let mint = create_random_mint(&mut rng);
        let value = rng.gen::<u64>();
        changelog.push(Entry::new(mint, value));
    }

    // Insert target entries at specific positions from the end
    // Insert target_key_100 at position that will be found after 100 iterations
    for _ in 0..100 {
        let mint = create_random_mint(&mut rng);
        let value = rng.gen::<u64>();
        changelog.push(Entry::new(mint, value));
    }
    changelog.push(Entry::new(target_key_100, 12345));

    // Continue adding random entries
    for _ in 0..889 {
        // 1000 - 100 - 1 - 10 = 889
        let mint = create_random_mint(&mut rng);
        let value = rng.gen::<u64>();
        changelog.push(Entry::new(mint, value));
    }

    // Insert target_key_10 at position that will be found after 10 iterations
    for _ in 0..10 {
        let mint = create_random_mint(&mut rng);
        let value = rng.gen::<u64>();
        changelog.push(Entry::new(mint, value));
    }
    changelog.push(Entry::new(target_key_10, 54321));

    (
        backing_store,
        target_key_10,
        target_key_100,
        target_key_not_found,
    )
}

fn main() {
    // Disable logging for cleaner benchmark output
    solana_logger::setup_with("");

    let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/optimize_cmp");

    // Create changelog account with 1000 entries
    let (account_data, target_key_10, target_key_100, target_key_not_found) =
        create_changelog_account_data();

    // Create a changelog account
    let changelog_pubkey = Pubkey::new_unique();

    // Create instructions for each benchmark type - Built-in comparison
    let mut instruction_data_10_builtin = vec![FIND_AFTER_10_ITERATIONS_BUILTIN];
    instruction_data_10_builtin.extend_from_slice(&target_key_10);

    let mut instruction_data_100_builtin = vec![FIND_AFTER_100_ITERATIONS_BUILTIN];
    instruction_data_100_builtin.extend_from_slice(&target_key_100);

    let mut instruction_data_not_found_builtin = vec![FIND_NOT_FOUND_BUILTIN];
    instruction_data_not_found_builtin.extend_from_slice(&target_key_not_found);

    // Create instructions for each benchmark type - Manual comparison
    let mut instruction_data_10_manual = vec![FIND_AFTER_10_ITERATIONS_MANUAL];
    instruction_data_10_manual.extend_from_slice(&target_key_10);

    let mut instruction_data_100_manual = vec![FIND_AFTER_100_ITERATIONS_MANUAL];
    instruction_data_100_manual.extend_from_slice(&target_key_100);

    let mut instruction_data_not_found_manual = vec![FIND_NOT_FOUND_MANUAL];
    instruction_data_not_found_manual.extend_from_slice(&target_key_not_found);

    // Create optimization instruction data
    let mut instruction_data_unrolled = vec![OPTIMIZATION_UNROLLED];
    instruction_data_unrolled.extend_from_slice(&target_key_10);

    let mut instruction_data_simd = vec![OPTIMIZATION_SIMD];
    instruction_data_simd.extend_from_slice(&target_key_10);

    let mut instruction_data_branchless = vec![OPTIMIZATION_BRANCHLESS];
    instruction_data_branchless.extend_from_slice(&target_key_10);

    let mut instruction_data_unsafe = vec![OPTIMIZATION_UNSAFE];
    instruction_data_unsafe.extend_from_slice(&target_key_10);

    let mut instruction_data_unrolled_not_found = vec![OPTIMIZATION_UNROLLED_NOT_FOUND];
    instruction_data_unrolled_not_found.extend_from_slice(&target_key_not_found);

    let mut instruction_data_simd_100 = vec![OPTIMIZATION_SIMD_100];
    instruction_data_simd_100.extend_from_slice(&target_key_100);

    let mut instruction_data_simd_1000_not_found = vec![OPTIMIZATION_SIMD_1000_NOT_FOUND];
    instruction_data_simd_1000_not_found.extend_from_slice(&target_key_not_found);

    // P-Token optimization instruction data
    let mut instruction_data_ptoken_sol_memcmp = vec![PTOKEN_SOL_MEMCMP];
    instruction_data_ptoken_sol_memcmp.extend_from_slice(&target_key_10);

    let mut instruction_data_ptoken_u128_cast = vec![PTOKEN_U128_CAST];
    instruction_data_ptoken_u128_cast.extend_from_slice(&target_key_10);

    let mut instruction_data_ptoken_pointer_equality = vec![PTOKEN_POINTER_EQUALITY];
    instruction_data_ptoken_pointer_equality.extend_from_slice(&target_key_10);

    let mut instruction_data_ptoken_combined_fast = vec![PTOKEN_COMBINED_FAST];
    instruction_data_ptoken_combined_fast.extend_from_slice(&target_key_10);

    let mut instruction_data_ptoken_u128_cast_100 = vec![PTOKEN_U128_CAST_100];
    instruction_data_ptoken_u128_cast_100.extend_from_slice(&target_key_100);

    let mut instruction_data_ptoken_u128_cast_1000_not_found = vec![PTOKEN_U128_CAST_1000_NOT_FOUND];
    instruction_data_ptoken_u128_cast_1000_not_found.extend_from_slice(&target_key_not_found);

    // SIMD iteration instruction data
    let mut instruction_data_simd_iterator = vec![SIMD_ITERATOR];
    instruction_data_simd_iterator.extend_from_slice(&target_key_10);

    let mut instruction_data_simd_zip = vec![SIMD_ZIP];
    instruction_data_simd_zip.extend_from_slice(&target_key_10);

    let mut instruction_data_simd_slice = vec![SIMD_SLICE];
    instruction_data_simd_slice.extend_from_slice(&target_key_10);

    let mut instruction_data_simd_iterator_100 = vec![SIMD_ITERATOR_100];
    instruction_data_simd_iterator_100.extend_from_slice(&target_key_100);

    let mut instruction_data_simd_iterator_1000_not_found = vec![SIMD_ITERATOR_1000_NOT_FOUND];
    instruction_data_simd_iterator_1000_not_found.extend_from_slice(&target_key_not_found);

    let instruction_find_10_builtin = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_10_builtin,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_find_100_builtin = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_100_builtin,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_find_not_found_builtin = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_not_found_builtin,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_find_10_manual = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_10_manual,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_find_100_manual = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_100_manual,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_find_not_found_manual = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_not_found_manual,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    // Create optimization instructions
    let instruction_unrolled = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_unrolled,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_simd = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_simd,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_branchless = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_branchless,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_unsafe = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_unsafe,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_unrolled_not_found = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_unrolled_not_found,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_simd_100 = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_simd_100,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_simd_1000_not_found = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_simd_1000_not_found,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    // P-Token optimization instructions
    let instruction_ptoken_sol_memcmp = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_ptoken_sol_memcmp,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_ptoken_u128_cast = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_ptoken_u128_cast,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_ptoken_pointer_equality = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_ptoken_pointer_equality,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_ptoken_combined_fast = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_ptoken_combined_fast,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_ptoken_u128_cast_100 = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_ptoken_u128_cast_100,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_ptoken_u128_cast_1000_not_found = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_ptoken_u128_cast_1000_not_found,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    // SIMD iteration instructions
    let instruction_simd_iterator = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_simd_iterator,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_simd_zip = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_simd_zip,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_simd_slice = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_simd_slice,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_simd_iterator_100 = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_simd_iterator_100,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_simd_iterator_1000_not_found = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_simd_iterator_1000_not_found,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    // Create accounts with the changelog data - convert Vec<u8> to Account
    let create_account = |data: Vec<u8>| Account {
        lamports: 0,
        data,
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };

    let accounts_10_builtin = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_100_builtin = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_not_found_builtin = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_10_manual = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_100_manual = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_not_found_manual = vec![(changelog_pubkey, create_account(account_data.clone()))];
    
    // Optimization accounts
    let accounts_unrolled = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_simd = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_branchless = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_unsafe = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_unrolled_not_found = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_simd_100 = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_simd_1000_not_found = vec![(changelog_pubkey, create_account(account_data.clone()))];
    
    // P-Token optimization accounts
    let accounts_ptoken_sol_memcmp = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_ptoken_u128_cast = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_ptoken_pointer_equality = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_ptoken_combined_fast = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_ptoken_u128_cast_100 = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_ptoken_u128_cast_1000_not_found = vec![(changelog_pubkey, create_account(account_data.clone()))];
    
    // SIMD iteration accounts
    let accounts_simd_iterator = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_simd_zip = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_simd_slice = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_simd_iterator_100 = vec![(changelog_pubkey, create_account(account_data.clone()))];
    let accounts_simd_iterator_1000_not_found = vec![(changelog_pubkey, create_account(account_data))];

    MolluskComputeUnitBencher::new(mollusk)
        .bench((
            "find_after_10_iterations_builtin",
            &instruction_find_10_builtin,
            &accounts_10_builtin,
        ))
        .bench((
            "find_after_100_iterations_builtin",
            &instruction_find_100_builtin,
            &accounts_100_builtin,
        ))
        .bench((
            "find_not_found_builtin",
            &instruction_find_not_found_builtin,
            &accounts_not_found_builtin,
        ))
        .bench((
            "find_after_10_iterations_manual",
            &instruction_find_10_manual,
            &accounts_10_manual,
        ))
        .bench((
            "find_after_100_iterations_manual",
            &instruction_find_100_manual,
            &accounts_100_manual,
        ))
        .bench((
            "find_not_found_manual",
            &instruction_find_not_found_manual,
            &accounts_not_found_manual,
        ))
        .bench((
            "optimization_unrolled",
            &instruction_unrolled,
            &accounts_unrolled,
        ))
        .bench((
            "optimization_simd",
            &instruction_simd,
            &accounts_simd,
        ))
        .bench((
            "optimization_branchless",
            &instruction_branchless,
            &accounts_branchless,
        ))
        .bench((
            "optimization_unsafe",
            &instruction_unsafe,
            &accounts_unsafe,
        ))
        .bench((
            "optimization_unrolled_not_found",
            &instruction_unrolled_not_found,
            &accounts_unrolled_not_found,
        ))
        .bench((
            "optimization_simd_100",
            &instruction_simd_100,
            &accounts_simd_100,
        ))
        .bench((
            "optimization_simd_1000_not_found",
            &instruction_simd_1000_not_found,
            &accounts_simd_1000_not_found,
        ))
        .bench((
            "ptoken_sol_memcmp",
            &instruction_ptoken_sol_memcmp,
            &accounts_ptoken_sol_memcmp,
        ))
        .bench((
            "ptoken_u128_cast",
            &instruction_ptoken_u128_cast,
            &accounts_ptoken_u128_cast,
        ))
        .bench((
            "ptoken_pointer_equality",
            &instruction_ptoken_pointer_equality,
            &accounts_ptoken_pointer_equality,
        ))
        .bench((
            "ptoken_combined_fast",
            &instruction_ptoken_combined_fast,
            &accounts_ptoken_combined_fast,
        ))
        .bench((
            "ptoken_u128_cast_100",
            &instruction_ptoken_u128_cast_100,
            &accounts_ptoken_u128_cast_100,
        ))
        .bench((
            "ptoken_u128_cast_1000_not_found",
            &instruction_ptoken_u128_cast_1000_not_found,
            &accounts_ptoken_u128_cast_1000_not_found,
        ))
        .bench((
            "simd_iterator",
            &instruction_simd_iterator,
            &accounts_simd_iterator,
        ))
        .bench((
            "simd_zip",
            &instruction_simd_zip,
            &accounts_simd_zip,
        ))
        .bench((
            "simd_slice",
            &instruction_simd_slice,
            &accounts_simd_slice,
        ))
        .bench((
            "simd_iterator_100",
            &instruction_simd_iterator_100,
            &accounts_simd_iterator_100,
        ))
        .bench((
            "simd_iterator_1000_not_found",
            &instruction_simd_iterator_1000_not_found,
            &accounts_simd_iterator_1000_not_found,
        ))
        .must_pass(true)
        .out_dir("target/benches")
        .execute();
}
