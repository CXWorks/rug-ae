use crate::{config::Endian, de::read::Reader, error::{DecodeError, IntegerType}};
pub fn varint_decode_i16<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<i16, DecodeError> {
    let n = super::varint_decode_u16(read, endian)
        .map_err(DecodeError::change_integer_type_to_signed)?;
    Ok(if n % 2 == 0 { (n / 2) as _ } else { !(n / 2) as _ })
}
pub fn varint_decode_i32<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<i32, DecodeError> {
    let n = super::varint_decode_u32(read, endian)
        .map_err(DecodeError::change_integer_type_to_signed)?;
    Ok(if n % 2 == 0 { (n / 2) as _ } else { !(n / 2) as _ })
}
pub fn varint_decode_i64<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<i64, DecodeError> {
    let n = super::varint_decode_u64(read, endian)
        .map_err(DecodeError::change_integer_type_to_signed)?;
    Ok(if n % 2 == 0 { (n / 2) as _ } else { !(n / 2) as _ })
}
pub fn varint_decode_i128<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<i128, DecodeError> {
    let n = super::varint_decode_u128(read, endian)
        .map_err(DecodeError::change_integer_type_to_signed)?;
    Ok(if n % 2 == 0 { (n / 2) as _ } else { !(n / 2) as _ })
}
pub fn varint_decode_isize<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<isize, DecodeError> {
    match varint_decode_i64(read, endian) {
        Ok(val) => Ok(val as isize),
        Err(DecodeError::InvalidIntegerType { found, .. }) => {
            Err(DecodeError::InvalidIntegerType {
                expected: IntegerType::Isize,
                found: found.into_signed(),
            })
        }
        Err(e) => Err(e),
    }
}
#[cfg(test)]
mod tests_llm_16_363 {
    use crate::varint::decode_signed::varint_decode_i128;
    use crate::de::read::SliceReader;
    use crate::config;
    use crate::error::DecodeError;
    #[test]
    fn test_varint_decode_i128_positive() {
        let _rug_st_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_positive = 0;
        let rug_fuzz_0 = 0x80;
        let rug_fuzz_1 = 0x01;
        let bytes = &[rug_fuzz_0, rug_fuzz_1];
        let mut reader = SliceReader::new(bytes);
        let result = varint_decode_i128(&mut reader, config::Endian::Little).unwrap();
        debug_assert_eq!(result, 64);
        let _rug_ed_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_positive = 0;
    }
    #[test]
    fn test_varint_decode_i128_negative() {
        let _rug_st_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_negative = 0;
        let rug_fuzz_0 = 0x81;
        let rug_fuzz_1 = 0x01;
        let bytes = &[rug_fuzz_0, rug_fuzz_1];
        let mut reader = SliceReader::new(bytes);
        let result = varint_decode_i128(&mut reader, config::Endian::Little).unwrap();
        debug_assert_eq!(result, - 64);
        let _rug_ed_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_negative = 0;
    }
    #[test]
    fn test_varint_decode_i128_zero() {
        let _rug_st_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_zero = 0;
        let rug_fuzz_0 = 0x00;
        let bytes = &[rug_fuzz_0];
        let mut reader = SliceReader::new(bytes);
        let result = varint_decode_i128(&mut reader, config::Endian::Little).unwrap();
        debug_assert_eq!(result, 0);
        let _rug_ed_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_zero = 0;
    }
    #[test]
    fn test_varint_decode_i128_unexpected_end() {
        let _rug_st_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_unexpected_end = 0;
        let rug_fuzz_0 = 0x80;
        let bytes = &[rug_fuzz_0];
        let mut reader = SliceReader::new(bytes);
        let result = varint_decode_i128(&mut reader, config::Endian::Little);
        debug_assert!(matches!(result, Err(DecodeError::UnexpectedEnd { .. })));
        let _rug_ed_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_unexpected_end = 0;
    }
    #[test]
    fn test_varint_decode_i128_max() {
        let _rug_st_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_max = 0;
        let rug_fuzz_0 = 0xFF;
        let rug_fuzz_1 = 0xFF;
        let rug_fuzz_2 = 0xFF;
        let rug_fuzz_3 = 0xFF;
        let rug_fuzz_4 = 0xFF;
        let rug_fuzz_5 = 0xFF;
        let rug_fuzz_6 = 0xFF;
        let rug_fuzz_7 = 0xFF;
        let rug_fuzz_8 = 0xFF;
        let rug_fuzz_9 = 0xFF;
        let rug_fuzz_10 = 0xFF;
        let rug_fuzz_11 = 0xFF;
        let rug_fuzz_12 = 0xFF;
        let rug_fuzz_13 = 0xFF;
        let rug_fuzz_14 = 0xFF;
        let rug_fuzz_15 = 0xFF;
        let rug_fuzz_16 = 0x7F;
        let bytes = &[
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
            rug_fuzz_16,
        ];
        let mut reader = SliceReader::new(bytes);
        let result = varint_decode_i128(&mut reader, config::Endian::Little).unwrap();
        debug_assert_eq!(result, i128::MAX);
        let _rug_ed_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_max = 0;
    }
    #[test]
    fn test_varint_decode_i128_min() {
        let _rug_st_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_min = 0;
        let rug_fuzz_0 = 0x81;
        let rug_fuzz_1 = 0x80;
        let rug_fuzz_2 = 0x80;
        let rug_fuzz_3 = 0x80;
        let rug_fuzz_4 = 0x80;
        let rug_fuzz_5 = 0x80;
        let rug_fuzz_6 = 0x80;
        let rug_fuzz_7 = 0x80;
        let rug_fuzz_8 = 0x80;
        let rug_fuzz_9 = 0x80;
        let rug_fuzz_10 = 0x80;
        let rug_fuzz_11 = 0x80;
        let rug_fuzz_12 = 0x80;
        let rug_fuzz_13 = 0x80;
        let rug_fuzz_14 = 0x80;
        let rug_fuzz_15 = 0x80;
        let rug_fuzz_16 = 0x80;
        let bytes = &[
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
            rug_fuzz_16,
        ];
        let mut reader = SliceReader::new(bytes);
        let result = varint_decode_i128(&mut reader, config::Endian::Little).unwrap();
        debug_assert_eq!(result, i128::MIN);
        let _rug_ed_tests_llm_16_363_rrrruuuugggg_test_varint_decode_i128_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_364_llm_16_364 {
    use crate::config::Endian;
    use crate::de::read::SliceReader;
    use crate::de::read::Reader;
    use crate::error::DecodeError;
    use crate::varint::decode_signed::varint_decode_i16;
    #[test]
    fn test_varint_decode_i16_positive() {
        let _rug_st_tests_llm_16_364_llm_16_364_rrrruuuugggg_test_varint_decode_i16_positive = 0;
        let rug_fuzz_0 = 0x04;
        let data = vec![rug_fuzz_0];
        let endian = Endian::Little;
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(varint_decode_i16(& mut reader, endian).unwrap(), 2_i16);
        let _rug_ed_tests_llm_16_364_llm_16_364_rrrruuuugggg_test_varint_decode_i16_positive = 0;
    }
    #[test]
    fn test_varint_decode_i16_negative() {
        let _rug_st_tests_llm_16_364_llm_16_364_rrrruuuugggg_test_varint_decode_i16_negative = 0;
        let rug_fuzz_0 = 0x03;
        let data = vec![rug_fuzz_0];
        let endian = Endian::Little;
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(varint_decode_i16(& mut reader, endian).unwrap(), - 1_i16);
        let _rug_ed_tests_llm_16_364_llm_16_364_rrrruuuugggg_test_varint_decode_i16_negative = 0;
    }
    #[test]
    fn test_varint_decode_i16_unexpected_end() {
        let _rug_st_tests_llm_16_364_llm_16_364_rrrruuuugggg_test_varint_decode_i16_unexpected_end = 0;
        let data = vec![];
        let endian = Endian::Little;
        let mut reader = SliceReader::new(&data);
        debug_assert!(
            matches!(varint_decode_i16(& mut reader, endian),
            Err(DecodeError::UnexpectedEnd { .. }))
        );
        let _rug_ed_tests_llm_16_364_llm_16_364_rrrruuuugggg_test_varint_decode_i16_unexpected_end = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_366_llm_16_366 {
    use crate::varint::decode_signed::varint_decode_i64;
    use crate::de::read::SliceReader;
    use crate::de::read::Reader;
    use crate::config;
    use crate::error::DecodeError;
    #[test]
    fn test_varint_decode_i64_positive() -> Result<(), DecodeError> {
        let data = vec![0x08];
        let mut slice_reader = SliceReader::new(&data);
        let result = varint_decode_i64(&mut slice_reader, config::Endian::Little)?;
        assert_eq!(result, 4);
        Ok(())
    }
    #[test]
    fn test_varint_decode_i64_negative() -> Result<(), DecodeError> {
        let data = vec![0x09];
        let mut slice_reader = SliceReader::new(&data);
        let result = varint_decode_i64(&mut slice_reader, config::Endian::Little)?;
        assert_eq!(result, - 4);
        Ok(())
    }
    #[test]
    fn test_varint_decode_i64_zero() -> Result<(), DecodeError> {
        let data = vec![0x00];
        let mut slice_reader = SliceReader::new(&data);
        let result = varint_decode_i64(&mut slice_reader, config::Endian::Little)?;
        assert_eq!(result, 0);
        Ok(())
    }
    #[test]
    fn test_varint_decode_i64_error() {
        let data = vec![];
        let mut slice_reader = SliceReader::new(&data);
        assert!(varint_decode_i64(& mut slice_reader, config::Endian::Little).is_err());
    }
}
