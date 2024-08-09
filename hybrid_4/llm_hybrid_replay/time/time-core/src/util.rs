//! Utility functions.
/// Returns if the provided year is a leap year in the proleptic Gregorian calendar. Uses
/// [astronomical year numbering](https://en.wikipedia.org/wiki/Astronomical_year_numbering).
///
/// ```rust
/// # use time::util::is_leap_year;
/// assert!(!is_leap_year(1900));
/// assert!(is_leap_year(2000));
/// assert!(is_leap_year(2004));
/// assert!(!is_leap_year(2005));
/// assert!(!is_leap_year(2100));
/// ```
pub const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 25 != 0 || year % 16 == 0)
}
/// Get the number of calendar days in a given year.
///
/// The returned value will always be either 365 or 366.
///
/// ```rust
/// # use time::util::days_in_year;
/// assert_eq!(days_in_year(1900), 365);
/// assert_eq!(days_in_year(2000), 366);
/// assert_eq!(days_in_year(2004), 366);
/// assert_eq!(days_in_year(2005), 365);
/// assert_eq!(days_in_year(2100), 365);
/// ```
pub const fn days_in_year(year: i32) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}
/// Get the number of weeks in the ISO year.
///
/// The returned value will always be either 52 or 53.
///
/// ```rust
/// # use time::util::weeks_in_year;
/// assert_eq!(weeks_in_year(2019), 52);
/// assert_eq!(weeks_in_year(2020), 53);
/// ```
pub const fn weeks_in_year(year: i32) -> u8 {
    match year.rem_euclid(400) {
        4 | 9 | 15 | 20 | 26 | 32 | 37 | 43 | 48 | 54 | 60 | 65 | 71 | 76 | 82 | 88 | 93
        | 99 | 105 | 111 | 116 | 122 | 128 | 133 | 139 | 144 | 150 | 156 | 161 | 167
        | 172 | 178 | 184 | 189 | 195 | 201 | 207 | 212 | 218 | 224 | 229 | 235 | 240
        | 246 | 252 | 257 | 263 | 268 | 274 | 280 | 285 | 291 | 296 | 303 | 308 | 314
        | 320 | 325 | 331 | 336 | 342 | 348 | 353 | 359 | 364 | 370 | 376 | 381 | 387
        | 392 | 398 => 53,
        _ => 52,
    }
}
#[cfg(test)]
mod tests_rug_321 {
    use super::*;
    #[test]
    fn test_is_leap_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        debug_assert_eq!(crate ::util::is_leap_year(p0), false);
        p0 = rug_fuzz_1;
        debug_assert_eq!(crate ::util::is_leap_year(p0), true);
        p0 = rug_fuzz_2;
        debug_assert_eq!(crate ::util::is_leap_year(p0), true);
        p0 = rug_fuzz_3;
        debug_assert_eq!(crate ::util::is_leap_year(p0), false);
        p0 = rug_fuzz_4;
        debug_assert_eq!(crate ::util::is_leap_year(p0), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_322 {
    use super::*;
    #[test]
    fn test_days_in_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i32 = rug_fuzz_0;
        debug_assert_eq!(crate ::util::days_in_year(p0), 365);
        let p0: i32 = rug_fuzz_1;
        debug_assert_eq!(crate ::util::days_in_year(p0), 366);
        let p0: i32 = rug_fuzz_2;
        debug_assert_eq!(crate ::util::days_in_year(p0), 366);
        let p0: i32 = rug_fuzz_3;
        debug_assert_eq!(crate ::util::days_in_year(p0), 365);
        let p0: i32 = rug_fuzz_4;
        debug_assert_eq!(crate ::util::days_in_year(p0), 365);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_323 {
    use super::*;
    #[test]
    fn test_weeks_in_year() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i32 = rug_fuzz_0;
        debug_assert_eq!(crate ::util::weeks_in_year(p0), 52);
             }
}
}
}    }
}
