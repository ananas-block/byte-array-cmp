use {
    light_zero_copy::cyclic_vec::ZeroCopyCyclicVecU64,
    mollusk_svm::Mollusk,
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    optimize_cmp::changelog::{Entry, GenericChangelog},
    rand::{thread_rng, RngCore},
    solana_account::Account,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
};

const PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12,
    0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0xe, 0x1f, 0x20,
]);

// Use SIMD iterator and PartialEq for comparison
const SIMD_ITERATOR: u8 = 34;
const FIND_AFTER_10_ITERATIONS_PARTIALEQ: u8 = 10;

/// Creates a changelog account with exactly 1 entry using a random key
/// Both entry_key and target_key will be identical (equal case)
fn create_random_equal_changelog() -> ([u8; 32], Account) {
    let capacity = 10u64;
    let mut backing_store =
        vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
    let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

    // Generate random 32-byte key
    let mut rng = thread_rng();
    let mut random_key = [0u8; 32];
    rng.fill_bytes(&mut random_key);

    // Add the single entry with the random key
    changelog.push(Entry::new(random_key, 12345));

    // Use the same key as target (equal case)
    let target_key = random_key;

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

    // Generate 100 random key pairs for equal case testing
    for i in 0..100 {
        let (target_key, account) = create_random_equal_changelog();
        let accounts = vec![(changelog_pubkey, account)];
        println!("target key {:?}", target_key);
        // SIMD iterator instruction for equal case
        let mut simd_instruction_data = vec![SIMD_ITERATOR];
        simd_instruction_data.extend_from_slice(&target_key);

        let simd_instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &simd_instruction_data,
            vec![AccountMeta::new(changelog_pubkey, false)],
        );

        benchmark_data.push((
            format!("simd_equal_random_{:03}", i),
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
            format!("partialeq_equal_random_{:03}", i),
            partialeq_instruction,
            accounts,
        ));
    }

    // Run all benchmarks
    let mut bencher = MolluskComputeUnitBencher::new(mollusk);
    for (name, instruction, accounts) in &benchmark_data {
        bencher = bencher.bench((name.as_str(), instruction, accounts));
    }

    // Execute all benchmarks
    bencher.must_pass(true).out_dir("target/benches").execute();
}
