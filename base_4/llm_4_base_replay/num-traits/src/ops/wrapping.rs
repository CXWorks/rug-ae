use core::num::Wrapping;
use core::ops::{Add, Mul, Neg, Shl, Shr, Sub};
macro_rules! wrapping_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t { #[inline] fn $method (& self, v : & Self) -> Self { <$t
        >::$method (* self, * v) } }
    };
    ($trait_name:ident, $method:ident, $t:ty, $rhs:ty) => {
        impl $trait_name <$rhs > for $t { #[inline] fn $method (& self, v : &$rhs) ->
        Self { <$t >::$method (* self, * v) } }
    };
}
/// Performs addition that wraps around on overflow.
pub trait WrappingAdd: Sized + Add<Self, Output = Self> {
    /// Wrapping (modular) addition. Computes `self + other`, wrapping around at the boundary of
    /// the type.
    fn wrapping_add(&self, v: &Self) -> Self;
}
wrapping_impl!(WrappingAdd, wrapping_add, u8);
wrapping_impl!(WrappingAdd, wrapping_add, u16);
wrapping_impl!(WrappingAdd, wrapping_add, u32);
wrapping_impl!(WrappingAdd, wrapping_add, u64);
wrapping_impl!(WrappingAdd, wrapping_add, usize);
wrapping_impl!(WrappingAdd, wrapping_add, u128);
wrapping_impl!(WrappingAdd, wrapping_add, i8);
wrapping_impl!(WrappingAdd, wrapping_add, i16);
wrapping_impl!(WrappingAdd, wrapping_add, i32);
wrapping_impl!(WrappingAdd, wrapping_add, i64);
wrapping_impl!(WrappingAdd, wrapping_add, isize);
wrapping_impl!(WrappingAdd, wrapping_add, i128);
/// Performs subtraction that wraps around on overflow.
pub trait WrappingSub: Sized + Sub<Self, Output = Self> {
    /// Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the boundary
    /// of the type.
    fn wrapping_sub(&self, v: &Self) -> Self;
}
wrapping_impl!(WrappingSub, wrapping_sub, u8);
wrapping_impl!(WrappingSub, wrapping_sub, u16);
wrapping_impl!(WrappingSub, wrapping_sub, u32);
wrapping_impl!(WrappingSub, wrapping_sub, u64);
wrapping_impl!(WrappingSub, wrapping_sub, usize);
wrapping_impl!(WrappingSub, wrapping_sub, u128);
wrapping_impl!(WrappingSub, wrapping_sub, i8);
wrapping_impl!(WrappingSub, wrapping_sub, i16);
wrapping_impl!(WrappingSub, wrapping_sub, i32);
wrapping_impl!(WrappingSub, wrapping_sub, i64);
wrapping_impl!(WrappingSub, wrapping_sub, isize);
wrapping_impl!(WrappingSub, wrapping_sub, i128);
/// Performs multiplication that wraps around on overflow.
pub trait WrappingMul: Sized + Mul<Self, Output = Self> {
    /// Wrapping (modular) multiplication. Computes `self * other`, wrapping around at the boundary
    /// of the type.
    fn wrapping_mul(&self, v: &Self) -> Self;
}
wrapping_impl!(WrappingMul, wrapping_mul, u8);
wrapping_impl!(WrappingMul, wrapping_mul, u16);
wrapping_impl!(WrappingMul, wrapping_mul, u32);
wrapping_impl!(WrappingMul, wrapping_mul, u64);
wrapping_impl!(WrappingMul, wrapping_mul, usize);
wrapping_impl!(WrappingMul, wrapping_mul, u128);
wrapping_impl!(WrappingMul, wrapping_mul, i8);
wrapping_impl!(WrappingMul, wrapping_mul, i16);
wrapping_impl!(WrappingMul, wrapping_mul, i32);
wrapping_impl!(WrappingMul, wrapping_mul, i64);
wrapping_impl!(WrappingMul, wrapping_mul, isize);
wrapping_impl!(WrappingMul, wrapping_mul, i128);
macro_rules! wrapping_unary_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t { #[inline] fn $method (& self) -> $t { <$t >::$method (*
        self) } }
    };
}
/// Performs a negation that does not panic.
pub trait WrappingNeg: Sized {
    /// Wrapping (modular) negation. Computes `-self`,
    /// wrapping around at the boundary of the type.
    ///
    /// Since unsigned types do not have negative equivalents
    /// all applications of this function will wrap (except for `-0`).
    /// For values smaller than the corresponding signed type's maximum
    /// the result is the same as casting the corresponding signed value.
    /// Any larger values are equivalent to `MAX + 1 - (val - MAX - 1)` where
    /// `MAX` is the corresponding signed type's maximum.
    ///
    /// ```
    /// use num_traits::WrappingNeg;
    ///
    /// assert_eq!(100i8.wrapping_neg(), -100);
    /// assert_eq!((-100i8).wrapping_neg(), 100);
    /// assert_eq!((-128i8).wrapping_neg(), -128); // wrapped!
    /// ```
    fn wrapping_neg(&self) -> Self;
}
wrapping_unary_impl!(WrappingNeg, wrapping_neg, u8);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, u16);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, u32);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, u64);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, usize);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, u128);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i8);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i16);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i32);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i64);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, isize);
wrapping_unary_impl!(WrappingNeg, wrapping_neg, i128);
macro_rules! wrapping_shift_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t { #[inline] fn $method (& self, rhs : u32) -> $t { <$t
        >::$method (* self, rhs) } }
    };
}
/// Performs a left shift that does not panic.
pub trait WrappingShl: Sized + Shl<usize, Output = Self> {
    /// Panic-free bitwise shift-left; yields `self << mask(rhs)`,
    /// where `mask` removes any high order bits of `rhs` that would
    /// cause the shift to exceed the bitwidth of the type.
    ///
    /// ```
    /// use num_traits::WrappingShl;
    ///
    /// let x: u16 = 0x0001;
    ///
    /// assert_eq!(WrappingShl::wrapping_shl(&x, 0),  0x0001);
    /// assert_eq!(WrappingShl::wrapping_shl(&x, 1),  0x0002);
    /// assert_eq!(WrappingShl::wrapping_shl(&x, 15), 0x8000);
    /// assert_eq!(WrappingShl::wrapping_shl(&x, 16), 0x0001);
    /// ```
    fn wrapping_shl(&self, rhs: u32) -> Self;
}
wrapping_shift_impl!(WrappingShl, wrapping_shl, u8);
wrapping_shift_impl!(WrappingShl, wrapping_shl, u16);
wrapping_shift_impl!(WrappingShl, wrapping_shl, u32);
wrapping_shift_impl!(WrappingShl, wrapping_shl, u64);
wrapping_shift_impl!(WrappingShl, wrapping_shl, usize);
wrapping_shift_impl!(WrappingShl, wrapping_shl, u128);
wrapping_shift_impl!(WrappingShl, wrapping_shl, i8);
wrapping_shift_impl!(WrappingShl, wrapping_shl, i16);
wrapping_shift_impl!(WrappingShl, wrapping_shl, i32);
wrapping_shift_impl!(WrappingShl, wrapping_shl, i64);
wrapping_shift_impl!(WrappingShl, wrapping_shl, isize);
wrapping_shift_impl!(WrappingShl, wrapping_shl, i128);
/// Performs a right shift that does not panic.
pub trait WrappingShr: Sized + Shr<usize, Output = Self> {
    /// Panic-free bitwise shift-right; yields `self >> mask(rhs)`,
    /// where `mask` removes any high order bits of `rhs` that would
    /// cause the shift to exceed the bitwidth of the type.
    ///
    /// ```
    /// use num_traits::WrappingShr;
    ///
    /// let x: u16 = 0x8000;
    ///
    /// assert_eq!(WrappingShr::wrapping_shr(&x, 0),  0x8000);
    /// assert_eq!(WrappingShr::wrapping_shr(&x, 1),  0x4000);
    /// assert_eq!(WrappingShr::wrapping_shr(&x, 15), 0x0001);
    /// assert_eq!(WrappingShr::wrapping_shr(&x, 16), 0x8000);
    /// ```
    fn wrapping_shr(&self, rhs: u32) -> Self;
}
wrapping_shift_impl!(WrappingShr, wrapping_shr, u8);
wrapping_shift_impl!(WrappingShr, wrapping_shr, u16);
wrapping_shift_impl!(WrappingShr, wrapping_shr, u32);
wrapping_shift_impl!(WrappingShr, wrapping_shr, u64);
wrapping_shift_impl!(WrappingShr, wrapping_shr, usize);
wrapping_shift_impl!(WrappingShr, wrapping_shr, u128);
wrapping_shift_impl!(WrappingShr, wrapping_shr, i8);
wrapping_shift_impl!(WrappingShr, wrapping_shr, i16);
wrapping_shift_impl!(WrappingShr, wrapping_shr, i32);
wrapping_shift_impl!(WrappingShr, wrapping_shr, i64);
wrapping_shift_impl!(WrappingShr, wrapping_shr, isize);
wrapping_shift_impl!(WrappingShr, wrapping_shr, i128);
impl<T: WrappingAdd> WrappingAdd for Wrapping<T>
where
    Wrapping<T>: Add<Output = Wrapping<T>>,
{
    fn wrapping_add(&self, v: &Self) -> Self {
        Wrapping(self.0.wrapping_add(&v.0))
    }
}
impl<T: WrappingSub> WrappingSub for Wrapping<T>
where
    Wrapping<T>: Sub<Output = Wrapping<T>>,
{
    fn wrapping_sub(&self, v: &Self) -> Self {
        Wrapping(self.0.wrapping_sub(&v.0))
    }
}
impl<T: WrappingMul> WrappingMul for Wrapping<T>
where
    Wrapping<T>: Mul<Output = Wrapping<T>>,
{
    fn wrapping_mul(&self, v: &Self) -> Self {
        Wrapping(self.0.wrapping_mul(&v.0))
    }
}
impl<T: WrappingNeg> WrappingNeg for Wrapping<T>
where
    Wrapping<T>: Neg<Output = Wrapping<T>>,
{
    fn wrapping_neg(&self) -> Self {
        Wrapping(self.0.wrapping_neg())
    }
}
impl<T: WrappingShl> WrappingShl for Wrapping<T>
where
    Wrapping<T>: Shl<usize, Output = Wrapping<T>>,
{
    fn wrapping_shl(&self, rhs: u32) -> Self {
        Wrapping(self.0.wrapping_shl(rhs))
    }
}
impl<T: WrappingShr> WrappingShr for Wrapping<T>
where
    Wrapping<T>: Shr<usize, Output = Wrapping<T>>,
{
    fn wrapping_shr(&self, rhs: u32) -> Self {
        Wrapping(self.0.wrapping_shr(rhs))
    }
}
#[test]
fn test_wrapping_traits() {
    fn wrapping_add<T: WrappingAdd>(a: T, b: T) -> T {
        a.wrapping_add(&b)
    }
    fn wrapping_sub<T: WrappingSub>(a: T, b: T) -> T {
        a.wrapping_sub(&b)
    }
    fn wrapping_mul<T: WrappingMul>(a: T, b: T) -> T {
        a.wrapping_mul(&b)
    }
    fn wrapping_neg<T: WrappingNeg>(a: T) -> T {
        a.wrapping_neg()
    }
    fn wrapping_shl<T: WrappingShl>(a: T, b: u32) -> T {
        a.wrapping_shl(b)
    }
    fn wrapping_shr<T: WrappingShr>(a: T, b: u32) -> T {
        a.wrapping_shr(b)
    }
    assert_eq!(wrapping_add(255, 1), 0u8);
    assert_eq!(wrapping_sub(0, 1), 255u8);
    assert_eq!(wrapping_mul(255, 2), 254u8);
    assert_eq!(wrapping_neg(255), 1u8);
    assert_eq!(wrapping_shl(255, 8), 255u8);
    assert_eq!(wrapping_shr(255, 8), 255u8);
    assert_eq!(wrapping_add(255, 1), (Wrapping(255u8) + Wrapping(1u8)).0);
    assert_eq!(wrapping_sub(0, 1), (Wrapping(0u8) - Wrapping(1u8)).0);
    assert_eq!(wrapping_mul(255, 2), (Wrapping(255u8) * Wrapping(2u8)).0);
    assert_eq!(wrapping_neg(255), (- Wrapping(255u8)).0);
    assert_eq!(wrapping_shl(255, 8), (Wrapping(255u8) << 8).0);
    assert_eq!(wrapping_shr(255, 8), (Wrapping(255u8) >> 8).0);
}
#[test]
fn wrapping_is_wrappingadd() {
    fn require_wrappingadd<T: WrappingAdd>(_: &T) {}
    require_wrappingadd(&Wrapping(42));
}
#[test]
fn wrapping_is_wrappingsub() {
    fn require_wrappingsub<T: WrappingSub>(_: &T) {}
    require_wrappingsub(&Wrapping(42));
}
#[test]
fn wrapping_is_wrappingmul() {
    fn require_wrappingmul<T: WrappingMul>(_: &T) {}
    require_wrappingmul(&Wrapping(42));
}
#[test]
fn wrapping_is_wrappingneg() {
    fn require_wrappingneg<T: WrappingNeg>(_: &T) {}
    require_wrappingneg(&Wrapping(42));
}
#[test]
fn wrapping_is_wrappingshl() {
    fn require_wrappingshl<T: WrappingShl>(_: &T) {}
    require_wrappingshl(&Wrapping(42));
}
#[test]
fn wrapping_is_wrappingshr() {
    fn require_wrappingshr<T: WrappingShr>(_: &T) {}
    require_wrappingshr(&Wrapping(42));
}
#[cfg(test)]
mod tests_llm_16_690_llm_16_690 {
    use super::*;
    use crate::*;
    #[test]
    fn wrapping_add_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: i128 = i128::max_value();
        let y: i128 = rug_fuzz_0;
        let result = <i128 as WrappingAdd>::wrapping_add(&x, &y);
        debug_assert_eq!(result, i128::min_value());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_691_llm_16_691 {
    use crate::ops::wrapping::WrappingMul;
    #[test]
    fn test_wrapping_mul() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(i128::wrapping_mul(i128::MAX, rug_fuzz_0), i128::MAX);
        debug_assert_eq!(i128::wrapping_mul(i128::MAX, rug_fuzz_1), 0);
        debug_assert_eq!(i128::wrapping_mul(i128::MAX, i128::MAX), - 1);
        debug_assert_eq!(i128::wrapping_mul(i128::MIN, rug_fuzz_2), i128::MIN);
        debug_assert_eq!(i128::wrapping_mul(i128::MIN, i128::MIN), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_692_llm_16_692 {
    use crate::ops::wrapping::WrappingNeg;
    #[test]
    fn wrapping_neg_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i128 as WrappingNeg > ::wrapping_neg(& rug_fuzz_0), 0);
        debug_assert_eq!(< i128 as WrappingNeg > ::wrapping_neg(& rug_fuzz_1), - 1);
        debug_assert_eq!(< i128 as WrappingNeg > ::wrapping_neg(& - rug_fuzz_2), 1);
        debug_assert_eq!(
            < i128 as WrappingNeg > ::wrapping_neg(& i128::MAX), - i128::MAX
        );
        debug_assert_eq!(< i128 as WrappingNeg > ::wrapping_neg(& i128::MIN), i128::MIN);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_693_llm_16_693 {
    use crate::ops::wrapping::WrappingShl;
    #[test]
    fn test_wrapping_shl() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i128, u32, i128, u32, i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i128 as WrappingShl > ::wrapping_shl(& rug_fuzz_0, rug_fuzz_1), 1
        );
        debug_assert_eq!(
            < i128 as WrappingShl > ::wrapping_shl(& rug_fuzz_2, rug_fuzz_3), -
            9223372036854775808
        );
        debug_assert_eq!(
            < i128 as WrappingShl > ::wrapping_shl(& rug_fuzz_4, rug_fuzz_5), 1
        );
        debug_assert_eq!(
            < i128 as WrappingShl > ::wrapping_shl(& - rug_fuzz_6, rug_fuzz_7), -
            170141183460469231731687303715884105728
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_694_llm_16_694 {
    use crate::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(i128, u32, i128, u32, i128, u32, i128, u32, i128, u32, u32, u32, u32, i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(WrappingShr::wrapping_shr(& rug_fuzz_0, rug_fuzz_1), 0i128);
        debug_assert_eq!(WrappingShr::wrapping_shr(& - rug_fuzz_2, rug_fuzz_3), - 1i128);
        debug_assert_eq!(WrappingShr::wrapping_shr(& - rug_fuzz_4, rug_fuzz_5), - 1i128);
        debug_assert_eq!(WrappingShr::wrapping_shr(& rug_fuzz_6, rug_fuzz_7), 0i128);
        debug_assert_eq!(WrappingShr::wrapping_shr(& - rug_fuzz_8, rug_fuzz_9), - 1i128);
        debug_assert_eq!(WrappingShr::wrapping_shr(& (i128::MAX), rug_fuzz_10), 0i128);
        debug_assert_eq!(WrappingShr::wrapping_shr(& (i128::MIN), rug_fuzz_11), - 1i128);
        debug_assert_eq!(
            WrappingShr::wrapping_shr(& (i128::MIN), rug_fuzz_12), i128::MIN
        );
        debug_assert_eq!(WrappingShr::wrapping_shr(& rug_fuzz_13, rug_fuzz_14), 0i128);
        debug_assert_eq!(
            WrappingShr::wrapping_shr(& - rug_fuzz_15, rug_fuzz_16), - 1i128
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_695_llm_16_695 {
    use super::*;
    use crate::*;
    #[test]
    fn test_wrapping_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: i128 = i128::MAX;
        let b: i128 = rug_fuzz_0;
        let c = <i128 as WrappingSub>::wrapping_sub(&a, &b);
        debug_assert_eq!(c, i128::MAX - 1);
        let a: i128 = i128::MIN;
        let b: i128 = -rug_fuzz_1;
        let c = <i128 as WrappingSub>::wrapping_sub(&a, &b);
        debug_assert_eq!(c, i128::MAX);
        let a: i128 = rug_fuzz_2;
        let b: i128 = i128::MIN;
        let c = <i128 as WrappingSub>::wrapping_sub(&a, &b);
        debug_assert_eq!(c, i128::MIN.wrapping_sub(0));
        let a: i128 = -rug_fuzz_3;
        let b: i128 = i128::MAX;
        let c = <i128 as WrappingSub>::wrapping_sub(&a, &b);
        debug_assert_eq!(c, 1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_800_llm_16_800 {
    use crate::ops::wrapping::WrappingAdd;
    #[test]
    fn test_wrapping_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: i16 = i16::MAX;
        let y: i16 = rug_fuzz_0;
        let result = <i16 as WrappingAdd>::wrapping_add(&x, &y);
        debug_assert_eq!(result, i16::MIN);
        let x: i16 = rug_fuzz_1;
        let y: i16 = rug_fuzz_2;
        let result = <i16 as WrappingAdd>::wrapping_add(&x, &y);
        debug_assert_eq!(result, 150);
        let x: i16 = -rug_fuzz_3;
        let y: i16 = -rug_fuzz_4;
        let result = <i16 as WrappingAdd>::wrapping_add(&x, &y);
        debug_assert_eq!(result, - 150);
        let x: i16 = i16::MIN;
        let y: i16 = -rug_fuzz_5;
        let result = <i16 as WrappingAdd>::wrapping_add(&x, &y);
        debug_assert_eq!(result, i16::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_801_llm_16_801 {
    use crate::ops::wrapping::WrappingMul;
    #[test]
    fn test_wrapping_mul() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& rug_fuzz_0, & rug_fuzz_1), 10000i16
            .wrapping_mul(1234)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& - rug_fuzz_2, & rug_fuzz_3), (-
            10000i16).wrapping_mul(1234)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& rug_fuzz_4, & - rug_fuzz_5), 10000i16
            .wrapping_mul(- 1234)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& - rug_fuzz_6, & - rug_fuzz_7), (-
            10000i16).wrapping_mul(- 1234)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& i16::MAX, & rug_fuzz_8), i16::MAX
            .wrapping_mul(1)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& i16::MIN, & rug_fuzz_9), i16::MIN
            .wrapping_mul(1)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& i16::MAX, & i16::MAX), i16::MAX
            .wrapping_mul(i16::MAX)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& i16::MIN, & i16::MAX), i16::MIN
            .wrapping_mul(i16::MAX)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& i16::MAX, & i16::MIN), i16::MAX
            .wrapping_mul(i16::MIN)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& i16::MIN, & i16::MIN), i16::MIN
            .wrapping_mul(i16::MIN)
        );
        debug_assert_eq!(
            < i16 as WrappingMul > ::wrapping_mul(& rug_fuzz_10, & rug_fuzz_11), 0i16
            .wrapping_mul(0)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_802_llm_16_802 {
    use super::*;
    use crate::*;
    use crate::ops::wrapping::WrappingNeg;
    #[test]
    fn wrapping_neg_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i16 as WrappingNeg > ::wrapping_neg(& rug_fuzz_0), 0);
        debug_assert_eq!(< i16 as WrappingNeg > ::wrapping_neg(& - rug_fuzz_1), 1);
        debug_assert_eq!(< i16 as WrappingNeg > ::wrapping_neg(& rug_fuzz_2), - 1);
        debug_assert_eq!(< i16 as WrappingNeg > ::wrapping_neg(& i16::MIN), i16::MIN);
        debug_assert_eq!(< i16 as WrappingNeg > ::wrapping_neg(& i16::MAX), - 32767);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_804_llm_16_804 {
    use crate::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            WrappingShr::wrapping_shr(& - rug_fuzz_0, rug_fuzz_1), -
            0b1000_0000_0000_0000
        );
        debug_assert_eq!(
            WrappingShr::wrapping_shr(& - rug_fuzz_2, rug_fuzz_3), -
            0b0100_0000_0000_0000
        );
        debug_assert_eq!(
            WrappingShr::wrapping_shr(& rug_fuzz_4, rug_fuzz_5), 0b0011_1111_1111_1111
        );
        debug_assert_eq!(WrappingShr::wrapping_shr(& - rug_fuzz_6, rug_fuzz_7), - 1);
        debug_assert_eq!(WrappingShr::wrapping_shr(& - rug_fuzz_8, rug_fuzz_9), - 1);
        debug_assert_eq!(WrappingShr::wrapping_shr(& rug_fuzz_10, rug_fuzz_11), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_805_llm_16_805 {
    use crate::ops::wrapping::WrappingSub;
    #[test]
    fn test_wrapping_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(i16::wrapping_sub(rug_fuzz_0, rug_fuzz_1), - 1);
        debug_assert_eq!(i16::wrapping_sub(i16::MIN, rug_fuzz_2), i16::MAX);
        debug_assert_eq!(i16::wrapping_sub(i16::MAX, - rug_fuzz_3), i16::MIN);
        debug_assert_eq!(i16::wrapping_sub(i16::MIN, i16::MAX), 1);
        debug_assert_eq!(i16::wrapping_sub(i16::MAX, i16::MIN), - 1);
        debug_assert_eq!(i16::wrapping_sub(rug_fuzz_4, rug_fuzz_5), 50);
        debug_assert_eq!(i16::wrapping_sub(- rug_fuzz_6, rug_fuzz_7), - 150);
        debug_assert_eq!(i16::wrapping_sub(rug_fuzz_8, - rug_fuzz_9), 150);
        debug_assert_eq!(i16::wrapping_sub(- rug_fuzz_10, - rug_fuzz_11), - 50);
        let neg_one = -rug_fuzz_12;
        debug_assert_eq!(i16::wrapping_sub(i16::MIN, neg_one.wrapping_neg()), 0);
        debug_assert_eq!(i16::wrapping_sub(rug_fuzz_13, i16::MIN), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_910_llm_16_910 {
    use crate::ops::wrapping::WrappingAdd;
    #[test]
    fn wrapping_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i32 as WrappingAdd > ::wrapping_add(& rug_fuzz_0, & rug_fuzz_1), 200_i32
            .wrapping_add(100)
        );
        debug_assert_eq!(
            < i32 as WrappingAdd > ::wrapping_add(& i32::MAX, & rug_fuzz_2), i32::MAX
            .wrapping_add(1)
        );
        debug_assert_eq!(
            < i32 as WrappingAdd > ::wrapping_add(& i32::MIN, & (- rug_fuzz_3)), i32::MIN
            .wrapping_add(- 1)
        );
        debug_assert_eq!(
            < i32 as WrappingAdd > ::wrapping_add(& rug_fuzz_4, & rug_fuzz_5), 0_i32
            .wrapping_add(0)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_912_llm_16_912 {
    use crate::WrappingNeg;
    #[test]
    fn test_wrapping_neg() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as WrappingNeg > ::wrapping_neg(& rug_fuzz_0), 0);
        debug_assert_eq!(< i32 as WrappingNeg > ::wrapping_neg(& rug_fuzz_1), - 1);
        debug_assert_eq!(< i32 as WrappingNeg > ::wrapping_neg(& - rug_fuzz_2), 1);
        debug_assert_eq!(
            < i32 as WrappingNeg > ::wrapping_neg(& i32::MAX), - 2147483647
        );
        debug_assert_eq!(< i32 as WrappingNeg > ::wrapping_neg(& i32::MIN), i32::MIN);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_913_llm_16_913 {
    use crate::ops::wrapping::WrappingShl;
    #[test]
    fn test_wrapping_shl() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).wrapping_shl(rug_fuzz_1), 5i32.wrapping_shl(1));
        debug_assert_eq!(
            (- rug_fuzz_2).wrapping_shl(rug_fuzz_3), - 1i32.wrapping_shl(31)
        );
        debug_assert_eq!(
            (rug_fuzz_4).wrapping_shl(rug_fuzz_5), 1i32.wrapping_shl(32 % 32)
        );
        debug_assert_eq!((rug_fuzz_6).wrapping_shl(rug_fuzz_7), 1i32.wrapping_shl(0));
        debug_assert_eq!((rug_fuzz_8).wrapping_shl(rug_fuzz_9), 0i32.wrapping_shl(8));
        debug_assert_eq!((i32::MAX).wrapping_shl(rug_fuzz_10), i32::MAX.wrapping_shl(2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_914_llm_16_914 {
    use crate::ops::wrapping::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, i32, u32, u32, i32, u32, i32, u32, i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i32 as WrappingShr > ::wrapping_shr(& rug_fuzz_0, rug_fuzz_1), 0
        );
        debug_assert_eq!(
            < i32 as WrappingShr > ::wrapping_shr(& - rug_fuzz_2, rug_fuzz_3), i32::MAX
        );
        debug_assert_eq!(
            < i32 as WrappingShr > ::wrapping_shr(& i32::MAX, rug_fuzz_4), i32::MAX / 2
        );
        debug_assert_eq!(
            < i32 as WrappingShr > ::wrapping_shr(& rug_fuzz_5, rug_fuzz_6), 1
        );
        debug_assert_eq!(
            < i32 as WrappingShr > ::wrapping_shr(& rug_fuzz_7, rug_fuzz_8), 0
        );
        debug_assert_eq!(
            < i32 as WrappingShr > ::wrapping_shr(& - rug_fuzz_9, rug_fuzz_10), 1
        );
        debug_assert_eq!(
            < i32 as WrappingShr > ::wrapping_shr(& rug_fuzz_11, rug_fuzz_12), 1
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_915_llm_16_915 {
    use crate::ops::wrapping::WrappingSub;
    use core::num::Wrapping;
    #[test]
    fn test_wrapping_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Wrapping(rug_fuzz_0).wrapping_sub(& Wrapping(rug_fuzz_1)), Wrapping(0i32)
        );
        debug_assert_eq!(
            Wrapping(rug_fuzz_2).wrapping_sub(& Wrapping(rug_fuzz_3)), Wrapping(- 1i32)
        );
        debug_assert_eq!(
            Wrapping(i32::MIN).wrapping_sub(& Wrapping(rug_fuzz_4)), Wrapping(i32::MAX)
        );
        debug_assert_eq!(
            Wrapping(i32::MAX).wrapping_sub(& Wrapping(- rug_fuzz_5)), Wrapping(i32::MIN)
        );
        debug_assert_eq!(
            Wrapping(rug_fuzz_6).wrapping_sub(& Wrapping(- rug_fuzz_7)), Wrapping(2i32)
        );
        debug_assert_eq!(
            Wrapping(- rug_fuzz_8).wrapping_sub(& Wrapping(rug_fuzz_9)), Wrapping(- 2i32)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1020_llm_16_1020 {
    use crate::WrappingAdd;
    #[test]
    fn test_wrapping_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i64 as WrappingAdd > ::wrapping_add(& rug_fuzz_0, & rug_fuzz_1), 0
        );
        debug_assert_eq!(
            < i64 as WrappingAdd > ::wrapping_add(& i64::MAX, & rug_fuzz_2), i64::MIN
        );
        debug_assert_eq!(
            < i64 as WrappingAdd > ::wrapping_add(& i64::MIN, & (- rug_fuzz_3)), i64::MAX
        );
        debug_assert_eq!(
            < i64 as WrappingAdd > ::wrapping_add(& rug_fuzz_4, & rug_fuzz_5), 1111111110
        );
        debug_assert_eq!(
            < i64 as WrappingAdd > ::wrapping_add(& (- rug_fuzz_6), & (- rug_fuzz_7)), -
            1111111110
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1022_llm_16_1022 {
    use super::*;
    use crate::*;
    use crate::ops::wrapping::WrappingNeg;
    #[test]
    fn test_wrapping_neg() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as WrappingNeg > ::wrapping_neg(& rug_fuzz_0), 0);
        debug_assert_eq!(< i64 as WrappingNeg > ::wrapping_neg(& rug_fuzz_1), - 1);
        debug_assert_eq!(< i64 as WrappingNeg > ::wrapping_neg(& - rug_fuzz_2), 1);
        debug_assert_eq!(< i64 as WrappingNeg > ::wrapping_neg(& i64::MIN), i64::MIN);
        debug_assert_eq!(< i64 as WrappingNeg > ::wrapping_neg(& i64::MAX), - i64::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1023_llm_16_1023 {
    use crate::ops::wrapping::WrappingShl;
    #[test]
    fn test_wrapping_shl() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i64 as WrappingShl > ::wrapping_shl(& rug_fuzz_0, rug_fuzz_1), 1
        );
        debug_assert_eq!(
            < i64 as WrappingShl > ::wrapping_shl(& rug_fuzz_2, rug_fuzz_3), 2
        );
        debug_assert_eq!(
            < i64 as WrappingShl > ::wrapping_shl(& rug_fuzz_4, rug_fuzz_5), -
            9223372036854775808i64
        );
        debug_assert_eq!(
            < i64 as WrappingShl > ::wrapping_shl(& rug_fuzz_6, rug_fuzz_7), 1
        );
        debug_assert_eq!(
            < i64 as WrappingShl > ::wrapping_shl(& rug_fuzz_8, rug_fuzz_9), 2
        );
        debug_assert_eq!(
            < i64 as WrappingShl > ::wrapping_shl(& - rug_fuzz_10, rug_fuzz_11), -
            9223372036854775808i64
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1024 {
    use crate::ops::wrapping::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i64, u32, i64, u32, i64, u32, i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i64 as WrappingShr > ::wrapping_shr(& rug_fuzz_0, rug_fuzz_1), 0b0101
        );
        debug_assert_eq!(
            < i64 as WrappingShr > ::wrapping_shr(& - rug_fuzz_2, rug_fuzz_3), i64::MIN
            .wrapping_shr(1) | 0b0101
        );
        debug_assert_eq!(
            < i64 as WrappingShr > ::wrapping_shr(& rug_fuzz_4, rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < i64 as WrappingShr > ::wrapping_shr(& rug_fuzz_6, rug_fuzz_7), 1
        );
        debug_assert_eq!(
            < i64 as WrappingShr > ::wrapping_shr(& rug_fuzz_8, rug_fuzz_9), 1
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1025_llm_16_1025 {
    use crate::WrappingSub;
    #[test]
    fn test_wrapping_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: i64 = rug_fuzz_0;
        let b: i64 = rug_fuzz_1;
        let wrapped_sub_ab: i64 = a.wrapping_sub(b);
        debug_assert_eq!(wrapped_sub_ab, i64::max_value());
        let c: i64 = i64::max_value();
        let wrapped_sub_ac: i64 = a.wrapping_sub(c);
        debug_assert_eq!(wrapped_sub_ac, 0i64.wrapping_sub(i64::max_value()));
        let d: i64 = -rug_fuzz_2;
        let wrapped_sub_cd: i64 = c.wrapping_sub(d);
        debug_assert_eq!(wrapped_sub_cd, i64::max_value().wrapping_sub(- 1));
        let e: i64 = i64::min_value();
        let wrapped_sub_ed: i64 = e.wrapping_sub(d);
        debug_assert_eq!(wrapped_sub_ed, i64::min_value().wrapping_sub(- 1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1130_llm_16_1130 {
    use crate::WrappingAdd;
    #[test]
    fn test_wrapping_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(WrappingAdd::wrapping_add(& rug_fuzz_0, & rug_fuzz_1), - 128);
        debug_assert_eq!(WrappingAdd::wrapping_add(& - rug_fuzz_2, & - rug_fuzz_3), 127);
        debug_assert_eq!(WrappingAdd::wrapping_add(& rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(WrappingAdd::wrapping_add(& - rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(WrappingAdd::wrapping_add(& rug_fuzz_8, & rug_fuzz_9), - 2);
        debug_assert_eq!(WrappingAdd::wrapping_add(& - rug_fuzz_10, & - rug_fuzz_11), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1131_llm_16_1131 {
    use crate::WrappingMul;
    #[test]
    fn test_wrapping_mul() {
        assert_eq!(
            < i8 as WrappingMul >::wrapping_mul(& 100, & 27), 100i8.wrapping_mul(27)
        );
        assert_eq!(
            < i8 as WrappingMul >::wrapping_mul(&- 100, & 27), (- 100i8).wrapping_mul(27)
        );
        assert_eq!(
            < i8 as WrappingMul >::wrapping_mul(& 100, &- 27), 100i8.wrapping_mul(- 27)
        );
        assert_eq!(
            < i8 as WrappingMul >::wrapping_mul(&- 100, &- 27), (- 100i8).wrapping_mul(-
            27)
        );
        assert_eq!(< i8 as WrappingMul >::wrapping_mul(& 0, & 27), 0i8.wrapping_mul(27));
        assert_eq!(
            < i8 as WrappingMul >::wrapping_mul(& 100, & 0), 100i8.wrapping_mul(0)
        );
        assert_eq!(
            < i8 as WrappingMul >::wrapping_mul(&- 1, & 127), (- 1i8).wrapping_mul(127)
        );
        assert_eq!(
            < i8 as WrappingMul >::wrapping_mul(& 1, &- 128), 1i8.wrapping_mul(- 128)
        );
        assert_eq!(
            < i8 as WrappingMul >::wrapping_mul(&- 1, &- 128), (- 1i8).wrapping_mul(-
            128)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1133_llm_16_1133 {
    use crate::ops::wrapping::WrappingShl;
    #[test]
    fn test_wrapping_shl() {
        assert_eq!(< i8 as WrappingShl >::wrapping_shl(& 1, 5), 1i8.wrapping_shl(5));
        assert_eq!(
            < i8 as WrappingShl >::wrapping_shl(&- 1, 6), (- 1i8).wrapping_shl(6)
        );
        assert_eq!(< i8 as WrappingShl >::wrapping_shl(& 0, 8), 0i8.wrapping_shl(8));
        assert_eq!(< i8 as WrappingShl >::wrapping_shl(& 127, 1), 127i8.wrapping_shl(1));
        assert_eq!(
            < i8 as WrappingShl >::wrapping_shl(&- 128, 1), (- 128i8).wrapping_shl(1)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1134_llm_16_1134 {
    use crate::ops::wrapping::WrappingShr;
    #[test]
    fn test_wrapping_shr() {
        assert_eq!(< i8 as WrappingShr >::wrapping_shr(&- 128, 1), - 64);
        assert_eq!(< i8 as WrappingShr >::wrapping_shr(& 127, 1), 63);
        assert_eq!(< i8 as WrappingShr >::wrapping_shr(& 1, 8), 0);
        assert_eq!(< i8 as WrappingShr >::wrapping_shr(&- 1, 1), - 1);
        assert_eq!(< i8 as WrappingShr >::wrapping_shr(&- 1, 8), - 1);
        assert_eq!(< i8 as WrappingShr >::wrapping_shr(&- 128, 8), - 1);
        assert_eq!(< i8 as WrappingShr >::wrapping_shr(& 127, 7), 0);
    }
}
#[cfg(test)]
mod tests_llm_16_1135_llm_16_1135 {
    use crate::ops::wrapping::WrappingSub;
    #[test]
    fn test_wrapping_sub() {
        assert_eq!(5i8.wrapping_sub(10i8), - 5i8);
        assert_eq!((- 128i8).wrapping_sub(1i8), 127i8);
        assert_eq!((- 1i8).wrapping_sub(- 127i8), - 128i8);
        assert_eq!((0i8).wrapping_sub(0i8), 0i8);
        assert_eq!((127i8).wrapping_sub(- 128i8), - 1i8);
    }
}
#[cfg(test)]
mod tests_llm_16_1240_llm_16_1240 {
    use crate::WrappingAdd;
    #[test]
    fn wrapping_add_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(isize, isize, isize, isize, isize, isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            isize::wrapping_add(std::isize::MAX, rug_fuzz_0), std::isize::MIN
        );
        debug_assert_eq!(
            isize::wrapping_add(std::isize::MIN, - rug_fuzz_1), std::isize::MAX
        );
        debug_assert_eq!(isize::wrapping_add(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(isize::wrapping_add(rug_fuzz_4, rug_fuzz_5), 579);
        debug_assert_eq!(isize::wrapping_add(- rug_fuzz_6, - rug_fuzz_7), - 579);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1241_llm_16_1241 {
    use crate::WrappingMul;
    #[test]
    fn test_wrapping_mul() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: isize = rug_fuzz_0;
        let b: isize = isize::MAX;
        let result = a.wrapping_mul(b);
        debug_assert_eq!(result, 10isize.wrapping_mul(isize::MAX));
             }
}
}
}    }
    #[test]
    fn test_wrapping_mul_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: isize = isize::MAX;
        let b: isize = rug_fuzz_0;
        let result = a.wrapping_mul(b);
        debug_assert_eq!(result, isize::MAX.wrapping_mul(2));
             }
}
}
}    }
    #[test]
    fn test_wrapping_mul_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: isize = rug_fuzz_0;
        let b: isize = isize::MAX;
        let result = a.wrapping_mul(b);
        debug_assert_eq!(result, 0);
             }
}
}
}    }
    #[test]
    fn test_wrapping_mul_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: isize = -rug_fuzz_0;
        let b: isize = isize::MAX;
        let result = a.wrapping_mul(b);
        debug_assert_eq!(result, (- 1isize).wrapping_mul(isize::MAX));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1243_llm_16_1243 {
    use crate::ops::wrapping::WrappingShl;
    #[test]
    fn test_wrapping_shl() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(isize, u32, isize, u32, isize, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < isize as WrappingShl > ::wrapping_shl(& rug_fuzz_0, rug_fuzz_1), 1
        );
        debug_assert_eq!(
            < isize as WrappingShl > ::wrapping_shl(& rug_fuzz_2, rug_fuzz_3), 2
        );
        debug_assert_eq!(
            < isize as WrappingShl > ::wrapping_shl(& rug_fuzz_4, rug_fuzz_5), 4
        );
        debug_assert_eq!(
            < isize as WrappingShl > ::wrapping_shl(& isize::MAX, rug_fuzz_6), isize::MAX
            .wrapping_shl(1)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1244_llm_16_1244 {
    use crate::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(isize, u32, isize, u32, u32, isize, u32, isize, u32, isize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).wrapping_shr(rug_fuzz_1), 2isize);
        debug_assert_eq!((- rug_fuzz_2).wrapping_shr(rug_fuzz_3), isize::MAX / 2 + 1);
        debug_assert_eq!((isize::MIN).wrapping_shr(rug_fuzz_4), isize::MIN / 2);
        #[cfg(target_pointer_width = "64")]
        {
            debug_assert_eq!((rug_fuzz_5).wrapping_shr(rug_fuzz_6), 1isize >> 32);
            debug_assert_eq!((rug_fuzz_7).wrapping_shr(rug_fuzz_8), 1isize);
        }
        #[cfg(target_pointer_width = "32")]
        {
            debug_assert_eq!((rug_fuzz_9).wrapping_shr(rug_fuzz_10), 1isize);
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1296 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    use std::ops::Add;
    #[test]
    fn test_wrapping_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Wrapping(rug_fuzz_0).wrapping_add(& Wrapping(rug_fuzz_1)), Wrapping(100u32 +
            55u32)
        );
        debug_assert_eq!(
            Wrapping(u32::MAX).wrapping_add(& Wrapping(rug_fuzz_2)), Wrapping(u32::MIN)
        );
        debug_assert_eq!(
            Wrapping(rug_fuzz_3).wrapping_add(& Wrapping(rug_fuzz_4)), Wrapping(0u32)
        );
        debug_assert_eq!(
            Wrapping(u32::MAX).wrapping_add(& Wrapping(u32::MAX)), Wrapping(u32::MAX - 1)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1298 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_wrapping_neg() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0).wrapping_neg(), Wrapping(0i32));
        debug_assert_eq!(Wrapping(rug_fuzz_1).wrapping_neg(), Wrapping(- 1i32));
        debug_assert_eq!(Wrapping(- rug_fuzz_2).wrapping_neg(), Wrapping(1i32));
        debug_assert_eq!(Wrapping(i32::MAX).wrapping_neg(), Wrapping(i32::MIN + 1));
        debug_assert_eq!(Wrapping(i32::MIN).wrapping_neg(), Wrapping(i32::MIN));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1299_llm_16_1299 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn wrapping_shl_u8_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u32, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Wrapping(rug_fuzz_0).wrapping_shl(rug_fuzz_1), Wrapping(0x20u8)
        );
        debug_assert_eq!(Wrapping(rug_fuzz_2).wrapping_shl(rug_fuzz_3), Wrapping(0u8));
             }
}
}
}    }
    #[test]
    fn wrapping_shl_u16_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u16, u32, u16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Wrapping(rug_fuzz_0).wrapping_shl(rug_fuzz_1), Wrapping(0x3400u16)
        );
        debug_assert_eq!(Wrapping(rug_fuzz_2).wrapping_shl(rug_fuzz_3), Wrapping(0u16));
             }
}
}
}    }
    #[test]
    fn wrapping_shl_u32_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Wrapping(rug_fuzz_0).wrapping_shl(rug_fuzz_1), Wrapping(0x56780000u32)
        );
        debug_assert_eq!(Wrapping(rug_fuzz_2).wrapping_shl(rug_fuzz_3), Wrapping(0u32));
             }
}
}
}    }
    #[test]
    fn wrapping_shl_u64_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u32, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Wrapping(rug_fuzz_0).wrapping_shl(rug_fuzz_1),
            Wrapping(0x9ABCDEF000000000u64)
        );
        debug_assert_eq!(Wrapping(rug_fuzz_2).wrapping_shl(rug_fuzz_3), Wrapping(0u64));
             }
}
}
}    }
    #[test]
    fn wrapping_shl_u128_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u128, u32, u128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Wrapping(rug_fuzz_0).wrapping_shl(rug_fuzz_1),
            Wrapping(0x9ABCDEF0123456780000000000000000u128)
        );
        debug_assert_eq!(Wrapping(rug_fuzz_2).wrapping_shl(rug_fuzz_3), Wrapping(0u128));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1300 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            (Wrapping(rug_fuzz_0)).wrapping_shr(rug_fuzz_1), Wrapping(0b0000_1111u32)
        );
        debug_assert_eq!(
            (Wrapping(rug_fuzz_2)).wrapping_shr(rug_fuzz_3), Wrapping(0u32)
        );
        debug_assert_eq!(
            (Wrapping(rug_fuzz_4)).wrapping_shr(rug_fuzz_5), Wrapping(0x7FFF_FFFFu32)
        );
        debug_assert_eq!(
            (Wrapping(rug_fuzz_6)).wrapping_shr(rug_fuzz_7), Wrapping(0xFFFF_FFFFu32)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1301_llm_16_1301 {
    use crate::ops::wrapping::WrappingSub;
    use crate::Wrapping;
    #[test]
    fn test_wrapping_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a = Wrapping(rug_fuzz_0);
        let b = Wrapping(rug_fuzz_1);
        let c = Wrapping(u32::MAX - rug_fuzz_2);
        let d = Wrapping(rug_fuzz_3);
        debug_assert_eq!(a.wrapping_sub(& b), Wrapping(100u32.wrapping_sub(200u32)));
        debug_assert_eq!(c.wrapping_sub(& d), Wrapping(u32::MAX));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1446_llm_16_1446 {
    use crate::WrappingAdd;
    #[test]
    fn test_wrapping_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u128, u128, u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(u128::wrapping_add(u128::MAX, rug_fuzz_0), 0);
        debug_assert_eq!(u128::wrapping_add(rug_fuzz_1, u128::MAX), u128::MAX);
        debug_assert_eq!(u128::wrapping_add(u128::MAX, u128::MAX), u128::MAX - 1);
        debug_assert_eq!(u128::wrapping_add(rug_fuzz_2, rug_fuzz_3), 579);
        debug_assert_eq!(u128::wrapping_add(rug_fuzz_4, u128::MAX), 122);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1447_llm_16_1447 {
    use crate::ops::wrapping::WrappingMul;
    #[test]
    fn test_wrapping_mul() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u128, u128, u128, u128, u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u128 as WrappingMul > ::wrapping_mul(& rug_fuzz_0, & rug_fuzz_1), 0
        );
        debug_assert_eq!(
            < u128 as WrappingMul > ::wrapping_mul(& u128::MAX, & rug_fuzz_2), u128::MAX
            .wrapping_mul(2u128)
        );
        debug_assert_eq!(
            < u128 as WrappingMul > ::wrapping_mul(& rug_fuzz_3, & u128::MAX), 0
        );
        debug_assert_eq!(
            < u128 as WrappingMul > ::wrapping_mul(& rug_fuzz_4, & u128::MAX), u128::MAX
        );
        debug_assert_eq!(
            < u128 as WrappingMul > ::wrapping_mul(& rug_fuzz_5, & (u128::MAX /
            rug_fuzz_6)), u128::MAX.wrapping_mul(2u128) / 2u128
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1448_llm_16_1448 {
    use crate::ops::wrapping::WrappingNeg;
    #[test]
    fn test_wrapping_neg_u128() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(WrappingNeg::wrapping_neg(& rug_fuzz_0), 0u128.wrapping_neg());
        debug_assert_eq!(WrappingNeg::wrapping_neg(& rug_fuzz_1), 1u128.wrapping_neg());
        debug_assert_eq!(
            WrappingNeg::wrapping_neg(& u128::MAX), u128::MAX.wrapping_neg()
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1449 {
    use crate::ops::wrapping::WrappingShl;
    #[test]
    fn wrapping_shl_u128() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u32, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(WrappingShl::wrapping_shl(& rug_fuzz_0, rug_fuzz_1), 1u128);
        debug_assert_eq!(
            WrappingShl::wrapping_shl(& rug_fuzz_2, rug_fuzz_3), 1u128 << 127
        );
        debug_assert_eq!(WrappingShl::wrapping_shl(& rug_fuzz_4, rug_fuzz_5), 1u128);
        debug_assert_eq!(
            WrappingShl::wrapping_shl(& rug_fuzz_6, rug_fuzz_7), 1u128 << 127
        );
        debug_assert_eq!(WrappingShl::wrapping_shl(& rug_fuzz_8, rug_fuzz_9), 1u128);
        let num: u128 = rug_fuzz_10;
        debug_assert_eq!(WrappingShl::wrapping_shl(& num, rug_fuzz_11), num << 4);
        debug_assert_eq!(WrappingShl::wrapping_shl(& num, rug_fuzz_12), num << 4);
        debug_assert_eq!(
            WrappingShl::wrapping_shl(& rug_fuzz_13, u32::MAX), 1u128 << (u32::MAX % 128)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1450_llm_16_1450 {
    use crate::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u128, u32, u128, u32, u128, u32, u128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            WrappingShr::wrapping_shr(& rug_fuzz_0, rug_fuzz_1), 0xFFFFFFFFFFFFFFFF_u128
        );
        debug_assert_eq!(WrappingShr::wrapping_shr(& rug_fuzz_2, rug_fuzz_3), 0_u128);
        debug_assert_eq!(WrappingShr::wrapping_shr(& rug_fuzz_4, rug_fuzz_5), 0_u128);
        debug_assert_eq!(WrappingShr::wrapping_shr(& rug_fuzz_6, rug_fuzz_7), 1_u128);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1552_llm_16_1552 {
    use crate::ops::wrapping::WrappingMul;
    #[test]
    fn test_wrapping_mul() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u16, u16, u16, u16, u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(WrappingMul::wrapping_mul(& rug_fuzz_0, & rug_fuzz_1), 50);
        debug_assert_eq!(WrappingMul::wrapping_mul(& rug_fuzz_2, & rug_fuzz_3), 65535);
        debug_assert_eq!(WrappingMul::wrapping_mul(& rug_fuzz_4, & rug_fuzz_5), 65534);
        debug_assert_eq!(WrappingMul::wrapping_mul(& rug_fuzz_6, & rug_fuzz_7), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1553 {
    use super::*;
    use crate::*;
    #[test]
    fn test_wrapping_neg() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.wrapping_neg(), 0u16);
        debug_assert_eq!(rug_fuzz_1.wrapping_neg(), 65535u16);
        debug_assert_eq!(rug_fuzz_2.wrapping_neg(), 65534u16);
        debug_assert_eq!(rug_fuzz_3.wrapping_neg(), 1u16);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1554_llm_16_1554 {
    use crate::WrappingShl;
    #[test]
    fn test_wrapping_shl() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u16, u32, u16, u32, u16, u32, u16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u16 as WrappingShl > ::wrapping_shl(& rug_fuzz_0, rug_fuzz_1),
            0b0010_0000_0000_0000
        );
        debug_assert_eq!(
            < u16 as WrappingShl > ::wrapping_shl(& rug_fuzz_2, rug_fuzz_3),
            0b0010_0000_0000_0000
        );
        debug_assert_eq!(
            < u16 as WrappingShl > ::wrapping_shl(& rug_fuzz_4, rug_fuzz_5),
            0b0001_0000_0000_0000
        );
        debug_assert_eq!(
            < u16 as WrappingShl > ::wrapping_shl(& rug_fuzz_6, rug_fuzz_7),
            0b0010_0000_0000_0000
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1555 {
    use super::*;
    use crate::*;
    use crate::ops::wrapping::WrappingShr;
    #[test]
    fn wrapping_shr_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u16, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u16 = rug_fuzz_0;
        let shr_value = <u16 as WrappingShr>::wrapping_shr(&value, rug_fuzz_1);
        debug_assert_eq!(shr_value, 0b0000_1100_1001_1111);
        let shr_value = <u16 as WrappingShr>::wrapping_shr(&value, rug_fuzz_2);
        debug_assert_eq!(shr_value, 0b0000_0000_1100_1001);
        let shr_value = <u16 as WrappingShr>::wrapping_shr(&value, rug_fuzz_3);
        debug_assert_eq!(shr_value, 0b0000_0000_0000_1100);
        let shr_value = <u16 as WrappingShr>::wrapping_shr(&value, rug_fuzz_4);
        debug_assert_eq!(shr_value, 0b1100_1001_1111_0000);
        let shr_value = <u16 as WrappingShr>::wrapping_shr(&value, rug_fuzz_5);
        debug_assert_eq!(shr_value, 0b1111_0000_1100_1001);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1556_llm_16_1556 {
    use crate::WrappingSub;
    #[test]
    fn test_wrapping_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u16, u16, u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: u16 = rug_fuzz_0;
        let b: u16 = rug_fuzz_1;
        let c: u16 = u16::MAX;
        debug_assert_eq!(WrappingSub::wrapping_sub(& a, & b), a.wrapping_sub(b));
        debug_assert_eq!(WrappingSub::wrapping_sub(& b, & a), b.wrapping_sub(a));
        debug_assert_eq!(WrappingSub::wrapping_sub(& c, & b), c.wrapping_sub(b));
        debug_assert_eq!(WrappingSub::wrapping_sub(& a, & c), a.wrapping_sub(c));
        debug_assert_eq!(
            WrappingSub::wrapping_sub(& rug_fuzz_2, & rug_fuzz_3), 0u16
            .wrapping_sub(1u16)
        );
        debug_assert_eq!(
            WrappingSub::wrapping_sub(& rug_fuzz_4, & rug_fuzz_5), 1u16
            .wrapping_sub(0u16)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1656_llm_16_1656 {
    use crate::WrappingAdd;
    #[test]
    fn test_wrapping_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.wrapping_add(rug_fuzz_1), 5);
        debug_assert_eq!(u32::MAX.wrapping_add(rug_fuzz_2), 0);
        debug_assert_eq!(u32::MAX.wrapping_add(rug_fuzz_3), 1);
        debug_assert_eq!(u32::MAX.wrapping_add(u32::MAX), u32::MAX - 1);
        debug_assert_eq!(rug_fuzz_4.wrapping_add(rug_fuzz_5), 1111111110);
        debug_assert_eq!(u32::MAX.wrapping_add(u32::MAX.wrapping_add(rug_fuzz_6)), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1657_llm_16_1657 {
    use crate::WrappingMul;
    #[test]
    fn wrapping_mul_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as WrappingMul > ::wrapping_mul(& rug_fuzz_0, & rug_fuzz_1), 10
        );
        debug_assert_eq!(
            < u32 as WrappingMul > ::wrapping_mul(& u32::MAX, & rug_fuzz_2), u32::MAX
            .wrapping_mul(2)
        );
        debug_assert_eq!(
            < u32 as WrappingMul > ::wrapping_mul(& rug_fuzz_3, & u32::MAX), 0
        );
        debug_assert_eq!(
            < u32 as WrappingMul > ::wrapping_mul(& u32::MAX, & u32::MAX), u32::MAX
            .wrapping_mul(u32::MAX)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1658_llm_16_1658 {
    use crate::ops::wrapping::WrappingNeg;
    #[test]
    fn test_wrapping_neg() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as WrappingNeg > ::wrapping_neg(& rug_fuzz_0), 0);
        debug_assert_eq!(< u32 as WrappingNeg > ::wrapping_neg(& rug_fuzz_1), u32::MAX);
        debug_assert_eq!(< u32 as WrappingNeg > ::wrapping_neg(& u32::MAX), 1);
        debug_assert_eq!(
            < u32 as WrappingNeg > ::wrapping_neg(& rug_fuzz_2), u32::MAX - 1
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1659_llm_16_1659 {
    use crate::WrappingShl;
    #[test]
    fn test_wrapping_shl() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(u32::wrapping_shl(rug_fuzz_0, rug_fuzz_1), 1_u32);
        debug_assert_eq!(u32::wrapping_shl(rug_fuzz_2, rug_fuzz_3), 0x80000000);
        debug_assert_eq!(u32::wrapping_shl(rug_fuzz_4, rug_fuzz_5), 1_u32);
        debug_assert_eq!(u32::wrapping_shl(rug_fuzz_6, rug_fuzz_7), 2_u32);
        debug_assert_eq!(u32::wrapping_shl(rug_fuzz_8, rug_fuzz_9), 0x80000000);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1660_llm_16_1660 {
    use crate::ops::wrapping::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            WrappingShr::wrapping_shr(& rug_fuzz_0, rug_fuzz_1), 0b0000_1111
        );
        debug_assert_eq!(
            WrappingShr::wrapping_shr(& rug_fuzz_2, rug_fuzz_3), 0b0100_0000
        );
        debug_assert_eq!(
            WrappingShr::wrapping_shr(& rug_fuzz_4, rug_fuzz_5), 0b0001_0000
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1661_llm_16_1661 {
    use crate::WrappingSub;
    #[test]
    fn test_wrapping_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as WrappingSub > ::wrapping_sub(& rug_fuzz_0, & rug_fuzz_1), 0u32
        );
        debug_assert_eq!(
            < u32 as WrappingSub > ::wrapping_sub(& rug_fuzz_2, & rug_fuzz_3), u32::MAX
        );
        debug_assert_eq!(
            < u32 as WrappingSub > ::wrapping_sub(& rug_fuzz_4, & rug_fuzz_5), 100u32
        );
        debug_assert_eq!(
            < u32 as WrappingSub > ::wrapping_sub(& rug_fuzz_6, & u32::MAX), 1u32
        );
        debug_assert_eq!(
            < u32 as WrappingSub > ::wrapping_sub(& u32::MAX, & u32::MAX), 0u32
        );
        debug_assert_eq!(
            < u32 as WrappingSub > ::wrapping_sub(& u32::MAX, & rug_fuzz_7), u32::MAX
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1761_llm_16_1761 {
    use crate::ops::wrapping::WrappingAdd;
    #[test]
    fn wrapping_add_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u64 as WrappingAdd > ::wrapping_add(& rug_fuzz_0, & rug_fuzz_1), 1
        );
        debug_assert_eq!(
            < u64 as WrappingAdd > ::wrapping_add(& u64::MAX, & rug_fuzz_2), 0
        );
        debug_assert_eq!(
            < u64 as WrappingAdd > ::wrapping_add(& u64::MAX, & rug_fuzz_3), u64::MAX
        );
        debug_assert_eq!(
            < u64 as WrappingAdd > ::wrapping_add(& u64::MAX, & u64::MAX), u64::MAX - 1
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1762_llm_16_1762 {
    use super::*;
    use crate::*;
    #[test]
    fn wrapping_mul_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a = Wrapping(rug_fuzz_0);
        let b = Wrapping(rug_fuzz_1);
        let result = a.wrapping_mul(&b);
        debug_assert_eq!(result, Wrapping(u64::MAX.wrapping_mul(2)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1763_llm_16_1763 {
    use super::*;
    use crate::*;
    use crate::ops::wrapping::WrappingNeg;
    #[test]
    fn test_wrapping_neg() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.wrapping_neg(), 0u64.wrapping_neg());
        debug_assert_eq!(u64::MAX.wrapping_neg(), 1u64.wrapping_neg());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1764_llm_16_1764 {
    use crate::ops::wrapping::WrappingShl;
    #[test]
    fn wrapping_shl_u64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u64, u32, u64, u32, u64, u32, u64, u32, u64, u32, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u64 as WrappingShl > ::wrapping_shl(& rug_fuzz_0, rug_fuzz_1), 0
        );
        debug_assert_eq!(
            < u64 as WrappingShl > ::wrapping_shl(& rug_fuzz_2, rug_fuzz_3), 2
        );
        debug_assert_eq!(
            < u64 as WrappingShl > ::wrapping_shl(& rug_fuzz_4, rug_fuzz_5), 1
        );
        debug_assert_eq!(
            < u64 as WrappingShl > ::wrapping_shl(& rug_fuzz_6, rug_fuzz_7), 1 << 63
        );
        debug_assert_eq!(
            < u64 as WrappingShl > ::wrapping_shl(& rug_fuzz_8, rug_fuzz_9), 1
        );
        debug_assert_eq!(
            < u64 as WrappingShl > ::wrapping_shl(& rug_fuzz_10, rug_fuzz_11), 1 << (127
            % 64)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1765_llm_16_1765 {
    use crate::ops::wrapping::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u32, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u64 = rug_fuzz_0;
        let shift: u32 = rug_fuzz_1;
        let result = WrappingShr::wrapping_shr(&value, shift);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let shift = rug_fuzz_3;
        let result = WrappingShr::wrapping_shr(&value, shift);
        let expected = value;
        debug_assert_eq!(result, expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1766_llm_16_1766 {
    use crate::ops::wrapping::WrappingSub;
    use std::num::Wrapping;
    #[test]
    fn test_wrapping_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Wrapping(rug_fuzz_0).wrapping_sub(& Wrapping(rug_fuzz_1)), Wrapping(0u64)
        );
        debug_assert_eq!(
            Wrapping(rug_fuzz_2).wrapping_sub(& Wrapping(rug_fuzz_3)), Wrapping(u64::MAX)
        );
        debug_assert_eq!(
            Wrapping(u64::MAX).wrapping_sub(& Wrapping(rug_fuzz_4)), Wrapping(u64::MAX)
        );
        debug_assert_eq!(
            Wrapping(u64::MAX).wrapping_sub(& Wrapping(rug_fuzz_5)), Wrapping(u64::MAX -
            1)
        );
        debug_assert_eq!(
            Wrapping(rug_fuzz_6).wrapping_sub(& Wrapping(u64::MAX)), Wrapping(1u64)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1869_llm_16_1869 {
    use crate::ops::wrapping::WrappingNeg;
    #[test]
    fn wrapping_neg_u8() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.wrapping_neg(), 0u8);
        debug_assert_eq!(rug_fuzz_1.wrapping_neg(), 255u8);
        debug_assert_eq!(rug_fuzz_2.wrapping_neg(), 56u8);
        debug_assert_eq!(rug_fuzz_3.wrapping_neg(), 1u8);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1871_llm_16_1871 {
    use crate::ops::wrapping::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, u32, u8, u32, u8, u32, u8, u32, u8, u32, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u8 as WrappingShr > ::wrapping_shr(& rug_fuzz_0, rug_fuzz_1), 0b0000_1111
        );
        debug_assert_eq!(
            < u8 as WrappingShr > ::wrapping_shr(& rug_fuzz_2, rug_fuzz_3), 0b0000_0010
        );
        debug_assert_eq!(
            < u8 as WrappingShr > ::wrapping_shr(& rug_fuzz_4, rug_fuzz_5), 0b0001_0000
        );
        debug_assert_eq!(
            < u8 as WrappingShr > ::wrapping_shr(& rug_fuzz_6, rug_fuzz_7), 0b0100_0000
        );
        debug_assert_eq!(
            < u8 as WrappingShr > ::wrapping_shr(& rug_fuzz_8, rug_fuzz_9), 0
        );
        debug_assert_eq!(
            < u8 as WrappingShr > ::wrapping_shr(& rug_fuzz_10, rug_fuzz_11), 0b0111_1111
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1872_llm_16_1872 {
    use crate::ops::wrapping::WrappingSub;
    #[test]
    fn wrapping_sub_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(WrappingSub::wrapping_sub(& rug_fuzz_0, & rug_fuzz_1), 0u8);
        debug_assert_eq!(WrappingSub::wrapping_sub(& rug_fuzz_2, & rug_fuzz_3), 255u8);
        debug_assert_eq!(WrappingSub::wrapping_sub(& rug_fuzz_4, & rug_fuzz_5), 201u8);
        debug_assert_eq!(WrappingSub::wrapping_sub(& rug_fuzz_6, & rug_fuzz_7), u8::MAX);
        debug_assert_eq!(WrappingSub::wrapping_sub(& u8::MIN, & rug_fuzz_8), u8::MAX);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1972_llm_16_1972 {
    use crate::ops::wrapping::WrappingAdd;
    #[test]
    fn test_wrapping_add() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < usize as WrappingAdd > ::wrapping_add(& rug_fuzz_0, & usize::MAX),
            usize::MAX
        );
        debug_assert_eq!(
            < usize as WrappingAdd > ::wrapping_add(& rug_fuzz_1, & usize::MAX), 0
        );
        debug_assert_eq!(
            < usize as WrappingAdd > ::wrapping_add(& rug_fuzz_2, & rug_fuzz_3), 300
        );
        debug_assert_eq!(
            < usize as WrappingAdd > ::wrapping_add(& usize::MAX, & rug_fuzz_4), 0
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1973_llm_16_1973 {
    use crate::ops::wrapping::WrappingMul;
    #[test]
    fn test_wrapping_mul() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.wrapping_mul(rug_fuzz_1), 10);
        debug_assert_eq!(usize::MAX.wrapping_mul(rug_fuzz_2), usize::MAX - 1);
        debug_assert_eq!(rug_fuzz_3.wrapping_mul(usize::MAX), 0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1974 {
    use crate::WrappingNeg;
    use std::ops::Neg;
    #[test]
    fn wrapping_neg_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let zero: usize = rug_fuzz_0;
        let max = usize::MAX;
        debug_assert_eq!(WrappingNeg::wrapping_neg(& zero), zero.wrapping_neg());
        debug_assert_eq!(WrappingNeg::wrapping_neg(& max), max.wrapping_neg());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1975_llm_16_1975 {
    use crate::ops::wrapping::WrappingShl;
    #[test]
    fn test_wrapping_shl() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: usize = rug_fuzz_0;
        debug_assert_eq!(WrappingShl::wrapping_shl(& value, rug_fuzz_1), 1);
        debug_assert_eq!(WrappingShl::wrapping_shl(& value, rug_fuzz_2), 32);
        debug_assert_eq!(WrappingShl::wrapping_shl(& value, usize::BITS), 1);
        debug_assert_eq!(WrappingShl::wrapping_shl(& value, rug_fuzz_3), 1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1976_llm_16_1976 {
    use crate::ops::wrapping::WrappingShr;
    #[test]
    fn test_wrapping_shr() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(usize, u32, usize, u32, usize, u32, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < usize as WrappingShr > ::wrapping_shr(& rug_fuzz_0, rug_fuzz_1),
            0x7FFF_FFFF
        );
        debug_assert_eq!(
            < usize as WrappingShr > ::wrapping_shr(& rug_fuzz_2, rug_fuzz_3),
            0xFFFF_FFFF
        );
        debug_assert_eq!(
            < usize as WrappingShr > ::wrapping_shr(& rug_fuzz_4, rug_fuzz_5), if
            usize::BITS == 32 { 0 } else { 1 << (usize::BITS - 32) }
        );
        debug_assert_eq!(
            < usize as WrappingShr > ::wrapping_shr(& rug_fuzz_6, usize::BITS -
            rug_fuzz_7), 1
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_1977_llm_16_1977 {
    use super::*;
    use crate::*;
    use crate::ops::wrapping::WrappingSub;
    #[test]
    fn test_wrapping_sub() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            WrappingSub::wrapping_sub(& rug_fuzz_0, & rug_fuzz_1), usize::MAX - 1
        );
        debug_assert_eq!(
            WrappingSub::wrapping_sub(& rug_fuzz_2, & rug_fuzz_3), usize::MAX
        );
        debug_assert_eq!(
            WrappingSub::wrapping_sub(& usize::MAX, & rug_fuzz_4), usize::MAX
        );
        debug_assert_eq!(WrappingSub::wrapping_sub(& usize::MAX, & usize::MAX), 0);
             }
}
}
}    }
}
