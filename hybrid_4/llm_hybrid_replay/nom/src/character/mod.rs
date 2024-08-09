//! Character specific parsers and combinators
//!
//! Functions recognizing specific characters
#[cfg(test)]
mod tests;
pub mod complete;
pub mod streaming;
/// Tests if byte is ASCII alphabetic: A-Z, a-z
///
/// # Example
///
/// ```
/// # use nom::character::is_alphabetic;
/// assert_eq!(is_alphabetic(b'9'), false);
/// assert_eq!(is_alphabetic(b'a'), true);
/// ```
#[inline]
pub fn is_alphabetic(chr: u8) -> bool {
    matches!(chr, 0x41..= 0x5A | 0x61..= 0x7A)
}
/// Tests if byte is ASCII digit: 0-9
///
/// # Example
///
/// ```
/// # use nom::character::is_digit;
/// assert_eq!(is_digit(b'a'), false);
/// assert_eq!(is_digit(b'9'), true);
/// ```
#[inline]
pub fn is_digit(chr: u8) -> bool {
    matches!(chr, 0x30..= 0x39)
}
/// Tests if byte is ASCII hex digit: 0-9, A-F, a-f
///
/// # Example
///
/// ```
/// # use nom::character::is_hex_digit;
/// assert_eq!(is_hex_digit(b'a'), true);
/// assert_eq!(is_hex_digit(b'9'), true);
/// assert_eq!(is_hex_digit(b'A'), true);
/// assert_eq!(is_hex_digit(b'x'), false);
/// ```
#[inline]
pub fn is_hex_digit(chr: u8) -> bool {
    matches!(chr, 0x30..= 0x39 | 0x41..= 0x46 | 0x61..= 0x66)
}
/// Tests if byte is ASCII octal digit: 0-7
///
/// # Example
///
/// ```
/// # use nom::character::is_oct_digit;
/// assert_eq!(is_oct_digit(b'a'), false);
/// assert_eq!(is_oct_digit(b'9'), false);
/// assert_eq!(is_oct_digit(b'6'), true);
/// ```
#[inline]
pub fn is_oct_digit(chr: u8) -> bool {
    matches!(chr, 0x30..= 0x37)
}
/// Tests if byte is ASCII alphanumeric: A-Z, a-z, 0-9
///
/// # Example
///
/// ```
/// # use nom::character::is_alphanumeric;
/// assert_eq!(is_alphanumeric(b'-'), false);
/// assert_eq!(is_alphanumeric(b'a'), true);
/// assert_eq!(is_alphanumeric(b'9'), true);
/// assert_eq!(is_alphanumeric(b'A'), true);
/// ```
#[inline]
pub fn is_alphanumeric(chr: u8) -> bool {
    is_alphabetic(chr) || is_digit(chr)
}
/// Tests if byte is ASCII space or tab
///
/// # Example
///
/// ```
/// # use nom::character::is_space;
/// assert_eq!(is_space(b'\n'), false);
/// assert_eq!(is_space(b'\r'), false);
/// assert_eq!(is_space(b' '), true);
/// assert_eq!(is_space(b'\t'), true);
/// ```
#[inline]
pub fn is_space(chr: u8) -> bool {
    chr == b' ' || chr == b'\t'
}
/// Tests if byte is ASCII newline: \n
///
/// # Example
///
/// ```
/// # use nom::character::is_newline;
/// assert_eq!(is_newline(b'\n'), true);
/// assert_eq!(is_newline(b'\r'), false);
/// assert_eq!(is_newline(b' '), false);
/// assert_eq!(is_newline(b'\t'), false);
/// ```
#[inline]
pub fn is_newline(chr: u8) -> bool {
    chr == b'\n'
}
#[cfg(test)]
mod tests_rug_110 {
    use crate::character::is_alphabetic;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        debug_assert!(is_alphabetic(p0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_111 {
    use super::*;
    #[test]
    fn test_is_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        debug_assert_eq!(crate ::character::is_digit(p0), true);
        let p0: u8 = rug_fuzz_1;
        debug_assert_eq!(crate ::character::is_digit(p0), true);
        let p0: u8 = rug_fuzz_2;
        debug_assert_eq!(crate ::character::is_digit(p0), false);
        let p0: u8 = rug_fuzz_3;
        debug_assert_eq!(crate ::character::is_digit(p0), false);
        let p0: u8 = rug_fuzz_4;
        debug_assert_eq!(crate ::character::is_digit(p0), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_112 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), true);
        p0 = rug_fuzz_1;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), true);
        p0 = rug_fuzz_2;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), true);
        p0 = rug_fuzz_3;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), true);
        p0 = rug_fuzz_4;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), true);
        p0 = rug_fuzz_5;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), true);
        p0 = rug_fuzz_6;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), false);
        p0 = rug_fuzz_7;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), false);
        p0 = rug_fuzz_8;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), false);
        p0 = rug_fuzz_9;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), false);
        p0 = rug_fuzz_10;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), false);
        p0 = rug_fuzz_11;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), false);
        p0 = rug_fuzz_12;
        debug_assert_eq!(crate ::character::is_hex_digit(p0), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_113 {
    use super::*;
    #[test]
    fn test_is_oct_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        debug_assert_eq!(crate ::character::is_oct_digit(p0), true);
             }
}
}
}    }
    #[test]
    fn test_is_oct_digit_non_octal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        debug_assert_eq!(crate ::character::is_oct_digit(p0), false);
             }
}
}
}    }
    #[test]
    fn test_is_oct_digit_boundaries() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        debug_assert_eq!(crate ::character::is_oct_digit(p0), true);
        let p0: u8 = rug_fuzz_1;
        debug_assert_eq!(crate ::character::is_oct_digit(p0), true);
             }
}
}
}    }
    #[test]
    fn test_is_oct_digit_ascii_char() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        debug_assert_eq!(crate ::character::is_oct_digit(p0), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_114 {
    use crate::character::is_alphanumeric;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        debug_assert_eq!(is_alphanumeric(p0), true);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_115 {
    use crate::character::is_space;
    #[test]
    fn test_is_space() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        debug_assert_eq!(is_space(p0), true);
        p0 = rug_fuzz_1;
        debug_assert_eq!(is_space(p0), true);
        p0 = rug_fuzz_2;
        debug_assert_eq!(is_space(p0), false);
        p0 = rug_fuzz_3;
        debug_assert_eq!(is_space(p0), false);
        p0 = rug_fuzz_4;
        debug_assert_eq!(is_space(p0), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_116 {
    use super::*;
    #[test]
    fn test_is_newline() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u8 = rug_fuzz_0;
        debug_assert_eq!(crate ::character::is_newline(p0), true);
        p0 = rug_fuzz_1;
        debug_assert_eq!(crate ::character::is_newline(p0), false);
        p0 = rug_fuzz_2;
        debug_assert_eq!(crate ::character::is_newline(p0), false);
        p0 = rug_fuzz_3;
        debug_assert_eq!(crate ::character::is_newline(p0), false);
             }
}
}
}    }
}
