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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = vec![rug_fuzz_0, 2, 3, 4, 5];
        let expected = data.clone();
        let mut mock_writer = MockWriter::new(expected, rug_fuzz_1);
        let mut io_writer = IoWriter::new(&mut mock_writer);
        let result = io_writer.write(&data);
        debug_assert!(result.is_ok());
        drop(io_writer);
        debug_assert_eq!(mock_writer.written, data);
             }
}
}
}    }
    #[test]
    fn test_write_failure() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_338 {
    use crate::de::read::Reader;
    use std::io::BufReader;
    use std::io::Cursor;
    #[test]
    fn test_consume() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <([u8; 13], u8, usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
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
             }
}
}
}    }
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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let path = Path::new(rug_fuzz_0);
        let cfg = Configuration::<BigEndian, Fixint, NoLimit>::default()
            .with_big_endian();
        let mut writer = SizeWriter::default();
        let mut encoder = EncoderImpl::new(&mut writer, cfg);
        let res = path.encode(&mut encoder);
        debug_assert!(res.is_ok());
        debug_assert!(writer.bytes_written > rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn encode_invalid_path() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext)) = <([u8; 3]) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
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
             }
}
}
}    }
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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let path_buf = PathBuf::from(rug_fuzz_0);
        let config = crate::config::standard().with_big_endian();
        let mut size_writer = SizeWriter::default();
        let mut encoder = EncoderImpl::new(&mut size_writer, config);
        let result = path_buf.encode(&mut encoder);
        debug_assert!(result.is_ok());
        debug_assert!(size_writer.bytes_written > rug_fuzz_1);
             }
}
}
}    }
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
#[cfg(test)]
mod tests_rug_39 {
    use super::*;
    use crate::config::{self, Configuration, BigEndian, Fixint, Limit};
    use crate::features::impl_std::{self, decode_from_std_read};
    use crate::error::DecodeError;
    use std::io::{self, Read};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_39_rrrruuuugggg_test_rug = 0;
        let stdin = io::stdin();
        let mut p0: io::StdinLock = stdin.lock();
        let mut p1: Configuration<BigEndian, Fixint, Limit<1024>> = Configuration::default();
        let _result: Result<(), DecodeError> = decode_from_std_read(&mut p0, p1);
        let _rug_ed_tests_rug_39_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    use crate::config::Configuration;
    use std::collections::BinaryHeap;
    use std::io::{self, Write};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: BinaryHeap<i32> = BinaryHeap::new();
        p0.push(rug_fuzz_0);
        p0.push(rug_fuzz_1);
        p0.push(rug_fuzz_2);
        let stdout = io::stdout();
        let mut p1 = stdout.lock();
        let p2 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let _ = crate::features::impl_std::encode_into_std_write(p0, &mut p1, p2)
            .unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_41 {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn test_new() {
        let _rug_st_tests_rug_41_rrrruuuugggg_test_new = 0;
        let data = vec![0u8; 10];
        let cursor = Cursor::new(data);
        let p0: Cursor<Vec<u8>> = cursor;
        let reader = crate::features::impl_std::IoReader::<Cursor<Vec<u8>>>::new(p0);
        let _rug_ed_tests_rug_41_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_43 {
    use super::*;
    use crate::de::read::Reader;
    use std::io::{BufReader, Cursor};
    #[test]
    fn test_peek_read() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 17], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let data = rug_fuzz_0;
        let cursor = Cursor::new(data);
        let mut p0 = BufReader::new(cursor);
        let p1: usize = rug_fuzz_1;
        debug_assert_eq!(p0.peek_read(p1), Some(& data[..p1]));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_44 {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn test_new() {
        const N: usize = 128;
        let data: [u8; N] = [0u8; N];
        let mut p0 = Cursor::new(data);
        let io_writer = crate::features::impl_std::IoWriter::new(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_45 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::{VecWriter, impl_std};
    use std::ffi::CStr;
    use std::os::raw::c_char;
    #[test]
    fn test_encode_cstr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 14], usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let c_string_content: &[u8] = rug_fuzz_0;
        let c_string_content_as_c_char = c_string_content.as_ptr() as *const c_char;
        let p0 = unsafe { CStr::from_ptr(c_string_content_as_c_char) };
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::<VecWriter, _>::new(v10, config);
        <&'static std::ffi::CStr>::encode(&p0, &mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_46 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::ffi::CString;
    #[test]
    fn test_encode_cstring_with_encoderimpl() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut cstring = CString::new(rug_fuzz_0).unwrap();
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut encoder = EncoderImpl::new(vec_writer, config);
        <CString>::encode(&mut cstring, &mut encoder).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_47 {
    use super::*;
    use crate::de::{self, DecoderImpl, Decode};
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0 = SliceReader::new(&data);
        let mut config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(&mut p0, config);
        let _result = <std::ffi::CString>::decode(&mut p0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_48 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, Encoder, write::Writer};
    use crate::config::Configuration;
    use crate::features::{VecWriter, impl_std};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = std::sync::Mutex::new(rug_fuzz_0);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        let result = <std::sync::Mutex<i32> as Encode>::encode(&p0, &mut p1);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_50 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::{self, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
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
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let result = <std::sync::Mutex<u32>>::borrow_decode(&mut p0);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_51 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = std::sync::RwLock::new(rug_fuzz_0);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut encoder = EncoderImpl::<VecWriter, _>::new(writer, config);
        <std::sync::RwLock<i32> as Encode>::encode(&p0, &mut encoder).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_52 {
    use super::*;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::sync::RwLock;
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
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let result = <RwLock<u32>>::decode(&mut p0);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_53 {
    use super::*;
    use crate::de;
    use crate::de::{BorrowDecoder, DecoderImpl};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::de::read::SliceReader;
    use std::sync::RwLock;
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
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        debug_assert!(< RwLock < u32 > > ::borrow_decode(& mut p0).is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_54 {
    use super::*;
    use crate::enc::{EncoderImpl, write::Writer, Encode};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::time::{SystemTime, UNIX_EPOCH};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = SystemTime::now();
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_0);
        let mut p1 = EncoderImpl::new(writer, config);
        <SystemTime as Encode>::encode(&p0, &mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_56 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, BorrowDecode, BorrowDecoder};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::error::DecodeError;
    use std::path::Path;
    #[test]
    fn test_borrow_decode() -> Result<(), DecodeError> {
        let data = [1u8, 2, 3, 4, 5];
        let reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <&Path>::borrow_decode(&mut decoder)?;
        assert!(result.to_str().is_some());
        Ok(())
    }
}
#[cfg(test)]
mod tests_rug_57 {
    use super::*;
    use crate::Decode;
    use crate::de::{self, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_decode_pathbuf() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let mut v9 = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(&mut v11, v9);
        let _result = <std::path::PathBuf>::decode(&mut p0).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_58 {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    use crate::enc::{self, Encoder};
    use crate::enc::write::Writer;
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode_ipaddr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = IpAddr::V4(
            Ipv4Addr::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
        );
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_4);
        let mut p1 = enc::EncoderImpl::new(vec_writer, config);
        <IpAddr as enc::Encode>::encode(&p0, &mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_59 {
    use crate::de::{Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::Configuration;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    #[test]
    fn test_decode_ipaddr() {

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
        let result = <IpAddr as crate::Decode>::decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_60 {
    use super::*;
    use crate::{
        Encode, enc::{EncoderImpl, write::Writer},
        config::{self, Configuration},
        features::VecWriter,
    };
    use std::net::Ipv4Addr;
    #[test]
    fn test_encode_ipv4addr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = Ipv4Addr::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let writer = VecWriter::with_capacity(rug_fuzz_4);
        let mut p1 = EncoderImpl::new(writer, config);
        debug_assert!(p0.encode(& mut p1).is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_61 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{BigEndian, Fixint, Limit, Configuration};
    #[test]
    fn test_rug() {

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
        let result = <std::net::Ipv4Addr>::decode(&mut decoder);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), std::net::Ipv4Addr::new(1, 2, 3, 4));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_62 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u16, u16, u16, u16, u16, u16, u16, u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = std::net::Ipv6Addr::new(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        );
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_8);
        let mut p1 = EncoderImpl::new(&mut writer, config);
        <std::net::Ipv6Addr as Encode>::encode(&p0, &mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_63 {
    use super::*;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    use std::net::Ipv6Addr;
    #[test]
    fn test_decode_ipv6_addr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [
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
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        let ipv6_addr = <Ipv6Addr as Decode>::decode(&mut decoder);
        debug_assert!(ipv6_addr.is_ok());
        debug_assert_eq!(ipv6_addr.unwrap(), Ipv6Addr::from(data));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_65 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{Decode, DecoderImpl};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
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
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        let result = <std::net::SocketAddr as Decode>::decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_66 {
    use super::*;
    use crate::{
        Encode, enc::{EncoderImpl, write::Writer, Encoder},
        config::{Configuration, BigEndian, Fixint, Limit},
        features::VecWriter,
    };
    use std::net::{Ipv4Addr, SocketAddrV4};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = SocketAddrV4::new(
            Ipv4Addr::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            rug_fuzz_4,
        );
        let mut p1_config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p1_writer = VecWriter::with_capacity(rug_fuzz_5);
        let mut p1 = EncoderImpl::new(p1_writer, p1_config);
        <std::net::SocketAddrV4>::encode(&p0, &mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_67 {
    use super::*;
    use crate::{Decode, de::{self, DecoderImpl, read::SliceReader}};
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut r = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(r, config);
        debug_assert!(< std::net::SocketAddrV4 as Decode > ::decode(& mut p0).is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_68 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::net::{Ipv6Addr, SocketAddrV6};
    #[test]
    fn test_encode() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u16, u16, u16, u16, u16, u16, u16, u16, u16, u32, u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = SocketAddrV6::new(
            Ipv6Addr::new(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
                rug_fuzz_4,
                rug_fuzz_5,
                rug_fuzz_6,
                rug_fuzz_7,
            ),
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
        );
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_11);
        let mut p1 = EncoderImpl::new(writer, config);
        <std::net::SocketAddrV6 as Encode>::encode(&p0, &mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_69 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
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
        let reader = SliceReader::new(&data);
        let config = crate::config::Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _result = <std::net::SocketAddrV6 as Decode>::decode(&mut p0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_70 {
    use super::*;
    use std::error::Error;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_70_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Sample error message";
        let p0 = crate::error::EncodeError::Other(rug_fuzz_0);
        debug_assert!(
            < crate ::error::EncodeError as std::error::Error > ::source(& p0).is_none()
        );
        let _rug_ed_tests_rug_70_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_71 {
    use super::*;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    use crate::enc::{self, Encode, EncoderImpl};
    use crate::features::VecWriter;
    use std::collections::{hash_map::DefaultHasher, HashMap};
    use std::hash::BuildHasherDefault;
    #[test]
    fn test_encode() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, &str, i32, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        type K = i32;
        type V = String;
        type S = BuildHasherDefault<DefaultHasher>;
        let mut p0: HashMap<K, V, S> = HashMap::default();
        p0.insert(rug_fuzz_0, rug_fuzz_1.to_string());
        p0.insert(rug_fuzz_2, rug_fuzz_3.to_string());
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_4);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        p0.encode(&mut p1).unwrap();
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_74 {
    use super::*;
    use crate::Decode;
    use crate::de::{DecoderImpl, read::SliceReader};
    use crate::de;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::collections::HashSet;
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
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let result: Result<HashSet<u8>, _> = HashSet::<u8>::decode(&mut p0);
        debug_assert!(result.is_ok());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_76 {
    use super::*;
    use crate::enc::{self, Encode, Encoder, EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::{VecWriter, impl_std};
    use std::collections::{HashSet, hash_map::DefaultHasher};
    use std::hash::BuildHasherDefault;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        type T = i32;
        type S = BuildHasherDefault<DefaultHasher>;
        let mut p0: HashSet<T, S> = HashSet::with_hasher(S::default());
        p0.insert(rug_fuzz_0);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        <HashSet<T, S> as Encode>::encode(&p0, &mut p1).unwrap();
             }
}
}
}    }
}
