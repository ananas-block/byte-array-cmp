use {
    light_zero_copy::cyclic_vec::ZeroCopyCyclicVecU64,
    mollusk_svm::Mollusk,
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    optimize_cmp::changelog::{Entry, GenericChangelog},
    rand::rngs::StdRng,
    rand::{Rng, SeedableRng},
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

// Integer variant instruction types (1000 element searches)
const SIMD_ITERATOR_U16_1000_NOT_FOUND: u8 = 40;
const SIMD_ITERATOR_U32_1000_NOT_FOUND: u8 = 41;
const SIMD_ITERATOR_U128_1000_NOT_FOUND: u8 = 42;

// For comparison with existing u64 variant
const SIMD_ITERATOR_U64_1000_NOT_FOUND: u8 = 38; // Reuse existing

// Deterministic seed for consistent benchmark results
const BENCHMARK_SEED: u64 = 9876543210987654321;

fn create_random_mint(rng: &mut StdRng) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    bytes
}

fn create_changelog_account_data_1000() -> (Vec<u8>, [u8; 32]) {
    let capacity = 1000u64;
    let mut backing_store =
        vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
    let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

    let mut rng = StdRng::seed_from_u64(BENCHMARK_SEED);

    // Create target key that will not be found
    let target_key_not_found = create_random_mint(&mut rng);

    // Fill with 1000 random entries (none matching target_key_not_found)
    for _i in 0..1000 {
        let mint = create_random_mint(&mut rng);
        let value = rng.gen::<u64>();
        changelog.push(Entry::new(mint, value));
    }

    (backing_store, target_key_not_found)
}

fn main() {
    // Disable logging for cleaner benchmark output
    solana_logger::setup_with("");

    let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/optimize_cmp");

    // Create changelog account with 1000 entries
    let (account_data, target_key_not_found) = create_changelog_account_data_1000();

    // Create a changelog account
    let changelog_pubkey = Pubkey::new_unique();

    // Create instruction data for each integer variant
    let mut instruction_data_u16 = vec![SIMD_ITERATOR_U16_1000_NOT_FOUND];
    instruction_data_u16.extend_from_slice(&target_key_not_found);

    let mut instruction_data_u32 = vec![SIMD_ITERATOR_U32_1000_NOT_FOUND];
    instruction_data_u32.extend_from_slice(&target_key_not_found);

    let mut instruction_data_u64 = vec![SIMD_ITERATOR_U64_1000_NOT_FOUND];
    instruction_data_u64.extend_from_slice(&target_key_not_found);

    let mut instruction_data_u128 = vec![SIMD_ITERATOR_U128_1000_NOT_FOUND];
    instruction_data_u128.extend_from_slice(&target_key_not_found);

    // Create instructions
    let instruction_u16 = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_u16,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_u32 = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_u32,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_u64 = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_u64,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    let instruction_u128 = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data_u128,
        vec![AccountMeta::new(changelog_pubkey, false)],
    );

    // Create accounts
    let accounts = vec![(
        changelog_pubkey,
        Account {
            lamports: 0,
            data: account_data,
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        },
    )];

    // Run benchmarks
    MolluskComputeUnitBencher::new(mollusk)
        .bench(("simd_iterator_u16_1000_not_found", &instruction_u16, &accounts))
        .bench(("simd_iterator_u32_1000_not_found", &instruction_u32, &accounts))
        .bench(("simd_iterator_u64_1000_not_found", &instruction_u64, &accounts))
        .bench(("simd_iterator_u128_1000_not_found", &instruction_u128, &accounts))
        .must_pass(true)
        .out_dir("target/benches")
        .execute();
}