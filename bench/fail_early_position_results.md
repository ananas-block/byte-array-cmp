# Fail-Early Position Analysis Results

This benchmark analyzes the CU costs of 32-byte array comparisons based on the position of the first differing byte, comparing SIMD iterator (u64 chunks) vs PartialEq approaches.

## Test Setup
- Target: 32-byte arrays with single byte differences at positions 0-31
- Methods: SIMD iterator (u64 chunks) vs PartialEq
- Equal case: Both arrays identical (no difference)

## Results Summary

### Equal Case (Arrays Match)
| Method | CUs | Notes |
|--------|-----|-------|
| simd_equal_case | 772 | SIMD when arrays are identical |
| partialeq_equal_case | 789 | PartialEq when arrays are identical |

### Fail-Early Position Analysis
| Position | SIMD CUs | PartialEq CUs | Notes |
|----------|----------|---------------|-------|
| 00 | 765 | 802 | First byte of chunk 1 (bytes 0-7) |
| 01 | 765 | 802 | |
| 02 | 765 | 802 | |
| 03 | 765 | 802 | |
| 04 | 765 | 802 | |
| 05 | 765 | 802 | |
| 06 | 765 | 802 | |
| 07 | 765 | 802 | Last byte of chunk 1 |
| 08 | 768 | 802 | First byte of chunk 2 (bytes 8-15) |
| 09 | 768 | 802 | |
| 10 | 768 | 802 | |
| 11 | 768 | 802 | |
| 12 | 768 | 802 | |
| 13 | 768 | 802 | |
| 14 | 768 | 802 | |
| 15 | 768 | 802 | Last byte of chunk 2 |
| 16 | 771 | 802 | First byte of chunk 3 (bytes 16-23) |
| 17 | 771 | 802 | |
| 18 | 771 | 802 | |
| 19 | 771 | 802 | |
| 20 | 771 | 802 | |
| 21 | 771 | 802 | |
| 22 | 771 | 802 | |
| 23 | 771 | 802 | Last byte of chunk 3 |
| 24 | 774 | 802 | First byte of chunk 4 (bytes 24-31) |
| 25 | 774 | 802 | |
| 26 | 774 | 802 | |
| 27 | 774 | 802 | |
| 28 | 774 | 802 | |
| 29 | 774 | 802 | |
| 30 | 774 | 802 | |
| 31 | 774 | 802 | Last byte of chunk 4 |

## Key Findings

### SIMD Iterator (u64 chunks)
- **Chunk-based fail-early optimization**: CU costs increase by ~3 per u64 chunk boundary
- **Pattern**: 765 → 768 → 771 → 774 CUs as failure moves through chunks 1-4
- **Best case**: 765 CUs (failure in first chunk, positions 0-7)
- **Worst case**: 774 CUs (failure in last chunk, positions 24-31)
- **Equal case**: 772 CUs (must check all chunks)

### PartialEq
- **Position-independent**: Consistent 802 CUs regardless of failure position
- **Equal case**: 789 CUs (more efficient than SIMD when arrays match)
- **No fail-early optimization**: Always processes entire array

### Performance Comparison
- **SIMD advantage**: 37 CUs better than PartialEq in best case (765 vs 802)
- **SIMD advantage**: 28 CUs better than PartialEq in worst case (774 vs 802)
- **PartialEq advantage**: 17 CUs better when arrays are equal (789 vs 772)

## Conclusion

The SIMD iterator approach with u64 chunks demonstrates clear fail-early optimization, with performance degrading predictably as the difference position moves through the 4 u64 chunks. PartialEq provides consistent performance but lacks fail-early optimization, making SIMD the better choice when most comparisons are expected to fail early.