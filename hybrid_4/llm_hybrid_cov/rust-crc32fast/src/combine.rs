const GF2_DIM: usize = 32;
fn gf2_matrix_times(mat: &[u32; GF2_DIM], mut vec: u32) -> u32 {
    let mut sum = 0;
    let mut idx = 0;
    while vec > 0 {
        if vec & 1 == 1 {
            sum ^= mat[idx];
        }
        vec >>= 1;
        idx += 1;
    }
    return sum;
}
fn gf2_matrix_square(square: &mut [u32; GF2_DIM], mat: &[u32; GF2_DIM]) {
    for n in 0..GF2_DIM {
        square[n] = gf2_matrix_times(mat, mat[n]);
    }
}
pub(crate) fn combine(mut crc1: u32, crc2: u32, mut len2: u64) -> u32 {
    let mut row: u32;
    let mut even = [0u32; GF2_DIM];
    let mut odd = [0u32; GF2_DIM];
    if len2 <= 0 {
        return crc1;
    }
    odd[0] = 0xedb88320;
    row = 1;
    for n in 1..GF2_DIM {
        odd[n] = row;
        row <<= 1;
    }
    gf2_matrix_square(&mut even, &odd);
    gf2_matrix_square(&mut odd, &even);
    loop {
        gf2_matrix_square(&mut even, &odd);
        if len2 & 1 == 1 {
            crc1 = gf2_matrix_times(&even, crc1);
        }
        len2 >>= 1;
        if len2 == 0 {
            break;
        }
        gf2_matrix_square(&mut odd, &even);
        if len2 & 1 == 1 {
            crc1 = gf2_matrix_times(&odd, crc1);
        }
        len2 >>= 1;
        if len2 == 0 {
            break;
        }
    }
    crc1 ^= crc2;
    return crc1;
}
#[cfg(test)]
mod tests_llm_16_20 {
    use crate::combine::combine;
    use crate::combine::GF2_DIM;
    #[test]
    fn test_combine() {
        let _rug_st_tests_llm_16_20_rrrruuuugggg_test_combine = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x9abcdef0;
        let rug_fuzz_2 = 123456;
        let rug_fuzz_3 = 0x00000000;
        let crc1 = rug_fuzz_0;
        let crc2 = rug_fuzz_1;
        let len2 = rug_fuzz_2;
        let combined_crc = combine(crc1, crc2, len2);
        let expected_crc = rug_fuzz_3;
        debug_assert_eq!(combined_crc, expected_crc, "CRC combination failed");
        let _rug_ed_tests_llm_16_20_rrrruuuugggg_test_combine = 0;
    }
    #[test]
    fn test_combine_zero_length() {
        let _rug_st_tests_llm_16_20_rrrruuuugggg_test_combine_zero_length = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x9abcdef0;
        let rug_fuzz_2 = 0;
        let crc1 = rug_fuzz_0;
        let crc2 = rug_fuzz_1;
        let len2 = rug_fuzz_2;
        debug_assert_eq!(
            combine(crc1, crc2, len2), crc1,
            "CRC combination with zero length should return first CRC"
        );
        let _rug_ed_tests_llm_16_20_rrrruuuugggg_test_combine_zero_length = 0;
    }
    #[test]
    fn test_combine_degenerate_case() {
        let _rug_st_tests_llm_16_20_rrrruuuugggg_test_combine_degenerate_case = 0;
        let rug_fuzz_0 = 0x0;
        let rug_fuzz_1 = 0x0;
        let rug_fuzz_2 = 0x0;
        let crc1 = rug_fuzz_0;
        let crc2 = rug_fuzz_1;
        let len2 = rug_fuzz_2;
        debug_assert_eq!(
            combine(crc1, crc2, len2), 0x0,
            "CRC combination with zero for all inputs should return zero"
        );
        let _rug_ed_tests_llm_16_20_rrrruuuugggg_test_combine_degenerate_case = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_21_llm_16_21 {
    use super::*;
    use crate::*;
    const GF2_DIM: usize = 32;
    #[test]
    fn test_gf2_matrix_square() {
        let _rug_st_tests_llm_16_21_llm_16_21_rrrruuuugggg_test_gf2_matrix_square = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 0u32;
        let rug_fuzz_2 = 0u32;
        let rug_fuzz_3 = 0;
        let mut matrix = [rug_fuzz_0; GF2_DIM];
        let mut square = [rug_fuzz_1; GF2_DIM];
        let mut expected_square = [rug_fuzz_2; GF2_DIM];
        for n in rug_fuzz_3..GF2_DIM {
            matrix[n] = n as u32;
            expected_square[n] = gf2_matrix_times(&matrix, matrix[n]);
        }
        gf2_matrix_square(&mut square, &matrix);
        debug_assert_eq!(
            square, expected_square,
            "Matrix squaring did not produce the expected result."
        );
        let _rug_ed_tests_llm_16_21_llm_16_21_rrrruuuugggg_test_gf2_matrix_square = 0;
    }
}
