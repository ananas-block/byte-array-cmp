# Optimize 32 byte array comparison in SVM

Benchmarked a Generic Changelog with [u8;32] byte array as key in a Solana program with mollusk.

Strategy, fail early.
Only makes sense if most comparisons will fail.
If comparisons are expected to succeed partial eq is better.

```bash
cargo build-sbf
cargo bench
```

Bench output: `target/benches/compute_units.md`

Base program CU cost: 563

| Name                                   | CUs    |
|----------------------------------------|--------|
| **10 iterations**                      |        |
| simd_iterator                          | 763    |
| simd_zip                               | 764    |
| ptoken_u128_cast                       | 767    |
| ptoken_pointer_equality                | 769    |
| ptoken_sol_memcmp                      | 779    |
| ptoken_combined_fast                   | 779    |
| optimization_simd                      | 785    |
| simd_slice                             | 786    |
| find_after_10_iterations_partialeq     | 804    |
| optimization_unrolled                  | 871    |
| find_after_10_iterations_manual        | 874    |
| optimization_unsafe                    | 881    |
| optimization_branchless                | 984    |
| **100 iterations**                     |        |
| simd_iterator_100                      | 6,850  |
| optimization_simd_100                  | 6,871  |
| ptoken_u128_cast_100                   | 7,349  |
| find_after_100_iterations_manual       | 7,906  |
| find_after_100_iterations_partialeq    | 11,202 |
| **1000 iterations (not found)**        |        |
| simd_iterator_1000_not_found           | 61,750 |
| optimization_simd_1000_not_found       | 61,773 |
| optimization_unrolled_not_found        | 64,781 |
| ptoken_u128_cast_1000_not_found        | 66,751 |
| find_not_found_manual                  | 72,730 |
| find_not_found_partialeq               | 105,720|

## SIMD Iterator Integer Type Variants (1000 iterations, not found)

Detailed comparison of different integer chunk sizes for the most efficient SIMD iterator approach:

```bash
cargo bench --bench simd_integer_variants
```

| Integer Type | Chunk Count | Chunk Size | CUs     | Performance |
|--------------|-------------|------------|---------|-------------|
| **1000 iterations**                     |         |             |
| **u64**      | 4 chunks   | 8 bytes     | 61,708  | **Optimal** |
| **u32**      | 8 chunks   | 4 bytes     | 64,706  | -2,998 CUs  |
| **u16**      | 16 chunks  | 2 bytes     | 64,707  | -2,999 CUs  |
| **u128**     | 2 chunks   | 16 bytes    | 66,708  | -5,000 CUs  |
