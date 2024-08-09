use super::{Encode, Encoder};
use crate::error::EncodeError;
impl<A> Encode for (A,)
where
    A: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        Ok(())
    }
}
impl<A, B> Encode for (A, B)
where
    A: Encode,
    B: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C> Encode for (A, B, C)
where
    A: Encode,
    B: Encode,
    C: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D> Encode for (A, B, C, D)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E> Encode for (A, B, C, D, E)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F> Encode for (A, B, C, D, E, F)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G> Encode for (A, B, C, D, E, F, G)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G, H> Encode for (A, B, C, D, E, F, G, H)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        self.7.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G, H, I> Encode for (A, B, C, D, E, F, G, H, I)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
    I: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        self.7.encode(encoder)?;
        self.8.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G, H, I, J> Encode for (A, B, C, D, E, F, G, H, I, J)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
    I: Encode,
    J: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        self.7.encode(encoder)?;
        self.8.encode(encoder)?;
        self.9.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> Encode for (A, B, C, D, E, F, G, H, I, J, K)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
    I: Encode,
    J: Encode,
    K: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        self.7.encode(encoder)?;
        self.8.encode(encoder)?;
        self.9.encode(encoder)?;
        self.10.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> Encode for (A, B, C, D, E, F, G, H, I, J, K, L)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
    I: Encode,
    J: Encode,
    K: Encode,
    L: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        self.7.encode(encoder)?;
        self.8.encode(encoder)?;
        self.9.encode(encoder)?;
        self.10.encode(encoder)?;
        self.11.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> Encode
for (A, B, C, D, E, F, G, H, I, J, K, L, M)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
    I: Encode,
    J: Encode,
    K: Encode,
    L: Encode,
    M: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        self.7.encode(encoder)?;
        self.8.encode(encoder)?;
        self.9.encode(encoder)?;
        self.10.encode(encoder)?;
        self.11.encode(encoder)?;
        self.12.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> Encode
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
    I: Encode,
    J: Encode,
    K: Encode,
    L: Encode,
    M: Encode,
    N: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        self.7.encode(encoder)?;
        self.8.encode(encoder)?;
        self.9.encode(encoder)?;
        self.10.encode(encoder)?;
        self.11.encode(encoder)?;
        self.12.encode(encoder)?;
        self.13.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> Encode
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
    I: Encode,
    J: Encode,
    K: Encode,
    L: Encode,
    M: Encode,
    N: Encode,
    O: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        self.7.encode(encoder)?;
        self.8.encode(encoder)?;
        self.9.encode(encoder)?;
        self.10.encode(encoder)?;
        self.11.encode(encoder)?;
        self.12.encode(encoder)?;
        self.13.encode(encoder)?;
        self.14.encode(encoder)?;
        Ok(())
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> Encode
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
    I: Encode,
    J: Encode,
    K: Encode,
    L: Encode,
    M: Encode,
    N: Encode,
    O: Encode,
    P: Encode,
{
    fn encode<_E: Encoder>(&self, encoder: &mut _E) -> Result<(), EncodeError> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)?;
        self.3.encode(encoder)?;
        self.4.encode(encoder)?;
        self.5.encode(encoder)?;
        self.6.encode(encoder)?;
        self.7.encode(encoder)?;
        self.8.encode(encoder)?;
        self.9.encode(encoder)?;
        self.10.encode(encoder)?;
        self.11.encode(encoder)?;
        self.12.encode(encoder)?;
        self.13.encode(encoder)?;
        self.14.encode(encoder)?;
        self.15.encode(encoder)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_206 {
    use crate::enc::{Encoder, Encode, EncoderImpl};
    use crate::enc::write::{SizeWriter, Writer};
    use crate::config::{Config, Configuration};
    use crate::error::EncodeError;
    fn create_encoder() -> EncoderImpl<SizeWriter, Configuration> {
        let config = Configuration::default();
        let size_writer = SizeWriter::default();
        EncoderImpl::new(size_writer, config)
    }
    fn encode_tuple<A, B, C, D, E, _E>(
        tuple: &(A, B, C, D, E),
        encoder: &mut _E,
    ) -> Result<(), EncodeError>
    where
        A: Encode,
        B: Encode,
        C: Encode,
        D: Encode,
        E: Encode,
        _E: Encoder,
    {
        tuple.0.encode(encoder)?;
        tuple.1.encode(encoder)?;
        tuple.2.encode(encoder)?;
        tuple.3.encode(encoder)?;
        tuple.4.encode(encoder)?;
        Ok(())
    }
    #[test]
    fn test_encode_tuple() {
        let tuple = (1u8, 2u16, 3u32, 4u64, 5u128);
        let mut encoder = create_encoder();
        let result = encode_tuple(&tuple, &mut encoder);
        assert!(result.is_ok(), "Encoding tuple should be successful");
        assert!(
            encoder.writer().bytes_written > 0, "Encoder should have written some bytes"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_212_llm_16_212 {
    use crate::enc::{Encode, Encoder};
    use crate::enc::encoder::EncoderImpl;
    use crate::enc::write::SizeWriter;
    use crate::config::Config;
    use crate::config::Configuration;
    use crate::error::EncodeError;
    use crate::config::BigEndian;
    use crate::config::Varint;
    use crate::config::NoLimit;
    use std::marker::PhantomData;
    #[test]
    fn test_encode_tuple() -> Result<(), EncodeError> {
        let tuple = (1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32);
        let config = Configuration::<BigEndian, Varint, NoLimit>::default()
            .with_big_endian();
        let size_writer = SizeWriter::default();
        let mut encoder = EncoderImpl::new(size_writer, config);
        tuple.encode(&mut encoder)?;
        assert_eq!(encoder.into_writer().bytes_written, 44);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_214 {
    use super::*;
    use crate::*;
    use crate::enc::{Encoder, EncoderImpl, Encode};
    use crate::enc::write::{SizeWriter, Writer};
    use crate::error::EncodeError;
    use crate::config::{Config, Configuration, BigEndian};
    use std::marker::PhantomData;
    struct TestEncoder {
        writer: SizeWriter,
        config: Configuration<BigEndian, crate::config::Varint, crate::config::NoLimit>,
    }
    impl Encoder for TestEncoder {
        type W = SizeWriter;
        type C = Configuration<BigEndian, crate::config::Varint, crate::config::NoLimit>;
        fn writer(&mut self) -> &mut Self::W {
            &mut self.writer
        }
        fn config(&self) -> &Self::C {
            &self.config
        }
    }
    impl TestEncoder {
        fn new() -> TestEncoder {
            TestEncoder {
                writer: SizeWriter::default(),
                config: Configuration::default(),
            }
        }
    }
    impl crate::utils::Sealed for TestEncoder {}
    #[test]
    fn test_encode_tuple() {
        let _rug_st_tests_llm_16_214_rrrruuuugggg_test_encode_tuple = 0;
        let rug_fuzz_0 = 1u32;
        let rug_fuzz_1 = 2u32;
        let rug_fuzz_2 = 3u32;
        let rug_fuzz_3 = 4u32;
        let rug_fuzz_4 = 5u32;
        let rug_fuzz_5 = 6u32;
        let rug_fuzz_6 = 7u32;
        let rug_fuzz_7 = 8u32;
        let rug_fuzz_8 = 9u32;
        let rug_fuzz_9 = 10u32;
        let rug_fuzz_10 = 11u32;
        let rug_fuzz_11 = 12u32;
        let rug_fuzz_12 = 13u32;
        let rug_fuzz_13 = 0;
        let tuple = (
            rug_fuzz_0,
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
        );
        let mut encoder = TestEncoder::new();
        let result = tuple.encode(&mut encoder);
        debug_assert!(result.is_ok());
        debug_assert!(encoder.writer.bytes_written > rug_fuzz_13);
        let _rug_ed_tests_llm_16_214_rrrruuuugggg_test_encode_tuple = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_216_llm_16_216 {
    use crate::enc::{Encode, Encoder, Sealed};
    use crate::enc::write::{Writer, SizeWriter};
    use crate::error::EncodeError;
    use crate::config;
    use crate::config::Configuration;
    struct MockEncoder {
        writer: SizeWriter,
        config: Configuration,
    }
    impl Sealed for MockEncoder {}
    impl Encoder for MockEncoder {
        type W = SizeWriter;
        type C = Configuration;
        fn writer(&mut self) -> &mut Self::W {
            &mut self.writer
        }
        fn config(&self) -> &Self::C {
            &self.config
        }
    }
    impl MockEncoder {
        fn new() -> Self {
            MockEncoder {
                writer: SizeWriter::default(),
                config: Configuration::default(),
            }
        }
    }
    struct MockTuple(
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    );
    impl Encode for MockTuple {
        fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
            self.0.encode(encoder)?;
            self.1.encode(encoder)?;
            self.2.encode(encoder)?;
            self.3.encode(encoder)?;
            self.4.encode(encoder)?;
            self.5.encode(encoder)?;
            self.6.encode(encoder)?;
            self.7.encode(encoder)?;
            self.8.encode(encoder)?;
            self.9.encode(encoder)?;
            self.10.encode(encoder)?;
            self.11.encode(encoder)?;
            self.12.encode(encoder)?;
            self.13.encode(encoder)?;
            self.14.encode(encoder)?;
            Ok(())
        }
    }
    #[test]
    fn test_encode_tuple() {
        let _rug_st_tests_llm_16_216_llm_16_216_rrrruuuugggg_test_encode_tuple = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 6;
        let rug_fuzz_6 = 7;
        let rug_fuzz_7 = 8;
        let rug_fuzz_8 = 9;
        let rug_fuzz_9 = 10;
        let rug_fuzz_10 = 11;
        let rug_fuzz_11 = 12;
        let rug_fuzz_12 = 13;
        let rug_fuzz_13 = 14;
        let rug_fuzz_14 = 15;
        let tuple = MockTuple(
            rug_fuzz_0,
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
        );
        let mut encoder = MockEncoder::new();
        let result = tuple.encode(&mut encoder);
        debug_assert!(result.is_ok());
        debug_assert_eq!(encoder.writer.bytes_written, 60);
        let _rug_ed_tests_llm_16_216_llm_16_216_rrrruuuugggg_test_encode_tuple = 0;
    }
}
#[cfg(test)]
mod tests_rug_328 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {
        let _rug_st_tests_rug_328_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 0u8;
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = false;
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_0);
        let mut encoder = EncoderImpl::new(v10, v9);
        let tuple = (rug_fuzz_1,);
        match tuple.encode(&mut encoder) {
            Ok(_) => debug_assert!(rug_fuzz_2),
            Err(_) => debug_assert!(rug_fuzz_3),
        };
        let _rug_ed_tests_rug_328_rrrruuuugggg_test_encode = 0;
    }
}
#[cfg(test)]
mod tests_rug_331 {
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {
        let _rug_st_tests_rug_331_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = 0i32;
        let rug_fuzz_2 = true;
        let rug_fuzz_3 = 1.0f32;
        let rug_fuzz_4 = 10;
        let mut p0 = (rug_fuzz_0.to_string(), rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_4);
        let mut p1 = EncoderImpl::<
            VecWriter,
            Configuration<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >,
        >::new(v10, v9);
        <(String, i32, bool, f32)>::encode(&p0, &mut p1).unwrap();
        let _rug_ed_tests_rug_331_rrrruuuugggg_test_encode = 0;
    }
}
#[cfg(test)]
mod tests_rug_332 {
    use super::*;
    use crate::{
        enc::{Encoder, EncoderImpl, write::Writer, impl_tuples::Encode},
        config::Configuration, features::VecWriter,
    };
    #[test]
    fn test_encode() {
        let _rug_st_tests_rug_332_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u16;
        let rug_fuzz_2 = 2u32;
        let rug_fuzz_3 = 3u64;
        let rug_fuzz_4 = 4i8;
        let rug_fuzz_5 = 5i16;
        let rug_fuzz_6 = 10;
        let p0 = (
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
        );
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_6);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        <(u8, u16, u32, u64, i8, i16) as Encode>::encode(&p0, &mut p1).unwrap();
        let _rug_ed_tests_rug_332_rrrruuuugggg_test_encode = 0;
    }
}
#[cfg(test)]
mod tests_rug_337 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {
        let _rug_st_tests_rug_337_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 1u8;
        let rug_fuzz_2 = 2u16;
        let rug_fuzz_3 = 3u64;
        let rug_fuzz_4 = 4i32;
        let rug_fuzz_5 = 5i8;
        let rug_fuzz_6 = 6i16;
        let rug_fuzz_7 = 7i64;
        let rug_fuzz_8 = "8";
        let rug_fuzz_9 = "9";
        let rug_fuzz_10 = "10";
        let rug_fuzz_11 = "11";
        let rug_fuzz_12 = 10;
        let p0 = (
            rug_fuzz_0,
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
        );
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let writer = VecWriter::with_capacity(rug_fuzz_12);
        let mut p1 = EncoderImpl::new(writer, config);
        <(
            u32,
            u8,
            u16,
            u64,
            i32,
            i8,
            i16,
            i64,
            &str,
            &str,
            &str,
            &str,
        ) as Encode>::encode(&p0, &mut p1)
            .unwrap();
        let _rug_ed_tests_rug_337_rrrruuuugggg_test_encode = 0;
    }
}
#[cfg(test)]
mod tests_rug_339 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {
        let _rug_st_tests_rug_339_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 4;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 6;
        let rug_fuzz_7 = 7;
        let rug_fuzz_8 = 8;
        let rug_fuzz_9 = 9;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 11;
        let rug_fuzz_12 = 12;
        let rug_fuzz_13 = 13;
        let rug_fuzz_14 = 14;
        let rug_fuzz_15 = 15;
        let rug_fuzz_16 = 10;
        let mut p0 = (
            rug_fuzz_0,
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
        );
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_16);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        <(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        )>::encode(&p0, &mut p1)
            .unwrap();
        let _rug_ed_tests_rug_339_rrrruuuugggg_test_encode = 0;
    }
}
