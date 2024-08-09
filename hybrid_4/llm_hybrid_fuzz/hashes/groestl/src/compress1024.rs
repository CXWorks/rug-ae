#![allow(clippy::needless_range_loop)]
use crate::table::TABLE;
use core::{convert::TryInto, u64};
pub(crate) const COLS: usize = 16;
const ROUNDS: u64 = 14;
#[inline(always)]
fn column(x: &[u64; COLS], c: [usize; 8]) -> u64 {
    let mut t = 0;
    for i in 0..8 {
        let sl = 8 * (7 - i);
        let idx = ((x[c[i]] >> sl) & 0xFF) as usize;
        t ^= TABLE[i][idx];
    }
    t
}
#[inline(always)]
fn rndq(mut x: [u64; COLS], r: u64) -> [u64; COLS] {
    for i in 0..COLS {
        x[i] ^= u64::MAX.wrapping_sub((i as u64) << 4) ^ r;
    }
    [
        column(&x, [1, 3, 5, 11, 0, 2, 4, 6]),
        column(&x, [2, 4, 6, 12, 1, 3, 5, 7]),
        column(&x, [3, 5, 7, 13, 2, 4, 6, 8]),
        column(&x, [4, 6, 8, 14, 3, 5, 7, 9]),
        column(&x, [5, 7, 9, 15, 4, 6, 8, 10]),
        column(&x, [6, 8, 10, 0, 5, 7, 9, 11]),
        column(&x, [7, 9, 11, 1, 6, 8, 10, 12]),
        column(&x, [8, 10, 12, 2, 7, 9, 11, 13]),
        column(&x, [9, 11, 13, 3, 8, 10, 12, 14]),
        column(&x, [10, 12, 14, 4, 9, 11, 13, 15]),
        column(&x, [11, 13, 15, 5, 10, 12, 14, 0]),
        column(&x, [12, 14, 0, 6, 11, 13, 15, 1]),
        column(&x, [13, 15, 1, 7, 12, 14, 0, 2]),
        column(&x, [14, 0, 2, 8, 13, 15, 1, 3]),
        column(&x, [15, 1, 3, 9, 14, 0, 2, 4]),
        column(&x, [0, 2, 4, 10, 15, 1, 3, 5]),
    ]
}
#[inline(always)]
fn rndp(mut x: [u64; COLS], r: u64) -> [u64; COLS] {
    for i in 0..COLS {
        x[i] ^= ((i as u64) << 60) ^ r;
    }
    [
        column(&x, [0, 1, 2, 3, 4, 5, 6, 11]),
        column(&x, [1, 2, 3, 4, 5, 6, 7, 12]),
        column(&x, [2, 3, 4, 5, 6, 7, 8, 13]),
        column(&x, [3, 4, 5, 6, 7, 8, 9, 14]),
        column(&x, [4, 5, 6, 7, 8, 9, 10, 15]),
        column(&x, [5, 6, 7, 8, 9, 10, 11, 0]),
        column(&x, [6, 7, 8, 9, 10, 11, 12, 1]),
        column(&x, [7, 8, 9, 10, 11, 12, 13, 2]),
        column(&x, [8, 9, 10, 11, 12, 13, 14, 3]),
        column(&x, [9, 10, 11, 12, 13, 14, 15, 4]),
        column(&x, [10, 11, 12, 13, 14, 15, 0, 5]),
        column(&x, [11, 12, 13, 14, 15, 0, 1, 6]),
        column(&x, [12, 13, 14, 15, 0, 1, 2, 7]),
        column(&x, [13, 14, 15, 0, 1, 2, 3, 8]),
        column(&x, [14, 15, 0, 1, 2, 3, 4, 9]),
        column(&x, [15, 0, 1, 2, 3, 4, 5, 10]),
    ]
}
pub(crate) fn compress(h: &mut [u64; COLS], block: &[u8; 128]) {
    let mut q = [0u64; COLS];
    for (chunk, v) in block.chunks_exact(8).zip(q.iter_mut()) {
        *v = u64::from_be_bytes(chunk.try_into().unwrap());
    }
    let mut p = [0u64; COLS];
    for i in 0..COLS {
        p[i] = h[i] ^ q[i];
    }
    for i in 0..ROUNDS {
        q = rndq(q, i);
    }
    for i in 0..ROUNDS {
        p = rndp(p, i << 56);
    }
    for i in 0..COLS {
        h[i] ^= q[i] ^ p[i];
    }
}
pub(crate) fn p(h: &[u64; COLS]) -> [u64; COLS] {
    let mut p = *h;
    for i in 0..ROUNDS {
        p = rndp(p, i << 56);
    }
    for i in 0..COLS {
        p[i] ^= h[i];
    }
    p
}
#[cfg(test)]
mod tests_llm_16_9_llm_16_9 {
    use super::*;
    use crate::*;
    #[test]
    fn test_column() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u64, usize, usize, usize, usize, usize, usize, usize, usize, u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_x = [rug_fuzz_0; COLS];
        let c = [
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
        ];
        let result = column(&test_x, c);
        let expected = TABLE
            .iter()
            .enumerate()
            .fold(rug_fuzz_9, |acc, (i, &row)| { acc ^ row[rug_fuzz_10] });
        debug_assert_eq!(
            result, expected,
            "column() did not return the expected value with zero-initialized input"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    use crate::compress1024::rndp;
    #[test]
    fn test_rndp() {
        let input = [
            0x0000000000000000,
            0x1111111111111111,
            0x2222222222222222,
            0x3333333333333333,
            0x4444444444444444,
            0x5555555555555555,
            0x6666666666666666,
            0x7777777777777777,
            0x8888888888888888,
            0x9999999999999999,
            0xAAAAAAAAAAAAAAAA,
            0xBBBBBBBBBBBBBBBB,
            0xCCCCCCCCCCCCCCCC,
            0xDDDDDDDDDDDDDDDD,
            0xEEEEEEEEEEEEEEEE,
            0xFFFFFFFFFFFFFFFF,
        ];
        let round_number = 0x0000000000000001;
        let result = rndp(input, round_number);
        let expected = [
            column(&input, [0, 1, 2, 3, 4, 5, 6, 11]),
            column(&input, [1, 2, 3, 4, 5, 6, 7, 12]),
            column(&input, [2, 3, 4, 5, 6, 7, 8, 13]),
            column(&input, [3, 4, 5, 6, 7, 8, 9, 14]),
            column(&input, [4, 5, 6, 7, 8, 9, 10, 15]),
            column(&input, [5, 6, 7, 8, 9, 10, 11, 0]),
            column(&input, [6, 7, 8, 9, 10, 11, 12, 1]),
            column(&input, [7, 8, 9, 10, 11, 12, 13, 2]),
            column(&input, [8, 9, 10, 11, 12, 13, 14, 3]),
            column(&input, [9, 10, 11, 12, 13, 14, 15, 4]),
            column(&input, [10, 11, 12, 13, 14, 15, 0, 5]),
            column(&input, [11, 12, 13, 14, 15, 0, 1, 6]),
            column(&input, [12, 13, 14, 15, 0, 1, 2, 7]),
            column(&input, [13, 14, 15, 0, 1, 2, 3, 8]),
            column(&input, [14, 15, 0, 1, 2, 3, 4, 9]),
            column(&input, [15, 0, 1, 2, 3, 4, 5, 10]),
        ];
        let mut xor_input = input;
        for i in 0..COLS {
            xor_input[i] ^= ((i as u64) << 60) ^ round_number;
        }
        assert_eq!(result, expected);
        assert_eq!(result, rndp(xor_input, round_number));
    }
    fn column(x: &[u64; COLS], i: [usize; 8]) -> u64 {
        0
    }
    const COLS: usize = 16;
}
#[cfg(test)]
mod tests_llm_16_13 {
    use super::*;
    use crate::*;
    const COLS: usize = 16;
    fn column(_: &[u64; COLS], _: [usize; 8]) -> u64 {
        0
    }
    #[test]
    fn test_rndq() {
        let input_x: [u64; COLS] = [0; COLS];
        let input_r: u64 = 0;
        let expected_output: [u64; COLS] = [0; COLS];
        let result = rndq(input_x, input_r);
        assert_eq!(expected_output, result);
    }
}
#[cfg(test)]
mod tests_rug_109 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u8, usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u64; COLS] = [rug_fuzz_0; COLS];
        let mut p1: [u8; 128] = [rug_fuzz_1; 128];
        p1[rug_fuzz_2] = rug_fuzz_3;
        crate::compress1024::compress(&mut p0, &p1);
             }
});    }
}
