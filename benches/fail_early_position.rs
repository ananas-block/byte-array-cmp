use {
    light_zero_copy::cyclic_vec::ZeroCopyCyclicVecU64,
    mollusk_svm::Mollusk,
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    optimize_cmp::changelog::{Entry, GenericChangelog},
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

// Use SIMD iterator and PartialEq for comparison
const SIMD_ITERATOR: u8 = 34;
const FIND_AFTER_10_ITERATIONS_PARTIALEQ: u8 = 10;

/// Creates a changelog account with exactly 1 entry
/// The entry's key can have a different byte at the specified position
fn create_single_entry_changelog(differ_at_position: Option<usize>) -> ([u8; 32], Account) {
    let capacity = 10u64;
    let mut backing_store =
        vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
    let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

    // Create base key: use program ID bytes
    let mut entry_key = [9u8; 32];

    // If specified, make one byte different
    if let Some(pos) = differ_at_position {
        entry_key[pos] = entry_key[pos].wrapping_add(1); // Add 1 with wrapping
    }

    // Add the single entry
    changelog.push(Entry::new(entry_key, 12345));

    // Create the target key we'll search for (always original program ID)
    let target_key = [9u8; 32];

    let account = Account {
        lamports: 0,
        data: backing_store,
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };

    (target_key, account)
}

fn main() {
    // Disable logging for cleaner benchmark output
    solana_logger::setup_with("");

    let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/optimize_cmp");
    let changelog_pubkey = Pubkey::new_unique();

    let mut benchmark_data = Vec::new();

    // Test equal case first (no difference)
    {
        let (target_key, account) = create_single_entry_changelog(None);
        let accounts = vec![(changelog_pubkey, account)];

        // SIMD iterator instruction for equal case
        let mut simd_instruction_data = vec![SIMD_ITERATOR];
        simd_instruction_data.extend_from_slice(&target_key);

        let simd_instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &simd_instruction_data,
            vec![AccountMeta::new(changelog_pubkey, false)],
        );

        benchmark_data.push((
            "simd_equal_case".to_string(),
            simd_instruction,
            accounts.clone(),
        ));

        // PartialEq instruction for equal case
        let mut partialeq_instruction_data = vec![FIND_AFTER_10_ITERATIONS_PARTIALEQ];
        partialeq_instruction_data.extend_from_slice(&target_key);

        let partialeq_instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &partialeq_instruction_data,
            vec![AccountMeta::new(changelog_pubkey, false)],
        );

        benchmark_data.push((
            "partialeq_equal_case".to_string(),
            partialeq_instruction,
            accounts,
        ));
    }

    // Test all fail positions
    for i in 0..32 {
        let (target_key, account) = create_single_entry_changelog(Some(i));

        // Create accounts vector with the changelog account
        let accounts = vec![(changelog_pubkey, account)];

        // SIMD iterator instruction
        let mut simd_instruction_data = vec![SIMD_ITERATOR];
        simd_instruction_data.extend_from_slice(&target_key);

        let simd_instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &simd_instruction_data,
            vec![AccountMeta::new(changelog_pubkey, false)],
        );

        let simd_name = format!("simd_fail_at_position_{:02}", i);
        benchmark_data.push((simd_name, simd_instruction, accounts.clone()));

        // PartialEq instruction
        let mut partialeq_instruction_data = vec![FIND_AFTER_10_ITERATIONS_PARTIALEQ];
        partialeq_instruction_data.extend_from_slice(&target_key);

        let partialeq_instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &partialeq_instruction_data,
            vec![AccountMeta::new(changelog_pubkey, false)],
        );

        let partialeq_name = format!("partialeq_fail_at_position_{:02}", i);
        benchmark_data.push((partialeq_name, partialeq_instruction, accounts));
    }

    // Run all benchmarks
    let mut bencher = MolluskComputeUnitBencher::new(mollusk);
    for (name, instruction, accounts) in &benchmark_data {
        bencher = bencher.bench((name.as_str(), instruction, accounts));
    }

    // Execute all benchmarks
    bencher.must_pass(true).out_dir("target/benches").execute();
}
