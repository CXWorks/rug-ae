macro_rules! fsb_impl {
    (
        $full_state:ident, $state:ident, $state_num:expr, $blocksize:ident,
        $outputsize:ident, $n:expr, $w:expr, $r:expr, $p:expr, $s:expr, $full_doc:expr,
        $doc:expr,
    ) => {
        use digest::consts:: { $blocksize, $outputsize }; #[derive(Clone)] #[doc =$doc]
        pub struct $state { blocks_len : u64, state : [u8; $r / 8], } impl HashMarker for
        $state {} impl BlockSizeUser for $state { type BlockSize = $blocksize; } impl
        OutputSizeUser for $state { type OutputSize = $outputsize; } impl BufferKindUser
        for $state { type BufferKind = Eager; } impl UpdateCore for $state { #[inline] fn
        update_blocks(& mut self, blocks : & [Block < Self >]) { self.blocks_len +=
        blocks.len() as u64; for block in blocks { Self::compress(& mut self.state,
        block); } } } impl FixedOutputCore for $state { #[inline] fn
        finalize_fixed_core(& mut self, buffer : & mut Buffer < Self >, out : & mut
        Output < Self >) { let block_bytes = self.blocks_len * Self::BlockSize::U64; let
        bit_len = 8 * (block_bytes + buffer.get_pos() as u64); let mut h = self.state;
        buffer.len64_padding_be(bit_len, | b | Self::compress(& mut h, b)); let res =
        whirlpool::Whirlpool::digest(& h[..]); let n = out.len(); out.copy_from_slice(&
        res[..n]); } } impl Default for $state { #[inline] fn default() -> Self { Self {
        blocks_len : 0u64, state : [0u8; $r / 8], } } } impl Reset for $state { #[inline]
        fn reset(& mut self) { * self = Default::default(); } } impl AlgorithmName for
        $state { fn write_alg_name(f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(stringify!($full_state)) } } impl fmt::Debug for $state { fn fmt(&
        self, f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(concat!(stringify!($state), " { ... }")) } } #[doc =$full_doc] pub
        type $full_state = CoreWrapper <$state >; impl $state { const
        SIZE_OUTPUT_COMPRESS : usize = $r / 8; const SIZE_INPUT_COMPRESS : usize = $s /
        8; const SIZE_MSG_CHUNKS : usize = Self::SIZE_INPUT_COMPRESS -
        Self::SIZE_OUTPUT_COMPRESS; const SIZE_VECTORS : usize = $p / 8 + 1; const SHIFT
        : u8 = 8 - ($p % 8) as u8; fn define_iv(index : usize) -> [u8;
        Self::SIZE_VECTORS] { let mut subset_pi : [u8; Self::SIZE_VECTORS] = [0u8;
        Self::SIZE_VECTORS]; subset_pi.copy_from_slice(& PI[index * Self::SIZE_VECTORS..
        (index + 1) * Self::SIZE_VECTORS],); if let Some(last) = subset_pi.last_mut() { *
        last >>= Self::SHIFT; * last <<= Self::SHIFT; } subset_pi } #[doc =
        " Vector XORing. Given the s input bits of the function, we derive a set of w indexes"]
        #[doc =
        " $(W_i)_{i\\in[0;w-1]}$ between $0$ and $n - 1$. The value of each $W_i$ is computed"]
        #[doc = " from the inputs bits like this:"] #[doc =
        " $W_i = i \\times (n / w) + IV_i + M_i \\times 2^{r / w}."] fn
        computing_w_indices(input_vector : & [u8; Self::SIZE_OUTPUT_COMPRESS], message :
        & Block < Self >,) -> [u32; $w] { let mut wind : [u32; $w] = [0; $w]; let
        divided_message : [u8; $w] = Self::dividing_bits(message, ($s - $r) / $w); for i
        in 0.. ($w) { let message_i = divided_message[i] as u32; wind[i] = (i * $n / $w)
        as u32 + input_vector[i] as u32 + (message_i << ($r / $w) as u8); } wind } #[doc
        =
        " This function servers the purpose presented in table 3, of breaking a bit array into"]
        #[doc =
        " batches of size not multiple of 8. Note that the IV will be broken always in size 8, which"]
        #[doc =
        " is quite convenient. Also, the only numbers we'll have to worry for are 5 and 6."]
        fn dividing_bits(input_bits : & Block < Self >, size_batches : usize) -> [u8; $w]
        { if size_batches != 5usize && size_batches != 6usize {
        panic!("Expecting batches of size 5 or 6. Other values do not follow \
                    the standard specification")
        } let mut new_bits = [0u8; $w]; let shifting_factor = (8 - size_batches) as u8;
        for (i, new_bit) in new_bits.iter_mut().enumerate().take($w - 1) { let position =
        i * size_batches; let initial_byte = position / 8; let initial_bit = position %
        8; let switch = (initial_bit + size_batches - 1) / 8; if switch == 1 { * new_bit
        = (input_bits[initial_byte] << initial_bit as u8 | input_bits[initial_byte + 1]
        >> (8 - initial_bit as u8)) >> shifting_factor; } else { * new_bit =
        (input_bits[initial_byte] << initial_bit as u8) >> shifting_factor; } }
        new_bits[$w - 1] = (input_bits[Self::SIZE_MSG_CHUNKS - 1] << shifting_factor) >>
        shifting_factor; new_bits } #[doc =
        " This function outputs r bits, which are used to chain to the next iteration."]
        fn compress(hash : & mut [u8; Self::SIZE_OUTPUT_COMPRESS], message_block : &
        Block < Self >) { let mut initial_vector = [0u8; Self::SIZE_OUTPUT_COMPRESS]; let
        w_indices = Self::computing_w_indices(hash, message_block); for w_index in
        w_indices.iter() { let chosen_vec = w_index / $r as u32; let shift_value =
        w_index % $r as u32; let mut vector = Self::define_iv(chosen_vec as usize); let
        truncated = Self::shift_and_truncate(& mut vector, shift_value); initial_vector
        .iter_mut().zip(truncated.iter()).for_each(| (x1, x2) | * x1 ^= * x2); } * hash =
        initial_vector; } fn shift_and_truncate(array : & mut [u8; Self::SIZE_VECTORS],
        shift_value : u32,) -> [u8; Self::SIZE_OUTPUT_COMPRESS] { let array_len = array
        .len(); let bits_in_cue = ($p % 8) as u8; let mut truncated = [0u8;
        Self::SIZE_OUTPUT_COMPRESS]; if shift_value == 0 { truncated.copy_from_slice(&
        array[..Self::SIZE_OUTPUT_COMPRESS]); } else if shift_value <= (bits_in_cue as
        u32) { let bytes_to_shift = 1; let starting_byte = (array_len - bytes_to_shift)
        as usize; truncated[0] = array[starting_byte] << (bits_in_cue - shift_value as
        u8); truncated[0] ^= array[0] >> shift_value; for position in 1
        ..Self::SIZE_OUTPUT_COMPRESS { truncated[position] ^= array[position - 1] << (8 -
        shift_value); truncated[position] ^= array[position] >> shift_value; } } else {
        let bytes_to_shift = (((shift_value - bits_in_cue as u32 - 1) / 8) + 2) as usize;
        let starting_byte = (array_len - bytes_to_shift) as usize; let remaining_bits =
        ((shift_value - bits_in_cue as u32) % 8) as u8; if remaining_bits != 0 { for
        position in 0.. (bytes_to_shift - 1) { truncated[position] = array[starting_byte
        + position] << (8 - remaining_bits) | array[starting_byte + position + 1] >>
        remaining_bits; } let difference = bits_in_cue.checked_sub(8 - remaining_bits);
        match difference { Some(x) => { if x > 0 { truncated[bytes_to_shift - 1] ^=
        array[starting_byte + bytes_to_shift - 1] << (bits_in_cue - x);
        truncated[bytes_to_shift - 1] ^= array[0] >> x; for (index, position) in
        (bytes_to_shift..Self::SIZE_OUTPUT_COMPRESS).enumerate() { truncated[position] ^=
        array[index] << (8 - x); truncated[position] ^= array[index + 1] >> x; } } else {
        for (index, position) in ((bytes_to_shift - 1)..Self::SIZE_OUTPUT_COMPRESS)
        .enumerate() { truncated[position] = array[index]; } } } None => { let
        positive_diff = (8 - remaining_bits) - bits_in_cue; truncated[bytes_to_shift - 2]
        ^= array[0] >> (8 - positive_diff); for (index, position) in ((bytes_to_shift -
        1)..Self::SIZE_OUTPUT_COMPRESS).enumerate() { truncated[position] ^= array[index]
        << positive_diff; truncated[position] ^= array[index + 1] >> (8 - positive_diff);
        } } } } else { truncated[..bytes_to_shift].clone_from_slice(& array[starting_byte
        .. (starting_byte + bytes_to_shift)],); truncated[bytes_to_shift - 1] ^= array[0]
        >> bits_in_cue; for (index, position) in (bytes_to_shift
        ..Self::SIZE_OUTPUT_COMPRESS).enumerate() { truncated[position] ^= array[index]
        << (8 - bits_in_cue); truncated[position] ^= array[index + 1] >> bits_in_cue; } }
        } truncated } }
    };
}
#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;
    use crate::*;
    use core::default::Default;
    #[test]
    fn test_fsb160core_default() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_test_fsb160core_default = 0;
        let fsb160core_default = <Fsb160Core as core::default::Default>::default();
        debug_assert_eq!(fsb160core_default.blocks_len, 0u64);
        debug_assert_eq!(
            fsb160core_default.state, [0u8; Fsb160Core::SIZE_OUTPUT_COMPRESS]
        );
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_test_fsb160core_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    use core::default::Default;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_default = 0;
        let default_fsb224core = <Fsb224Core as Default>::default();
        debug_assert_eq!(default_fsb224core.blocks_len, 0u64);
        debug_assert_eq!(
            default_fsb224core.state, [0u8; Fsb224Core::SIZE_OUTPUT_COMPRESS]
        );
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_7_llm_16_7 {
    use super::*;
    use crate::*;
    use crate::Fsb224Core;
    use digest::Reset;
    use core::fmt;
    #[test]
    fn reset_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hasher = Fsb224Core::default();
        hasher.blocks_len = rug_fuzz_0;
        hasher.state = [rug_fuzz_1; Fsb224Core::SIZE_OUTPUT_COMPRESS];
        hasher.reset();
        let expected_state = Fsb224Core::default();
        debug_assert_eq!(
            hasher.blocks_len, expected_state.blocks_len,
            "Fsb224Core blocks_len not properly reset"
        );
        debug_assert_eq!(
            hasher.state, expected_state.state, "Fsb224Core state not properly reset"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use super::*;
    use crate::*;
    use crate::Fsb256Core;
    #[test]
    fn default_test() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_default_test = 0;
        let default_fsb256 = Fsb256Core::default();
        debug_assert_eq!(default_fsb256.blocks_len, 0u64);
        debug_assert_eq!(default_fsb256.state, [0u8; Fsb256Core::SIZE_OUTPUT_COMPRESS]);
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_default_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_16_llm_16_16 {
    use super::*;
    use crate::*;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_16_llm_16_16_rrrruuuugggg_test_default = 0;
        let default_fsb384core = <Fsb384Core as core::default::Default>::default();
        debug_assert_eq!(default_fsb384core.blocks_len, 0u64);
        debug_assert_eq!(
            default_fsb384core.state, [0u8; Fsb384Core::SIZE_OUTPUT_COMPRESS]
        );
        let _rug_ed_tests_llm_16_16_llm_16_16_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_19 {
    use super::*;
    use crate::*;
    use digest::core_api::{Buffer, FixedOutputCore};
    use digest::Update;
    #[test]
    fn test_finalize_fixed_core() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Fsb384Core::default();
        let mut buffer = Buffer::<Fsb384Core>::default();
        let mut out = Output::<Fsb384Core>::default();
        let block_bytes = core.blocks_len
            * <Fsb384Core as digest::core_api::BlockSizeUser>::BlockSize::U64;
        let bit_len = rug_fuzz_0 * (block_bytes + buffer.get_pos() as u64);
        buffer.len64_padding_be(bit_len, |b| Fsb384Core::compress(&mut core.state, b));
        let expected = whirlpool::Whirlpool::digest(&core.state[..]);
        let n = expected.len();
        core.finalize_fixed_core(&mut buffer, &mut out);
        debug_assert_eq!(& out[..n], & expected[..n]);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_21 {
    use super::*;
    use crate::*;
    #[test]
    fn default_initializes_correctly() {
        let _rug_st_tests_llm_16_21_rrrruuuugggg_default_initializes_correctly = 0;
        let fsb_core: Fsb512Core = Default::default();
        debug_assert_eq!(fsb_core.blocks_len, 0u64);
        debug_assert_eq!(fsb_core.state, [0u8; Fsb512Core::SIZE_OUTPUT_COMPRESS]);
        let _rug_ed_tests_llm_16_21_rrrruuuugggg_default_initializes_correctly = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_27 {
    use crate::Fsb160Core;
    use crate::Block;
    const S: usize = 160;
    const R: usize = 80;
    const W: usize = 5;
    const N: usize = 256;
    #[test]
    fn test_computing_w_indices() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input_vector: [u8; Fsb160Core::SIZE_OUTPUT_COMPRESS] = [rug_fuzz_0; Fsb160Core::SIZE_OUTPUT_COMPRESS];
        let message = Block::<Fsb160Core>::default();
        let w_indices = Fsb160Core::computing_w_indices(&input_vector, &message);
        for i in rug_fuzz_1..W {
            let expected_wi = (i * N / W) as u32 + input_vector[i] as u32;
            debug_assert_eq!(
                w_indices[i], expected_wi, "W_indices[{}] does not match expected value",
                i
            );
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_29 {
    use super::*;
    use crate::*;
    #[test]
    #[should_panic(expected = "Expecting batches of size 5 or 6")]
    fn dividing_bits_invalid_size() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input_bits = Block::<Fsb160Core>::default();
        Fsb160Core::dividing_bits(&input_bits, rug_fuzz_0);
             }
});    }
    #[test]
    fn dividing_bits_valid_size_five() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input_bits = Block::<Fsb160Core>::default();
        let result = Fsb160Core::dividing_bits(&input_bits, rug_fuzz_0);
             }
});    }
    #[test]
    fn dividing_bits_valid_size_six() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input_bits = Block::<Fsb160Core>::default();
        let result = Fsb160Core::dividing_bits(&input_bits, rug_fuzz_0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_30_llm_16_30 {
    use crate::Fsb160Core;
    #[test]
    fn test_shift_and_truncate_no_shift() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array = [rug_fuzz_0; Fsb160Core::SIZE_VECTORS];
        for i in rug_fuzz_1..Fsb160Core::SIZE_VECTORS {
            array[i] = i as u8;
        }
        let result = Fsb160Core::shift_and_truncate(&mut array, rug_fuzz_2);
        debug_assert_eq!(result, array[..Fsb160Core::SIZE_OUTPUT_COMPRESS]);
             }
});    }
    #[test]
    fn test_shift_and_truncate_small_shift() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, usize, u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array = [rug_fuzz_0; Fsb160Core::SIZE_VECTORS];
        for i in rug_fuzz_1..Fsb160Core::SIZE_VECTORS {
            array[i] = (i as u8).wrapping_add(rug_fuzz_2);
        }
        let shift_value = rug_fuzz_3;
        let result = Fsb160Core::shift_and_truncate(&mut array, shift_value);
        let mut expected = [rug_fuzz_4; Fsb160Core::SIZE_OUTPUT_COMPRESS];
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_shift_and_truncate_large_shift() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, usize, u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut array = [rug_fuzz_0; Fsb160Core::SIZE_VECTORS];
        for i in rug_fuzz_1..Fsb160Core::SIZE_VECTORS {
            array[i] = (i as u8).wrapping_add(rug_fuzz_2);
        }
        let shift_value = rug_fuzz_3;
        let result = Fsb160Core::shift_and_truncate(&mut array, shift_value);
        let mut expected = [rug_fuzz_4; Fsb160Core::SIZE_OUTPUT_COMPRESS];
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_39_llm_16_39 {
    use crate::{Fsb256Core, Block};
    const W: usize = 128;
    #[test]
    fn test_dividing_bits_5() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input_bits = Block::<Fsb256Core>::default();
        let size_batches = rug_fuzz_0;
        let result = Fsb256Core::dividing_bits(&input_bits, size_batches);
        let expected_output = [rug_fuzz_1; W];
        debug_assert_eq!(result, expected_output);
             }
});    }
    #[test]
    fn test_dividing_bits_6() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input_bits = Block::<Fsb256Core>::default();
        let size_batches = rug_fuzz_0;
        let result = Fsb256Core::dividing_bits(&input_bits, size_batches);
        let expected_output = [rug_fuzz_1; W];
        debug_assert_eq!(result, expected_output);
             }
});    }
    #[test]
    #[should_panic(
        expected = "Expecting batches of size 5 or 6. Other values do not follow the standard specification"
    )]
    fn test_dividing_bits_invalid_size() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input_bits = Block::<Fsb256Core>::default();
        let size_batches = rug_fuzz_0;
        let _result = Fsb256Core::dividing_bits(&input_bits, size_batches);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_43 {
    use super::*;
    use crate::*;
    #[test]
    fn test_define_iv() {
        struct MockFsb384Core;
        impl MockFsb384Core {
            const SIZE_VECTORS: usize = 32;
            const SHIFT: u8 = 4;
        }
        const PI: [u8; MockFsb384Core::SIZE_VECTORS * 10] = [0u8; MockFsb384Core::SIZE_VECTORS
            * 10];
        impl MockFsb384Core {
            fn define_iv(index: usize) -> [u8; Self::SIZE_VECTORS] {
                let mut subset_pi: [u8; Self::SIZE_VECTORS] = [0u8; Self::SIZE_VECTORS];
                subset_pi
                    .copy_from_slice(
                        &PI[index * Self::SIZE_VECTORS..(index + 1) * Self::SIZE_VECTORS],
                    );
                if let Some(last) = subset_pi.last_mut() {
                    *last >>= Self::SHIFT;
                    *last <<= Self::SHIFT;
                }
                subset_pi
            }
        }
        let iv0 = MockFsb384Core::define_iv(0);
        assert_eq!(iv0, [0; MockFsb384Core::SIZE_VECTORS]);
        let iv1 = MockFsb384Core::define_iv(1);
        assert_eq!(iv1, [0; MockFsb384Core::SIZE_VECTORS]);
        let test_index = 5;
        let mut expected_iv = [0u8; MockFsb384Core::SIZE_VECTORS];
        expected_iv
            .copy_from_slice(
                &PI[test_index
                    * MockFsb384Core::SIZE_VECTORS..(test_index + 1)
                    * MockFsb384Core::SIZE_VECTORS],
            );
        if let Some(last) = expected_iv.last_mut() {
            *last >>= MockFsb384Core::SHIFT;
            *last <<= MockFsb384Core::SHIFT;
        }
        let iv_test_index = MockFsb384Core::define_iv(test_index);
        assert_eq!(iv_test_index, expected_iv);
    }
}
#[cfg(test)]
mod tests_rug_60 {
    use crate::Fsb160Core;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_60_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Fsb160Core::default();
        let mut p1: [GenericArray<u8, <Fsb160Core as BlockSizeUser>::BlockSize>; 25] = Default::default();
        Fsb160Core::update_blocks(&mut p0, &p1);
        let _rug_ed_tests_rug_60_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_61 {
    use super::*;
    use crate::Fsb160Core;
    use crate::digest::core_api::{FixedOutputCore, BlockSizeUser, BufferKindUser};
    use crate::digest::generic_array::GenericArray;
    use crate::digest::OutputSizeUser;
    use crate::digest::block_buffer::BlockBuffer;
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_61_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0: Fsb160Core = Fsb160Core::default();
        let mut p1: BlockBuffer<
            <Fsb160Core as BlockSizeUser>::BlockSize,
            <Fsb160Core as BufferKindUser>::BufferKind,
        > = BlockBuffer::default();
        let mut p2: GenericArray<u8, <Fsb160Core as OutputSizeUser>::OutputSize> = GenericArray::<
            u8,
            <Fsb160Core as OutputSizeUser>::OutputSize,
        >::default();
        <Fsb160Core as FixedOutputCore>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_61_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_62 {
    use super::*;
    use crate::digest::Reset;
    use crate::Fsb160Core;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_62_rrrruuuugggg_test_rug = 0;
        let mut p0 = Fsb160Core::default();
        <Fsb160Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_62_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_64 {
    use super::*;
    #[test]
    fn test_define_iv() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let result = crate::Fsb160Core::define_iv(p0);
        debug_assert_eq!(result[..], [] [..]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_65 {
    use super::*;
    use crate::Fsb160Core;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_compress() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = [rug_fuzz_0; Fsb160Core::SIZE_OUTPUT_COMPRESS];
        let mut p1 = GenericArray::<
            u8,
            <Fsb160Core as BlockSizeUser>::BlockSize,
        >::default();
        Fsb160Core::compress(&mut p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_66 {
    use super::*;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    use crate::Fsb224Core;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_66_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Fsb224Core::default();
        let mut block = GenericArray::<
            u8,
            <Fsb224Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1 = &[block];
        <Fsb224Core as UpdateCore>::update_blocks(&mut p0, p1);
        let _rug_ed_tests_rug_66_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_67 {
    use super::*;
    use crate::digest::core_api::{FixedOutputCore, BlockSizeUser, BufferKindUser};
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::GenericArray;
    use crate::Fsb224Core;
    use crate::digest::OutputSizeUser;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_67_rrrruuuugggg_test_rug = 0;
        let mut p0 = Fsb224Core::default();
        let mut p1 = BlockBuffer::<
            <Fsb224Core as BlockSizeUser>::BlockSize,
            <Fsb224Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Fsb224Core as OutputSizeUser>::OutputSize,
        >::default();
        <Fsb224Core as FixedOutputCore>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_67_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_69 {
    use super::*;
    #[test]
    fn test_define_iv() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let result = crate::Fsb224Core::define_iv(p0);
        debug_assert_eq!(result.len(), crate ::Fsb224Core::SIZE_VECTORS);
             }
});    }
}
#[cfg(test)]
mod tests_rug_70 {
    use super::*;
    use crate::Fsb224Core;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_computing_w_indices() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; Fsb224Core::SIZE_OUTPUT_COMPRESS] = [rug_fuzz_0; Fsb224Core::SIZE_OUTPUT_COMPRESS];
        let mut p1: GenericArray<u8, <Fsb224Core as BlockSizeUser>::BlockSize> = GenericArray::<
            u8,
            <Fsb224Core as BlockSizeUser>::BlockSize,
        >::default();
        let _result = Fsb224Core::computing_w_indices(&p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_71 {
    use super::*;
    use crate::Fsb224Core;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_dividing_bits() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = GenericArray::<
            u8,
            <Fsb224Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1: usize = rug_fuzz_0;
        let result = <Fsb224Core>::dividing_bits(&p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_72 {
    use super::*;
    use crate::Fsb224Core;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_compress() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; Fsb224Core::SIZE_OUTPUT_COMPRESS] = [rug_fuzz_0; Fsb224Core::SIZE_OUTPUT_COMPRESS];
        let mut p1 = GenericArray::<
            u8,
            <Fsb224Core as BlockSizeUser>::BlockSize,
        >::default();
        <Fsb224Core>::compress(&mut p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_74 {
    use crate::Fsb256Core;
    use crate::digest::core_api::UpdateCore;
    use crate::digest::core_api::{BlockSizeUser, Block};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_74_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Fsb256Core::default();
        let block = GenericArray::<
            u8,
            <Fsb256Core as BlockSizeUser>::BlockSize,
        >::default();
        let mut p1 = &[block];
        <Fsb256Core as UpdateCore>::update_blocks(&mut p0, p1);
        let _rug_ed_tests_rug_74_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_75 {
    use crate::Fsb256Core;
    use crate::digest::core_api::{
        FixedOutputCore, BlockSizeUser, BufferKindUser, OutputSizeUser,
    };
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_75_rrrruuuugggg_test_rug = 0;
        let mut p0 = Fsb256Core::default();
        let mut p1 = BlockBuffer::<
            <Fsb256Core as BlockSizeUser>::BlockSize,
            <Fsb256Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Fsb256Core as OutputSizeUser>::OutputSize,
        >::default();
        <Fsb256Core as FixedOutputCore>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_75_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_76 {
    use crate::Fsb256Core;
    use crate::digest::Reset;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_76_rrrruuuugggg_test_rug = 0;
        let mut p0 = Fsb256Core::default();
        <Fsb256Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_76_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_78 {
    use super::*;
    #[test]
    fn test_define_iv() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let result = crate::Fsb256Core::define_iv(p0);
        debug_assert_eq!(result.len(), crate ::Fsb256Core::SIZE_VECTORS);
             }
});    }
}
#[cfg(test)]
mod tests_rug_79 {
    use super::*;
    use digest::generic_array::GenericArray;
    use crate::Fsb256Core;
    use digest::core_api::{BlockSizeUser, Block};
    #[test]
    fn test_computing_w_indices() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: [u8; Fsb256Core::SIZE_OUTPUT_COMPRESS] = [rug_fuzz_0; Fsb256Core::SIZE_OUTPUT_COMPRESS];
        let p1: Block<Fsb256Core> = GenericArray::<
            u8,
            <Fsb256Core as BlockSizeUser>::BlockSize,
        >::default();
        <Fsb256Core>::computing_w_indices(&p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_80 {
    use super::*;
    use digest::generic_array::GenericArray;
    use digest::core_api::BlockSizeUser;
    use crate::Fsb256Core;
    #[test]
    fn test_compress() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hash = [rug_fuzz_0; Fsb256Core::SIZE_OUTPUT_COMPRESS];
        let mut message_block = GenericArray::<
            u8,
            <Fsb256Core as BlockSizeUser>::BlockSize,
        >::default();
        Fsb256Core::compress(&mut hash, &message_block);
             }
});    }
}
#[cfg(test)]
mod tests_rug_82 {
    use super::*;
    use crate::Fsb384Core;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_82_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Fsb384Core::default();
        let mut block = GenericArray::<
            u8,
            <Fsb384Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1 = &mut [block];
        <Fsb384Core as UpdateCore>::update_blocks(&mut p0, p1);
        let _rug_ed_tests_rug_82_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_83 {
    use super::*;
    use crate::digest::Reset;
    use crate::Fsb384Core;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_83_rrrruuuugggg_test_reset = 0;
        let mut p0 = Fsb384Core::default();
        <Fsb384Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_83_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_85 {
    use super::*;
    use crate::Fsb384Core;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_computing_w_indices() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: [u8; Fsb384Core::SIZE_OUTPUT_COMPRESS] = [rug_fuzz_0; Fsb384Core::SIZE_OUTPUT_COMPRESS];
        let mut p1: GenericArray<u8, <Fsb384Core as BlockSizeUser>::BlockSize> = GenericArray::<
            u8,
            <Fsb384Core as BlockSizeUser>::BlockSize,
        >::default();
        let _result = Fsb384Core::computing_w_indices(&p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_86 {
    use super::*;
    use digest::generic_array::GenericArray;
    use digest::core_api::BlockSizeUser;
    use crate::Fsb384Core;
    #[test]
    #[should_panic]
    fn test_dividing_bits_invalid_batch_size() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = GenericArray::<
            u8,
            <Fsb384Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1: usize = rug_fuzz_0;
        <Fsb384Core>::dividing_bits(&p0, p1);
             }
});    }
    #[test]
    fn test_dividing_bits_valid_batch_size_5() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = GenericArray::<
            u8,
            <Fsb384Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1: usize = rug_fuzz_0;
        let result = <Fsb384Core>::dividing_bits(&p0, p1);
             }
});    }
    #[test]
    fn test_dividing_bits_valid_batch_size_6() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = GenericArray::<
            u8,
            <Fsb384Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1: usize = rug_fuzz_0;
        let result = <Fsb384Core>::dividing_bits(&p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_87 {
    use super::*;
    use crate::Fsb384Core;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_compress() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; Fsb384Core::SIZE_OUTPUT_COMPRESS] = [rug_fuzz_0; Fsb384Core::SIZE_OUTPUT_COMPRESS];
        let mut p1: GenericArray<u8, <Fsb384Core as BlockSizeUser>::BlockSize> = GenericArray::<
            u8,
            <Fsb384Core as BlockSizeUser>::BlockSize,
        >::default();
        <Fsb384Core>::compress(&mut p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_89 {
    use crate::Fsb512Core;
    use crate::digest::core_api::UpdateCore;
    use crate::digest::core_api::BlockSizeUser;
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_89_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Fsb512Core::default();
        let mut blocks = [
            GenericArray::<u8, <Fsb512Core as BlockSizeUser>::BlockSize>::default(),
        ];
        <Fsb512Core as UpdateCore>::update_blocks(&mut p0, &blocks);
        let _rug_ed_tests_rug_89_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_90 {
    use super::*;
    use crate::digest::core_api::{FixedOutputCore, BufferKindUser, BlockSizeUser};
    use crate::Fsb512Core;
    use digest::OutputSizeUser;
    use digest::{block_buffer::BlockBuffer, generic_array::GenericArray};
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_90_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Fsb512Core::default();
        let mut p1 = BlockBuffer::<
            <Fsb512Core as BlockSizeUser>::BlockSize,
            <Fsb512Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Fsb512Core as OutputSizeUser>::OutputSize,
        >::default();
        <Fsb512Core>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_90_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_91 {
    use super::*;
    use crate::Fsb512Core;
    use crate::digest::Reset;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_91_rrrruuuugggg_test_reset = 0;
        let mut p0 = Fsb512Core::default();
        <Fsb512Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_91_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_93 {
    use super::*;
    #[test]
    fn test_define_iv() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let iv = crate::Fsb512Core::define_iv(p0);
        debug_assert_eq!(iv.len(), crate ::Fsb512Core::SIZE_VECTORS);
             }
});    }
}
#[cfg(test)]
mod tests_rug_94 {
    use super::*;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    use crate::Fsb512Core;
    #[test]
    fn test_computing_w_indices() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; Fsb512Core::SIZE_OUTPUT_COMPRESS] = [rug_fuzz_0; Fsb512Core::SIZE_OUTPUT_COMPRESS];
        let mut p1 = GenericArray::<
            u8,
            <Fsb512Core as BlockSizeUser>::BlockSize,
        >::default();
        Fsb512Core::computing_w_indices(&p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_95 {
    use super::*;
    use digest::generic_array::GenericArray;
    use digest::core_api::BlockSizeUser;
    use crate::Fsb512Core;
    #[test]
    fn test_dividing_bits() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = GenericArray::<
            u8,
            <Fsb512Core as BlockSizeUser>::BlockSize,
        >::default();
        let mut p1: usize = rug_fuzz_0;
        <Fsb512Core>::dividing_bits(&p0, p1);
        p1 = rug_fuzz_1;
        <Fsb512Core>::dividing_bits(&p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_96 {
    use super::*;
    use digest::generic_array::GenericArray;
    use digest::core_api::BlockSizeUser;
    use crate::Fsb512Core;
    #[test]
    fn test_compress() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; Fsb512Core::SIZE_OUTPUT_COMPRESS] = [rug_fuzz_0; Fsb512Core::SIZE_OUTPUT_COMPRESS];
        let mut p1: GenericArray<u8, <Fsb512Core as BlockSizeUser>::BlockSize> = GenericArray::<
            u8,
            <Fsb512Core as BlockSizeUser>::BlockSize,
        >::default();
        crate::Fsb512Core::compress(&mut p0, &p1);
             }
});    }
}
