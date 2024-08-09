use crate::{
    config::Config,
    de::{read::Reader, BorrowDecode, BorrowDecoder, Decode, Decoder, DecoderImpl},
    enc::{write::Writer, Encode, Encoder, EncoderImpl},
    error::{DecodeError, EncodeError},
    impl_borrow_decode,
};
use core::time::Duration;
use std::{
    collections::{HashMap, HashSet},
    ffi::{CStr, CString},
    hash::Hash, io::Read,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    path::{Path, PathBuf},
    sync::{Mutex, RwLock},
    time::SystemTime,
};
/// Decode type `D` from the given reader with the given `Config`. The reader can be any type that implements `std::io::Read`, e.g. `std::fs::File`.
///
/// See the [config] module for more information about config options.
///
/// [config]: config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn decode_from_std_read<D: Decode, C: Config, R: std::io::Read>(
    src: &mut R,
    config: C,
) -> Result<D, DecodeError> {
    let reader = IoReader::new(src);
    let mut decoder = DecoderImpl::<_, C>::new(reader, config);
    D::decode(&mut decoder)
}
pub(crate) struct IoReader<R> {
    reader: R,
}
impl<R> IoReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}
impl<R> Reader for IoReader<R>
where
    R: std::io::Read,
{
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        self.reader
            .read_exact(bytes)
            .map_err(|inner| DecodeError::Io {
                inner,
                additional: bytes.len(),
            })
    }
}
impl<R> Reader for std::io::BufReader<R>
where
    R: std::io::Read,
{
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        self.read_exact(bytes)
            .map_err(|inner| DecodeError::Io {
                inner,
                additional: bytes.len(),
            })
    }
    #[inline]
    fn peek_read(&mut self, n: usize) -> Option<&[u8]> {
        self.buffer().get(..n)
    }
    #[inline]
    fn consume(&mut self, n: usize) {
        <Self as std::io::BufRead>::consume(self, n);
    }
}
/// Encode the given value into any type that implements `std::io::Write`, e.g. `std::fs::File`, with the given `Config`.
/// See the [config] module for more information.
/// Returns the amount of bytes written.
///
/// [config]: config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn encode_into_std_write<E: Encode, C: Config, W: std::io::Write>(
    val: E,
    dst: &mut W,
    config: C,
) -> Result<usize, EncodeError> {
    let writer = IoWriter::new(dst);
    let mut encoder = EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}
pub(crate) struct IoWriter<'a, W: std::io::Write> {
    writer: &'a mut W,
    bytes_written: usize,
}
impl<'a, W: std::io::Write> IoWriter<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self { writer, bytes_written: 0 }
    }
    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }
}
impl<'storage, W: std::io::Write> Writer for IoWriter<'storage, W> {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.writer
            .write_all(bytes)
            .map_err(|inner| EncodeError::Io {
                inner,
                index: self.bytes_written,
            })?;
        self.bytes_written += bytes.len();
        Ok(())
    }
}
impl<'a> Encode for &'a CStr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.to_bytes().encode(encoder)
    }
}
impl Encode for CString {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}
impl Decode for CString {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = std::vec::Vec::decode(decoder)?;
        CString::new(vec)
            .map_err(|inner| DecodeError::CStringNulError {
                position: inner.nul_position(),
            })
    }
}
impl_borrow_decode!(CString);
impl<T> Encode for Mutex<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let t = self
            .lock()
            .map_err(|_| EncodeError::LockFailed {
                type_name: core::any::type_name::<Mutex<T>>(),
            })?;
        t.encode(encoder)
    }
}
impl<T> Decode for Mutex<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Mutex::new(t))
    }
}
impl<'de, T> BorrowDecode<'de> for Mutex<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Mutex::new(t))
    }
}
impl<T> Encode for RwLock<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let t = self
            .read()
            .map_err(|_| EncodeError::LockFailed {
                type_name: core::any::type_name::<RwLock<T>>(),
            })?;
        t.encode(encoder)
    }
}
impl<T> Decode for RwLock<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(RwLock::new(t))
    }
}
impl<'de, T> BorrowDecode<'de> for RwLock<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(RwLock::new(t))
    }
}
impl Encode for SystemTime {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let duration = self
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| {
                EncodeError::InvalidSystemTime {
                    inner: e,
                    time: std::boxed::Box::new(*self),
                }
            })?;
        duration.encode(encoder)
    }
}
impl Decode for SystemTime {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let duration = Duration::decode(decoder)?;
        match SystemTime::UNIX_EPOCH.checked_add(duration) {
            Some(t) => Ok(t),
            None => {
                Err(DecodeError::InvalidSystemTime {
                    duration,
                })
            }
        }
    }
}
impl_borrow_decode!(SystemTime);
impl Encode for &'_ Path {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self.to_str() {
            Some(str) => str.encode(encoder),
            None => Err(EncodeError::InvalidPathCharacters),
        }
    }
}
impl<'de> BorrowDecode<'de> for &'de Path {
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let str = <&'de str>::borrow_decode(decoder)?;
        Ok(Path::new(str))
    }
}
impl Encode for PathBuf {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_path().encode(encoder)
    }
}
impl Decode for PathBuf {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let string = std::string::String::decode(decoder)?;
        Ok(string.into())
    }
}
impl_borrow_decode!(PathBuf);
impl Encode for IpAddr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            IpAddr::V4(v4) => {
                0u32.encode(encoder)?;
                v4.encode(encoder)
            }
            IpAddr::V6(v6) => {
                1u32.encode(encoder)?;
                v6.encode(encoder)
            }
        }
    }
}
impl Decode for IpAddr {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        match u32::decode(decoder)? {
            0 => Ok(IpAddr::V4(Ipv4Addr::decode(decoder)?)),
            1 => Ok(IpAddr::V6(Ipv6Addr::decode(decoder)?)),
            found => {
                Err(DecodeError::UnexpectedVariant {
                    allowed: &crate::error::AllowedEnumVariants::Range {
                        min: 0,
                        max: 1,
                    },
                    found,
                    type_name: core::any::type_name::<IpAddr>(),
                })
            }
        }
    }
}
impl_borrow_decode!(IpAddr);
impl Encode for Ipv4Addr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&self.octets())
    }
}
impl Decode for Ipv4Addr {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut buff = [0u8; 4];
        decoder.reader().read(&mut buff)?;
        Ok(Self::from(buff))
    }
}
impl_borrow_decode!(Ipv4Addr);
impl Encode for Ipv6Addr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&self.octets())
    }
}
impl Decode for Ipv6Addr {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut buff = [0u8; 16];
        decoder.reader().read(&mut buff)?;
        Ok(Self::from(buff))
    }
}
impl_borrow_decode!(Ipv6Addr);
impl Encode for SocketAddr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            SocketAddr::V4(v4) => {
                0u32.encode(encoder)?;
                v4.encode(encoder)
            }
            SocketAddr::V6(v6) => {
                1u32.encode(encoder)?;
                v6.encode(encoder)
            }
        }
    }
}
impl Decode for SocketAddr {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        match u32::decode(decoder)? {
            0 => Ok(SocketAddr::V4(SocketAddrV4::decode(decoder)?)),
            1 => Ok(SocketAddr::V6(SocketAddrV6::decode(decoder)?)),
            found => {
                Err(DecodeError::UnexpectedVariant {
                    allowed: &crate::error::AllowedEnumVariants::Range {
                        min: 0,
                        max: 1,
                    },
                    found,
                    type_name: core::any::type_name::<SocketAddr>(),
                })
            }
        }
    }
}
impl_borrow_decode!(SocketAddr);
impl Encode for SocketAddrV4 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.ip().encode(encoder)?;
        self.port().encode(encoder)
    }
}
impl Decode for SocketAddrV4 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let ip = Ipv4Addr::decode(decoder)?;
        let port = u16::decode(decoder)?;
        Ok(Self::new(ip, port))
    }
}
impl_borrow_decode!(SocketAddrV4);
impl Encode for SocketAddrV6 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.ip().encode(encoder)?;
        self.port().encode(encoder)
    }
}
impl Decode for SocketAddrV6 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let ip = Ipv6Addr::decode(decoder)?;
        let port = u16::decode(decoder)?;
        Ok(Self::new(ip, port, 0, 0))
    }
}
impl_borrow_decode!(SocketAddrV6);
impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RefCellAlreadyBorrowed { inner, .. } => Some(inner),
            Self::Io { inner, .. } => Some(inner),
            Self::InvalidSystemTime { inner, .. } => Some(inner),
            _ => None,
        }
    }
}
impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Utf8 { inner } => Some(inner),
            _ => None,
        }
    }
}
impl<K, V, S> Encode for HashMap<K, V, S>
where
    K: Encode,
    V: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for (k, v) in self.iter() {
            Encode::encode(k, encoder)?;
            Encode::encode(v, encoder)?;
        }
        Ok(())
    }
}
impl<K, V, S> Decode for HashMap<K, V, S>
where
    K: Decode + Eq + std::hash::Hash,
    V: Decode,
    S: std::hash::BuildHasher + Default,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<(K, V)>(len)?;
        let hash_builder: S = Default::default();
        let mut map = HashMap::with_capacity_and_hasher(len, hash_builder);
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());
            let k = K::decode(decoder)?;
            let v = V::decode(decoder)?;
            map.insert(k, v);
        }
        Ok(map)
    }
}
impl<'de, K, V, S> BorrowDecode<'de> for HashMap<K, V, S>
where
    K: BorrowDecode<'de> + Eq + std::hash::Hash,
    V: BorrowDecode<'de>,
    S: std::hash::BuildHasher + Default,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<(K, V)>(len)?;
        let hash_builder: S = Default::default();
        let mut map = HashMap::with_capacity_and_hasher(len, hash_builder);
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());
            let k = K::borrow_decode(decoder)?;
            let v = V::borrow_decode(decoder)?;
            map.insert(k, v);
        }
        Ok(map)
    }
}
impl<T, S> Decode for HashSet<T, S>
where
    T: Decode + Eq + Hash,
    S: std::hash::BuildHasher + Default,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;
        let hash_builder: S = Default::default();
        let mut map: HashSet<T, S> = HashSet::with_capacity_and_hasher(
            len,
            hash_builder,
        );
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            let key = T::decode(decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}
impl<'de, T, S> BorrowDecode<'de> for HashSet<T, S>
where
    T: BorrowDecode<'de> + Eq + Hash,
    S: std::hash::BuildHasher + Default,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;
        let mut map = HashSet::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            let key = T::borrow_decode(decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}
impl<T, S> Encode for HashSet<T, S>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_28 {
    use super::*;
    use crate::*;
    use crate::de::read::Reader;
    use crate::error::DecodeError;
    use std::io;
    struct MockReader {
        pub data: Vec<u8>,
        pub read_position: usize,
    }
    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, read_position: 0 }
        }
    }
    impl io::Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.read_position >= self.data.len() {
                return Ok(0);
            }
            let remaining = self.data.len() - self.read_position;
            let to_read = buf.len().min(remaining);
            buf[..to_read]
                .copy_from_slice(
                    &self.data[self.read_position..self.read_position + to_read],
                );
            self.read_position += to_read;
            Ok(to_read)
        }
    }
    fn create_reader(data: Vec<u8>) -> IoReader<MockReader> {
        IoReader::new(MockReader::new(data))
    }
    #[test]
    fn test_read_exact_amount() {
        let mut reader = create_reader(vec![1, 2, 3, 4, 5]);
        let mut buffer = [0u8; 5];
        assert!(reader.read(& mut buffer).is_ok());
        assert_eq!(buffer, [1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_read_less_than_available() {
        let mut reader = create_reader(vec![1, 2, 3, 4, 5]);
        let mut buffer = [0u8; 3];
        assert!(reader.read(& mut buffer).is_ok());
        assert_eq!(buffer, [1, 2, 3]);
    }
    #[test]
    fn test_read_more_than_available() {
        let mut reader = create_reader(vec![1, 2, 3]);
        let mut buffer = [0u8; 5];
        assert!(matches!(reader.read(& mut buffer), Err(DecodeError::Io { .. })));
    }
    #[test]
    fn test_read_no_data() {
        let mut reader = create_reader(vec![]);
        let mut buffer = [0u8; 1];
        assert!(matches!(reader.read(& mut buffer), Err(DecodeError::Io { .. })));
    }
}
#[cfg(test)]
mod tests_llm_16_29_llm_16_29 {
    use super::*;
    use crate::*;
    use crate::error::EncodeError;
    use std::io::{self, Write};
    struct MockWriter {
        expected: Vec<u8>,
        written: Vec<u8>,
        fail_on_write: bool,
    }
    impl MockWriter {
        fn new(expected: Vec<u8>, fail_on_write: bool) -> Self {
            Self {
                expected,
                written: Vec::new(),
                fail_on_write,
            }
        }
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.fail_on_write {
                Err(io::Error::new(io::ErrorKind::Other, "Mock writer error"))
            } else {
                self.written.extend_from_slice(buf);
                Ok(buf.len())
            }
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_write_success() {
        let _rug_st_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_write_success = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = false;
        let data = vec![rug_fuzz_0, 2, 3, 4, 5];
        let expected = data.clone();
        let mut mock_writer = MockWriter::new(expected, rug_fuzz_1);
        let mut io_writer = IoWriter::new(&mut mock_writer);
        let result = io_writer.write(&data);
        debug_assert!(result.is_ok());
        drop(io_writer);
        debug_assert_eq!(mock_writer.written, data);
        let _rug_ed_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_write_success = 0;
    }
    #[test]
    fn test_write_failure() {
        let _rug_st_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_write_failure = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = true;
        let data = vec![rug_fuzz_0, 2, 3, 4, 5];
        let expected = data.clone();
        let mut mock_writer = MockWriter::new(expected, rug_fuzz_1);
        let mut io_writer = IoWriter::new(&mut mock_writer);
        let result = io_writer.write(&data);
        debug_assert!(result.is_err());
        let bytes_written = io_writer.bytes_written();
        drop(io_writer);
        if let Err(EncodeError::Io { inner, index }) = result {
            debug_assert_eq!(inner.kind(), io::ErrorKind::Other);
            debug_assert_eq!(index, bytes_written);
        } else {
            panic!("Expected EncodeError::Io error");
        }
        let _rug_ed_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_write_failure = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_338 {
    use crate::de::read::Reader;
    use std::io::BufReader;
    use std::io::Cursor;
    #[test]
    fn test_consume() {
        let _rug_st_tests_llm_16_338_rrrruuuugggg_test_consume = 0;
        let rug_fuzz_0 = b"Hello, world!";
        let rug_fuzz_1 = 0u8;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0u8;
        let data = rug_fuzz_0;
        let cursor = Cursor::new(data);
        let mut buf_reader = BufReader::new(cursor);
        let mut buffer = [rug_fuzz_1; 5];
        buf_reader.read(&mut buffer).unwrap();
        debug_assert_eq!(& buffer, b"Hello");
        buf_reader.consume(rug_fuzz_2);
        let mut buffer = [rug_fuzz_3; 5];
        buf_reader.read(&mut buffer).unwrap();
        debug_assert_eq!(& buffer, b"world");
        let _rug_ed_tests_llm_16_338_rrrruuuugggg_test_consume = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_342 {
    use super::*;
    use crate::*;
    use crate::enc::{Encoder, EncoderImpl};
    use crate::enc::write::SizeWriter;
    use crate::config::{BigEndian, Config, Configuration, Fixint, NoLimit};
    use std::path::Path;
    use crate::error::EncodeError;
    #[test]
    fn encode_valid_path() {
        let _rug_st_tests_llm_16_342_rrrruuuugggg_encode_valid_path = 0;
        let rug_fuzz_0 = "test/path";
        let rug_fuzz_1 = 0;
        let path = Path::new(rug_fuzz_0);
        let cfg = Configuration::<BigEndian, Fixint, NoLimit>::default()
            .with_big_endian();
        let mut writer = SizeWriter::default();
        let mut encoder = EncoderImpl::new(&mut writer, cfg);
        let res = path.encode(&mut encoder);
        debug_assert!(res.is_ok());
        debug_assert!(writer.bytes_written > rug_fuzz_1);
        let _rug_ed_tests_llm_16_342_rrrruuuugggg_encode_valid_path = 0;
    }
    #[test]
    fn encode_invalid_path() {
        let _rug_st_tests_llm_16_342_rrrruuuugggg_encode_invalid_path = 0;
        let rug_fuzz_0 = b"\xFF\xFE\xFD";
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            let path = Path::new(std::ffi::OsStr::from_bytes(rug_fuzz_0));
            let cfg = Configuration::<BigEndian, Fixint, NoLimit>::default()
                .with_big_endian();
            let mut writer = SizeWriter::default();
            let mut encoder = EncoderImpl::new(&mut writer, cfg);
            let res = path.encode(&mut encoder);
            debug_assert!(matches!(res, Err(EncodeError::InvalidPathCharacters)));
        }
        let _rug_ed_tests_llm_16_342_rrrruuuugggg_encode_invalid_path = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_352 {
    use super::*;
    use crate::*;
    use crate::enc::{Encoder, EncoderImpl};
    use crate::enc::write::SizeWriter;
    use std::path::PathBuf;
    #[test]
    fn test_encode_path_buf_with_big_endian() {
        let _rug_st_tests_llm_16_352_rrrruuuugggg_test_encode_path_buf_with_big_endian = 0;
        let rug_fuzz_0 = "test_path";
        let rug_fuzz_1 = 0;
        let path_buf = PathBuf::from(rug_fuzz_0);
        let config = crate::config::standard().with_big_endian();
        let mut size_writer = SizeWriter::default();
        let mut encoder = EncoderImpl::new(&mut size_writer, config);
        let result = path_buf.encode(&mut encoder);
        debug_assert!(result.is_ok());
        debug_assert!(size_writer.bytes_written > rug_fuzz_1);
        let _rug_ed_tests_llm_16_352_rrrruuuugggg_test_encode_path_buf_with_big_endian = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_356_llm_16_356 {
    use super::*;
    use crate::*;
    use crate::error::{DecodeError, IntegerType, AllowedEnumVariants};
    use std::str::{self, Utf8Error};
    use std::error::Error;
    #[test]
    fn test_decode_error_source() {
        let utf8_error = str::from_utf8(&[0, 195, 128]).unwrap_err();
        let decode_error = DecodeError::Utf8 {
            inner: utf8_error,
        };
        assert!(
            matches!(decode_error.source().unwrap().downcast_ref::< Utf8Error > (),
            Some(_))
        );
        let decode_error_unexpected_end = DecodeError::UnexpectedEnd {
            additional: 1,
        };
        assert!(decode_error_unexpected_end.source().is_none());
        let decode_error_limit_exceeded = DecodeError::LimitExceeded;
        assert!(decode_error_limit_exceeded.source().is_none());
        let decode_error_invalid_integer_type = DecodeError::InvalidIntegerType {
            expected: IntegerType::U8,
            found: IntegerType::U16,
        };
        assert!(decode_error_invalid_integer_type.source().is_none());
        let decode_error_non_zero_type_is_zero = DecodeError::NonZeroTypeIsZero {
            non_zero_type: IntegerType::U8,
        };
        assert!(decode_error_non_zero_type_is_zero.source().is_none());
        let decode_error_unexpected_variant = DecodeError::UnexpectedVariant {
            type_name: "EnumType",
            allowed: &AllowedEnumVariants::Allowed(&[1, 2, 3]),
            found: 4,
        };
        assert!(decode_error_unexpected_variant.source().is_none());
        let decode_error_invalid_char_encoding = DecodeError::InvalidCharEncoding([
            0,
            0,
            0,
            0,
        ]);
        assert!(decode_error_invalid_char_encoding.source().is_none());
        let decode_error_invalid_boolean_value = DecodeError::InvalidBooleanValue(2);
        assert!(decode_error_invalid_boolean_value.source().is_none());
        let decode_error_array_length_mismatch = DecodeError::ArrayLengthMismatch {
            required: 4,
            found: 2,
        };
        assert!(decode_error_array_length_mismatch.source().is_none());
        let decode_error_outside_usize_range = DecodeError::OutsideUsizeRange(2);
        assert!(decode_error_outside_usize_range.source().is_none());
        let decode_error_empty_enum = DecodeError::EmptyEnum {
            type_name: "EnumType",
        };
        assert!(decode_error_empty_enum.source().is_none());
        let decode_error_invalid_duration = DecodeError::InvalidDuration {
            secs: 5,
            nanos: 1000000001,
        };
        assert!(decode_error_invalid_duration.source().is_none());
        #[cfg(feature = "std")]
        {
            use std::io;
            let io_error = io::Error::new(io::ErrorKind::Other, "io error");
            let decode_error_io = DecodeError::Io {
                inner: io_error,
                additional: 1,
            };
            assert!(
                matches!(decode_error_io.source().unwrap().downcast_ref::< io::Error >
                (), Some(_))
            );
        }
        #[cfg(feature = "alloc")]
        {
            let decode_error_other_string = DecodeError::OtherString(
                alloc::string::String::from("error"),
            );
            assert!(decode_error_other_string.source().is_none());
        }
        #[cfg(feature = "serde")]
        {
            let serde_error = serde::de::value::Error::custom("serde error");
            let decode_error_serde = DecodeError::Serde(serde_error);
            assert!(
                matches!(decode_error_serde.source().unwrap().downcast_ref::<
                serde::de::value::Error > (), Some(_))
            );
        }
    }
}
#[cfg(test)]
mod tests_llm_16_359 {
    use super::*;
    use crate::*;
    use std::io::Write;
    struct MockWriter {
        pub data: Vec<u8>,
    }
    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn test_bytes_written() {
        let _rug_st_tests_llm_16_359_rrrruuuugggg_test_bytes_written = 0;
        let rug_fuzz_0 = b"hello";
        let mut mock_writer = MockWriter { data: Vec::new() };
        let mut io_writer = IoWriter::new(&mut mock_writer);
        debug_assert_eq!(
            io_writer.bytes_written(), 0, "Initially, bytes_written should be 0"
        );
        let data_to_write = rug_fuzz_0;
        io_writer.write(data_to_write).unwrap();
        debug_assert_eq!(
            io_writer.bytes_written(), data_to_write.len(),
            "After writing, bytes_written should be equal to the length of data written"
        );
        let _rug_ed_tests_llm_16_359_rrrruuuugggg_test_bytes_written = 0;
    }
}
