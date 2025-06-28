use {
    light_zero_copy::cyclic_vec::ZeroCopyCyclicVecU64,
    mollusk_svm::Mollusk,
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

const SIMD_ITERATOR_CU_TRACKING: u8 = 39;
const BENCHMARK_SEED: u64 = 9876543210987654321;

fn create_random_mint(rng: &mut StdRng) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    bytes
}

fn main() {
    // Enable verbose logging to see CU tracking
    solana_logger::setup_with("debug");

    let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/optimize_cmp");

    // Create a small changelog with 100 entries
    let capacity = 100u64;
    let mut backing_store =
        vec![0u8; ZeroCopyCyclicVecU64::<Entry>::required_size_for_capacity(capacity)];
    let mut changelog = GenericChangelog::new(capacity, &mut backing_store).unwrap();

    let mut rng = StdRng::seed_from_u64(BENCHMARK_SEED);

    // Create a target key that will be found after 10 iterations
    let target_key_10 = create_random_mint(&mut rng);

    // Fill with some random entries
    for _i in 0..90 {
        let mint = create_random_mint(&mut rng);
        let value = rng.gen::<u64>();
        changelog.push(Entry::new(mint, value));
    }

    // Add 10 more entries, with our target at the end
    for _ in 0..9 {
        let mint = create_random_mint(&mut rng);
        let value = rng.gen::<u64>();
        changelog.push(Entry::new(mint, value));
    }
    changelog.push(Entry::new(target_key_10, 12345));

    // Create instruction data
    let mut instruction_data = vec![SIMD_ITERATOR_CU_TRACKING];
    instruction_data.extend_from_slice(&target_key_10);

    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID,
        &instruction_data,
        vec![AccountMeta::new(Pubkey::new_unique(), false)],
    );

    let account = Account {
        lamports: 0,
        data: backing_store,
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };

    let accounts = vec![(Pubkey::new_unique(), account)];

    println!("=== Running SIMD Iterator with detailed CU tracking ===");
    
    let result = mollusk.process_instruction(&instruction, &accounts);
    
    match result.raw_result {
        Ok(()) => {
            println!("Program executed successfully");
            println!("Total CU consumed: {}", result.compute_units_consumed);
        }
        Err(e) => {
            println!("Program execution failed: {:?}", e);
        }
    }
}