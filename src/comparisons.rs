use solana_program::msg;

pub fn benchmark_default_comparison(a1: &[u8; 32], a2: &[u8; 32], a3: &[u8; 32]) {
    msg!("=== Default Comparison Benchmark ===");
    
    // Test 1: Same arrays
    let result = a1 == a1;
    msg!("Same arrays result: {}", result);
    
    // Test 2: Different at end
    let result = a1 == a2;
    msg!("Different at end result: {}", result);
    
    // Test 3: Different at start
    let result = a1 == a3;
    msg!("Different at start result: {}", result);
}

pub fn benchmark_manual_loop(a1: &[u8; 32], a2: &[u8; 32], a3: &[u8; 32]) {
    msg!("=== Manual Loop Benchmark ===");
    
    // Test 1: Same arrays
    let result = manual_compare_32(a1, a1);
    msg!("Same arrays result: {}", result);
    
    // Test 2: Different at end
    let result = manual_compare_32(a1, a2);
    msg!("Different at end result: {}", result);
    
    // Test 3: Different at start
    let result = manual_compare_32(a1, a3);
    msg!("Different at start result: {}", result);
}

fn manual_compare_32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    for i in 0..32 {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

pub fn benchmark_unrolled_comparison(a1: &[u8; 32], a2: &[u8; 32], a3: &[u8; 32]) {
    msg!("=== Unrolled Comparison Benchmark ===");
    
    // Test 1: Same arrays
    let result = unrolled_compare_32(a1, a1);
    msg!("Same arrays result: {}", result);
    
    // Test 2: Different at end
    let result = unrolled_compare_32(a1, a2);
    msg!("Different at end result: {}", result);
    
    // Test 3: Different at start
    let result = unrolled_compare_32(a1, a3);
    msg!("Different at start result: {}", result);
}

fn unrolled_compare_32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    if a[0] != b[0] { return false; }
    if a[1] != b[1] { return false; }
    if a[2] != b[2] { return false; }
    if a[3] != b[3] { return false; }
    if a[4] != b[4] { return false; }
    if a[5] != b[5] { return false; }
    if a[6] != b[6] { return false; }
    if a[7] != b[7] { return false; }
    if a[8] != b[8] { return false; }
    if a[9] != b[9] { return false; }
    if a[10] != b[10] { return false; }
    if a[11] != b[11] { return false; }
    if a[12] != b[12] { return false; }
    if a[13] != b[13] { return false; }
    if a[14] != b[14] { return false; }
    if a[15] != b[15] { return false; }
    if a[16] != b[16] { return false; }
    if a[17] != b[17] { return false; }
    if a[18] != b[18] { return false; }
    if a[19] != b[19] { return false; }
    if a[20] != b[20] { return false; }
    if a[21] != b[21] { return false; }
    if a[22] != b[22] { return false; }
    if a[23] != b[23] { return false; }
    if a[24] != b[24] { return false; }
    if a[25] != b[25] { return false; }
    if a[26] != b[26] { return false; }
    if a[27] != b[27] { return false; }
    if a[28] != b[28] { return false; }
    if a[29] != b[29] { return false; }
    if a[30] != b[30] { return false; }
    if a[31] != b[31] { return false; }
    true
}

pub fn benchmark_unsafe_pointer(a1: &[u8; 32], a2: &[u8; 32], a3: &[u8; 32]) {
    msg!("=== Unsafe Pointer Arithmetic Benchmark ===");
    
    // Test 1: Same arrays
    let result = unsafe_pointer_compare_32(a1, a1);
    msg!("Same arrays result: {}", result);
    
    // Test 2: Different at end
    let result = unsafe_pointer_compare_32(a1, a2);
    msg!("Different at end result: {}", result);
    
    // Test 3: Different at start
    let result = unsafe_pointer_compare_32(a1, a3);
    msg!("Different at start result: {}", result);
}

fn unsafe_pointer_compare_32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    unsafe {
        let mut a_ptr = a.as_ptr();
        let mut b_ptr = b.as_ptr();
        let end_ptr = a_ptr.add(32);
        
        while a_ptr < end_ptr {
            if *a_ptr != *b_ptr {
                return false;
            }
            a_ptr = a_ptr.add(1);
            b_ptr = b_ptr.add(1);
        }
        true
    }
}