use super::{BorrowDecode, BorrowDecoder, Decode, Decoder};
use crate::error::DecodeError;
macro_rules! impl_tuple {
    () => {};
    ($first:ident $(, $extra:ident)*) => {
        impl <'de, $first $(, $extra)*> BorrowDecode <'de > for ($first, $($extra,)*)
        where $first : BorrowDecode <'de >, $($extra : BorrowDecode <'de >,)* { fn
        borrow_decode < BD : BorrowDecoder <'de >> (decoder : & mut BD) -> Result < Self,
        DecodeError > { Ok(($first ::borrow_decode(decoder) ?, $($extra
        ::borrow_decode(decoder) ?,)*)) } } impl <$first $(, $extra)*> Decode for
        ($first, $($extra,)*) where $first : Decode, $($extra : Decode,)* { fn decode <
        DE : Decoder > (decoder : & mut DE) -> Result < Self, DecodeError > { Ok(($first
        ::decode(decoder) ?, $($extra ::decode(decoder) ?,)*)) } }
    };
}
impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
#[cfg(test)]
mod tests_llm_16_99_llm_16_99 {
    use crate::de::{Decode, Decoder};
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{BigEndian, LittleEndian, Config, Configuration, Varint, NoLimit};
    use crate::error::DecodeError;
    #[test]
    fn decode_tuple() -> Result<(), DecodeError> {
        let data = &[0, 1, 0, 2, 0, 3, 0, 4];
        let mut reader = SliceReader::new(data);
        let config = Configuration::<BigEndian, Varint, NoLimit>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let decoded_tuple: (u8, u8, u8, u8) = Decode::decode(&mut decoder)?;
        assert_eq!(decoded_tuple, (1, 2, 3, 4));
        Ok(())
    }
}
#[cfg(test)]
mod tests_rug_297 {
    use super::*;
    use crate::de::{DecoderImpl, Decode, DecodeError, read::SliceReader};
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let v9 = crate::config::Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        debug_assert!(< (u8, u8) > ::decode(& mut p0).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_302 {
    use super::*;
    use crate::de::{self, Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        debug_assert!(< (u8, u8, u8, u8, u8) > ::decode(& mut p0).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_306 {
    use crate::de::{self, Decoder, Decode, DecodeError};
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{self, Configuration};
    #[test]
    fn test_decode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        debug_assert!(
            < (i32, i32, i32, i32, i32, i32, i32) > ::decode(& mut decoder).is_ok()
        );
             }
});    }
}
