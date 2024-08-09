use table::CRC32_TABLE;
#[derive(Clone)]
pub struct State {
    state: u32,
}
impl State {
    pub fn new(state: u32) -> Self {
        State { state }
    }
    pub fn update(&mut self, buf: &[u8]) {
        self.state = update_fast_16(self.state, buf);
    }
    pub fn finalize(self) -> u32 {
        self.state
    }
    pub fn reset(&mut self) {
        self.state = 0;
    }
    pub fn combine(&mut self, other: u32, amount: u64) {
        self.state = ::combine::combine(self.state, other, amount);
    }
}
pub(crate) fn update_fast_16(prev: u32, mut buf: &[u8]) -> u32 {
    const UNROLL: usize = 4;
    const BYTES_AT_ONCE: usize = 16 * UNROLL;
    let mut crc = !prev;
    while buf.len() >= BYTES_AT_ONCE {
        for _ in 0..UNROLL {
            crc = CRC32_TABLE[0x0][buf[0xf] as usize]
                ^ CRC32_TABLE[0x1][buf[0xe] as usize]
                ^ CRC32_TABLE[0x2][buf[0xd] as usize]
                ^ CRC32_TABLE[0x3][buf[0xc] as usize]
                ^ CRC32_TABLE[0x4][buf[0xb] as usize]
                ^ CRC32_TABLE[0x5][buf[0xa] as usize]
                ^ CRC32_TABLE[0x6][buf[0x9] as usize]
                ^ CRC32_TABLE[0x7][buf[0x8] as usize]
                ^ CRC32_TABLE[0x8][buf[0x7] as usize]
                ^ CRC32_TABLE[0x9][buf[0x6] as usize]
                ^ CRC32_TABLE[0xa][buf[0x5] as usize]
                ^ CRC32_TABLE[0xb][buf[0x4] as usize]
                ^ CRC32_TABLE[0xc][buf[0x3] as usize ^ ((crc >> 0x18) & 0xFF) as usize]
                ^ CRC32_TABLE[0xd][buf[0x2] as usize ^ ((crc >> 0x10) & 0xFF) as usize]
                ^ CRC32_TABLE[0xe][buf[0x1] as usize ^ ((crc >> 0x08) & 0xFF) as usize]
                ^ CRC32_TABLE[0xf][buf[0x0] as usize ^ ((crc >> 0x00) & 0xFF) as usize];
            buf = &buf[16..];
        }
    }
    update_slow(!crc, buf)
}
pub(crate) fn update_slow(prev: u32, buf: &[u8]) -> u32 {
    let mut crc = !prev;
    for &byte in buf.iter() {
        crc = CRC32_TABLE[0][((crc as u8) ^ byte) as usize] ^ (crc >> 8);
    }
    !crc
}
#[cfg(test)]
mod test {
    #[test]
    fn slow() {
        assert_eq!(super::update_slow(0, b""), 0);
        assert_eq!(super::update_slow(! 0x12345678, b""), ! 0x12345678);
        assert_eq!(super::update_slow(! 0xffffffff, b"hello world"), ! 0xf2b5ee7a);
        assert_eq!(super::update_slow(! 0xffffffff, b"hello"), ! 0xc9ef5979);
        assert_eq!(super::update_slow(! 0xc9ef5979, b" world"), ! 0xf2b5ee7a);
        assert_eq!(
            super::update_slow(0,
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            0x190A55AD
        );
        assert_eq!(
            super::update_slow(0,
            b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF"),
            0xFF6CAB0B
        );
        assert_eq!(
            super::update_slow(0,
            b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F"),
            0x91267E8A
        );
    }
    quickcheck! {
        fn fast_16_is_the_same_as_slow(crc : u32, bytes : Vec < u8 >) -> bool {
        super::update_fast_16(crc, & bytes) == super::update_slow(crc, & bytes) }
    }
}
#[cfg(test)]
mod tests_llm_16_13 {
    use crate::baseline::State;
    #[test]
    fn test_combine() {
        let _rug_st_tests_llm_16_13_rrrruuuugggg_test_combine = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x9abcdef0;
        let rug_fuzz_2 = 1024;
        let rug_fuzz_3 = 0;
        let initial_state_val = rug_fuzz_0;
        let other_state_val = rug_fuzz_1;
        let amount = rug_fuzz_2;
        let mut state = State::new(initial_state_val);
        let mut state_clone = state.clone();
        state_clone.update(&[rug_fuzz_3; 1024]);
        state.combine(other_state_val, amount);
        let final_state = state.finalize();
        let final_state_clone = state_clone.finalize();
        debug_assert_eq!(final_state, final_state_clone);
        let _rug_ed_tests_llm_16_13_rrrruuuugggg_test_combine = 0;
    }
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_1_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x89;
        let rug_fuzz_2 = 0xAB;
        let rug_fuzz_3 = 0xCD;
        let rug_fuzz_4 = 0xEF;
        let rug_fuzz_5 = 0x89;
        let rug_fuzz_6 = 0xAB;
        let rug_fuzz_7 = 0xCD;
        let rug_fuzz_8 = 0xEF;
        let rug_fuzz_9 = 0x89;
        let rug_fuzz_10 = 0xAB;
        let rug_fuzz_11 = 0xCD;
        let rug_fuzz_12 = 0xEF;
        let rug_fuzz_13 = 0x89;
        let rug_fuzz_14 = 0xAB;
        let rug_fuzz_15 = 0xCD;
        let rug_fuzz_16 = 0xEF;
        let rug_fuzz_17 = 0x89;
        let rug_fuzz_18 = 0xAB;
        let rug_fuzz_19 = 0xCD;
        let rug_fuzz_20 = 0xEF;
        let mut p0: u32 = rug_fuzz_0;
        let mut p1: &[u8] = &[
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
            rug_fuzz_17,
            rug_fuzz_18,
            rug_fuzz_19,
            rug_fuzz_20,
        ];
        let _ = crate::baseline::update_fast_16(p0, p1);
        let _rug_ed_tests_rug_1_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    #[test]
    fn test_update_slow() {
        let _rug_st_tests_rug_2_rrrruuuugggg_test_update_slow = 0;
        let rug_fuzz_0 = 0xFFFFFFFF;
        let rug_fuzz_1 = b"hello world";
        let p0: u32 = rug_fuzz_0;
        let p1: &[u8] = rug_fuzz_1;
        let result = crate::baseline::update_slow(p0, p1);
        debug_assert_eq!(result, 0x1C291CA3);
        let _rug_ed_tests_rug_2_rrrruuuugggg_test_update_slow = 0;
    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_rug_3_rrrruuuugggg_test_new = 0;
        let rug_fuzz_0 = 123456789;
        let p0: u32 = rug_fuzz_0;
        let state = crate::baseline::State::new(p0);
        debug_assert_eq!(state.state, p0);
        let _rug_ed_tests_rug_3_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    #[test]
    fn test_update() {
        let _rug_st_tests_rug_4_rrrruuuugggg_test_update = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 4;
        let rug_fuzz_5 = 5;
        let mut p0 = State::new(rug_fuzz_0);
        let p1: &[u8] = &[rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4, rug_fuzz_5];
        p0.update(p1);
        let _rug_ed_tests_rug_4_rrrruuuugggg_test_update = 0;
    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::State;
    #[test]
    fn test_finalize() {
        let _rug_st_tests_rug_5_rrrruuuugggg_test_finalize = 0;
        let rug_fuzz_0 = 0;
        let mut p0 = State::new(rug_fuzz_0);
        debug_assert_eq!(p0.finalize(), 0);
        let _rug_ed_tests_rug_5_rrrruuuugggg_test_finalize = 0;
    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::State;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_6_rrrruuuugggg_test_reset = 0;
        let rug_fuzz_0 = 0;
        let mut p0 = State::new(rug_fuzz_0);
        p0.reset();
        debug_assert_eq!(p0.state, 0);
        let _rug_ed_tests_rug_6_rrrruuuugggg_test_reset = 0;
    }
}
