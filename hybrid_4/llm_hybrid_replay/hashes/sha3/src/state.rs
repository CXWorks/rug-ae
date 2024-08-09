use core::convert::TryInto;
#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};
const PLEN: usize = 25;
const DEFAULT_ROUND_COUNT: usize = 24;
#[derive(Clone)]
pub(crate) struct Sha3State {
    pub state: [u64; PLEN],
    round_count: usize,
}
impl Default for Sha3State {
    fn default() -> Self {
        Self {
            state: [0u64; PLEN],
            round_count: DEFAULT_ROUND_COUNT,
        }
    }
}
#[cfg(feature = "zeroize")]
impl Drop for Sha3State {
    fn drop(&mut self) {
        self.state.zeroize();
    }
}
#[cfg(feature = "zeroize")]
impl ZeroizeOnDrop for Sha3State {}
impl Sha3State {
    pub(crate) fn new(round_count: usize) -> Self {
        Self {
            state: [0u64; PLEN],
            round_count,
        }
    }
    #[inline(always)]
    pub(crate) fn absorb_block(&mut self, block: &[u8]) {
        debug_assert_eq!(block.len() % 8, 0);
        for (b, s) in block.chunks_exact(8).zip(self.state.iter_mut()) {
            *s ^= u64::from_le_bytes(b.try_into().unwrap());
        }
        keccak::p1600(&mut self.state, self.round_count);
    }
    #[inline(always)]
    pub(crate) fn as_bytes(&self, out: &mut [u8]) {
        for (o, s) in out.chunks_mut(8).zip(self.state.iter()) {
            o.copy_from_slice(&s.to_le_bytes()[..o.len()]);
        }
    }
    #[inline(always)]
    pub(crate) fn permute(&mut self) {
        keccak::p1600(&mut self.state, self.round_count);
    }
}
#[cfg(test)]
mod tests_llm_16_76 {
    use super::*;
    use crate::*;
    const PLEN: usize = 25;
    const DEFAULT_ROUND_COUNT: usize = 24;
    #[test]
    fn default_initializes_to_zero_state_and_default_round_count() {
        let _rug_st_tests_llm_16_76_rrrruuuugggg_default_initializes_to_zero_state_and_default_round_count = 0;
        let default_state = Sha3State::default();
        for &val in default_state.state.iter() {
            debug_assert_eq!(val, 0u64);
        }
        debug_assert_eq!(default_state.round_count, DEFAULT_ROUND_COUNT);
        let _rug_ed_tests_llm_16_76_rrrruuuugggg_default_initializes_to_zero_state_and_default_round_count = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_84 {
    use super::*;
    use crate::*;
    #[test]
    fn test_absorb_block() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, usize, u64, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut state = Sha3State::default();
        let block = [rug_fuzz_0; 72];
        state.absorb_block(&block);
        let mut known_state = Sha3State::default();
        known_state.state[rug_fuzz_1] = rug_fuzz_2;
        for i in rug_fuzz_3..PLEN {
            known_state.state[i]
                ^= ((i * rug_fuzz_4) as u64).wrapping_add(block[i % block.len()] as u64);
        }
        known_state.permute();
        for (i, (actual, expected)) in state
            .state
            .iter()
            .zip(known_state.state.iter())
            .enumerate()
        {
            debug_assert_eq!(
                actual, expected, "State mismatch at index {}: actual: {}, expected: {}",
                i, actual, expected
            );
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_85 {
    use crate::Sha3State;
    const PLEN: usize = 25;
    #[test]
    fn test_as_bytes() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u64, u8, usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut test_state = Sha3State::default();
        let mut test_output: [u8; PLEN * 8] = [rug_fuzz_0; PLEN * 8];
        test_state.as_bytes(&mut test_output);
        debug_assert_eq!(test_output.to_vec(), vec![0u8; PLEN * 8]);
        let mut test_state = Sha3State::default();
        test_state.state = [rug_fuzz_1; PLEN];
        let mut test_output: [u8; PLEN * 8] = [rug_fuzz_2; PLEN * 8];
        test_state.as_bytes(&mut test_output);
        for chunk in test_output.chunks_exact(rug_fuzz_3) {
            debug_assert_eq!(chunk, & [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        }
        let mut partial_output: [u8; 4] = [rug_fuzz_4; 4];
        test_state.as_bytes(&mut partial_output);
        debug_assert_eq!(partial_output, [0xFF, 0xFF, 0xFF, 0xFF]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_87 {
    use super::*;
    use crate::*;
    const DEFAULT_ROUND_COUNT: usize = 24;
    #[test]
    fn test_permute() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut state = Sha3State::default();
        let initial_state = state.clone();
        state.permute();
        if state.round_count > rug_fuzz_0 {
            debug_assert_ne!(state.state, initial_state.state);
        } else {
            debug_assert_eq!(state.state, initial_state.state);
        }
        let mut state_before_permute = state.clone();
        state.permute();
        debug_assert_eq!(state.state, state_before_permute.state);
        let round_count = rug_fuzz_1;
        let mut non_zero_state = Sha3State::new(round_count);
        non_zero_state.state[rug_fuzz_2] = rug_fuzz_3;
        let non_zero_state_before_permute = non_zero_state.clone();
        non_zero_state.permute();
        debug_assert_ne!(non_zero_state.state, non_zero_state_before_permute.state);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_219 {
    use super::*;
    #[test]
    fn test_new() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let state = crate::state::Sha3State::new(p0);
        debug_assert_eq!(state.round_count, p0);
        for &value in &state.state {
            debug_assert_eq!(value, 0u64);
        }
             }
}
}
}    }
}
