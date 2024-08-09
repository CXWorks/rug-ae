macro_rules! blake2_impl {
    (
        $name:ident, $alg_name:expr, $word:ident, $vec:ident, $bytes:ident,
        $block_size:ident, $R1:expr, $R2:expr, $R3:expr, $R4:expr, $IV:expr,
        $vardoc:expr, $doc:expr,
    ) => {
        #[derive(Clone)] #[doc =$vardoc] pub struct $name { h : [$vec; 2], t : u64,
        #[cfg(feature = "reset")] h0 : [$vec; 2], } impl $name { #[inline(always)] fn
        iv0() -> $vec { $vec ::new($IV [0], $IV [1], $IV [2], $IV [3]) }
        #[inline(always)] fn iv1() -> $vec { $vec ::new($IV [4], $IV [5], $IV [6], $IV
        [7]) } #[doc =
        " Creates a new context with the full set of sequential-mode parameters."] pub fn
        new_with_params(salt : & [u8], persona : & [u8], key_size : usize, output_size :
        usize,) -> Self { assert!(key_size <= $bytes ::to_usize()); assert!(output_size
        <= $bytes ::to_usize()); let length = $bytes ::to_usize() / 4; assert!(salt.len()
        <= length); assert!(persona.len() <= length); let mut p = [0 as $word; 8]; p[0] =
        0x0101_0000 ^ ((key_size as $word) << 8) ^ (output_size as $word); if salt.len()
        < length { let mut padded_salt = GenericArray::< u8, <$bytes as Div < U4
        >>::Output >::default(); for i in 0..salt.len() { padded_salt[i] = salt[i]; }
        p[4] = $word ::from_le_bytes(padded_salt[0..length / 2].try_into().unwrap());
        p[5] = $word ::from_le_bytes(padded_salt[length / 2..padded_salt.len()]
        .try_into().unwrap(),); } else { p[4] = $word ::from_le_bytes(salt[0..salt.len()
        / 2].try_into().unwrap()); p[5] = $word ::from_le_bytes(salt[salt.len() / 2..salt
        .len()].try_into().unwrap()); } if persona.len() < length { let mut
        padded_persona = GenericArray::< u8, <$bytes as Div < U4 >>::Output >::default();
        for i in 0..persona.len() { padded_persona[i] = persona[i]; } p[6] = $word
        ::from_le_bytes(padded_persona[0..length / 2].try_into().unwrap()); p[7] = $word
        ::from_le_bytes(padded_persona[length / 2..padded_persona.len()].try_into()
        .unwrap(),); } else { p[6] = $word ::from_le_bytes(persona[0..length / 2]
        .try_into().unwrap()); p[7] = $word ::from_le_bytes(persona[length / 2..persona
        .len()].try_into().unwrap(),); } let h = [Self::iv0() ^ $vec ::new(p[0], p[1],
        p[2], p[3]), Self::iv1() ^ $vec ::new(p[4], p[5], p[6], p[7]),]; $name {
        #[cfg(feature = "reset")] h0 : h.clone(), h, t : 0, } } fn finalize_with_flag(&
        mut self, final_block : & GenericArray < u8, $block_size >, flag : $word, out : &
        mut Output < Self >,) { self.compress(final_block, ! 0, flag); let buf = [self
        .h[0].to_le(), self.h[1].to_le()]; out.copy_from_slice(buf.as_bytes()) } fn
        compress(& mut self, block : & Block < Self >, f0 : $word, f1 : $word) { use
        $crate ::consts::SIGMA; #[cfg_attr(not(feature = "size_opt"), inline(always))] fn
        quarter_round(v : & mut [$vec; 4], rd : u32, rb : u32, m : $vec) { v[0] = v[0]
        .wrapping_add(v[1]).wrapping_add(m.from_le()); v[3] = (v[3] ^ v[0])
        .rotate_right_const(rd); v[2] = v[2].wrapping_add(v[3]); v[1] = (v[1] ^ v[2])
        .rotate_right_const(rb); } #[cfg_attr(not(feature = "size_opt"), inline(always))]
        fn shuffle(v : & mut [$vec; 4]) { v[1] = v[1].shuffle_left_1(); v[2] = v[2]
        .shuffle_left_2(); v[3] = v[3].shuffle_left_3(); } #[cfg_attr(not(feature =
        "size_opt"), inline(always))] fn unshuffle(v : & mut [$vec; 4]) { v[1] = v[1]
        .shuffle_right_1(); v[2] = v[2].shuffle_right_2(); v[3] = v[3].shuffle_right_3();
        } #[cfg_attr(not(feature = "size_opt"), inline(always))] fn round(v : & mut
        [$vec; 4], m : & [$word; 16], s : & [usize; 16]) { quarter_round(v, $R1, $R2,
        $vec ::gather(m, s[0], s[2], s[4], s[6])); quarter_round(v, $R3, $R4, $vec
        ::gather(m, s[1], s[3], s[5], s[7])); shuffle(v); quarter_round(v, $R1, $R2, $vec
        ::gather(m, s[8], s[10], s[12], s[14])); quarter_round(v, $R3, $R4, $vec
        ::gather(m, s[9], s[11], s[13], s[15])); unshuffle(v); } let mut m : [$word; 16]
        = Default::default(); let n = core::mem::size_of::<$word > (); for (v, chunk) in
        m.iter_mut().zip(block.chunks_exact(n)) { * v = $word ::from_ne_bytes(chunk
        .try_into().unwrap()); } let h = & mut self.h; let t0 = self.t as $word; let t1 =
        match $bytes ::to_u8() { 64 => 0, 32 => (self.t >> 32) as $word, _ =>
        unreachable!(), }; let mut v = [h[0], h[1], Self::iv0(), Self::iv1() ^ $vec
        ::new(t0, t1, f0, f1),]; round(& mut v, & m, & SIGMA[0]); round(& mut v, & m, &
        SIGMA[1]); round(& mut v, & m, & SIGMA[2]); round(& mut v, & m, & SIGMA[3]);
        round(& mut v, & m, & SIGMA[4]); round(& mut v, & m, & SIGMA[5]); round(& mut v,
        & m, & SIGMA[6]); round(& mut v, & m, & SIGMA[7]); round(& mut v, & m, &
        SIGMA[8]); round(& mut v, & m, & SIGMA[9]); if $bytes ::to_u8() == 64 { round(&
        mut v, & m, & SIGMA[0]); round(& mut v, & m, & SIGMA[1]); } h[0] = h[0] ^ (v[0] ^
        v[2]); h[1] = h[1] ^ (v[1] ^ v[3]); } } impl HashMarker for $name {} impl
        BlockSizeUser for $name { type BlockSize = $block_size; } impl BufferKindUser for
        $name { type BufferKind = Lazy; } impl UpdateCore for $name { #[inline] fn
        update_blocks(& mut self, blocks : & [Block < Self >]) { for block in blocks {
        self.t += block.len() as u64; self.compress(block, 0, 0); } } } impl
        OutputSizeUser for $name { type OutputSize = $bytes; } impl VariableOutputCore
        for $name { const TRUNC_SIDE : TruncSide = TruncSide::Left; #[inline] fn
        new(output_size : usize) -> Result < Self, InvalidOutputSize > { if output_size >
        Self::OutputSize::USIZE { return Err(InvalidOutputSize); }
        Ok(Self::new_with_params(& [], & [], 0, output_size)) } #[inline] fn
        finalize_variable_core(& mut self, buffer : & mut Buffer < Self >, out : & mut
        Output < Self >,) { self.t += buffer.get_pos() as u64; let block = buffer
        .pad_with_zeros(); self.finalize_with_flag(block, 0, out); } } #[cfg(feature =
        "reset")] impl Reset for $name { fn reset(& mut self) { self.h = self.h0; self.t
        = 0; } } impl AlgorithmName for $name { #[inline] fn write_alg_name(f : & mut
        fmt::Formatter <'_ >) -> fmt::Result { f.write_str($alg_name) } } impl fmt::Debug
        for $name { fn fmt(& self, f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(concat!(stringify!($name), " { ... }")) } }
    };
}
macro_rules! blake2_mac_impl {
    ($name:ident, $hash:ty, $max_size:ty, $doc:expr) => {
        #[derive(Clone)] #[doc =$doc] pub struct $name < OutSize > where OutSize :
        ArrayLength < u8 > + IsLessOrEqual <$max_size >, LeEq < OutSize, $max_size >:
        NonZero, { core : $hash, buffer : LazyBuffer <<$hash as BlockSizeUser
        >::BlockSize >, #[cfg(feature = "reset")] key_block : Key < Self >, _out :
        PhantomData < OutSize >, } impl < OutSize > $name < OutSize > where OutSize :
        ArrayLength < u8 > + IsLessOrEqual <$max_size >, LeEq < OutSize, $max_size >:
        NonZero, { #[doc = " Create new instance using provided key, salt, and persona."]
        #[doc = ""] #[doc =
        " Key length should not be bigger than block size, salt and persona"] #[doc =
        " length should not be bigger than quarter of block size. If any"] #[doc =
        " of those conditions is false the method will return an error."] #[inline] pub
        fn new_with_salt_and_personal(key : & [u8], salt : & [u8], persona : & [u8],) ->
        Result < Self, InvalidLength > { let kl = key.len(); let bs = <$hash as
        BlockSizeUser >::BlockSize::USIZE; let qbs = bs / 4; if kl > bs || salt.len() >
        qbs || persona.len() > qbs { return Err(InvalidLength); } let mut padded_key =
        Block::<$hash >::default(); padded_key[..kl].copy_from_slice(key); Ok(Self { core
        : <$hash >::new_with_params(salt, persona, key.len(), OutSize::USIZE), buffer :
        LazyBuffer::new(& padded_key), #[cfg(feature = "reset")] key_block : { let mut t
        = Key::< Self >::default(); t[..kl].copy_from_slice(key); t }, _out :
        PhantomData, }) } } impl < OutSize > KeySizeUser for $name < OutSize > where
        OutSize : ArrayLength < u8 > + IsLessOrEqual <$max_size >, LeEq < OutSize,
        $max_size >: NonZero, { type KeySize = $max_size; } impl < OutSize > KeyInit for
        $name < OutSize > where OutSize : ArrayLength < u8 > + IsLessOrEqual <$max_size
        >, LeEq < OutSize, $max_size >: NonZero, { #[inline] fn new(key : & Key < Self >)
        -> Self { Self::new_from_slice(key).expect("Key has correct length") } #[inline]
        fn new_from_slice(key : & [u8]) -> Result < Self, InvalidLength > { let kl = key
        .len(); if kl > < Self as KeySizeUser >::KeySize::USIZE { return
        Err(InvalidLength); } let mut padded_key = Block::<$hash >::default();
        padded_key[..kl].copy_from_slice(key); Ok(Self { core : <$hash
        >::new_with_params(& [], & [], key.len(), OutSize::USIZE), buffer :
        LazyBuffer::new(& padded_key), #[cfg(feature = "reset")] key_block : { let mut t
        = Key::< Self >::default(); t[..kl].copy_from_slice(key); t }, _out :
        PhantomData, }) } } impl < OutSize > Update for $name < OutSize > where OutSize :
        ArrayLength < u8 > + IsLessOrEqual <$max_size >, LeEq < OutSize, $max_size >:
        NonZero, { #[inline] fn update(& mut self, input : & [u8]) { let Self { core,
        buffer, .. } = self; buffer.digest_blocks(input, | blocks | core
        .update_blocks(blocks)); } } impl < OutSize > OutputSizeUser for $name < OutSize
        > where OutSize : ArrayLength < u8 > + IsLessOrEqual <$max_size > + 'static, LeEq
        < OutSize, $max_size >: NonZero, { type OutputSize = OutSize; } impl < OutSize >
        FixedOutput for $name < OutSize > where OutSize : ArrayLength < u8 > +
        IsLessOrEqual <$max_size > + 'static, LeEq < OutSize, $max_size >: NonZero, {
        #[inline] fn finalize_into(mut self, out : & mut Output < Self >) { let Self {
        core, buffer, .. } = & mut self; let mut full_res = Default::default(); core
        .finalize_variable_core(buffer, & mut full_res); out.copy_from_slice(&
        full_res[..OutSize::USIZE]); } } #[cfg(feature = "reset")] impl < OutSize > Reset
        for $name < OutSize > where OutSize : ArrayLength < u8 > + IsLessOrEqual
        <$max_size >, LeEq < OutSize, $max_size >: NonZero, { fn reset(& mut self) { self
        .core.reset(); let kl = self.key_block.len(); let mut padded_key = Block::<$hash
        >::default(); padded_key[..kl].copy_from_slice(& self.key_block); self.buffer =
        LazyBuffer::new(& padded_key); } } #[cfg(feature = "reset")] impl < OutSize >
        FixedOutputReset for $name < OutSize > where OutSize : ArrayLength < u8 > +
        IsLessOrEqual <$max_size >, LeEq < OutSize, $max_size >: NonZero, { #[inline] fn
        finalize_into_reset(& mut self, out : & mut Output < Self >) { let Self { core,
        buffer, .. } = self; let mut full_res = Default::default(); core
        .finalize_variable_core(buffer, & mut full_res); out.copy_from_slice(&
        full_res[..OutSize::USIZE]); self.reset(); } } impl < OutSize > MacMarker for
        $name < OutSize > where OutSize : ArrayLength < u8 > + IsLessOrEqual <$max_size
        >, LeEq < OutSize, $max_size >: NonZero, {} impl < OutSize > fmt::Debug for $name
        < OutSize > where OutSize : ArrayLength < u8 > + IsLessOrEqual <$max_size >, LeEq
        < OutSize, $max_size >: NonZero, { fn fmt(& self, f : & mut fmt::Formatter <'_ >)
        -> fmt::Result { write!(f, "{}{} {{ ... }}", stringify!($name), OutSize::USIZE) }
        }
    };
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    #[test]
    fn test_iv1() {
        const IV: [u64; 8] = [
            0x6a09e667f3bcc908,
            0xbb67ae8584caa73b,
            0x3c6ef372fe94f82b,
            0xa54ff53a5f1d36f1,
            0x510e527fade682d1,
            0x9b05688c2b3e6c1f,
            0x1f83d9abfb41bd6b,
            0x5be0cd19137e2179,
        ];
        struct Blake2bVarCore;
        impl Blake2bVarCore {
            #[inline(always)]
            fn iv1() -> [u64; 4] {
                [IV[4], IV[5], IV[6], IV[7]]
            }
        }
        let expected: [u64; 4] = [IV[4], IV[5], IV[6], IV[7]];
        assert_eq!(Blake2bVarCore::iv1(), expected);
    }
}
#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use crate::Blake2bVarCore;
    #[test]
    fn test_new_with_params() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u8, u8, u8, u8, u8, u8, u8, u8, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let p1: &[u8] = &[rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let p2: usize = rug_fuzz_8;
        let p3: usize = rug_fuzz_9;
        Blake2bVarCore::new_with_params(p0, p1, p2, p3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_14 {
    use crate::{Blake2bVarCore, Block};
    use digest::{core_api::BlockSizeUser, generic_array::GenericArray};
    #[test]
    fn test_compress() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, usize, usize, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let salt = [rug_fuzz_0; 16];
        let personal = [rug_fuzz_1; 16];
        let key_size = rug_fuzz_2;
        let output_size = rug_fuzz_3;
        let mut p0 = Blake2bVarCore::new_with_params(
            &salt,
            &personal,
            key_size,
            output_size,
        );
        let mut p1: Block<Blake2bVarCore> = GenericArray::<
            u8,
            <Blake2bVarCore as BlockSizeUser>::BlockSize,
        >::default();
        let p2: u64 = rug_fuzz_4;
        let p3: u64 = rug_fuzz_5;
        Blake2bVarCore::compress(&mut p0, &p1, p2, p3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_15 {
    use crate::{Blake2bVarCore, Block};
    use digest::generic_array::GenericArray;
    use digest::core_api::{BlockSizeUser, UpdateCore};
    #[test]
    fn test_update_blocks() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, usize, usize, usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let salt = [rug_fuzz_0; 16];
        let persona = [rug_fuzz_1; 16];
        let key_size = rug_fuzz_2;
        let output_size = rug_fuzz_3;
        let mut p0 = Blake2bVarCore::new_with_params(
            &salt,
            &persona,
            key_size,
            output_size,
        );
        let mut block = GenericArray::<
            u8,
            <Blake2bVarCore as BlockSizeUser>::BlockSize,
        >::default();
        block[rug_fuzz_4] = rug_fuzz_5;
        let p1: &[Block<Blake2bVarCore>] = core::slice::from_ref(&block);
        <Blake2bVarCore as UpdateCore>::update_blocks(&mut p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::digest::core_api::VariableOutputCore;
    use crate::Blake2bVarCore;
    use crate::digest::InvalidOutputSize;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        debug_assert!(< Blake2bVarCore > ::new(p0).is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    use crate::Blake2bVarCore;
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::core_api::{
        BlockSizeUser, BufferKindUser, VariableOutputCore, OutputSizeUser,
    };
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_finalize_variable_core() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let salt = [rug_fuzz_0; 16];
        let persona = [rug_fuzz_1; 16];
        let key_size = rug_fuzz_2;
        let output_size = rug_fuzz_3;
        let mut p0 = Blake2bVarCore::new_with_params(
            &salt,
            &persona,
            key_size,
            output_size,
        );
        let mut p1 = BlockBuffer::<
            <Blake2bVarCore as BlockSizeUser>::BlockSize,
            <Blake2bVarCore as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Blake2bVarCore as OutputSizeUser>::OutputSize,
        >::default();
        <Blake2bVarCore as VariableOutputCore>::finalize_variable_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_24 {
    use super::*;
    #[test]
    fn test_iv0() {
        const IV: [u32; 4] = [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A];
        struct Blake2sVarCore;
        impl Blake2sVarCore {
            #[inline(always)]
            fn iv0() -> [u32; 4] {
                [IV[0], IV[1], IV[2], IV[3]]
            }
        }
        let expected: [u32; 4] = [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A];
        assert_eq!(Blake2sVarCore::iv0(), expected);
    }
}
#[cfg(test)]
mod tests_rug_26 {
    use super::*;
    #[test]
    fn test_new_with_params() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &[u8] = &[rug_fuzz_0; 16];
        let mut p1: &[u8] = &[rug_fuzz_1; 16];
        let mut p2: usize = rug_fuzz_2;
        let mut p3: usize = rug_fuzz_3;
        crate::Blake2sVarCore::new_with_params(p0, p1, p2, p3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_28 {
    use crate::Blake2sVarCore;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_compress() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u8, u8, u8, u8, u8, u8, u8, u8, usize, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let salt: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let persona: &[u8] = &[rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let key_size: usize = rug_fuzz_8;
        let output_size: usize = rug_fuzz_9;
        let mut p0 = Blake2sVarCore::new_with_params(
            salt,
            persona,
            key_size,
            output_size,
        );
        let mut p1 = GenericArray::<
            u8,
            <Blake2sVarCore as BlockSizeUser>::BlockSize,
        >::default();
        let p2: u32 = rug_fuzz_10;
        let p3: u32 = u32::MAX;
        Blake2sVarCore::compress(&mut p0, &p1, p2, p3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_29 {
    use crate::Blake2sVarCore;
    use digest::core_api::{BlockSizeUser, UpdateCore};
    use digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u8, u8, u8, u8, u8, u8, u8, u8, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let salt: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let persona: &[u8] = &[rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let key_size: usize = rug_fuzz_8;
        let output_size: usize = rug_fuzz_9;
        let mut p0 = Blake2sVarCore::new_with_params(
            salt,
            persona,
            key_size,
            output_size,
        );
        let mut block = GenericArray::<
            u8,
            <Blake2sVarCore as BlockSizeUser>::BlockSize,
        >::default();
        let blocks: &[_] = core::slice::from_ref(&block);
        let mut p1 = blocks;
        <Blake2sVarCore as UpdateCore>::update_blocks(&mut p0, p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_31 {
    use crate::Blake2sVarCore;
    use digest::block_buffer::BlockBuffer;
    use digest::core_api::{BlockSizeUser, BufferKindUser, VariableOutputCore};
    use digest::generic_array::GenericArray;
    use digest::OutputSizeUser;
    #[test]
    fn test_finalize_variable_core() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u8, u8, u8, u8, u8, u8, u8, u8, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let salt: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let persona: &[u8] = &[rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let key_size: usize = rug_fuzz_8;
        let output_size: usize = rug_fuzz_9;
        let mut p0 = Blake2sVarCore::new_with_params(
            salt,
            persona,
            key_size,
            output_size,
        );
        let mut p1 = BlockBuffer::<
            <Blake2sVarCore as BlockSizeUser>::BlockSize,
            <Blake2sVarCore as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Blake2sVarCore as OutputSizeUser>::OutputSize,
        >::default();
        <Blake2sVarCore as VariableOutputCore>::finalize_variable_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_33 {
    use super::*;
    use crate::Blake2sMac;
    use crate::digest::generic_array::typenum::U32;
    use crate::InvalidLength;
    #[test]
    fn test_new_with_salt_and_personal() {
        let _rug_st_tests_rug_33_rrrruuuugggg_test_new_with_salt_and_personal = 0;
        let rug_fuzz_0 = b"mykey";
        let rug_fuzz_1 = b"salt";
        let rug_fuzz_2 = b"persona";
        let p0: &[u8] = rug_fuzz_0;
        let p1: &[u8] = rug_fuzz_1;
        let p2: &[u8] = rug_fuzz_2;
        debug_assert!(
            Blake2sMac:: < U32 > ::new_with_salt_and_personal(p0, p1, p2).is_ok()
        );
        let _rug_ed_tests_rug_33_rrrruuuugggg_test_new_with_salt_and_personal = 0;
    }
}
