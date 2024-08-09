use crate::{de::Decode, enc::Encode, impl_borrow_decode};
use core::sync::atomic::Ordering;
#[cfg(target_has_atomic = "ptr")]
use core::sync::atomic::{AtomicIsize, AtomicUsize};
#[cfg(target_has_atomic = "8")]
use core::sync::atomic::{AtomicBool, AtomicI8, AtomicU8};
#[cfg(target_has_atomic = "16")]
use core::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{AtomicI64, AtomicU64};
#[cfg(target_has_atomic = "8")]
impl Encode for AtomicBool {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "8")]
impl Decode for AtomicBool {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicBool::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "8")]
impl_borrow_decode!(AtomicBool);
#[cfg(target_has_atomic = "8")]
impl Encode for AtomicU8 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "8")]
impl Decode for AtomicU8 {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU8::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "8")]
impl_borrow_decode!(AtomicU8);
#[cfg(target_has_atomic = "16")]
impl Encode for AtomicU16 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "16")]
impl Decode for AtomicU16 {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU16::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "16")]
impl_borrow_decode!(AtomicU16);
#[cfg(target_has_atomic = "32")]
impl Encode for AtomicU32 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "32")]
impl Decode for AtomicU32 {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU32::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "32")]
impl_borrow_decode!(AtomicU32);
#[cfg(target_has_atomic = "64")]
impl Encode for AtomicU64 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "64")]
impl Decode for AtomicU64 {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU64::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "64")]
impl_borrow_decode!(AtomicU64);
#[cfg(target_has_atomic = "ptr")]
impl Encode for AtomicUsize {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "ptr")]
impl Decode for AtomicUsize {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicUsize::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "ptr")]
impl_borrow_decode!(AtomicUsize);
#[cfg(target_has_atomic = "8")]
impl Encode for AtomicI8 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "8")]
impl Decode for AtomicI8 {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI8::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "8")]
impl_borrow_decode!(AtomicI8);
#[cfg(target_has_atomic = "16")]
impl Encode for AtomicI16 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "16")]
impl Decode for AtomicI16 {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI16::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "16")]
impl_borrow_decode!(AtomicI16);
#[cfg(target_has_atomic = "32")]
impl Encode for AtomicI32 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "32")]
impl Decode for AtomicI32 {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI32::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "32")]
impl_borrow_decode!(AtomicI32);
#[cfg(target_has_atomic = "64")]
impl Encode for AtomicI64 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "64")]
impl Decode for AtomicI64 {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI64::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "64")]
impl_borrow_decode!(AtomicI64);
#[cfg(target_has_atomic = "ptr")]
impl Encode for AtomicIsize {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}
#[cfg(target_has_atomic = "ptr")]
impl Decode for AtomicIsize {
    fn decode<D: crate::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicIsize::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "ptr")]
impl_borrow_decode!(AtomicIsize);
#[cfg(test)]
mod tests_llm_16_46_llm_16_46 {
    use super::*;
    use crate::*;
    use crate::de::{Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::error::DecodeError;
    use crate::config::{BigEndian, LittleEndian, Config, Configuration, Fixint, NoLimit};
    use std::sync::atomic::AtomicIsize;
    use std::marker::PhantomData;
    #[test]
    fn decode_atomic_isize_big_endian() -> Result<(), DecodeError> {
        type ConfigBE = Configuration<BigEndian, Fixint, NoLimit>;
        let data = &[0, 0, 0, 0, 0, 0, 0, 1];
        let mut reader = SliceReader::new(data);
        let config = ConfigBE::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        let atomic = AtomicIsize::decode(&mut decoder)?;
        assert_eq!(atomic.load(std::sync::atomic::Ordering::SeqCst), 1);
        Ok(())
    }
    #[test]
    fn decode_atomic_isize_little_endian() -> Result<(), DecodeError> {
        type ConfigLE = Configuration<LittleEndian, Fixint, NoLimit>;
        let data = &[1, 0, 0, 0, 0, 0, 0, 0];
        let mut reader = SliceReader::new(data);
        let config = ConfigLE::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        let atomic = AtomicIsize::decode(&mut decoder)?;
        assert_eq!(atomic.load(std::sync::atomic::Ordering::SeqCst), 1);
        Ok(())
    }
}
#[cfg(test)]
mod tests_rug_267 {
    use super::*;
    use crate::Encode;
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use crate::enc::{EncoderImpl, write::Writer};
    use std::sync::atomic::{AtomicBool, Ordering};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = AtomicBool::new(rug_fuzz_0);
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(v10, v9);
        p0.encode(&mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_268 {
    use super::*;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_atomic_bool_decode() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        let result = <std::sync::atomic::AtomicBool as Decode>::decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_269 {
    use super::*;
    use crate::Encode;
    use crate::enc::{self, EncoderImpl, write::Writer};
    use crate::config::{self, Configuration};
    use crate::features::VecWriter;
    use std::sync::atomic::{AtomicU8, Ordering};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = AtomicU8::new(rug_fuzz_0);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        p0.encode(&mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_270 {
    use super::*;
    use crate::de::{DecoderImpl, read::SliceReader};
    use crate::config::Configuration;
    use crate::Decode;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        debug_assert!(< std::sync::atomic::AtomicU8 > ::decode(& mut p0).is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_271 {
    use super::*;
    use std::sync::atomic::AtomicU16;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::features::VecWriter;
    use crate::Encode;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = AtomicU16::new(rug_fuzz_0);
        let mut v9 = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(v10, v9);
        <AtomicU16 as Encode>::encode(&p0, &mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_272 {
    use super::*;
    use crate::de::{Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::Configuration;
    use crate::Decode;
    use std::sync::atomic::AtomicU16;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        let result = <AtomicU16 as Decode>::decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_273 {
    use super::*;
    use std::sync::atomic::AtomicU32;
    use crate::enc::{Encoder, EncoderImpl};
    use crate::enc::write::Writer;
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = AtomicU32::new(rug_fuzz_0);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(&mut writer, config);
        p0.encode(&mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_274 {
    use super::*;
    use crate::de::Decoder;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_decode_atomic_u32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        match <std::sync::atomic::AtomicU32 as Decode>::decode(&mut decoder) {
            Ok(atomic) => {
                debug_assert_eq!(
                    atomic.load(std::sync::atomic::Ordering::SeqCst), 0x01020304
                );
            }
            Err(e) => {
                panic!("Failed to decode AtomicU32: {}", e);
            }
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_275 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::sync::atomic::{AtomicU64, Ordering};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = AtomicU64::new(rug_fuzz_0);
        let mut p1 = EncoderImpl::new(
            VecWriter::with_capacity(rug_fuzz_1),
            Configuration::<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >::default(),
        );
        p0.encode(&mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_276 {
    use super::*;
    use crate::de::{self, Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::Configuration;
    use std::sync::atomic::AtomicU64;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _result = AtomicU64::decode(&mut p0).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_277 {
    use super::*;
    use crate::enc::{Encoder, EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::sync::atomic::{AtomicUsize, Ordering};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = AtomicUsize::new(rug_fuzz_0);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        p0.encode(&mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_278 {
    use super::*;
    use crate::de::{DecoderImpl, Decoder};
    use crate::de::read::SliceReader;
    use crate::config::Configuration;
    use std::sync::atomic::AtomicUsize;
    use crate::error::DecodeError;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        debug_assert!(matches!(< AtomicUsize as Decode > ::decode(& mut p0), Ok(_)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_279 {
    use super::*;
    use std::sync::atomic::AtomicI8;
    use crate::enc::{EncoderImpl, Encoder, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode_atomici8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = AtomicI8::new(rug_fuzz_0);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        debug_assert!(p0.encode(& mut p1).is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_280 {
    use super::*;
    use crate::config::Configuration;
    use crate::de::{DecoderImpl, Decode};
    use crate::de::read::SliceReader;
    use std::sync::atomic::AtomicI8;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let _result = AtomicI8::decode(&mut decoder).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_281 {
    use super::*;
    use crate::enc::{Encoder, EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::sync::atomic::{AtomicI16, Ordering};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = AtomicI16::new(rug_fuzz_0);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(writer, config);
        <AtomicI16 as Encode>::encode(&p0, &mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_282 {
    use super::*;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_decode_atomic_i16() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <std::sync::atomic::AtomicI16>::decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_283 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::sync::atomic::{AtomicI32, Ordering};
    #[test]
    fn test_encode_atomic_i32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = AtomicI32::new(rug_fuzz_0);
        let v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(v10, v9);
        debug_assert!(p0.encode(& mut p1).is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_284 {
    use super::*;
    use crate::Decode;
    use crate::de::{self, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::Configuration;
    use std::sync::atomic::AtomicI32;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let result = <AtomicI32 as Decode>::decode(&mut p0);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_285 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::sync::atomic::{AtomicI64, Ordering};
    #[test]
    fn test_encode_atomic_i64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = AtomicI64::new(rug_fuzz_0);
        let v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(v10, v9);
        p0.encode(&mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_286 {
    use super::*;
    use crate::de::{DecoderImpl, read::SliceReader, Decoder};
    use crate::config::Configuration;
    use std::sync::atomic::AtomicI64;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(&mut reader, config);
        let _result = AtomicI64::decode(&mut p0).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_287 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::sync::atomic::{AtomicIsize, Ordering};
    #[test]
    fn test_encode_atomic_isize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = AtomicIsize::new(rug_fuzz_0);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(writer, config);
        p0.encode(&mut p1).unwrap();
             }
}
}
}    }
}
