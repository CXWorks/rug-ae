#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;
#[derive(Clone)]
pub struct State {
    state: u32,
}
impl State {
    #[cfg(not(feature = "std"))]
    pub fn new(state: u32) -> Option<Self> {
        if cfg!(target_feature = "pclmulqdq") && cfg!(target_feature = "sse2")
            && cfg!(target_feature = "sse4.1")
        {
            Some(Self { state })
        } else {
            None
        }
    }
    #[cfg(feature = "std")]
    pub fn new(state: u32) -> Option<Self> {
        if is_x86_feature_detected!("pclmulqdq") && is_x86_feature_detected!("sse2")
            && is_x86_feature_detected!("sse4.1")
        {
            Some(Self { state })
        } else {
            None
        }
    }
    pub fn update(&mut self, buf: &[u8]) {
        self.state = unsafe { calculate(self.state, buf) };
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
const K1: i64 = 0x154442bd4;
const K2: i64 = 0x1c6e41596;
const K3: i64 = 0x1751997d0;
const K4: i64 = 0x0ccaa009e;
const K5: i64 = 0x163cd6124;
const K6: i64 = 0x1db710640;
const P_X: i64 = 0x1DB710641;
const U_PRIME: i64 = 0x1F7011641;
#[cfg(feature = "std")]
unsafe fn debug(s: &str, a: arch::__m128i) -> arch::__m128i {
    if false {
        union A {
            a: arch::__m128i,
            b: [u8; 16],
        }
        let x = A { a }.b;
        print!(" {:20} | ", s);
        for x in x.iter() {
            print!("{:02x} ", x);
        }
        println!();
    }
    return a;
}
#[cfg(not(feature = "std"))]
unsafe fn debug(_s: &str, a: arch::__m128i) -> arch::__m128i {
    a
}
#[target_feature(enable = "pclmulqdq", enable = "sse2", enable = "sse4.1")]
unsafe fn calculate(crc: u32, mut data: &[u8]) -> u32 {
    if data.len() < 128 {
        return ::baseline::update_fast_16(crc, data);
    }
    let mut x3 = get(&mut data);
    let mut x2 = get(&mut data);
    let mut x1 = get(&mut data);
    let mut x0 = get(&mut data);
    x3 = arch::_mm_xor_si128(x3, arch::_mm_cvtsi32_si128(!crc as i32));
    let k1k2 = arch::_mm_set_epi64x(K2, K1);
    while data.len() >= 64 {
        x3 = reduce128(x3, get(&mut data), k1k2);
        x2 = reduce128(x2, get(&mut data), k1k2);
        x1 = reduce128(x1, get(&mut data), k1k2);
        x0 = reduce128(x0, get(&mut data), k1k2);
    }
    let k3k4 = arch::_mm_set_epi64x(K4, K3);
    let mut x = reduce128(x3, x2, k3k4);
    x = reduce128(x, x1, k3k4);
    x = reduce128(x, x0, k3k4);
    while data.len() >= 16 {
        x = reduce128(x, get(&mut data), k3k4);
    }
    debug("128 > 64 init", x);
    drop(K6);
    let x = arch::_mm_xor_si128(
        arch::_mm_clmulepi64_si128(x, k3k4, 0x10),
        arch::_mm_srli_si128(x, 8),
    );
    let x = arch::_mm_xor_si128(
        arch::_mm_clmulepi64_si128(
            arch::_mm_and_si128(x, arch::_mm_set_epi32(0, 0, 0, !0)),
            arch::_mm_set_epi64x(0, K5),
            0x00,
        ),
        arch::_mm_srli_si128(x, 4),
    );
    debug("128 > 64 xx", x);
    let pu = arch::_mm_set_epi64x(U_PRIME, P_X);
    let t1 = arch::_mm_clmulepi64_si128(
        arch::_mm_and_si128(x, arch::_mm_set_epi32(0, 0, 0, !0)),
        pu,
        0x10,
    );
    let t2 = arch::_mm_clmulepi64_si128(
        arch::_mm_and_si128(t1, arch::_mm_set_epi32(0, 0, 0, !0)),
        pu,
        0x00,
    );
    let c = arch::_mm_extract_epi32(arch::_mm_xor_si128(x, t2), 1) as u32;
    if !data.is_empty() { ::baseline::update_fast_16(!c, data) } else { !c }
}
unsafe fn reduce128(
    a: arch::__m128i,
    b: arch::__m128i,
    keys: arch::__m128i,
) -> arch::__m128i {
    let t1 = arch::_mm_clmulepi64_si128(a, keys, 0x00);
    let t2 = arch::_mm_clmulepi64_si128(a, keys, 0x11);
    arch::_mm_xor_si128(arch::_mm_xor_si128(b, t1), t2)
}
unsafe fn get(a: &mut &[u8]) -> arch::__m128i {
    debug_assert!(a.len() >= 16);
    let r = arch::_mm_loadu_si128(a.as_ptr() as *const arch::__m128i);
    *a = &a[16..];
    return r;
}
#[cfg(test)]
mod test {
    quickcheck! {
        fn check_against_baseline(init : u32, chunks : Vec < (Vec < u8 >, usize) >) ->
        bool { let mut baseline = super::super::super::baseline::State::new(init); let
        mut pclmulqdq = super::State::new(init).expect("not supported"); for (chunk, mut
        offset) in chunks { offset &= 0xF; if chunk.len() <= offset { baseline.update(&
        chunk); pclmulqdq.update(& chunk); } else { baseline.update(& chunk[offset..]);
        pclmulqdq.update(& chunk[offset..]); } } pclmulqdq.finalize() == baseline
        .finalize() }
    }
}
#[cfg(test)]
mod tests_llm_16_24 {
    use crate::specialized::pclmulqdq::State;
    #[test]
    fn test_combine() {
        let initial_state_value = 0x12345678;
        let other_value = 0x90abcdef;
        let amount = 1024;
        let expected_combined_state = combine_fn(
            initial_state_value,
            other_value,
            amount,
        );
        let mut state = State::new(initial_state_value).expect("Failed to create state");
        state.combine(other_value, amount);
        assert_eq!(
            state.finalize(), expected_combined_state,
            "State::combine did not produce the expected result"
        );
    }
    fn combine_fn(initial_state: u32, other: u32, amount: u64) -> u32 {
        (initial_state ^ other).wrapping_add(amount as u32)
    }
}
#[cfg(test)]
mod tests_llm_16_25 {
    use crate::specialized::pclmulqdq::State;
    #[test]
    fn test_finalize_returns_state() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let initial_state = rug_fuzz_0;
        let state_struct = State::new(initial_state).unwrap();
        let final_state = state_struct.finalize();
        debug_assert_eq!(initial_state, final_state);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_26 {
    use crate::specialized::pclmulqdq::State;
    use std::arch::x86_64::*;
    #[test]
    fn test_new_returns_some_with_supported_cpu() {
        if is_x86_feature_detected!("pclmulqdq") && is_x86_feature_detected!("sse2")
            && is_x86_feature_detected!("sse4.1")
        {
            assert!(State::new(0).is_some());
        }
    }
    #[test]
    fn test_new_returns_none_with_unsupported_cpu() {
        if !(is_x86_feature_detected!("pclmulqdq") && is_x86_feature_detected!("sse2")
            && is_x86_feature_detected!("sse4.1"))
        {
            assert!(State::new(0).is_none());
        }
    }
}
#[cfg(test)]
mod tests_llm_16_27_llm_16_27 {
    use crate::specialized::pclmulqdq::State;
    #[test]
    fn test_reset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let initial_state_value = rug_fuzz_0;
        let mut state = State::new(initial_state_value).unwrap();
        state.reset();
        debug_assert_eq!(state.finalize(), 0);
             }
}
}
}    }
}
