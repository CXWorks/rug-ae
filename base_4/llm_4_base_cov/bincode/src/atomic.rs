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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
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

        let data = &[0, 0, 0, 0, 0, 0, 0, 1]; // BigEndian encoding of 1isize
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

        let data = &[1, 0, 0, 0, 0, 0, 0, 0]; // LittleEndian encoding of 1isize
        let mut reader = SliceReader::new(data);
        let config = ConfigLE::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);

        let atomic = AtomicIsize::decode(&mut decoder)?;
        assert_eq!(atomic.load(std::sync::atomic::Ordering::SeqCst), 1);
        Ok(())
    }
}