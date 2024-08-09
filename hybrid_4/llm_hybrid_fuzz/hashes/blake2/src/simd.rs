mod simd_opt;
mod simdint;
mod simdop;
mod simdty;
pub use self::simdty::{u32x4, u64x4};
pub trait Vector4<T>: Copy {
    fn gather(src: &[T], i0: usize, i1: usize, i2: usize, i3: usize) -> Self;
    #[allow(clippy::wrong_self_convention)]
    fn from_le(self) -> Self;
    fn to_le(self) -> Self;
    fn wrapping_add(self, rhs: Self) -> Self;
    fn rotate_right_const(self, n: u32) -> Self;
    fn shuffle_left_1(self) -> Self;
    fn shuffle_left_2(self) -> Self;
    fn shuffle_left_3(self) -> Self;
    #[inline(always)]
    fn shuffle_right_1(self) -> Self {
        self.shuffle_left_3()
    }
    #[inline(always)]
    fn shuffle_right_2(self) -> Self {
        self.shuffle_left_2()
    }
    #[inline(always)]
    fn shuffle_right_3(self) -> Self {
        self.shuffle_left_1()
    }
}
macro_rules! impl_vector4 {
    ($vec:ident, $word:ident) => {
        impl Vector4 <$word > for $vec { #[inline(always)] fn gather(src : & [$word], i0
        : usize, i1 : usize, i2 : usize, i3 : usize) -> Self { $vec ::new(src[i0],
        src[i1], src[i2], src[i3]) } #[cfg(target_endian = "little")] #[inline(always)]
        fn from_le(self) -> Self { self } #[cfg(not(target_endian = "little"))]
        #[inline(always)] fn from_le(self) -> Self { $vec ::new($word ::from_le(self.0),
        $word ::from_le(self.1), $word ::from_le(self.2), $word ::from_le(self.3),) }
        #[cfg(target_endian = "little")] #[inline(always)] fn to_le(self) -> Self { self
        } #[cfg(not(target_endian = "little"))] #[inline(always)] fn to_le(self) -> Self
        { $vec ::new(self.0.to_le(), self.1.to_le(), self.2.to_le(), self.3.to_le(),) }
        #[inline(always)] fn wrapping_add(self, rhs : Self) -> Self { self + rhs }
        #[inline(always)] fn rotate_right_const(self, n : u32) -> Self { simd_opt::$vec
        ::rotate_right_const(self, n) } #[cfg(feature = "simd")] #[inline(always)] fn
        shuffle_left_1(self) -> Self { use crate ::simd::simdint::simd_shuffle4; const
        IDX : [u32; 4] = [1, 2, 3, 0]; unsafe { simd_shuffle4(self, self, IDX) } }
        #[cfg(not(feature = "simd"))] #[inline(always)] fn shuffle_left_1(self) -> Self {
        $vec ::new(self.1, self.2, self.3, self.0) } #[cfg(feature = "simd")]
        #[inline(always)] fn shuffle_left_2(self) -> Self { use crate
        ::simd::simdint::simd_shuffle4; const IDX : [u32; 4] = [2, 3, 0, 1]; unsafe {
        simd_shuffle4(self, self, IDX) } } #[cfg(not(feature = "simd"))]
        #[inline(always)] fn shuffle_left_2(self) -> Self { $vec ::new(self.2, self.3,
        self.0, self.1) } #[cfg(feature = "simd")] #[inline(always)] fn
        shuffle_left_3(self) -> Self { use crate ::simd::simdint::simd_shuffle4; const
        IDX : [u32; 4] = [3, 0, 1, 2]; unsafe { simd_shuffle4(self, self, IDX) } }
        #[cfg(not(feature = "simd"))] #[inline(always)] fn shuffle_left_3(self) -> Self {
        $vec ::new(self.3, self.0, self.1, self.2) } }
    };
}
impl_vector4!(u32x4, u32);
impl_vector4!(u64x4, u64);
#[cfg(test)]
mod tests_llm_16_19_llm_16_19 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let original = simd::simdty::Simd4::new(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let result = <simd::simdty::Simd4<u32> as simd::Vector4<u32>>::from_le(original);
        debug_assert_eq!(
            (original.0, original.1, original.2, original.3), (result.0, result.1, result
            .2, result.3)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_28_llm_16_28 {
    use crate::simd::simdty::Simd4;
    use crate::simd::Vector4;
    #[test]
    fn gather_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u64, u64, u64, u64, u64, u64, u64, u64, usize, usize, usize, usize, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let source = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let gathered = Simd4::gather(
            &source,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
        );
        let expected = Simd4::new(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14, rug_fuzz_15);
        debug_assert!(
            gathered.0 == expected.0 && gathered.1 == expected.1 && gathered.2 ==
            expected.2 && gathered.3 == expected.3
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_30_llm_16_30 {
    use crate::simd::simdty::Simd4;
    use crate::simd::Vector4;
    #[test]
    fn test_shuffle_left_1() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let original = Simd4::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let expected = Simd4::new(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let result = Vector4::shuffle_left_1(original);
        debug_assert!(
            result.0 == expected.0 && result.1 == expected.1 && result.2 == expected.2 &&
            result.3 == expected.3
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_33_llm_16_33 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let original = simd::simdty::Simd4::new(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let converted = original.to_le();
        debug_assert_eq!((original.0).to_le(), (converted.0).to_le());
        debug_assert_eq!((original.1).to_le(), (converted.1).to_le());
        debug_assert_eq!((original.2).to_le(), (converted.2).to_le());
        debug_assert_eq!((original.3).to_le(), (converted.3).to_le());
             }
});    }
}
#[cfg(test)]
mod tests_rug_38 {
    use super::*;
    #[test]
    fn test_shuffle_right_1() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u32, u32, u32, u32, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Vector4Impl<u32> = Vector4Impl::gather(
            &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3],
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        );
        let _result = p0.shuffle_right_1();
             }
});    }
    #[derive(Copy, Clone)]
    struct Vector4Impl<T>([T; 4]);
    impl<T> Vector4<T> for Vector4Impl<T>
    where
        T: Copy,
    {
        fn gather(src: &[T], i0: usize, i1: usize, i2: usize, i3: usize) -> Self {
            Vector4Impl([src[i0], src[i1], src[i2], src[i3]])
        }
        fn from_le(self) -> Self {
            unimplemented!()
        }
        fn to_le(self) -> Self {
            unimplemented!()
        }
        fn wrapping_add(self, rhs: Self) -> Self {
            unimplemented!()
        }
        fn rotate_right_const(self, _: u32) -> Self {
            unimplemented!()
        }
        fn shuffle_left_1(self) -> Self {
            unimplemented!()
        }
        fn shuffle_left_2(self) -> Self {
            unimplemented!()
        }
        fn shuffle_left_3(self) -> Self {
            Vector4Impl([self.0[3], self.0[0], self.0[1], self.0[2]])
        }
    }
}
#[cfg(test)]
mod tests_rug_39 {
    use super::*;
    use core::marker::Copy;
    #[derive(Copy, Clone)]
    struct TestVector4([u32; 4]);
    impl Vector4<u32> for TestVector4 {
        fn gather(src: &[u32], i0: usize, i1: usize, i2: usize, i3: usize) -> Self {
            TestVector4([src[i0], src[i1], src[i2], src[i3]])
        }
        fn from_le(self) -> Self {
            self
        }
        fn to_le(self) -> Self {
            self
        }
        fn wrapping_add(self, rhs: Self) -> Self {
            let mut result = [0u32; 4];
            for i in 0..4 {
                result[i] = self.0[i].wrapping_add(rhs.0[i]);
            }
            TestVector4(result)
        }
        fn rotate_right_const(self, n: u32) -> Self {
            self
        }
        fn shuffle_left_1(self) -> Self {
            self
        }
        fn shuffle_left_2(self) -> Self {
            TestVector4([self.0[2], self.0[3], self.0[0], self.0[1]])
        }
        fn shuffle_left_3(self) -> Self {
            self
        }
    }
    #[test]
    fn test_shuffle_right_2() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = TestVector4([rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        let shuffled = p0.shuffle_right_2();
        debug_assert_eq!(shuffled.0, [3, 4, 1, 2]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    #[test]
    fn test_shuffle_right_3() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u32, u32, u32, u32, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &[u32; 4] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let p0 = Vector4Impl::gather(p0, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        Vector4Impl::shuffle_right_3(p0);
             }
});    }
    #[derive(Copy, Clone)]
    struct Vector4Impl(u32, u32, u32, u32);
    impl Vector4<u32> for Vector4Impl {
        fn gather(src: &[u32], i0: usize, i1: usize, i2: usize, i3: usize) -> Self {
            Self(src[i0], src[i1], src[i2], src[i3])
        }
        fn from_le(self) -> Self {
            self
        }
        fn to_le(self) -> Self {
            self
        }
        fn wrapping_add(self, rhs: Self) -> Self {
            Self(
                self.0.wrapping_add(rhs.0),
                self.1.wrapping_add(rhs.1),
                self.2.wrapping_add(rhs.2),
                self.3.wrapping_add(rhs.3),
            )
        }
        fn rotate_right_const(self, _: u32) -> Self {
            unimplemented!()
        }
        fn shuffle_left_1(self) -> Self {
            Self(self.1, self.2, self.3, self.0)
        }
        fn shuffle_left_2(self) -> Self {
            unimplemented!()
        }
        fn shuffle_left_3(self) -> Self {
            unimplemented!()
        }
    }
}
#[cfg(test)]
mod tests_rug_42 {
    use crate::simd::simdty::Simd4;
    use crate::simd::Vector4;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Simd4::<u32>::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        <Simd4<u32> as Vector4<u32>>::to_le(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_43 {
    use crate::simd::simdty::Simd4;
    use crate::simd::Vector4;
    #[test]
    fn test_wrapping_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = Simd4::<u32>::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let p1 = Simd4::<u32>::new(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        <Simd4<u32> as Vector4<u32>>::wrapping_add(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_44 {
    use crate::simd::Vector4;
    use crate::simd::simdty::Simd4;
    #[test]
    fn test_rotate_right_const() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Simd4::<u32>::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let mut p1: u32 = rug_fuzz_4;
        <Simd4<u32> as Vector4<u32>>::rotate_right_const(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_48 {
    use super::*;
    use crate::simd::simdty::Simd4;
    use crate::simd::Vector4;
    #[test]
    fn test_from_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Simd4<u64> = Simd4::<
            u64,
        >(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        p0.from_le();
             }
});    }
}
#[cfg(test)]
mod tests_rug_49 {
    use crate::simd::simdty::Simd4;
    use crate::simd::Vector4;
    #[test]
    fn test_wrapping_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Simd4<u64> = Simd4::new(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let mut p1: Simd4<u64> = Simd4::new(
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        );
        p0.wrapping_add(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_50 {
    use super::*;
    use crate::simd::{self, simdty::Simd4, Vector4};
    #[test]
    fn test_rotate_right_const() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u64, u64, u64, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Simd4<u64> = Simd4::new(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let mut p1: u32 = rug_fuzz_4;
        <Simd4<u64> as Vector4<u64>>::rotate_right_const(p0, p1);
             }
});    }
}
