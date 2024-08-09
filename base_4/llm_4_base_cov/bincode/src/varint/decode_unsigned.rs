use core::{convert::TryInto, u32};
use super::{SINGLE_BYTE_MAX, U128_BYTE, U16_BYTE, U32_BYTE, U64_BYTE};
use crate::{config::Endian, de::read::Reader, error::{DecodeError, IntegerType}};
#[inline(never)]
#[cold]
fn deserialize_varint_cold_u16<R>(
    read: &mut R,
    endian: Endian,
) -> Result<u16, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u16),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u16::from_be_bytes(bytes),
                    Endian::Little => u16::from_le_bytes(bytes),
                },
            )
        }
        U32_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U32),
        U64_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U64),
        U128_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U16, IntegerType::Reserved),
    }
}
#[inline(never)]
#[cold]
fn deserialize_varint_cold_u32<R>(
    read: &mut R,
    endian: Endian,
) -> Result<u32, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u32),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u16::from_be_bytes(bytes) as u32,
                    Endian::Little => u16::from_le_bytes(bytes) as u32,
                },
            )
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u32::from_be_bytes(bytes),
                    Endian::Little => u32::from_le_bytes(bytes),
                },
            )
        }
        U64_BYTE => invalid_varint_discriminant(IntegerType::U32, IntegerType::U64),
        U128_BYTE => invalid_varint_discriminant(IntegerType::U32, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U32, IntegerType::Reserved),
    }
}
#[inline(never)]
#[cold]
fn deserialize_varint_cold_u64<R>(
    read: &mut R,
    endian: Endian,
) -> Result<u64, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u64),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u16::from_be_bytes(bytes) as u64,
                    Endian::Little => u16::from_le_bytes(bytes) as u64,
                },
            )
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u32::from_be_bytes(bytes) as u64,
                    Endian::Little => u32::from_le_bytes(bytes) as u64,
                },
            )
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u64::from_be_bytes(bytes),
                    Endian::Little => u64::from_le_bytes(bytes),
                },
            )
        }
        U128_BYTE => invalid_varint_discriminant(IntegerType::U64, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U64, IntegerType::Reserved),
    }
}
#[inline(never)]
#[cold]
fn deserialize_varint_cold_usize<R>(
    read: &mut R,
    endian: Endian,
) -> Result<usize, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as usize),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u16::from_be_bytes(bytes) as usize,
                    Endian::Little => u16::from_le_bytes(bytes) as usize,
                },
            )
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u32::from_be_bytes(bytes) as usize,
                    Endian::Little => u32::from_le_bytes(bytes) as usize,
                },
            )
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u64::from_be_bytes(bytes) as usize,
                    Endian::Little => u64::from_le_bytes(bytes) as usize,
                },
            )
        }
        U128_BYTE => invalid_varint_discriminant(IntegerType::Usize, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::Usize, IntegerType::Reserved),
    }
}
#[inline(never)]
#[cold]
fn deserialize_varint_cold_u128<R>(
    read: &mut R,
    endian: Endian,
) -> Result<u128, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u128),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u16::from_be_bytes(bytes) as u128,
                    Endian::Little => u16::from_le_bytes(bytes) as u128,
                },
            )
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u32::from_be_bytes(bytes) as u128,
                    Endian::Little => u32::from_le_bytes(bytes) as u128,
                },
            )
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u64::from_be_bytes(bytes) as u128,
                    Endian::Little => u64::from_le_bytes(bytes) as u128,
                },
            )
        }
        U128_BYTE => {
            let mut bytes = [0u8; 16];
            read.read(&mut bytes)?;
            Ok(
                match endian {
                    Endian::Big => u128::from_be_bytes(bytes),
                    Endian::Little => u128::from_le_bytes(bytes),
                },
            )
        }
        _ => invalid_varint_discriminant(IntegerType::U128, IntegerType::Reserved),
    }
}
#[inline(never)]
#[cold]
fn invalid_varint_discriminant<T>(
    expected: IntegerType,
    found: IntegerType,
) -> Result<T, DecodeError> {
    Err(DecodeError::InvalidIntegerType {
        expected,
        found,
    })
}
pub fn varint_decode_u16<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<u16, DecodeError> {
    if let Some(bytes) = read.peek_read(3) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as u16, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };
                (val, 3)
            }
            U32_BYTE => {
                return invalid_varint_discriminant(IntegerType::U16, IntegerType::U32);
            }
            U64_BYTE => {
                return invalid_varint_discriminant(IntegerType::U16, IntegerType::U64);
            }
            U128_BYTE => {
                return invalid_varint_discriminant(IntegerType::U16, IntegerType::U128);
            }
            _ => {
                return invalid_varint_discriminant(
                    IntegerType::U16,
                    IntegerType::Reserved,
                );
            }
        };
        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_u16(read, endian)
    }
}
pub fn varint_decode_u32<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<u32, DecodeError> {
    if let Some(bytes) = read.peek_read(5) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as u32, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };
                (val as u32, 3)
            }
            U32_BYTE => {
                let val = match endian {
                    Endian::Big => u32::from_be_bytes(bytes[..4].try_into().unwrap()),
                    Endian::Little => u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                };
                (val, 5)
            }
            U64_BYTE => {
                return invalid_varint_discriminant(IntegerType::U32, IntegerType::U64);
            }
            U128_BYTE => {
                return invalid_varint_discriminant(IntegerType::U32, IntegerType::U128);
            }
            _ => {
                return invalid_varint_discriminant(
                    IntegerType::U32,
                    IntegerType::Reserved,
                );
            }
        };
        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_u32(read, endian)
    }
}
pub fn varint_decode_u64<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<u64, DecodeError> {
    if let Some(bytes) = read.peek_read(9) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as u64, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };
                (val as u64, 3)
            }
            U32_BYTE => {
                let val = match endian {
                    Endian::Big => u32::from_be_bytes(bytes[..4].try_into().unwrap()),
                    Endian::Little => u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                };
                (val as u64, 5)
            }
            U64_BYTE => {
                let val = match endian {
                    Endian::Big => u64::from_be_bytes(bytes[..8].try_into().unwrap()),
                    Endian::Little => u64::from_le_bytes(bytes[..8].try_into().unwrap()),
                };
                (val, 9)
            }
            U128_BYTE => {
                return invalid_varint_discriminant(IntegerType::U32, IntegerType::U128);
            }
            _ => {
                return invalid_varint_discriminant(
                    IntegerType::U32,
                    IntegerType::Reserved,
                );
            }
        };
        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_u64(read, endian)
    }
}
pub fn varint_decode_usize<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<usize, DecodeError> {
    if let Some(bytes) = read.peek_read(9) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as usize, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };
                (val as usize, 3)
            }
            U32_BYTE => {
                let val = match endian {
                    Endian::Big => u32::from_be_bytes(bytes[..4].try_into().unwrap()),
                    Endian::Little => u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                };
                (val as usize, 5)
            }
            U64_BYTE => {
                let val = match endian {
                    Endian::Big => u64::from_be_bytes(bytes[..8].try_into().unwrap()),
                    Endian::Little => u64::from_le_bytes(bytes[..8].try_into().unwrap()),
                };
                (val as usize, 9)
            }
            U128_BYTE => {
                return invalid_varint_discriminant(
                    IntegerType::Usize,
                    IntegerType::U128,
                );
            }
            _ => {
                return invalid_varint_discriminant(
                    IntegerType::Usize,
                    IntegerType::Reserved,
                );
            }
        };
        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_usize(read, endian)
    }
}
pub fn varint_decode_u128<R: Reader>(
    read: &mut R,
    endian: Endian,
) -> Result<u128, DecodeError> {
    if let Some(bytes) = read.peek_read(17) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as u128, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };
                (val as u128, 3)
            }
            U32_BYTE => {
                let val = match endian {
                    Endian::Big => u32::from_be_bytes(bytes[..4].try_into().unwrap()),
                    Endian::Little => u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                };
                (val as u128, 5)
            }
            U64_BYTE => {
                let val = match endian {
                    Endian::Big => u64::from_be_bytes(bytes[..8].try_into().unwrap()),
                    Endian::Little => u64::from_le_bytes(bytes[..8].try_into().unwrap()),
                };
                (val as u128, 9)
            }
            U128_BYTE => {
                let val = match endian {
                    Endian::Big => u128::from_be_bytes(bytes[..16].try_into().unwrap()),
                    Endian::Little => {
                        u128::from_le_bytes(bytes[..16].try_into().unwrap())
                    }
                };
                (val, 17)
            }
            _ => {
                return invalid_varint_discriminant(
                    IntegerType::Usize,
                    IntegerType::Reserved,
                );
            }
        };
        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_u128(read, endian)
    }
}
#[test]
fn test_decode_u16() {
    let cases: &[(&[u8], u16, u16)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u16(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u16(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }
    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U32_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U16,
                found: IntegerType::U32,
            },
        ),
        (
            &[U64_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U16,
                found: IntegerType::U64,
            },
        ),
        (
            &[U128_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U16,
                found: IntegerType::U128,
            },
        ),
        (
            &[U16_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U16_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
    ];
    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u16(&mut reader, Endian::Little).unwrap_err();
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}
#[test]
fn test_decode_u32() {
    let cases: &[(&[u8], u32, u32)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
        (&[U32_BYTE, 0, 0, 0, 10], 167_772_160, 10),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u32(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u32(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }
    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U64_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U32,
                found: IntegerType::U64,
            },
        ),
        (
            &[U128_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U32,
                found: IntegerType::U128,
            },
        ),
        (
            &[U16_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U16_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
        (
            &[U32_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 4,
            },
        ),
        (
            &[U32_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 3,
            },
        ),
        (
            &[U32_BYTE, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U32_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
    ];
    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u32(&mut reader, Endian::Little).unwrap_err();
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}
#[test]
fn test_decode_u64() {
    let cases: &[(&[u8], u64, u64)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
        (&[U32_BYTE, 0, 0, 0, 10], 167_772_160, 10),
        (&[U64_BYTE, 0, 0, 0, 0, 0, 0, 0, 10], 720_575_940_379_279_360, 10),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u64(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u64(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }
    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U128_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U64,
                found: IntegerType::U128,
            },
        ),
        (
            &[U16_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U16_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
        (
            &[U32_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 4,
            },
        ),
        (
            &[U32_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 3,
            },
        ),
        (
            &[U32_BYTE, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U32_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
        (
            &[U64_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 8,
            },
        ),
        (
            &[U64_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 7,
            },
        ),
        (
            &[U64_BYTE, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 6,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 5,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 4,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 3,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
    ];
    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u64(&mut reader, Endian::Little).unwrap_err();
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}
#[test]
fn test_decode_u128() {
    let cases: &[(&[u8], u128, u128)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
        (&[U32_BYTE, 0, 0, 0, 10], 167_772_160, 10),
        (&[U64_BYTE, 0, 0, 0, 0, 0, 0, 0, 10], 720_575_940_379_279_360, 10),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10],
            13_292_279_957_849_158_729_038_070_602_803_445_760,
            10,
        ),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u128(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u128(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }
    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U16_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U16_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
        (
            &[U32_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 4,
            },
        ),
        (
            &[U32_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 3,
            },
        ),
        (
            &[U32_BYTE, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U32_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
        (
            &[U64_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 8,
            },
        ),
        (
            &[U64_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 7,
            },
        ),
        (
            &[U64_BYTE, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 6,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 5,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 4,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 3,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
        (
            &[U128_BYTE],
            DecodeError::UnexpectedEnd {
                additional: 16,
            },
        ),
        (
            &[U128_BYTE, 0],
            DecodeError::UnexpectedEnd {
                additional: 15,
            },
        ),
        (
            &[U128_BYTE, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 14,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 13,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 12,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 11,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 10,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 9,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 8,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 7,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 6,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 5,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 4,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 3,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 2,
            },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd {
                additional: 1,
            },
        ),
    ];
    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u128(&mut reader, Endian::Little).unwrap_err();
        std::dbg!(slice);
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}
#[cfg(test)]
mod tests_llm_16_374_llm_16_374 {
    use crate::varint::decode_unsigned::varint_decode_u128;
    use crate::de::read::Reader;
    use crate::de::read::SliceReader;
    use crate::config::Endian;
    use crate::error::DecodeError;
    const U16_BYTE: u8 = 0xfd;
    const U32_BYTE: u8 = 0xfe;
    const U64_BYTE: u8 = 0xff;
    const U128_BYTE: u8 = 0x01;
    const SINGLE_BYTE_MAX: u8 = 0xfc;
    #[test]
    fn test_varint_decode_u128_single_byte() {
        let _rug_st_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_single_byte = 0;
        let data = [SINGLE_BYTE_MAX];
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Little);
        debug_assert_eq!(result.unwrap(), SINGLE_BYTE_MAX as u128);
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Big);
        debug_assert_eq!(result.unwrap(), SINGLE_BYTE_MAX as u128);
        let _rug_ed_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_single_byte = 0;
    }
    #[test]
    fn test_varint_decode_u128_u16() {
        let _rug_st_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_u16 = 0;
        let rug_fuzz_0 = 0x01;
        let rug_fuzz_1 = 0x02;
        let data = [U16_BYTE, rug_fuzz_0, rug_fuzz_1];
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Little);
        debug_assert_eq!(result.unwrap(), 0x0201);
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Big);
        debug_assert_eq!(result.unwrap(), 0x0102);
        let _rug_ed_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_u16 = 0;
    }
    #[test]
    fn test_varint_decode_u128_u32() {
        let _rug_st_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_u32 = 0;
        let rug_fuzz_0 = 0x01;
        let rug_fuzz_1 = 0x02;
        let rug_fuzz_2 = 0x03;
        let rug_fuzz_3 = 0x04;
        let data = [U32_BYTE, rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Little);
        debug_assert_eq!(result.unwrap(), 0x04030201);
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Big);
        debug_assert_eq!(result.unwrap(), 0x01020304);
        let _rug_ed_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_u32 = 0;
    }
    #[test]
    fn test_varint_decode_u128_u64() {
        let _rug_st_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_u64 = 0;
        let rug_fuzz_0 = 0x01;
        let rug_fuzz_1 = 0x02;
        let rug_fuzz_2 = 0x03;
        let rug_fuzz_3 = 0x04;
        let rug_fuzz_4 = 0x05;
        let rug_fuzz_5 = 0x06;
        let rug_fuzz_6 = 0x07;
        let rug_fuzz_7 = 0x08;
        let data = [
            U64_BYTE,
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Little);
        debug_assert_eq!(result.unwrap(), 0x0807060504030201);
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Big);
        debug_assert_eq!(result.unwrap(), 0x0102030405060708);
        let _rug_ed_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_u64 = 0;
    }
    #[test]
    fn test_varint_decode_u128_u128() {
        let _rug_st_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_u128 = 0;
        let rug_fuzz_0 = 0x01;
        let rug_fuzz_1 = 0x02;
        let rug_fuzz_2 = 0x03;
        let rug_fuzz_3 = 0x04;
        let rug_fuzz_4 = 0x05;
        let rug_fuzz_5 = 0x06;
        let rug_fuzz_6 = 0x07;
        let rug_fuzz_7 = 0x08;
        let rug_fuzz_8 = 0x09;
        let rug_fuzz_9 = 0x0A;
        let rug_fuzz_10 = 0x0B;
        let rug_fuzz_11 = 0x0C;
        let rug_fuzz_12 = 0x0D;
        let rug_fuzz_13 = 0x0E;
        let rug_fuzz_14 = 0x0F;
        let rug_fuzz_15 = 0x10;
        let data = [
            U128_BYTE,
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
        ];
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Little);
        debug_assert_eq!(result.unwrap(), 0x100F0E0D0C0B0A090807060504030201);
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Big);
        debug_assert_eq!(result.unwrap(), 0x0102030405060708090A0B0C0D0E0F10);
        let _rug_ed_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_u128 = 0;
    }
    #[test]
    fn test_varint_decode_u128_errors() {
        let _rug_st_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_errors = 0;
        let rug_fuzz_0 = 0x05u8;
        let data = [rug_fuzz_0];
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Little);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_errors = 0;
    }
    #[test]
    fn test_varint_decode_u128_unexpected_end() {
        let _rug_st_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_unexpected_end = 0;
        let rug_fuzz_0 = 0x01;
        let rug_fuzz_1 = 0x02;
        let data = [U32_BYTE, rug_fuzz_0, rug_fuzz_1];
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u128(&mut reader, Endian::Little);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_374_llm_16_374_rrrruuuugggg_test_varint_decode_u128_unexpected_end = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_376_llm_16_376 {
    use super::*;
    use crate::*;
    use crate::de::read::SliceReader;
    use crate::config::Endian;
    use crate::error::DecodeError;
    use crate::varint::decode_unsigned::varint_decode_u32;
    #[test]
    fn test_varint_decode_u32_single_byte() {
        let _rug_st_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_single_byte = 0;
        let rug_fuzz_0 = 0x05;
        let data = [rug_fuzz_0];
        let mut reader = SliceReader::new(&data);
        let decoded = varint_decode_u32(&mut reader, Endian::Little).unwrap();
        debug_assert_eq!(decoded, 5);
        let _rug_ed_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_single_byte = 0;
    }
    #[test]
    fn test_varint_decode_u32_u16_little_endian() {
        let _rug_st_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_u16_little_endian = 0;
        let rug_fuzz_0 = 0x01;
        let rug_fuzz_1 = 0x34;
        let rug_fuzz_2 = 0x12;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let mut reader = SliceReader::new(&data);
        let decoded = varint_decode_u32(&mut reader, Endian::Little).unwrap();
        debug_assert_eq!(decoded, 0x1234);
        let _rug_ed_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_u16_little_endian = 0;
    }
    #[test]
    fn test_varint_decode_u32_u16_big_endian() {
        let _rug_st_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_u16_big_endian = 0;
        let rug_fuzz_0 = 0x01;
        let rug_fuzz_1 = 0x12;
        let rug_fuzz_2 = 0x34;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let mut reader = SliceReader::new(&data);
        let decoded = varint_decode_u32(&mut reader, Endian::Big).unwrap();
        debug_assert_eq!(decoded, 0x1234);
        let _rug_ed_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_u16_big_endian = 0;
    }
    #[test]
    fn test_varint_decode_u32_u32_little_endian() {
        let _rug_st_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_u32_little_endian = 0;
        let rug_fuzz_0 = 0x02;
        let rug_fuzz_1 = 0x78;
        let rug_fuzz_2 = 0x56;
        let rug_fuzz_3 = 0x34;
        let rug_fuzz_4 = 0x12;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let decoded = varint_decode_u32(&mut reader, Endian::Little).unwrap();
        debug_assert_eq!(decoded, 0x12345678);
        let _rug_ed_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_u32_little_endian = 0;
    }
    #[test]
    fn test_varint_decode_u32_u32_big_endian() {
        let _rug_st_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_u32_big_endian = 0;
        let rug_fuzz_0 = 0x02;
        let rug_fuzz_1 = 0x12;
        let rug_fuzz_2 = 0x34;
        let rug_fuzz_3 = 0x56;
        let rug_fuzz_4 = 0x78;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let decoded = varint_decode_u32(&mut reader, Endian::Big).unwrap();
        debug_assert_eq!(decoded, 0x12345678);
        let _rug_ed_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_u32_big_endian = 0;
    }
    #[test]
    fn test_varint_decode_u32_unexpected_discriminant() {
        let _rug_st_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_unexpected_discriminant = 0;
        let rug_fuzz_0 = 0x04;
        let data = [rug_fuzz_0];
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u32(&mut reader, Endian::Little);
        debug_assert!(matches!(result, Err(DecodeError::UnexpectedVariant { .. })));
        let _rug_ed_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_unexpected_discriminant = 0;
    }
    #[test]
    fn test_varint_decode_u32_unexpected_eof() {
        let _rug_st_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_unexpected_eof = 0;
        let rug_fuzz_0 = 0x02;
        let rug_fuzz_1 = 0x78;
        let rug_fuzz_2 = 0x56;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let mut reader = SliceReader::new(&data);
        let result = varint_decode_u32(&mut reader, Endian::Little);
        debug_assert!(matches!(result, Err(DecodeError::UnexpectedEnd { .. })));
        let _rug_ed_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_varint_decode_u32_unexpected_eof = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_377_llm_16_377 {
    use super::*;
    use crate::*;
    use crate::de::read::SliceReader;
    use crate::error::DecodeError;
    fn create_reader(bytes: &[u8]) -> SliceReader {
        SliceReader::new(bytes)
    }
    #[test]
    fn test_varint_decode_u64_single_byte() {
        let mut reader = create_reader(&[0b1100_0110]);
        let result = varint_decode_u64(&mut reader, Endian::Little);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0b1100_0110);
    }
    #[test]
    fn test_varint_decode_u64_two_bytes() {
        let bytes = &[0b1111_1000, 0x01, 0x02];
        let mut reader = create_reader(bytes);
        let result = varint_decode_u64(&mut reader, Endian::Little);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x0201);
    }
    #[test]
    fn test_varint_decode_u64_four_bytes() {
        let bytes = &[0b1111_1001, 0x01, 0x02, 0x03, 0x04];
        let mut reader = create_reader(bytes);
        let result = varint_decode_u64(&mut reader, Endian::Little);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x0403_0201);
    }
    #[test]
    fn test_varint_decode_u64_eight_bytes() {
        let bytes = &[0b1111_1010, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = create_reader(bytes);
        let result = varint_decode_u64(&mut reader, Endian::Little);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x0807_0605_0403_0201);
    }
    #[test]
    fn test_varint_decode_u64_invalid_discriminant() {
        let bytes = &[0b1111_1111];
        let mut reader = create_reader(bytes);
        let result = varint_decode_u64(&mut reader, Endian::Little);
        assert!(result.is_err());
    }
    #[test]
    fn test_varint_decode_u64_unexpected_end() {
        let bytes = &[0b1111_1010, 0x01, 0x02];
        let mut reader = create_reader(bytes);
        let result = varint_decode_u64(&mut reader, Endian::Little);
        assert!(matches!(result, Err(DecodeError::UnexpectedEnd { .. })));
    }
}
