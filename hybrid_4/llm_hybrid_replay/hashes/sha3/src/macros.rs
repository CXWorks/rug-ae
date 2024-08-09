macro_rules! impl_sha3 {
    (
        $name:ident, $full_name:ident, $output_size:ident, $rate:ident, $pad:expr,
        $alg_name:expr $(,)?
    ) => {
        #[doc = "Core "] #[doc = $alg_name] #[doc = " hasher state."] #[derive(Clone)]
        #[allow(non_camel_case_types)] pub struct $name { state : Sha3State, } impl
        HashMarker for $name {} impl BlockSizeUser for $name { type BlockSize = $rate; }
        impl BufferKindUser for $name { type BufferKind = Eager; } impl OutputSizeUser
        for $name { type OutputSize = $output_size; } impl UpdateCore for $name {
        #[inline] fn update_blocks(& mut self, blocks : & [Block < Self >]) { for block
        in blocks { self.state.absorb_block(block) } } } impl FixedOutputCore for $name {
        #[inline] fn finalize_fixed_core(& mut self, buffer : & mut Buffer < Self >, out
        : & mut Output < Self >) { let pos = buffer.get_pos(); let block = buffer
        .pad_with_zeros(); block[pos] = $pad; let n = block.len(); block[n - 1] |= 0x80;
        self.state.absorb_block(block); self.state.as_bytes(out); } } impl Default for
        $name { #[inline] fn default() -> Self { Self { state : Default::default(), } } }
        impl Reset for $name { #[inline] fn reset(& mut self) { * self =
        Default::default(); } } impl AlgorithmName for $name { fn write_alg_name(f : &
        mut fmt::Formatter <'_ >) -> fmt::Result { f.write_str(stringify!($full_name)) }
        } impl fmt::Debug for $name { fn fmt(& self, f : & mut fmt::Formatter <'_ >) ->
        fmt::Result { f.write_str(concat!(stringify!($name), " { ... }")) } } #[doc =
        $alg_name] #[doc = " hasher state."] pub type $full_name = CoreWrapper <$name >;
    };
    (
        $name:ident, $full_name:ident, $output_size:ident, $rate:ident, $pad:expr,
        $alg_name:expr, $oid:literal $(,)?
    ) => {
        impl_sha3!($name, $full_name, $output_size, $rate, $pad, $alg_name);
        #[cfg(feature = "oid")] #[cfg_attr(docsrs, doc(cfg(feature = "oid")))] impl
        AssociatedOid for $name { const OID : ObjectIdentifier =
        ObjectIdentifier::new_unwrap($oid); }
    };
}
macro_rules! impl_shake {
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident, $rate:ident,
        $pad:expr, $alg_name:expr $(,)?
    ) => {
        #[doc = "Core "] #[doc = $alg_name] #[doc = " hasher state."] #[derive(Clone)]
        #[allow(non_camel_case_types)] pub struct $name { state : Sha3State, } impl
        HashMarker for $name {} impl BlockSizeUser for $name { type BlockSize = $rate; }
        impl BufferKindUser for $name { type BufferKind = Eager; } impl UpdateCore for
        $name { #[inline] fn update_blocks(& mut self, blocks : & [Block < Self >]) { for
        block in blocks { self.state.absorb_block(block) } } } impl ExtendableOutputCore
        for $name { type ReaderCore = $reader; #[inline] fn finalize_xof_core(& mut self,
        buffer : & mut Buffer < Self >) -> Self::ReaderCore { let pos = buffer.get_pos();
        let block = buffer.pad_with_zeros(); block[pos] = $pad; let n = block.len();
        block[n - 1] |= 0x80; self.state.absorb_block(block); $reader { state : self
        .state.clone(), } } } impl Default for $name { #[inline] fn default() -> Self {
        Self { state : Default::default(), } } } impl Reset for $name { #[inline] fn
        reset(& mut self) { * self = Default::default(); } } impl AlgorithmName for $name
        { fn write_alg_name(f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(stringify!($full_name)) } } impl fmt::Debug for $name { fn fmt(& self,
        f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(concat!(stringify!($name), " { ... }")) } } #[doc = "Core "] #[doc =
        $alg_name] #[doc = " reader state."] #[derive(Clone)]
        #[allow(non_camel_case_types)] pub struct $reader { state : Sha3State, } impl
        BlockSizeUser for $reader { type BlockSize = $rate; } impl XofReaderCore for
        $reader { #[inline] fn read_block(& mut self) -> Block < Self > { let mut block =
        Block::< Self >::default(); self.state.as_bytes(& mut block); self.state
        .permute(); block } } #[doc = $alg_name] #[doc = " hasher state."] pub type
        $full_name = CoreWrapper <$name >; #[doc = $alg_name] #[doc = " reader state."]
        pub type $reader_full = XofReaderCoreWrapper <$reader >;
    };
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident, $rate:ident,
        $pad:expr, $alg_name:expr, $oid:literal $(,)?
    ) => {
        impl_shake!($name, $full_name, $reader, $reader_full, $rate, $pad, $alg_name);
        #[cfg(feature = "oid")] #[cfg_attr(docsrs, doc(cfg(feature = "oid")))] impl
        AssociatedOid for $name { const OID : ObjectIdentifier =
        ObjectIdentifier::new_unwrap($oid); }
    };
}
macro_rules! impl_turbo_shake {
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident, $rate:ident,
        $alg_name:expr $(,)?
    ) => {
        #[doc = "Core "] #[doc = $alg_name] #[doc = " hasher state."] #[derive(Clone)]
        #[allow(non_camel_case_types)] pub struct $name { domain_separation : u8, state :
        Sha3State, } impl $name { #[doc =
        " Creates a new TurboSHAKE instance with the given domain separation."] #[doc =
        " Note that the domain separation needs to be a byte with a value in"] #[doc =
        " the range [0x01, . . . , 0x7F]"] pub fn new(domain_separation : u8) -> Self {
        assert!((0x01..= 0x7F).contains(& domain_separation)); Self { domain_separation,
        state : Sha3State::new(TURBO_SHAKE_ROUND_COUNT), } } } impl HashMarker for $name
        {} impl BlockSizeUser for $name { type BlockSize = $rate; } impl BufferKindUser
        for $name { type BufferKind = Eager; } impl UpdateCore for $name { #[inline] fn
        update_blocks(& mut self, blocks : & [Block < Self >]) { for block in blocks {
        self.state.absorb_block(block) } } } impl ExtendableOutputCore for $name { type
        ReaderCore = $reader; #[inline] fn finalize_xof_core(& mut self, buffer : & mut
        Buffer < Self >) -> Self::ReaderCore { let pos = buffer.get_pos(); let block =
        buffer.pad_with_zeros(); block[pos] = self.domain_separation; let n = block
        .len(); block[n - 1] |= 0x80; self.state.absorb_block(block); $reader { state :
        self.state.clone(), } } } impl Reset for $name { #[inline] fn reset(& mut self) {
        * self = Self::new(self.domain_separation); } } impl AlgorithmName for $name { fn
        write_alg_name(f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(stringify!($full_name)) } } impl fmt::Debug for $name { fn fmt(& self,
        f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(concat!(stringify!($name), " { ... }")) } } #[doc = "Core "] #[doc =
        $alg_name] #[doc = " reader state."] #[derive(Clone)]
        #[allow(non_camel_case_types)] pub struct $reader { state : Sha3State, } impl
        BlockSizeUser for $reader { type BlockSize = $rate; } impl XofReaderCore for
        $reader { #[inline] fn read_block(& mut self) -> Block < Self > { let mut block =
        Block::< Self >::default(); self.state.as_bytes(& mut block); self.state
        .permute(); block } } #[doc = $alg_name] #[doc = " hasher state."] pub type
        $full_name = CoreWrapper <$name >; #[doc = $alg_name] #[doc = " reader state."]
        pub type $reader_full = XofReaderCoreWrapper <$reader >;
    };
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident, $rate:ident,
        $alg_name:expr, $oid:literal $(,)?
    ) => {
        impl_turbo_shake!($name, $full_name, $reader, $reader_full, $rate, $alg_name);
        #[cfg(feature = "oid")] #[cfg_attr(docsrs, doc(cfg(feature = "oid")))] impl
        AssociatedOid for $name { const OID : ObjectIdentifier =
        ObjectIdentifier::new_unwrap($oid); }
    };
}
macro_rules! impl_cshake {
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident, $rate:ident,
        $shake_pad:expr, $cshake_pad:expr, $alg_name:expr,
    ) => {
        #[doc = "Core "] #[doc = $alg_name] #[doc = " hasher state."] #[derive(Clone)]
        #[allow(non_camel_case_types)] pub struct $name { padding : u8, state :
        Sha3State, #[cfg(feature = "reset")] initial_state : Sha3State, } impl $name {
        #[doc = " Creates a new CSHAKE instance with the given customization."] pub fn
        new(customization : & [u8]) -> Self { Self::new_with_function_name(& [],
        customization) } #[doc =
        " Creates a new CSHAKE instance with the given function name and customization."]
        #[doc =
        " Note that the function name is intended for use by NIST and should only be set to"]
        #[doc = " values defined by NIST. You probably don't need to use this function."]
        pub fn new_with_function_name(function_name : & [u8], customization : & [u8]) ->
        Self { let mut state = Sha3State::default(); if function_name.is_empty() &&
        customization.is_empty() { return Self { padding : $shake_pad, state : state
        .clone(), #[cfg(feature = "reset")] initial_state : state, }; } let mut buffer =
        Buffer::< Self >::default(); let mut b = [0u8; 9]; buffer
        .digest_blocks(left_encode($rate ::to_u64(), & mut b), | blocks | { for block in
        blocks { state.absorb_block(block); } }); buffer
        .digest_blocks(left_encode((function_name.len() * 8) as u64, & mut b), | blocks |
        { for block in blocks { state.absorb_block(block); } },); buffer
        .digest_blocks(function_name, | blocks | { for block in blocks { state
        .absorb_block(block); } }); buffer.digest_blocks(left_encode((customization.len()
        * 8) as u64, & mut b), | blocks | { for block in blocks { state
        .absorb_block(block); } },); buffer.digest_blocks(customization, | blocks | { for
        block in blocks { state.absorb_block(block); } }); state.absorb_block(buffer
        .pad_with_zeros()); Self { padding : $cshake_pad, state : state.clone(),
        #[cfg(feature = "reset")] initial_state : state, } } } impl HashMarker for $name
        {} impl BlockSizeUser for $name { type BlockSize = $rate; } impl BufferKindUser
        for $name { type BufferKind = Eager; } impl UpdateCore for $name { #[inline] fn
        update_blocks(& mut self, blocks : & [Block < Self >]) { for block in blocks {
        self.state.absorb_block(block) } } } impl ExtendableOutputCore for $name { type
        ReaderCore = $reader; #[inline] fn finalize_xof_core(& mut self, buffer : & mut
        Buffer < Self >) -> Self::ReaderCore { let pos = buffer.get_pos(); let block =
        buffer.pad_with_zeros(); block[pos] = self.padding; let n = block.len(); block[n
        - 1] |= 0x80; self.state.absorb_block(block); $reader { state : self.state
        .clone(), } } } #[cfg(feature = "reset")] impl Reset for $name { #[inline] fn
        reset(& mut self) { self.state = self.initial_state.clone(); } } impl
        AlgorithmName for $name { fn write_alg_name(f : & mut fmt::Formatter <'_ >) ->
        fmt::Result { f.write_str(stringify!($full_name)) } } impl fmt::Debug for $name {
        fn fmt(& self, f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(concat!(stringify!($name), " { ... }")) } } #[doc = "Core "] #[doc =
        $alg_name] #[doc = " reader state."] #[derive(Clone)]
        #[allow(non_camel_case_types)] pub struct $reader { state : Sha3State, } impl
        BlockSizeUser for $reader { type BlockSize = $rate; } impl XofReaderCore for
        $reader { #[inline] fn read_block(& mut self) -> Block < Self > { let mut block =
        Block::< Self >::default(); self.state.as_bytes(& mut block); self.state
        .permute(); block } } #[doc = $alg_name] #[doc = " hasher state."] pub type
        $full_name = CoreWrapper <$name >; #[doc = $alg_name] #[doc = " reader state."]
        pub type $reader_full = XofReaderCoreWrapper <$reader >;
    };
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    use digest::core_api::{BlockSizeUser, UpdateCore};
    use digest::generic_array::GenericArray;
    #[test]
    fn update_blocks_single() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2)) = <([u8; 0], usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let customization = rug_fuzz_0;
        let mut core = CShake256Core::new(customization);
        let block_size = <CShake256Core as BlockSizeUser>::BlockSize::to_usize();
        let mut block = vec![0u8; block_size];
        block[block_size - rug_fuzz_1] = rug_fuzz_2;
        let block_generic_array = GenericArray::clone_from_slice(&block);
        core.update_blocks(core::slice::from_ref(&block_generic_array));
        let dummy_state = Sha3State::default();
        let mut expected_state = dummy_state.clone();
        expected_state.absorb_block(&block);
        debug_assert_eq!(core.state.state, expected_state.state);
             }
}
}
}    }
    #[test]
    fn update_blocks_multiple() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <([u8; 0], i32, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let customization = rug_fuzz_0;
        let mut core = CShake256Core::new(customization);
        let block_size = <CShake256Core as BlockSizeUser>::BlockSize::to_usize();
        let blocks = (rug_fuzz_1..rug_fuzz_2)
            .map(|i| {
                let mut block = vec![0; block_size];
                block[block_size - rug_fuzz_3] = i as u8;
                GenericArray::clone_from_slice(&block)
            })
            .collect::<Vec<_>>();
        core.update_blocks(&blocks);
        let dummy_state = Sha3State::default();
        let mut expected_state = dummy_state.clone();
        for block in blocks.iter() {
            expected_state.absorb_block(block.as_slice());
        }
        debug_assert_eq!(core.state.state, expected_state.state);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_17 {
    use super::*;
    use crate::*;
    use crate::Keccak256Core;
    use digest::core_api::UpdateCore;
    use digest::generic_array::GenericArray;
    use digest::FixedOutput;
    use crate::state::Sha3State;
    #[test]
    fn keccak256core_update_blocks_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Keccak256Core::default();
        let block_size = <Keccak256Core as BlockSizeUser>::BlockSize::to_usize();
        let block = GenericArray::clone_from_slice(&[rug_fuzz_0; 136][..block_size]);
        let blocks = [block; 2];
        let initial_state = core.state.clone();
        core.update_blocks(&blocks);
        let updated_state = core.state;
        debug_assert_ne!(
            initial_state.state, updated_state.state,
            "State should change after processing blocks"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_23_llm_16_23 {
    use super::*;
    use crate::*;
    #[test]
    fn default_keccak256fullcore_has_correct_initial_state() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let keccak256fullcore: Keccak256FullCore = Default::default();
        let debug_representation = format!("{:?}", keccak256fullcore);
        debug_assert!(debug_representation.contains(rug_fuzz_0));
        let mut keccak256fullcore_after_reset = keccak256fullcore.clone();
        keccak256fullcore_after_reset.reset();
        debug_assert_eq!(
            format!("{:?}", keccak256fullcore), format!("{:?}",
            keccak256fullcore_after_reset)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_25_llm_16_25 {
    use super::*;
    use crate::*;
    use core::fmt::{self, Write};
    use digest::core_api::AlgorithmName;
    #[derive(Clone)]
    struct Dummy;
    impl fmt::Debug for Dummy {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write_alg_name(f)
        }
    }
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(stringify!(Keccak384Core))
    }
    #[test]
    fn test_write_alg_name() {
        let dummy = Dummy;
        let output = format!("{:?}", dummy);
        assert_eq!(output, "Keccak384Core");
    }
}
#[cfg(test)]
mod tests_llm_16_28_llm_16_28 {
    use super::*;
    use crate::*;
    #[test]
    fn default_initializes_to_zero_state() {
        let keccak384_core: Keccak384Core = Default::default();
        const PLEN: usize = 25;
        assert_eq!(keccak384_core.state.state, [0u64; PLEN]);
    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use super::*;
    use crate::*;
    use crate::Keccak512Core;
    use digest::generic_array::GenericArray;
    use digest::generic_array::typenum::Unsigned;
    use digest::core_api::{Block, BlockSizeUser, UpdateCore, BufferKindUser};
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_update_blocks = 0;
        let mut hasher = Keccak512Core::default();
        let block_size = <Keccak512Core as BlockSizeUser>::BlockSize::to_usize();
        let block = GenericArray::default();
        let block = Block::<Keccak512Core>::from(block);
        let mut blocks = Vec::new();
        blocks.push(block);
        let initial_state = hasher.clone().state;
        hasher.update_blocks(&blocks);
        let updated_state = hasher.state;
        debug_assert_ne!(
            initial_state.state, updated_state.state,
            "State should change after absorbing blocks"
        );
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_34 {
    use super::*;
    use crate::*;
    use crate::state::Sha3State;
    use crate::Sha3_224Core;
    use digest::Reset;
    #[test]
    fn sha3_224_reset_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, u64, usize, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hasher = Sha3_224Core::default();
        hasher.state.state[rug_fuzz_0] = rug_fuzz_1;
        hasher.state.state[rug_fuzz_2] = rug_fuzz_3;
        hasher.reset();
        let default_state = Sha3_224Core::default();
        debug_assert_eq!(hasher.state.state, default_state.state.state);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_40 {
    use super::*;
    use crate::*;
    use digest::core_api::AlgorithmName;
    use std::fmt::Write;
    use std::fmt;
    struct Sha3_256Core;
    #[test]
    fn write_alg_name_test() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_write_alg_name_test = 0;
        let mut buffer = String::new();
        let result = write!(& mut buffer, "{}", Sha3_256Core);
        debug_assert!(result.is_ok());
        debug_assert_eq!(buffer, "SHA3_256Core");
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_write_alg_name_test = 0;
    }
    impl fmt::Display for Sha3_256Core {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            <Self as AlgorithmName>::write_alg_name(f)
        }
    }
    impl AlgorithmName for Sha3_256Core {
        fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(stringify!(SHA3_256Core))
        }
    }
}
#[cfg(test)]
mod tests_llm_16_63 {
    use super::*;
    use crate::*;
    use digest::core_api::{Block, UpdateCore};
    use digest::Digest;
    #[test]
    fn test_update_blocks_empty() {
        let _rug_st_tests_llm_16_63_rrrruuuugggg_test_update_blocks_empty = 0;
        let mut shake256 = Shake256Core::default();
        let blocks: &[Block<Shake256Core>] = &[];
        shake256.update_blocks(blocks);
        let _rug_ed_tests_llm_16_63_rrrruuuugggg_test_update_blocks_empty = 0;
    }
    #[test]
    fn test_update_blocks_single() {
        let _rug_st_tests_llm_16_63_rrrruuuugggg_test_update_blocks_single = 0;
        let mut shake256 = Shake256Core::default();
        let block = Block::<Shake256Core>::default();
        let blocks: &[Block<Shake256Core>] = &[block];
        shake256.update_blocks(blocks);
        let _rug_ed_tests_llm_16_63_rrrruuuugggg_test_update_blocks_single = 0;
    }
    #[test]
    fn test_update_blocks_multiple() {
        let _rug_st_tests_llm_16_63_rrrruuuugggg_test_update_blocks_multiple = 0;
        let mut shake256 = Shake256Core::default();
        let block1 = Block::<Shake256Core>::default();
        let block2 = Block::<Shake256Core>::default();
        let blocks: &[Block<Shake256Core>] = &[block1, block2];
        shake256.update_blocks(blocks);
        let _rug_ed_tests_llm_16_63_rrrruuuugggg_test_update_blocks_multiple = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_74 {
    use crate::{TurboShake256Core, Block, Sha3State, UpdateCore};
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    use digest::typenum::Unsigned;
    #[test]
    fn test_update_blocks() {
        const TEST_DOMAIN: u8 = 0x1B;
        const TEST_BLOCK_SIZE: usize = <TurboShake256Core as BlockSizeUser>::BlockSize::USIZE;
        const TEST_ROUND_COUNT: usize = 24;
        let mut core = TurboShake256Core::new(TEST_DOMAIN);
        let block1 = GenericArray::clone_from_slice(&vec![0u8; TEST_BLOCK_SIZE]);
        let block2 = GenericArray::clone_from_slice(&vec![0u8; TEST_BLOCK_SIZE]);
        let blocks = vec![block1, block2];
        let mut initial_state = Sha3State::new(TEST_ROUND_COUNT);
        initial_state.absorb_block(blocks[0].as_slice());
        initial_state.absorb_block(blocks[1].as_slice());
        core.update_blocks(&blocks);
        let mut expected_state_bytes = vec![0u8; TEST_BLOCK_SIZE];
        let mut result_state_bytes = vec![0u8; TEST_BLOCK_SIZE];
        initial_state.as_bytes(&mut expected_state_bytes);
        core.state.as_bytes(&mut result_state_bytes);
        assert_eq!(
            result_state_bytes, expected_state_bytes,
            "Update blocks did not result in expected state."
        );
    }
}
#[cfg(test)]
mod tests_llm_16_75_llm_16_75 {
    use crate::TurboShake256ReaderCore;
    use digest::core_api::{Block, BlockSizeUser, XofReaderCore};
    use crate::state::Sha3State;
    use digest::typenum::Unsigned;
    #[test]
    fn read_block_test() {
        let _rug_st_tests_llm_16_75_llm_16_75_rrrruuuugggg_read_block_test = 0;
        let mut reader = TurboShake256ReaderCore {
            state: Sha3State::default(),
        };
        let initial_block = Block::<TurboShake256ReaderCore>::default();
        let mut test_block = Block::<TurboShake256ReaderCore>::default();
        reader.state.as_bytes(&mut test_block);
        debug_assert_eq!(initial_block, test_block, "Initial block should be all zeros");
        let block = reader.read_block();
        debug_assert_eq!(
            test_block, block, "Block read should match the initial state block"
        );
        let mut post_read_block = Block::<TurboShake256ReaderCore>::default();
        reader.state.as_bytes(&mut post_read_block);
        debug_assert_ne!(
            test_block, post_read_block, "State should change after reading the block"
        );
        let block_size = <TurboShake256ReaderCore as BlockSizeUser>::BlockSize::to_usize();
        debug_assert_eq!(
            block.len(), block_size,
            "Block size should be as defined by the BlockSizeUser"
        );
        let _rug_ed_tests_llm_16_75_llm_16_75_rrrruuuugggg_read_block_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_81 {
    use super::*;
    use crate::*;
    #[test]
    #[should_panic]
    fn new_with_invalid_domain_separation_below_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = TurboShake128Core::new(rug_fuzz_0);
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn new_with_invalid_domain_separation_above_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = TurboShake128Core::new(rug_fuzz_0);
             }
}
}
}    }
    #[test]
    fn new_with_valid_domain_separation() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for separation in rug_fuzz_0..=rug_fuzz_1 {
            let shake = TurboShake128Core::new(separation);
            debug_assert_eq!(shake.domain_separation, separation);
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_82 {
    use crate::TurboShake256Core;
    #[test]
    fn new_valid_domain_separation() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for domain_separation in rug_fuzz_0..=rug_fuzz_1 {
            let instance = TurboShake256Core::new(domain_separation);
            debug_assert_eq!(instance.domain_separation, domain_separation);
        }
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn new_invalid_domain_separation_low() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _instance = TurboShake256Core::new(rug_fuzz_0);
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn new_invalid_domain_separation_high() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _instance = TurboShake256Core::new(rug_fuzz_0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_220 {
    use crate::Keccak224Core;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_220_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Keccak224Core::default();
        let p1 = &[
            GenericArray::<u8, <Keccak224Core as BlockSizeUser>::BlockSize>::default(),
            GenericArray::<u8, <Keccak224Core as BlockSizeUser>::BlockSize>::default(),
        ];
        <Keccak224Core as UpdateCore>::update_blocks(&mut p0, p1);
        let _rug_ed_tests_rug_220_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_221 {
    use crate::{Keccak224Core, Buffer, Output};
    use digest::core_api::{FixedOutputCore, BufferKindUser, BlockSizeUser};
    use digest::block_buffer::BlockBuffer;
    use digest::generic_array::GenericArray;
    use digest::OutputSizeUser;
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_221_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Keccak224Core::default();
        let mut p1 = BlockBuffer::<
            <Keccak224Core as BlockSizeUser>::BlockSize,
            <Keccak224Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Keccak224Core as OutputSizeUser>::OutputSize,
        >::default();
        Keccak224Core::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_221_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_223 {
    use super::*;
    use crate::digest::Reset;
    use crate::Keccak224Core;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_223_rrrruuuugggg_test_reset = 0;
        let mut p0: Keccak224Core = Keccak224Core::default();
        <Keccak224Core as digest::Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_223_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_225 {
    use super::*;
    use crate::Keccak256Core;
    use digest::block_buffer::BlockBuffer;
    use digest::core_api::{FixedOutputCore, BufferKindUser, BlockSizeUser};
    use digest::generic_array::GenericArray;
    use digest::OutputSizeUser;
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_225_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Keccak256Core::default();
        let mut p1 = BlockBuffer::<
            <Keccak256Core as BlockSizeUser>::BlockSize,
            <Keccak256Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Keccak256Core as OutputSizeUser>::OutputSize,
        >::default();
        <Keccak256Core as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_225_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_227 {
    use crate::Keccak256Core;
    use crate::digest::Reset;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_227_rrrruuuugggg_test_reset = 0;
        let mut p0 = Keccak256Core::default();
        Keccak256Core::reset(&mut p0);
        let _rug_ed_tests_rug_227_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_229 {
    use super::*;
    use crate::Keccak384Core;
    use crate::digest::core_api::{BlockSizeUser, UpdateCore};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_229_rrrruuuugggg_test_rug = 0;
        let mut p0 = Keccak384Core::default();
        let mut block = GenericArray::<
            u8,
            <Keccak384Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1 = &[block];
        <Keccak384Core as UpdateCore>::update_blocks(&mut p0, p1);
        let _rug_ed_tests_rug_229_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_230 {
    use super::*;
    use crate::digest::core_api::{FixedOutputCore, BlockSizeUser, BufferKindUser};
    use crate::digest::block_buffer::BlockBuffer;
    use crate::Keccak384Core;
    use digest::OutputSizeUser;
    use digest::generic_array::{GenericArray, typenum::U48};
    #[test]
    fn test_finalize_fixed_core() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Keccak384Core = Keccak384Core::default();
        let mut p1: BlockBuffer<
            <Keccak384Core as BlockSizeUser>::BlockSize,
            <Keccak384Core as BufferKindUser>::BufferKind,
        > = BlockBuffer::default();
        let mut p2: GenericArray<u8, <Keccak384Core as OutputSizeUser>::OutputSize> = GenericArray::<
            u8,
            U48,
        >::default();
        <Keccak384Core as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        debug_assert!(
            ! p2.iter().all(| & x | x == rug_fuzz_0), "Output buffer is all zeros"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_231 {
    use super::*;
    use crate::Keccak384Core;
    use digest::Reset;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_231_rrrruuuugggg_test_reset = 0;
        let mut p0 = Keccak384Core::default();
        <Keccak384Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_231_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_232 {
    use crate::Keccak512Core;
    use digest::block_buffer::BlockBuffer;
    use digest::core_api::{
        BlockSizeUser, BufferKindUser, FixedOutputCore, OutputSizeUser,
    };
    use digest::generic_array::GenericArray;
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_232_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Keccak512Core::default();
        let mut p1 = BlockBuffer::<
            <Keccak512Core as BlockSizeUser>::BlockSize,
            <Keccak512Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Keccak512Core as OutputSizeUser>::OutputSize,
        >::default();
        <Keccak512Core as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_232_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_234 {
    use crate::Keccak512Core;
    use digest::Reset;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_234_rrrruuuugggg_test_reset = 0;
        let mut p0 = Keccak512Core::default();
        <Keccak512Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_234_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_236 {
    use crate::{Keccak256FullCore, Block};
    use digest::generic_array::GenericArray;
    use digest::core_api::{UpdateCore, BlockSizeUser};
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_236_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Keccak256FullCore::default();
        let mut block = GenericArray::<
            u8,
            <Keccak256FullCore as BlockSizeUser>::BlockSize,
        >::default();
        let p1 = &[block];
        <Keccak256FullCore as UpdateCore>::update_blocks(&mut p0, p1);
        let _rug_ed_tests_rug_236_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_237 {
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::core_api::{BlockSizeUser, BufferKindUser, FixedOutputCore};
    use crate::digest::generic_array::GenericArray;
    use crate::digest::OutputSizeUser;
    use crate::{Buffer, Keccak256FullCore, Output};
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_237_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Keccak256FullCore::default();
        let mut p1 = BlockBuffer::<
            <Keccak256FullCore as BlockSizeUser>::BlockSize,
            <Keccak256FullCore as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Keccak256FullCore as OutputSizeUser>::OutputSize,
        >::default();
        <Keccak256FullCore as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_237_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_238 {
    use crate::{Keccak256FullCore, digest::Reset};
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_238_rrrruuuugggg_test_reset = 0;
        let mut p0 = Keccak256FullCore::default();
        <Keccak256FullCore as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_238_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_240 {
    use crate::Sha3_224Core;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_240_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Sha3_224Core::default();
        let mut block = GenericArray::<
            u8,
            <Sha3_224Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1 = &mut [block];
        <Sha3_224Core as UpdateCore>::update_blocks(&mut p0, p1);
        let _rug_ed_tests_rug_240_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_241 {
    use crate::{Sha3_224, Sha3_224Core};
    use digest::{
        Digest, OutputSizeUser,
        core_api::{FixedOutputCore, BlockSizeUser, BufferKindUser},
    };
    use digest::block_buffer::BlockBuffer;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_241_rrrruuuugggg_test_rug = 0;
        let mut p0: Sha3_224Core = Sha3_224Core::default();
        let mut p1: BlockBuffer<
            <Sha3_224Core as BlockSizeUser>::BlockSize,
            <Sha3_224Core as BufferKindUser>::BufferKind,
        > = BlockBuffer::default();
        let mut p2: GenericArray<u8, <Sha3_224 as OutputSizeUser>::OutputSize> = GenericArray::<
            u8,
            <Sha3_224 as OutputSizeUser>::OutputSize,
        >::default();
        <Sha3_224Core as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_241_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_244 {
    use crate::Sha3_256Core;
    use digest::core_api::{BlockSizeUser, UpdateCore};
    use digest::generic_array::GenericArray;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_244_rrrruuuugggg_test_rug = 0;
        let mut p0 = Sha3_256Core::default();
        let mut block = GenericArray::<
            u8,
            <Sha3_256Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1 = &[block];
        <Sha3_256Core as UpdateCore>::update_blocks(&mut p0, p1);
        let _rug_ed_tests_rug_244_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_245 {
    use super::*;
    use crate::Sha3_256Core;
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::core_api::{BlockSizeUser, BufferKindUser, FixedOutputCore};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_245_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Sha3_256Core::default();
        let mut p1 = BlockBuffer::<
            <Sha3_256Core as BlockSizeUser>::BlockSize,
            <Sha3_256Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Sha3_256Core as digest::core_api::OutputSizeUser>::OutputSize,
        >::default();
        <Sha3_256Core as digest::core_api::FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_245_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_247 {
    use super::*;
    use crate::digest::Reset;
    use crate::Sha3_256Core;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_247_rrrruuuugggg_test_rug = 0;
        let mut p0 = Sha3_256Core::default();
        <Sha3_256Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_247_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_248 {
    use crate::Sha3_384Core;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_248_rrrruuuugggg_test_rug = 0;
        let mut p0: Sha3_384Core = Sha3_384Core::default();
        let mut p1: Vec<GenericArray<u8, <Sha3_384Core as BlockSizeUser>::BlockSize>> = Vec::new();
        p0.update_blocks(&p1);
        let _rug_ed_tests_rug_248_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_249 {
    use super::*;
    use crate::Sha3_384Core;
    use digest::core_api::{FixedOutputCore, BufferKindUser, BlockSizeUser};
    use digest::block_buffer::BlockBuffer;
    use digest::generic_array::GenericArray;
    use digest::OutputSizeUser;
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_249_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Sha3_384Core::default();
        let mut p1 = BlockBuffer::<
            _,
            <Sha3_384Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Sha3_384Core as OutputSizeUser>::OutputSize,
        >::default();
        <Sha3_384Core as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_249_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_251 {
    use crate::digest::Reset;
    use crate::Sha3_384Core;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_251_rrrruuuugggg_test_rug = 0;
        let mut p0: Sha3_384Core = Sha3_384Core::default();
        <Sha3_384Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_251_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_253 {
    use crate::Sha3_512Core;
    use crate::digest::core_api::{BlockSizeUser, UpdateCore};
    use crate::digest::generic_array::{GenericArray, typenum::U72};
    use crate::digest::core_api::Block;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_253_rrrruuuugggg_test_rug = 0;
        let mut p0: Sha3_512Core = Sha3_512Core::default();
        let p1: &[Block<Sha3_512Core>] = &[GenericArray::<u8, U72>::default()];
        <Sha3_512Core as UpdateCore>::update_blocks(&mut p0, p1);
        let _rug_ed_tests_rug_253_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_254 {
    use super::*;
    use crate::Sha3_512Core;
    use crate::digest::{
        self, core_api::{FixedOutputCore, BufferKindUser, BlockSizeUser},
        block_buffer::BlockBuffer, generic_array::GenericArray, OutputSizeUser,
    };
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_254_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Sha3_512Core::default();
        let mut p1 = BlockBuffer::<
            <Sha3_512Core as BlockSizeUser>::BlockSize,
            <Sha3_512Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Sha3_512Core as OutputSizeUser>::OutputSize,
        >::default();
        <Sha3_512Core as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_254_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_256 {
    use super::*;
    use crate::Sha3_512Core;
    use crate::digest::Reset;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_256_rrrruuuugggg_test_reset = 0;
        let mut p0 = Sha3_512Core::default();
        <Sha3_512Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_256_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_258 {
    use crate::Shake128Core;
    use digest::core_api::{UpdateCore, BlockSizeUser};
    use digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_258_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Shake128Core::default();
        let single_block = GenericArray::<
            u8,
            <Shake128Core as BlockSizeUser>::BlockSize,
        >::default();
        let p1 = [single_block];
        <Shake128Core as UpdateCore>::update_blocks(&mut p0, &p1);
        let _rug_ed_tests_rug_258_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_259 {
    use super::*;
    use crate::{
        Shake128Core, digest::core_api::ExtendableOutputCore,
        digest::block_buffer::BlockBuffer,
    };
    use digest::core_api::{BlockSizeUser, BufferKindUser};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_259_rrrruuuugggg_test_rug = 0;
        let mut p0 = Shake128Core::default();
        let mut p1 = BlockBuffer::<
            <Shake128Core as BlockSizeUser>::BlockSize,
            <Shake128Core as BufferKindUser>::BufferKind,
        >::default();
        <Shake128Core as ExtendableOutputCore>::finalize_xof_core(&mut p0, &mut p1);
        let _rug_ed_tests_rug_259_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_261 {
    use super::*;
    use crate::Shake128Core;
    use crate::digest::Reset;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_261_rrrruuuugggg_test_reset = 0;
        let mut p0 = Shake128Core::default();
        <Shake128Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_261_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_264 {
    use super::*;
    use crate::Shake256Core;
    use digest::core_api::{BufferKindUser, BlockSizeUser, ExtendableOutputCore};
    use digest::block_buffer::BlockBuffer;
    #[test]
    fn test_finalize_xof_core() {
        let _rug_st_tests_rug_264_rrrruuuugggg_test_finalize_xof_core = 0;
        let mut p0: Shake256Core = Shake256Core::default();
        let mut p1: BlockBuffer<
            <Shake256Core as BlockSizeUser>::BlockSize,
            <Shake256Core as BufferKindUser>::BufferKind,
        > = BlockBuffer::default();
        <Shake256Core as ExtendableOutputCore>::finalize_xof_core(&mut p0, &mut p1);
        let _rug_ed_tests_rug_264_rrrruuuugggg_test_finalize_xof_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_266 {
    use super::*;
    use crate::digest::Reset;
    use crate::Shake256Core;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_266_rrrruuuugggg_test_rug = 0;
        let mut p0 = Shake256Core::default();
        <Shake256Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_266_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_269 {
    use crate::digest::core_api::{BlockSizeUser, UpdateCore};
    use crate::digest::generic_array::GenericArray;
    use crate::{Block, TurboShake128Core};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = TurboShake128Core::new(rug_fuzz_0);
        let mut p1 = [
            GenericArray::<
                u8,
                <TurboShake128Core as BlockSizeUser>::BlockSize,
            >::default(),
        ];
        <TurboShake128Core as UpdateCore>::update_blocks(&mut p0, &p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_270 {
    use super::*;
    use crate::digest::core_api::ExtendableOutputCore;
    use crate::digest::block_buffer::BlockBuffer;
    use crate::TurboShake128Core;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = TurboShake128Core::new(rug_fuzz_0);
        let mut p1 = BlockBuffer::<
            <TurboShake128Core as digest::core_api::BlockSizeUser>::BlockSize,
            <TurboShake128Core as digest::core_api::BufferKindUser>::BufferKind,
        >::default();
        <TurboShake128Core>::finalize_xof_core(&mut p0, &mut p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_271 {
    use crate::TurboShake128Core;
    use crate::digest::Reset;
    #[test]
    fn test_reset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = TurboShake128Core::new(rug_fuzz_0);
        <TurboShake128Core as Reset>::reset(&mut p0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_274 {
    use crate::{TurboShake256Core, digest};
    use digest::core_api::{ExtendableOutputCore, BlockSizeUser, BufferKindUser};
    use digest::block_buffer::BlockBuffer;
    #[test]
    fn test_finalize_xof_core() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = TurboShake256Core::new(rug_fuzz_0);
        let mut p1 = BlockBuffer::<
            <TurboShake256Core as BlockSizeUser>::BlockSize,
            <TurboShake256Core as BufferKindUser>::BufferKind,
        >::default();
        p0.finalize_xof_core(&mut p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_275 {
    use crate::TurboShake256Core;
    use crate::digest::Reset;
    #[test]
    fn test_reset() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = TurboShake256Core::new(rug_fuzz_0);
        <TurboShake256Core as Reset>::reset(&mut p0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_277 {
    use super::*;
    #[test]
    fn test_cshake128core_new() {
        let _rug_st_tests_rug_277_rrrruuuugggg_test_cshake128core_new = 0;
        let rug_fuzz_0 = b"My Customization";
        let customization: &[u8] = rug_fuzz_0;
        let p0 = customization;
        crate::CShake128Core::new(p0);
        let _rug_ed_tests_rug_277_rrrruuuugggg_test_cshake128core_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_278 {
    use super::*;
    #[test]
    fn test_new_with_function_name() {
        let _rug_st_tests_rug_278_rrrruuuugggg_test_new_with_function_name = 0;
        let rug_fuzz_0 = b"NIST Function";
        let rug_fuzz_1 = b"Customization";
        let mut p0: &[u8] = rug_fuzz_0;
        let mut p1: &[u8] = rug_fuzz_1;
        crate::CShake128Core::new_with_function_name(p0, p1);
        let _rug_ed_tests_rug_278_rrrruuuugggg_test_new_with_function_name = 0;
    }
}
#[cfg(test)]
mod tests_rug_279 {
    use crate::CShake128Core;
    use crate::digest;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_279_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = CShake128Core::new(&[]);
        let mut p1: [GenericArray<u8, <CShake128Core as BlockSizeUser>::BlockSize>; 1] = [
            GenericArray::default(),
        ];
        <CShake128Core as UpdateCore>::update_blocks(&mut p0, &p1);
        let _rug_ed_tests_rug_279_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_280 {
    use crate::CShake128Core;
    use crate::digest::core_api::ExtendableOutputCore;
    use crate::digest::core_api::{BlockSizeUser, BufferKindUser};
    use crate::digest::block_buffer::BlockBuffer;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_280_rrrruuuugggg_test_rug = 0;
        let mut p0 = CShake128Core::new(&[]);
        let mut p1 = BlockBuffer::<
            <CShake128Core as BlockSizeUser>::BlockSize,
            <CShake128Core as BufferKindUser>::BufferKind,
        >::default();
        p0.finalize_xof_core(&mut p1);
        let _rug_ed_tests_rug_280_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_283 {
    use super::*;
    #[test]
    fn test_cshake256core_new() {
        let _rug_st_tests_rug_283_rrrruuuugggg_test_cshake256core_new = 0;
        let rug_fuzz_0 = b"some customization";
        let mut p0: &[u8] = rug_fuzz_0;
        crate::CShake256Core::new(p0);
        let _rug_ed_tests_rug_283_rrrruuuugggg_test_cshake256core_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_284 {
    use super::*;
    #[test]
    fn test_new_with_function_name() {
        let _rug_st_tests_rug_284_rrrruuuugggg_test_new_with_function_name = 0;
        let rug_fuzz_0 = b"N";
        let rug_fuzz_1 = b"Customization";
        let p0: &[u8] = rug_fuzz_0;
        let p1: &[u8] = rug_fuzz_1;
        crate::CShake256Core::new_with_function_name(p0, p1);
        let _rug_ed_tests_rug_284_rrrruuuugggg_test_new_with_function_name = 0;
    }
}
#[cfg(test)]
mod tests_rug_285 {
    use crate::{
        CShake256Core,
        digest::core_api::{BlockSizeUser, BufferKindUser, ExtendableOutputCore},
    };
    use digest::block_buffer::BlockBuffer;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_285_rrrruuuugggg_test_rug = 0;
        let mut p0 = CShake256Core::new(&[]);
        let mut p1: BlockBuffer<
            <CShake256Core as BlockSizeUser>::BlockSize,
            <CShake256Core as BufferKindUser>::BufferKind,
        > = BlockBuffer::default();
        <CShake256Core>::finalize_xof_core(&mut p0, &mut p1);
        let _rug_ed_tests_rug_285_rrrruuuugggg_test_rug = 0;
    }
}
