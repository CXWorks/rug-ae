use std::io::{self, ErrorKind};
use std::iter;
use std::num::ParseIntError;
use std::str::{self, FromStr};
use super::rule::TransitionRule;
use super::timezone::{LeapSecond, LocalTimeType, TimeZone, Transition};
use super::Error;
pub(super) fn parse(bytes: &[u8]) -> Result<TimeZone, Error> {
    let mut cursor = Cursor::new(bytes);
    let state = State::new(&mut cursor, true)?;
    let (state, footer) = match state.header.version {
        Version::V1 => {
            match cursor.is_empty() {
                true => (state, None),
                false => {
                    return Err(
                        Error::InvalidTzFile(
                            "remaining data after end of TZif v1 data block",
                        ),
                    );
                }
            }
        }
        Version::V2 | Version::V3 => {
            let state = State::new(&mut cursor, false)?;
            (state, Some(cursor.remaining()))
        }
    };
    let mut transitions = Vec::with_capacity(state.header.transition_count);
    for (arr_time, &local_time_type_index) in state
        .transition_times
        .chunks_exact(state.time_size)
        .zip(state.transition_types)
    {
        let unix_leap_time = state
            .parse_time(&arr_time[0..state.time_size], state.header.version)?;
        let local_time_type_index = local_time_type_index as usize;
        transitions.push(Transition::new(unix_leap_time, local_time_type_index));
    }
    let mut local_time_types = Vec::with_capacity(state.header.type_count);
    for arr in state.local_time_types.chunks_exact(6) {
        let ut_offset = read_be_i32(&arr[..4])?;
        let is_dst = match arr[4] {
            0 => false,
            1 => true,
            _ => return Err(Error::InvalidTzFile("invalid DST indicator")),
        };
        let char_index = arr[5] as usize;
        if char_index >= state.header.char_count {
            return Err(Error::InvalidTzFile("invalid time zone name char index"));
        }
        let position = match state.names[char_index..].iter().position(|&c| c == b'\0') {
            Some(position) => position,
            None => return Err(Error::InvalidTzFile("invalid time zone name char index")),
        };
        let name = &state.names[char_index..char_index + position];
        let name = if !name.is_empty() { Some(name) } else { None };
        local_time_types.push(LocalTimeType::new(ut_offset, is_dst, name)?);
    }
    let mut leap_seconds = Vec::with_capacity(state.header.leap_count);
    for arr in state.leap_seconds.chunks_exact(state.time_size + 4) {
        let unix_leap_time = state
            .parse_time(&arr[0..state.time_size], state.header.version)?;
        let correction = read_be_i32(&arr[state.time_size..state.time_size + 4])?;
        leap_seconds.push(LeapSecond::new(unix_leap_time, correction));
    }
    let std_walls_iter = state.std_walls.iter().copied().chain(iter::repeat(0));
    let ut_locals_iter = state.ut_locals.iter().copied().chain(iter::repeat(0));
    if std_walls_iter
        .zip(ut_locals_iter)
        .take(state.header.type_count)
        .any(|pair| pair == (0, 1))
    {
        return Err(
            Error::InvalidTzFile(
                "invalid couple of standard/wall and UT/local indicators",
            ),
        );
    }
    let extra_rule = match footer {
        Some(footer) => {
            let footer = str::from_utf8(footer)?;
            if !(footer.starts_with('\n') && footer.ends_with('\n')) {
                return Err(Error::InvalidTzFile("invalid footer"));
            }
            let tz_string = footer.trim_matches(|c: char| c.is_ascii_whitespace());
            if tz_string.starts_with(':') || tz_string.contains('\0') {
                return Err(Error::InvalidTzFile("invalid footer"));
            }
            match tz_string.is_empty() {
                true => None,
                false => {
                    Some(
                        TransitionRule::from_tz_string(
                            tz_string.as_bytes(),
                            state.header.version == Version::V3,
                        )?,
                    )
                }
            }
        }
        None => None,
    };
    TimeZone::new(transitions, local_time_types, leap_seconds, extra_rule)
}
/// TZif data blocks
struct State<'a> {
    header: Header,
    /// Time size in bytes
    time_size: usize,
    /// Transition times data block
    transition_times: &'a [u8],
    /// Transition types data block
    transition_types: &'a [u8],
    /// Local time types data block
    local_time_types: &'a [u8],
    /// Time zone names data block
    names: &'a [u8],
    /// Leap seconds data block
    leap_seconds: &'a [u8],
    /// UT/local indicators data block
    std_walls: &'a [u8],
    /// Standard/wall indicators data block
    ut_locals: &'a [u8],
}
impl<'a> State<'a> {
    /// Read TZif data blocks
    fn new(cursor: &mut Cursor<'a>, first: bool) -> Result<Self, Error> {
        let header = Header::new(cursor)?;
        let time_size = match first {
            true => 4,
            false => 8,
        };
        Ok(Self {
            time_size,
            transition_times: cursor.read_exact(header.transition_count * time_size)?,
            transition_types: cursor.read_exact(header.transition_count)?,
            local_time_types: cursor.read_exact(header.type_count * 6)?,
            names: cursor.read_exact(header.char_count)?,
            leap_seconds: cursor.read_exact(header.leap_count * (time_size + 4))?,
            std_walls: cursor.read_exact(header.std_wall_count)?,
            ut_locals: cursor.read_exact(header.ut_local_count)?,
            header,
        })
    }
    /// Parse time values
    fn parse_time(&self, arr: &[u8], version: Version) -> Result<i64, Error> {
        match version {
            Version::V1 => Ok(read_be_i32(&arr[..4])?.into()),
            Version::V2 | Version::V3 => read_be_i64(arr),
        }
    }
}
/// TZif header
#[derive(Debug)]
struct Header {
    /// TZif version
    version: Version,
    /// Number of UT/local indicators
    ut_local_count: usize,
    /// Number of standard/wall indicators
    std_wall_count: usize,
    /// Number of leap-second records
    leap_count: usize,
    /// Number of transition times
    transition_count: usize,
    /// Number of local time type records
    type_count: usize,
    /// Number of time zone names bytes
    char_count: usize,
}
impl Header {
    fn new(cursor: &mut Cursor) -> Result<Self, Error> {
        let magic = cursor.read_exact(4)?;
        if magic != *b"TZif" {
            return Err(Error::InvalidTzFile("invalid magic number"));
        }
        let version = match cursor.read_exact(1)? {
            [0x00] => Version::V1,
            [0x32] => Version::V2,
            [0x33] => Version::V3,
            _ => return Err(Error::UnsupportedTzFile("unsupported TZif version")),
        };
        cursor.read_exact(15)?;
        let ut_local_count = cursor.read_be_u32()?;
        let std_wall_count = cursor.read_be_u32()?;
        let leap_count = cursor.read_be_u32()?;
        let transition_count = cursor.read_be_u32()?;
        let type_count = cursor.read_be_u32()?;
        let char_count = cursor.read_be_u32()?;
        if !(type_count != 0 && char_count != 0
            && (ut_local_count == 0 || ut_local_count == type_count)
            && (std_wall_count == 0 || std_wall_count == type_count))
        {
            return Err(Error::InvalidTzFile("invalid header"));
        }
        Ok(Self {
            version,
            ut_local_count: ut_local_count as usize,
            std_wall_count: std_wall_count as usize,
            leap_count: leap_count as usize,
            transition_count: transition_count as usize,
            type_count: type_count as usize,
            char_count: char_count as usize,
        })
    }
}
/// A `Cursor` contains a slice of a buffer and a read count.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Cursor<'a> {
    /// Slice representing the remaining data to be read
    remaining: &'a [u8],
    /// Number of already read bytes
    read_count: usize,
}
impl<'a> Cursor<'a> {
    /// Construct a new `Cursor` from remaining data
    pub(crate) const fn new(remaining: &'a [u8]) -> Self {
        Self { remaining, read_count: 0 }
    }
    pub(crate) fn peek(&self) -> Option<&u8> {
        self.remaining().first()
    }
    /// Returns remaining data
    pub(crate) const fn remaining(&self) -> &'a [u8] {
        self.remaining
    }
    /// Returns `true` if data is remaining
    pub(crate) const fn is_empty(&self) -> bool {
        self.remaining.is_empty()
    }
    pub(crate) fn read_be_u32(&mut self) -> Result<u32, Error> {
        let mut buf = [0; 4];
        buf.copy_from_slice(self.read_exact(4)?);
        Ok(u32::from_be_bytes(buf))
    }
    /// Read exactly `count` bytes, reducing remaining data and incrementing read count
    pub(crate) fn read_exact(&mut self, count: usize) -> Result<&'a [u8], io::Error> {
        match (self.remaining.get(..count), self.remaining.get(count..)) {
            (Some(result), Some(remaining)) => {
                self.remaining = remaining;
                self.read_count += count;
                Ok(result)
            }
            _ => Err(io::Error::from(ErrorKind::UnexpectedEof)),
        }
    }
    /// Read bytes and compare them to the provided tag
    pub(crate) fn read_tag(&mut self, tag: &[u8]) -> Result<(), io::Error> {
        if self.read_exact(tag.len())? == tag {
            Ok(())
        } else {
            Err(io::Error::from(ErrorKind::InvalidData))
        }
    }
    /// Read bytes if the remaining data is prefixed by the provided tag
    pub(crate) fn read_optional_tag(&mut self, tag: &[u8]) -> Result<bool, io::Error> {
        if self.remaining.starts_with(tag) {
            self.read_exact(tag.len())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    /// Read bytes as long as the provided predicate is true
    pub(crate) fn read_while<F: Fn(&u8) -> bool>(
        &mut self,
        f: F,
    ) -> Result<&'a [u8], io::Error> {
        match self.remaining.iter().position(|x| !f(x)) {
            None => self.read_exact(self.remaining.len()),
            Some(position) => self.read_exact(position),
        }
    }
    pub(crate) fn read_int<T: FromStr<Err = ParseIntError>>(
        &mut self,
    ) -> Result<T, Error> {
        let bytes = self.read_while(u8::is_ascii_digit)?;
        Ok(str::from_utf8(bytes)?.parse()?)
    }
    /// Read bytes until the provided predicate is true
    pub(crate) fn read_until<F: Fn(&u8) -> bool>(
        &mut self,
        f: F,
    ) -> Result<&'a [u8], io::Error> {
        match self.remaining.iter().position(f) {
            None => self.read_exact(self.remaining.len()),
            Some(position) => self.read_exact(position),
        }
    }
}
pub(crate) fn read_be_i32(bytes: &[u8]) -> Result<i32, Error> {
    if bytes.len() != 4 {
        return Err(Error::InvalidSlice("too short for i32"));
    }
    let mut buf = [0; 4];
    buf.copy_from_slice(bytes);
    Ok(i32::from_be_bytes(buf))
}
pub(crate) fn read_be_i64(bytes: &[u8]) -> Result<i64, Error> {
    if bytes.len() != 8 {
        return Err(Error::InvalidSlice("too short for i64"));
    }
    let mut buf = [0; 8];
    buf.copy_from_slice(bytes);
    Ok(i64::from_be_bytes(buf))
}
/// TZif version
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Version {
    /// Version 1
    V1,
    /// Version 2
    V2,
    /// Version 3
    V3,
}
#[cfg(test)]
mod tests_llm_16_531 {
    use super::*;
    use crate::*;
    use std::io::{self, ErrorKind};
    #[test]
    fn test_read_optional_tag_success() -> io::Result<()> {
        let data = &[0x01, 0x02, 0x03, 0x01, 0x02];
        let mut cursor = Cursor::new(data);
        assert_eq!(cursor.read_optional_tag(& [0x01, 0x02]) ?, true);
        assert_eq!(cursor.remaining(), & [0x03, 0x01, 0x02]);
        assert_eq!(cursor.read_count, 2);
        Ok(())
    }
    #[test]
    fn test_read_optional_tag_no_match() -> io::Result<()> {
        let data = &[0x01, 0x02, 0x03, 0x01, 0x02];
        let mut cursor = Cursor::new(data);
        assert_eq!(cursor.read_optional_tag(& [0x03, 0x01]) ?, false);
        assert_eq!(cursor.remaining(), data);
        assert_eq!(cursor.read_count, 0);
        Ok(())
    }
    #[test]
    fn test_read_optional_tag_eof() -> io::Result<()> {
        let data = &[0x01, 0x02];
        let mut cursor = Cursor::new(data);
        assert!(
            matches!(cursor.read_optional_tag(& [0x01, 0x02, 0x03]).unwrap_err().kind(),
            ErrorKind::UnexpectedEof)
        );
        assert_eq!(cursor.remaining(), data);
        assert_eq!(cursor.read_count, 0);
        Ok(())
    }
    #[test]
    fn test_read_optional_tag_empty() -> io::Result<()> {
        let data = &[];
        let mut cursor = Cursor::new(data);
        assert_eq!(cursor.read_optional_tag(& [0x01, 0x02]) ?, false);
        assert!(cursor.remaining().is_empty());
        assert_eq!(cursor.read_count, 0);
        Ok(())
    }
    #[test]
    fn test_read_optional_tag_empty_tag() -> io::Result<()> {
        let data = &[0x01, 0x02, 0x03, 0x01, 0x02];
        let mut cursor = Cursor::new(data);
        assert_eq!(cursor.read_optional_tag(& []) ?, true);
        assert_eq!(cursor.remaining(), data);
        assert_eq!(cursor.read_count, 0);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_533 {
    use super::*;
    use crate::*;
    use std::io::{self, ErrorKind};
    #[test]
    fn read_until_with_predicate() -> Result<(), io::Error> {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut cursor = Cursor::new(&data);
        let result = cursor.read_until(|&x| x == 5)?;
        let expected = &data[..5];
        assert_eq!(result, expected, "Should read until 5 is encountered");
        assert_eq!(cursor.remaining(), & data[5..]);
        let result = cursor.read_until(|&x| x == 20)?;
        let expected = &data[5..];
        assert_eq!(result, expected, "Should read until the end as 20 is not found");
        assert!(cursor.is_empty());
        Ok(())
    }
    #[test]
    fn read_until_with_no_predicate_match() {
        let data = [0, 1, 2, 3, 4];
        let mut cursor = Cursor::new(&data);
        match cursor.read_until(|&x| x == 10) {
            Ok(result) => {
                let expected = &data[..];
                assert_eq!(result, expected, "Should read until the end if no matches");
            }
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
        assert!(cursor.is_empty());
    }
    #[test]
    fn read_until_when_already_at_end() {
        let data = [0, 1, 2, 3, 4];
        let mut cursor = Cursor::new(&data);
        let _ = cursor.read_until(|_| false).unwrap();
        match cursor.read_until(|&x| x == 3) {
            Ok(result) => {
                assert!(result.is_empty(), "No data should be read if cursor is at end");
            }
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }
    #[test]
    fn read_until_with_empty_data() {
        let data = [];
        let mut cursor = Cursor::new(&data);
        match cursor.read_until(|&x| x == 0) {
            Ok(result) => {
                assert!(result.is_empty(), "Should return empty slice with empty data");
            }
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }
    #[test]
    fn read_until_with_error() {
        let data = [0, 1, 2];
        let mut cursor = Cursor::new(&data);
        match cursor.read_until(|&x| x == 5) {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                assert_eq!(
                    e.kind(), ErrorKind::UnexpectedEof,
                    "Should return UnexpectedEof error"
                );
            }
        }
    }
}
#[cfg(test)]
mod tests_llm_16_534 {
    use super::*;
    use crate::*;
    use std::io::{self, ErrorKind};
    #[test]
    fn test_read_while() -> Result<(), io::Error> {
        let data = &[b'1', b'1', b'a', b'1', b'1'];
        let mut cursor = Cursor::new(data);
        let predicate = |x: &u8| *x == b'1';
        let result = cursor.read_while(predicate)?;
        assert_eq!(result, & [b'1', b'1']);
        assert_eq!(cursor.read_count, 2);
        let next_chunk = cursor.read_while(predicate)?;
        assert_eq!(next_chunk, & []);
        assert_eq!(cursor.read_count, 2);
        let remaining_data = cursor.remaining();
        assert_eq!(remaining_data, & [b'a', b'1', b'1']);
        let final_chunk = cursor.read_while(predicate)?;
        assert_eq!(final_chunk, & []);
        assert_eq!(cursor.remaining(), & [b'a', b'1', b'1']);
        Ok(())
    }
    #[test]
    fn test_read_while_with_no_predicate_match() -> Result<(), io::Error> {
        let data = &[b'a', b'b', b'c'];
        let mut cursor = Cursor::new(data);
        let predicate = |x: &u8| *x == b'1';
        let result = cursor.read_while(predicate)?;
        assert_eq!(result, & []);
        assert_eq!(cursor.remaining(), & [b'a', b'b', b'c']);
        assert_eq!(cursor.read_count, 0);
        Ok(())
    }
    #[test]
    fn test_read_while_until_eof() -> Result<(), io::Error> {
        let data = &[b'2', b'2', b'2'];
        let mut cursor = Cursor::new(data);
        let predicate = |x: &u8| *x == b'2';
        let result = cursor.read_while(predicate)?;
        assert_eq!(result, & [b'2', b'2', b'2']);
        assert_eq!(cursor.remaining(), & []);
        assert_eq!(cursor.read_count, 3);
        Ok(())
    }
    #[test]
    fn test_read_while_unexpected_eof() {
        let data = &[b'1'];
        let mut cursor = Cursor::new(data);
        cursor.read_count = 2;
        let predicate = |x: &u8| *x == b'1';
        let result = cursor.read_while(predicate);
        assert!(matches!(result, Err(ref e) if e.kind() == ErrorKind::UnexpectedEof));
    }
}
#[cfg(test)]
mod tests_llm_16_536 {
    use super::*;
    use crate::*;
    use crate::offset::local::tz_info::parser::{Cursor, Error, Header, Version};
    use std::io;
    #[test]
    fn test_header_new_valid_magic() {
        let _rug_st_tests_llm_16_536_rrrruuuugggg_test_header_new_valid_magic = 0;
        let rug_fuzz_0 = b"TZif2\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00";
        let data = rug_fuzz_0;
        let mut cursor = Cursor::new(data);
        let header_result = Header::new(&mut cursor);
        debug_assert!(header_result.is_ok());
        let header = header_result.unwrap();
        debug_assert_eq!(header.version, Version::V2);
        debug_assert_eq!(header.ut_local_count, 1);
        debug_assert_eq!(header.std_wall_count, 1);
        debug_assert_eq!(header.leap_count, 1);
        debug_assert_eq!(header.transition_count, 1);
        debug_assert_eq!(header.type_count, 1);
        debug_assert_eq!(header.char_count, 0);
        let _rug_ed_tests_llm_16_536_rrrruuuugggg_test_header_new_valid_magic = 0;
    }
    #[test]
    fn test_header_new_invalid_magic() {
        let _rug_st_tests_llm_16_536_rrrruuuugggg_test_header_new_invalid_magic = 0;
        let rug_fuzz_0 = b"BAD!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let data = rug_fuzz_0;
        let mut cursor = Cursor::new(data);
        let header_result = Header::new(&mut cursor);
        debug_assert!(matches!(header_result, Err(Error::InvalidTzFile(_))));
        let _rug_ed_tests_llm_16_536_rrrruuuugggg_test_header_new_invalid_magic = 0;
    }
    #[test]
    fn test_header_new_unsupported_version() {
        let _rug_st_tests_llm_16_536_rrrruuuugggg_test_header_new_unsupported_version = 0;
        let rug_fuzz_0 = b"TZifX\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let data = rug_fuzz_0;
        let mut cursor = Cursor::new(data);
        let header_result = Header::new(&mut cursor);
        debug_assert!(matches!(header_result, Err(Error::UnsupportedTzFile(_))));
        let _rug_ed_tests_llm_16_536_rrrruuuugggg_test_header_new_unsupported_version = 0;
    }
    #[test]
    fn test_header_new_invalid_header_structure() {
        let _rug_st_tests_llm_16_536_rrrruuuugggg_test_header_new_invalid_header_structure = 0;
        let rug_fuzz_0 = b"TZif2\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\x00";
        let data = rug_fuzz_0;
        let mut cursor = Cursor::new(data);
        let header_result = Header::new(&mut cursor);
        debug_assert!(matches!(header_result, Err(Error::InvalidTzFile(_))));
        let _rug_ed_tests_llm_16_536_rrrruuuugggg_test_header_new_invalid_header_structure = 0;
    }
    #[test]
    fn test_header_new_incomplete_data() {
        let _rug_st_tests_llm_16_536_rrrruuuugggg_test_header_new_incomplete_data = 0;
        let rug_fuzz_0 = b"TZif";
        let data = rug_fuzz_0;
        let mut cursor = Cursor::new(data);
        let header_result = Header::new(&mut cursor);
        debug_assert!(matches!(header_result, Err(Error::InvalidTzFile(_))));
        let _rug_ed_tests_llm_16_536_rrrruuuugggg_test_header_new_incomplete_data = 0;
    }
    #[test]
    fn test_header_new_empty_data() {
        let _rug_st_tests_llm_16_536_rrrruuuugggg_test_header_new_empty_data = 0;
        let rug_fuzz_0 = b"";
        let data = rug_fuzz_0;
        let mut cursor = Cursor::new(data);
        let header_result = Header::new(&mut cursor);
        debug_assert!(matches!(header_result, Err(Error::InvalidTzFile(_))));
        let _rug_ed_tests_llm_16_536_rrrruuuugggg_test_header_new_empty_data = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_538_llm_16_538 {
    use super::*;
    use crate::*;
    use crate::offset::local::tz_info::parser::{Error, State, Version, Header};
    use std::convert::TryInto;
    fn read_be_i32(input: &[u8]) -> Result<i32, Error> {
        input
            .get(..4)
            .and_then(|arr| arr.try_into().ok())
            .map(i32::from_be_bytes)
            .ok_or(Error::InvalidTzFile("invalid data"))
    }
    fn read_be_i64(input: &[u8]) -> Result<i64, Error> {
        input
            .get(..8)
            .and_then(|arr| arr.try_into().ok())
            .map(i64::from_be_bytes)
            .ok_or(Error::InvalidTzFile("invalid data"))
    }
    #[test]
    fn test_parse_time_v1() {
        let header = Header {
            version: Version::V1,
            ut_local_count: 0,
            std_wall_count: 0,
            leap_count: 0,
            transition_count: 1,
            type_count: 1,
            char_count: 0,
        };
        let transition_times: [u8; 4] = 0_i32.to_be_bytes();
        let state = State {
            header,
            time_size: 4,
            transition_times: &transition_times,
            transition_types: &[0],
            local_time_types: &[0; 6],
            names: &[],
            leap_seconds: &[],
            std_walls: &[],
            ut_locals: &[],
        };
        assert_eq!(state.parse_time(& transition_times, Version::V1).unwrap(), 0);
    }
    #[test]
    fn test_parse_time_v2() {
        let header = Header {
            version: Version::V2,
            ut_local_count: 0,
            std_wall_count: 0,
            leap_count: 0,
            transition_count: 1,
            type_count: 1,
            char_count: 0,
        };
        let transition_times: [u8; 8] = 0_i64.to_be_bytes();
        let state = State {
            header,
            time_size: 8,
            transition_times: &transition_times,
            transition_types: &[0],
            local_time_types: &[0; 6],
            names: &[],
            leap_seconds: &[],
            std_walls: &[],
            ut_locals: &[],
        };
        assert_eq!(state.parse_time(& transition_times, Version::V2).unwrap(), 0);
    }
    #[test]
    fn test_parse_time_v3() {
        let header = Header {
            version: Version::V3,
            ut_local_count: 0,
            std_wall_count: 0,
            leap_count: 0,
            transition_count: 1,
            type_count: 1,
            char_count: 0,
        };
        let transition_times: [u8; 8] = 0_i64.to_be_bytes();
        let state = State {
            header,
            time_size: 8,
            transition_times: &transition_times,
            transition_types: &[0],
            local_time_types: &[0; 6],
            names: &[],
            leap_seconds: &[],
            std_walls: &[],
            ut_locals: &[],
        };
        assert_eq!(state.parse_time(& transition_times, Version::V3).unwrap(), 0);
    }
    #[test]
    fn test_parse_time_v1_error() {
        let header = Header {
            version: Version::V1,
            ut_local_count: 0,
            std_wall_count: 0,
            leap_count: 0,
            transition_count: 0,
            type_count: 0,
            char_count: 0,
        };
        let transition_times: [u8; 4] = [0; 4];
        let state = State {
            header,
            time_size: 4,
            transition_times: &[],
            transition_types: &[],
            local_time_types: &[],
            names: &[],
            leap_seconds: &[],
            std_walls: &[],
            ut_locals: &[],
        };
        let result = state.parse_time(&transition_times, Version::V1);
        assert!(matches!(result, Err(Error::InvalidTzFile(_))));
    }
    #[test]
    fn test_parse_time_v2_v3_error() {
        let header = Header {
            version: Version::V2,
            ut_local_count: 0,
            std_wall_count: 0,
            leap_count: 0,
            transition_count: 0,
            type_count: 0,
            char_count: 0,
        };
        let transition_times: [u8; 8] = [0; 8];
        let state = State {
            header,
            time_size: 8,
            transition_times: &[],
            transition_types: &[],
            local_time_types: &[],
            names: &[],
            leap_seconds: &[],
            std_walls: &[],
            ut_locals: &[],
        };
        let result_v2 = state.parse_time(&transition_times, Version::V2);
        let result_v3 = state.parse_time(&transition_times, Version::V3);
        assert!(matches!(result_v2, Err(Error::InvalidTzFile(_))));
        assert!(matches!(result_v3, Err(Error::InvalidTzFile(_))));
    }
}
#[cfg(test)]
mod tests_rug_145 {
    use super::*;
    #[test]
    fn test_read_be_i32_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        debug_assert_eq!(
            crate ::offset::local::tz_info::parser::read_be_i32(p0).unwrap(), 1
        );
             }
});    }
    #[test]
    fn test_read_be_i32_invalid_too_short() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &[u8] = &[rug_fuzz_0, rug_fuzz_1];
        debug_assert!(
            matches!(crate ::offset::local::tz_info::parser::read_be_i32(p0),
            Err(Error::InvalidSlice("too short for i32")))
        );
             }
});    }
    #[test]
    fn test_read_be_i32_invalid_too_long() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        debug_assert!(
            matches!(crate ::offset::local::tz_info::parser::read_be_i32(p0),
            Err(Error::InvalidSlice("too short for i32")))
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_146 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &[u8] = &[
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let result = crate::offset::local::tz_info::parser::read_be_i64(p0);
        debug_assert_eq!(result.unwrap(), 72623859790382856);
             }
});    }
}
#[cfg(test)]
mod tests_rug_147 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let buffer: &[u8] = &[];
        let mut p0 = Cursor::new(buffer);
        let p1: bool = rug_fuzz_0;
        let result = State::new(&mut p0, p1);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_148 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let p0 = data;
        let _cursor = crate::offset::local::tz_info::parser::Cursor::new(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_152 {
    use super::*;
    #[test]
    fn test_read_be_u32() -> Result<(), Error> {
        let data: Vec<u8> = vec![0x00, 0x00, 0x01, 0x23];
        let mut p0 = Cursor::new(&data);
        assert_eq!(p0.read_be_u32() ?, 291);
        Ok(())
    }
}
