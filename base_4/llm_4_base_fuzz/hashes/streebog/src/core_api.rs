use core::{convert::TryInto, fmt};
use digest::{
    block_buffer::Eager, consts::U64,
    core_api::{
        AlgorithmName, Block as GenBlock, BlockSizeUser, Buffer, BufferKindUser,
        OutputSizeUser, TruncSide, UpdateCore, VariableOutputCore,
    },
    HashMarker, InvalidOutputSize, Output,
};
use crate::consts::{BLOCK_SIZE, C};
use crate::table::SHUFFLED_LIN_TABLE;
type Block = [u8; 64];
/// Core block-level Streebog hasher with variable output size.
///
/// Supports initialization only for 32 and 64 byte output sizes,
/// i.e. 256 and 512 bits respectively.
#[derive(Clone)]
pub struct StreebogVarCore {
    h: Block,
    n: [u64; 8],
    sigma: [u64; 8],
}
#[inline(always)]
fn lps(h: &mut Block, n: &Block) {
    for i in 0..64 {
        h[i] ^= n[i];
    }
    let mut buf = [0u64; 8];
    for i in 0..4 {
        for j in 0..8 {
            let b = h[2 * i + 8 * j] as usize;
            buf[2 * i] ^= SHUFFLED_LIN_TABLE[j][b];
            let b = h[2 * i + 1 + 8 * j] as usize;
            buf[2 * i + 1] ^= SHUFFLED_LIN_TABLE[j][b];
        }
    }
    *h = to_bytes(&buf);
}
impl StreebogVarCore {
    fn g(&mut self, n: &Block, m: &Block) {
        let mut key = [0u8; 64];
        let mut block = [0u8; 64];
        key.copy_from_slice(&self.h);
        block.copy_from_slice(m);
        lps(&mut key, n);
        #[allow(clippy::needless_range_loop)]
        for i in 0..12 {
            lps(&mut block, &key);
            lps(&mut key, &C[i]);
        }
        for i in 0..64 {
            self.h[i] ^= block[i] ^ key[i] ^ m[i];
        }
    }
    fn update_sigma(&mut self, m: &Block) {
        let t = from_bytes(m);
        let mut carry = 0;
        adc(&mut self.sigma[0], t[0], &mut carry);
        adc(&mut self.sigma[1], t[1], &mut carry);
        adc(&mut self.sigma[2], t[2], &mut carry);
        adc(&mut self.sigma[3], t[3], &mut carry);
        adc(&mut self.sigma[4], t[4], &mut carry);
        adc(&mut self.sigma[5], t[5], &mut carry);
        adc(&mut self.sigma[6], t[6], &mut carry);
        adc(&mut self.sigma[7], t[7], &mut carry);
    }
    fn update_n(&mut self, len: u64) {
        let mut carry = 0;
        adc(&mut self.n[0], 8 * len, &mut carry);
        adc(&mut self.n[1], 0, &mut carry);
        adc(&mut self.n[2], 0, &mut carry);
        adc(&mut self.n[3], 0, &mut carry);
        adc(&mut self.n[4], 0, &mut carry);
        adc(&mut self.n[5], 0, &mut carry);
        adc(&mut self.n[6], 0, &mut carry);
        adc(&mut self.n[7], 0, &mut carry);
    }
    fn compress(&mut self, block: &[u8; 64], msg_len: u64) {
        self.g(&to_bytes(&self.n), block);
        self.update_n(msg_len);
        self.update_sigma(block);
    }
}
impl HashMarker for StreebogVarCore {}
impl BlockSizeUser for StreebogVarCore {
    type BlockSize = U64;
}
impl BufferKindUser for StreebogVarCore {
    type BufferKind = Eager;
}
impl UpdateCore for StreebogVarCore {
    #[inline]
    fn update_blocks(&mut self, blocks: &[GenBlock<Self>]) {
        for block in blocks {
            self.compress(block.as_ref(), BLOCK_SIZE as u64);
        }
    }
}
impl OutputSizeUser for StreebogVarCore {
    type OutputSize = U64;
}
impl VariableOutputCore for StreebogVarCore {
    const TRUNC_SIDE: TruncSide = TruncSide::Right;
    #[inline]
    fn new(output_size: usize) -> Result<Self, InvalidOutputSize> {
        let h = match output_size {
            32 => [1; 64],
            64 => [0; 64],
            _ => return Err(InvalidOutputSize),
        };
        let (n, sigma) = Default::default();
        Ok(Self { h, n, sigma })
    }
    #[inline]
    fn finalize_variable_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let pos = buffer.get_pos();
        let block = buffer.pad_with_zeros();
        block[pos] = 1;
        self.compress(block.as_ref(), pos as u64);
        self.g(&[0u8; 64], &to_bytes(&self.n));
        self.g(&[0u8; 64], &to_bytes(&self.sigma));
        out.copy_from_slice(&self.h);
    }
}
impl AlgorithmName for StreebogVarCore {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Streebog")
    }
}
impl fmt::Debug for StreebogVarCore {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreebogVarCore { ... }")
    }
}
#[inline(always)]
fn adc(a: &mut u64, b: u64, carry: &mut u64) {
    let ret = (*a as u128) + (b as u128) + (*carry as u128);
    *a = ret as u64;
    *carry = (ret >> 64) as u64;
}
#[inline(always)]
fn to_bytes(b: &[u64; 8]) -> Block {
    let mut t = [0; 64];
    for (chunk, v) in t.chunks_exact_mut(8).zip(b.iter()) {
        chunk.copy_from_slice(&v.to_le_bytes());
    }
    t
}
#[inline(always)]
fn from_bytes(b: &Block) -> [u64; 8] {
    let mut t = [0u64; 8];
    for (v, chunk) in t.iter_mut().zip(b.chunks_exact(8)) {
        *v = u64::from_le_bytes(chunk.try_into().unwrap());
    }
    t
}
#[cfg(test)]
mod tests_llm_16_4_llm_16_4 {
    use super::*;
    use crate::*;
    use crate::*;
    use digest::core_api::VariableOutputCore;
    #[test]
    fn test_new_with_valid_output_sizes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        if let Ok(instance_32) = StreebogVarCore::new(rug_fuzz_0) {
            debug_assert_eq!(instance_32.h, [1; 64]);
        } else {
            panic!("StreebogVarCore::new(32) should not have failed");
        }
        if let Ok(instance_64) = StreebogVarCore::new(rug_fuzz_1) {
            debug_assert_eq!(instance_64.h, [0; 64]);
        } else {
            panic!("StreebogVarCore::new(64) should not have failed");
        }
             }
});    }
    #[test]
    fn test_new_with_invalid_output_sizes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let result = StreebogVarCore::new(rug_fuzz_0);
        debug_assert!(result.is_err());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    #[test]
    fn test_compress() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, usize, u64, usize, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let block = [rug_fuzz_0; 64];
        let msg_len = block.len() as u64;
        let mut hasher = StreebogVarCore::new(rug_fuzz_1).unwrap();
        let initial_n = hasher.n;
        let initial_sigma = hasher.sigma;
        hasher.compress(&block, msg_len);
        let expected_n = {
            let mut temp_n = initial_n;
            let mut carry = rug_fuzz_2;
            adc(&mut temp_n[rug_fuzz_3], msg_len * rug_fuzz_4, &mut carry);
            temp_n
        };
        debug_assert_eq!(hasher.n, expected_n, "n should be incremented correctly");
        let expected_sigma = {
            let mut temp_sigma = initial_sigma;
            let block_as_u64 = from_bytes(&block);
            let mut carry = rug_fuzz_5;
            for (sigma_val, &block_val) in temp_sigma.iter_mut().zip(block_as_u64.iter())
            {
                adc(sigma_val, block_val, &mut carry);
            }
            temp_sigma
        };
        debug_assert_eq!(
            hasher.sigma, expected_sigma, "sigma should be incremented correctly"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_bytes() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_test_to_bytes = 0;
        let rug_fuzz_0 = 0x0123456789abcdef;
        let rug_fuzz_1 = 0xfedcba9876543210;
        let rug_fuzz_2 = 0x0f1e2d3c4b5a6978;
        let rug_fuzz_3 = 0x89abcdef01234567;
        let rug_fuzz_4 = 0x0123456789abcdef;
        let rug_fuzz_5 = 0xfedcba9876543210;
        let rug_fuzz_6 = 0x0f1e2d3c4b5a6978;
        let rug_fuzz_7 = 0x89abcdef01234567;
        let rug_fuzz_8 = 0xef;
        let rug_fuzz_9 = 0xcd;
        let rug_fuzz_10 = 0xab;
        let rug_fuzz_11 = 0x89;
        let rug_fuzz_12 = 0x67;
        let rug_fuzz_13 = 0x45;
        let rug_fuzz_14 = 0x23;
        let rug_fuzz_15 = 0x01;
        let rug_fuzz_16 = 0x10;
        let rug_fuzz_17 = 0x32;
        let rug_fuzz_18 = 0x54;
        let rug_fuzz_19 = 0x76;
        let rug_fuzz_20 = 0x98;
        let rug_fuzz_21 = 0xba;
        let rug_fuzz_22 = 0xdc;
        let rug_fuzz_23 = 0xfe;
        let rug_fuzz_24 = 0x78;
        let rug_fuzz_25 = 0x69;
        let rug_fuzz_26 = 0x5a;
        let rug_fuzz_27 = 0x4b;
        let rug_fuzz_28 = 0x3c;
        let rug_fuzz_29 = 0x2d;
        let rug_fuzz_30 = 0x1e;
        let rug_fuzz_31 = 0x0f;
        let rug_fuzz_32 = 0x67;
        let rug_fuzz_33 = 0x45;
        let rug_fuzz_34 = 0x23;
        let rug_fuzz_35 = 0x01;
        let rug_fuzz_36 = 0xef;
        let rug_fuzz_37 = 0xcd;
        let rug_fuzz_38 = 0xab;
        let rug_fuzz_39 = 0x89;
        let rug_fuzz_40 = 0xef;
        let rug_fuzz_41 = 0xcd;
        let rug_fuzz_42 = 0xab;
        let rug_fuzz_43 = 0x89;
        let rug_fuzz_44 = 0x67;
        let rug_fuzz_45 = 0x45;
        let rug_fuzz_46 = 0x23;
        let rug_fuzz_47 = 0x01;
        let rug_fuzz_48 = 0x10;
        let rug_fuzz_49 = 0x32;
        let rug_fuzz_50 = 0x54;
        let rug_fuzz_51 = 0x76;
        let rug_fuzz_52 = 0x98;
        let rug_fuzz_53 = 0xba;
        let rug_fuzz_54 = 0xdc;
        let rug_fuzz_55 = 0xfe;
        let rug_fuzz_56 = 0x78;
        let rug_fuzz_57 = 0x69;
        let rug_fuzz_58 = 0x5a;
        let rug_fuzz_59 = 0x4b;
        let rug_fuzz_60 = 0x3c;
        let rug_fuzz_61 = 0x2d;
        let rug_fuzz_62 = 0x1e;
        let rug_fuzz_63 = 0x0f;
        let rug_fuzz_64 = 0x67;
        let rug_fuzz_65 = 0x45;
        let rug_fuzz_66 = 0x23;
        let rug_fuzz_67 = 0x01;
        let rug_fuzz_68 = 0xef;
        let rug_fuzz_69 = 0xcd;
        let rug_fuzz_70 = 0xab;
        let rug_fuzz_71 = 0x89;
        let input: [u64; 8] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let output = to_bytes(&input);
        let expected: [u8; 64] = [
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
            rug_fuzz_21,
            rug_fuzz_22,
            rug_fuzz_23,
            rug_fuzz_24,
            rug_fuzz_25,
            rug_fuzz_26,
            rug_fuzz_27,
            rug_fuzz_28,
            rug_fuzz_29,
            rug_fuzz_30,
            rug_fuzz_31,
            rug_fuzz_32,
            rug_fuzz_33,
            rug_fuzz_34,
            rug_fuzz_35,
            rug_fuzz_36,
            rug_fuzz_37,
            rug_fuzz_38,
            rug_fuzz_39,
            rug_fuzz_40,
            rug_fuzz_41,
            rug_fuzz_42,
            rug_fuzz_43,
            rug_fuzz_44,
            rug_fuzz_45,
            rug_fuzz_46,
            rug_fuzz_47,
            rug_fuzz_48,
            rug_fuzz_49,
            rug_fuzz_50,
            rug_fuzz_51,
            rug_fuzz_52,
            rug_fuzz_53,
            rug_fuzz_54,
            rug_fuzz_55,
            rug_fuzz_56,
            rug_fuzz_57,
            rug_fuzz_58,
            rug_fuzz_59,
            rug_fuzz_60,
            rug_fuzz_61,
            rug_fuzz_62,
            rug_fuzz_63,
            rug_fuzz_64,
            rug_fuzz_65,
            rug_fuzz_66,
            rug_fuzz_67,
            rug_fuzz_68,
            rug_fuzz_69,
            rug_fuzz_70,
            rug_fuzz_71,
        ];
        debug_assert_eq!(
            output, expected, "to_bytes did not produce the expected output"
        );
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_test_to_bytes = 0;
    }
}
