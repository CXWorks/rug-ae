/// Fused multiply-add. Computes `(self * a) + b` with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
///
/// Using `mul_add` can be more performant than an unfused multiply-add if
/// the target architecture has a dedicated `fma` CPU instruction.
///
/// Note that `A` and `B` are `Self` by default, but this is not mandatory.
///
/// # Example
///
/// ```
/// use std::f32;
///
/// let m = 10.0_f32;
/// let x = 4.0_f32;
/// let b = 60.0_f32;
///
/// // 100.0
/// let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();
///
/// assert!(abs_difference <= 100.0 * f32::EPSILON);
/// ```
pub trait MulAdd<A = Self, B = Self> {
    /// The resulting type after applying the fused multiply-add.
    type Output;
    /// Performs the fused multiply-add operation.
    fn mul_add(self, a: A, b: B) -> Self::Output;
}
/// The fused multiply-add assignment operation.
pub trait MulAddAssign<A = Self, B = Self> {
    /// Performs the fused multiply-add operation.
    fn mul_add_assign(&mut self, a: A, b: B);
}
#[cfg(any(feature = "std", feature = "libm"))]
impl MulAdd<f32, f32> for f32 {
    type Output = Self;
    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        <Self as crate::Float>::mul_add(self, a, b)
    }
}
#[cfg(any(feature = "std", feature = "libm"))]
impl MulAdd<f64, f64> for f64 {
    type Output = Self;
    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        <Self as crate::Float>::mul_add(self, a, b)
    }
}
macro_rules! mul_add_impl {
    ($trait_name:ident for $($t:ty)*) => {
        $(impl $trait_name for $t { type Output = Self; #[inline] fn mul_add(self, a :
        Self, b : Self) -> Self::Output { (self * a) + b } })*
    };
}
mul_add_impl!(MulAdd for isize i8 i16 i32 i64 i128);
mul_add_impl!(MulAdd for usize u8 u16 u32 u64 u128);
#[cfg(any(feature = "std", feature = "libm"))]
impl MulAddAssign<f32, f32> for f32 {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = <Self as crate::Float>::mul_add(*self, a, b);
    }
}
#[cfg(any(feature = "std", feature = "libm"))]
impl MulAddAssign<f64, f64> for f64 {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = <Self as crate::Float>::mul_add(*self, a, b);
    }
}
macro_rules! mul_add_assign_impl {
    ($trait_name:ident for $($t:ty)*) => {
        $(impl $trait_name for $t { #[inline] fn mul_add_assign(& mut self, a : Self, b :
        Self) { * self = (* self * a) + b } })*
    };
}
mul_add_assign_impl!(MulAddAssign for isize i8 i16 i32 i64 i128);
mul_add_assign_impl!(MulAddAssign for usize u8 u16 u32 u64 u128);
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mul_add_integer() {
        macro_rules! test_mul_add {
            ($($t:ident)+) => {
                $({ let m : $t = 2; let x : $t = 3; let b : $t = 4;
                assert_eq!(MulAdd::mul_add(m, x, b), (m * x + b)); })+
            };
        }
        test_mul_add!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
    }
    #[test]
    #[cfg(feature = "std")]
    fn mul_add_float() {
        macro_rules! test_mul_add {
            ($($t:ident)+) => {
                $({ use core::$t; let m : $t = 12.0; let x : $t = 3.4; let b : $t = 5.6;
                let abs_difference = (MulAdd::mul_add(m, x, b) - (m * x + b)).abs();
                assert!(abs_difference <= 46.4 * $t ::EPSILON); })+
            };
        }
        test_mul_add!(f32 f64);
    }
}
#[cfg(test)]
mod tests_llm_16_424_llm_16_424 {
    use crate::ops::mul_add::MulAdd;
    #[test]
    fn mul_add_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: f32 = rug_fuzz_0;
        let y: f32 = rug_fuzz_1;
        let z: f32 = rug_fuzz_2;
        let result = <f32 as MulAdd>::mul_add(x, y, z);
        debug_assert_eq!(result, x * y + z);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_425_llm_16_425 {
    use crate::MulAddAssign;
    #[test]
    fn mul_add_assign_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = rug_fuzz_0;
        value.mul_add_assign(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value, 10.0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_592_llm_16_592 {
    use crate::MulAdd;
    #[test]
    fn test_mul_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f64 = rug_fuzz_0;
        let mul: f64 = rug_fuzz_1;
        let add: f64 = rug_fuzz_2;
        let result = <f64 as MulAdd>::mul_add(value, mul, add);
        let expected = value * mul + add;
        debug_assert_eq!(result, expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_680_llm_16_680 {
    use super::*;
    use crate::*;
    use crate::MulAdd;
    #[test]
    fn test_mul_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i128 as MulAdd > ::mul_add(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 10
        );
        debug_assert_eq!(
            < i128 as MulAdd > ::mul_add(- rug_fuzz_3, - rug_fuzz_4, - rug_fuzz_5), - 1
        );
        debug_assert_eq!(
            < i128 as MulAdd > ::mul_add(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), 0
        );
        debug_assert_eq!(
            < i128 as MulAdd > ::mul_add(i128::MAX, rug_fuzz_9, rug_fuzz_10), i128::MIN
        );
        debug_assert_eq!(
            < i128 as MulAdd > ::mul_add(i128::MAX, rug_fuzz_11, rug_fuzz_12), - 2
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_681_llm_16_681 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mul_add_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: i128 = rug_fuzz_0;
        MulAddAssign::mul_add_assign(&mut value, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value, 10);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_790_llm_16_790 {
    use crate::ops::mul_add::MulAdd;
    #[test]
    fn mul_add_i16() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i16 as MulAdd > ::mul_add(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 52
        );
        debug_assert_eq!(
            < i16 as MulAdd > ::mul_add(- rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), - 48
        );
        debug_assert_eq!(
            < i16 as MulAdd > ::mul_add(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), 2
        );
        debug_assert_eq!(
            < i16 as MulAdd > ::mul_add(- rug_fuzz_9, - rug_fuzz_10, rug_fuzz_11), 52
        );
        debug_assert_eq!(
            < i16 as MulAdd > ::mul_add(i16::MAX, rug_fuzz_12, rug_fuzz_13), i16::MAX
        );
        debug_assert_eq!(
            < i16 as MulAdd > ::mul_add(i16::MIN, rug_fuzz_14, - rug_fuzz_15), i16::MIN
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_791_llm_16_791 {
    use crate::ops::mul_add::MulAddAssign;
    #[test]
    fn test_mul_add_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: i16 = rug_fuzz_0;
        MulAddAssign::mul_add_assign(&mut value, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value, 10);
        let mut value: i16 = -rug_fuzz_3;
        MulAddAssign::mul_add_assign(&mut value, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(value, 2);
        let mut value: i16 = rug_fuzz_6;
        MulAddAssign::mul_add_assign(&mut value, -rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(value, - 2);
        let mut value: i16 = -rug_fuzz_9;
        MulAddAssign::mul_add_assign(&mut value, -rug_fuzz_10, -rug_fuzz_11);
        debug_assert_eq!(value, 2);
        let mut value: i16 = i16::MAX;
        MulAddAssign::mul_add_assign(&mut value, rug_fuzz_12, rug_fuzz_13);
        debug_assert_eq!(value, i16::MIN);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_900_llm_16_900 {
    use crate::ops::mul_add::MulAdd;
    #[test]
    fn test_mul_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let multiplier = rug_fuzz_1;
        let addend = rug_fuzz_2;
        let result = <i32 as MulAdd>::mul_add(value, multiplier, addend);
        let expected = (value * multiplier) + addend;
        debug_assert_eq!(result, expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_901_llm_16_901 {
    use crate::ops::mul_add::MulAddAssign;
    #[test]
    fn mul_add_assign_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: i32 = rug_fuzz_0;
        <i32 as MulAddAssign>::mul_add_assign(&mut value, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value, 13);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1010_llm_16_1010 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mul_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i64 as MulAdd > ::mul_add(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 10
        );
        debug_assert_eq!(
            < i64 as MulAdd > ::mul_add(- rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), 1
        );
        debug_assert_eq!(
            < i64 as MulAdd > ::mul_add(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), 4
        );
        debug_assert_eq!(
            < i64 as MulAdd > ::mul_add(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11), 4
        );
        debug_assert_eq!(
            < i64 as MulAdd > ::mul_add(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14), 9
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1011_llm_16_1011 {
    use crate::ops::mul_add::MulAddAssign;
    #[test]
    fn test_mul_add_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = rug_fuzz_0;
        value.mul_add_assign(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value, 25);
        let mut value = rug_fuzz_3;
        value.mul_add_assign(rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(value, 5);
        let mut value = -rug_fuzz_6;
        value.mul_add_assign(rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(value, - 15);
        let mut value = rug_fuzz_9;
        value.mul_add_assign(-rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(value, - 15);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1121 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mul_add_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: i8 = rug_fuzz_0;
        <i8 as ops::mul_add::MulAddAssign>::mul_add_assign(
            &mut value,
            rug_fuzz_1,
            rug_fuzz_2,
        );
        debug_assert_eq!(value, 13);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1230_llm_16_1230 {
    use crate::ops::mul_add::MulAdd;
    #[test]
    fn isize_mul_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: isize = rug_fuzz_0;
        let y: isize = rug_fuzz_1;
        let z: isize = rug_fuzz_2;
        let result = isize::mul_add(x, y, z);
        debug_assert_eq!(result, 52);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1231 {
    use super::*;
    use crate::*;
    #[test]
    fn mul_add_assign_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: isize = rug_fuzz_0;
        value.mul_add_assign(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value, 13);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1436_llm_16_1436 {
    use crate::ops::mul_add::MulAdd;
    #[test]
    fn test_mul_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(MulAdd::mul_add(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 10u128);
        debug_assert_eq!(MulAdd::mul_add(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), 4u128);
        debug_assert_eq!(MulAdd::mul_add(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), 4u128);
        debug_assert_eq!(MulAdd::mul_add(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11), 6u128);
        debug_assert_eq!(
            MulAdd::mul_add(u128::MAX, rug_fuzz_12, rug_fuzz_13), u128::MAX
            .wrapping_add(1)
        );
        debug_assert_eq!(
            MulAdd::mul_add(u128::MAX, rug_fuzz_14, rug_fuzz_15), u128::MAX
            .wrapping_mul(0).wrapping_add(1)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1437 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mul_add_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: u128 = rug_fuzz_0;
        value.mul_add_assign(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value, 5 * 2 + 3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1542 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mul_add_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: u16 = rug_fuzz_0;
        value.mul_add_assign(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value, 53);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1646_llm_16_1646 {
    use crate::ops::mul_add::MulAdd;
    #[test]
    fn test_mul_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: u32 = rug_fuzz_0;
        let y: u32 = rug_fuzz_1;
        let z: u32 = rug_fuzz_2;
        let result = <u32 as MulAdd>::mul_add(x, y, z);
        debug_assert_eq!(result, (x * y) + z);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1647 {
    use super::*;
    use crate::*;
    #[test]
    fn mul_add_assign_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: u32 = rug_fuzz_0;
        <u32 as ops::mul_add::MulAddAssign>::mul_add_assign(
            &mut value,
            rug_fuzz_1,
            rug_fuzz_2,
        );
        debug_assert_eq!(value, 2 * 3 + 4);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1751_llm_16_1751 {
    use crate::MulAdd;
    #[test]
    fn test_mul_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u64 as MulAdd > ::mul_add(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 2 * 3 + 4
        );
        debug_assert_eq!(
            < u64 as MulAdd > ::mul_add(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), 5 * 6 + 7
        );
        debug_assert_eq!(
            < u64 as MulAdd > ::mul_add(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), 0 * 10 + 20
        );
        debug_assert_eq!(
            < u64 as MulAdd > ::mul_add(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11), 1 * 1 + 1
        );
        debug_assert_eq!(
            < u64 as MulAdd > ::mul_add(u64::MAX, rug_fuzz_12, rug_fuzz_13), u64::MAX * 1
            + 0
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1752 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mul_add_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: u64 = rug_fuzz_0;
        let a: u64 = rug_fuzz_1;
        let b: u64 = rug_fuzz_2;
        value.mul_add_assign(a, b);
        debug_assert_eq!(value, 5 * 2 + 3);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1962_llm_16_1962 {
    use crate::ops::mul_add::MulAdd;
    #[test]
    fn mul_add_basic() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < usize as MulAdd > ::mul_add(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 10
        );
             }
}
}
}    }
    #[test]
    fn mul_add_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < usize as MulAdd > ::mul_add(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 4
        );
        debug_assert_eq!(
            < usize as MulAdd > ::mul_add(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), 4
        );
        debug_assert_eq!(
            < usize as MulAdd > ::mul_add(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), 6
        );
             }
}
}
}    }
    #[test]
    fn mul_add_associativity() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x = rug_fuzz_0;
        let y = rug_fuzz_1;
        let z = rug_fuzz_2;
        debug_assert_eq!(
            < usize as MulAdd > ::mul_add(x, y, z), < usize as MulAdd > ::mul_add(y, x,
            z)
        );
             }
}
}
}    }
    #[test]
    fn mul_add_large_numbers() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x = rug_fuzz_0;
        let y = rug_fuzz_1;
        let z = rug_fuzz_2;
        debug_assert_eq!(< usize as MulAdd > ::mul_add(x, y, z), 2_000_000_000_000 + z);
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn mul_add_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x = usize::MAX / rug_fuzz_0;
        let y = rug_fuzz_1;
        let z = rug_fuzz_2;
        let _result = <usize as MulAdd>::mul_add(x, y, z);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1963 {
    use super::*;
    use crate::*;
    #[test]
    fn test_mul_add_assign() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: usize = rug_fuzz_0;
        value.mul_add_assign(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(value, 5 * 2 + 3);
             }
}
}
}    }
}
