use {
    mollusk_svm::Mollusk,
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
};

const PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12,
    0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
]);

fn main() {
    // Disable logging for cleaner benchmark output
    solana_logger::setup_with("");

    let mollusk = Mollusk::new(&PROGRAM_ID, "./target/deploy/optimize_cmp");

    // Create instructions for each benchmark type
    let instruction_default = Instruction::new_with_bytes(
        PROGRAM_ID,
        &[1], // test_type = 1 (default comparison)
        vec![],
    );

    let instruction_manual_loop = Instruction::new_with_bytes(
        PROGRAM_ID,
        &[2], // test_type = 2 (manual loop)
        vec![],
    );

    let instruction_unrolled = Instruction::new_with_bytes(
        PROGRAM_ID,
        &[3], // test_type = 3 (unrolled comparison)
        vec![],
    );

    let instruction_unsafe_pointer = Instruction::new_with_bytes(
        PROGRAM_ID,
        &[4], // test_type = 4 (unsafe pointer arithmetic)
        vec![],
    );

    // All benchmarks use empty accounts
    let accounts = vec![];

    MolluskComputeUnitBencher::new(mollusk)
        .bench(("default_comparison_same", &instruction_default, &accounts))
        .bench(("manual_loop_same", &instruction_manual_loop, &accounts))
        .bench(("unrolled_comparison_same", &instruction_unrolled, &accounts))
        .bench((
            "unsafe_pointer_same",
            &instruction_unsafe_pointer,
            &accounts,
        ))
        .must_pass(true)
        .out_dir("target/benches")
        .execute();
}
