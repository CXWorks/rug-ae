use crate::backport::*;
use crate::identifier::Identifier;
use crate::{BuildMetadata, Comparator, Prerelease, VersionReq};
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::iter::FromIterator;
use core::ops::Deref;
impl Default for Identifier {
    fn default() -> Self {
        Identifier::empty()
    }
}
impl Eq for Identifier {}
impl Hash for Identifier {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher);
    }
}
impl Deref for Prerelease {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.identifier.as_str()
    }
}
impl Deref for BuildMetadata {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.identifier.as_str()
    }
}
impl PartialOrd for Prerelease {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, rhs))
    }
}
impl PartialOrd for BuildMetadata {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, rhs))
    }
}
impl Ord for Prerelease {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.is_empty() {
            true if rhs.is_empty() => return Ordering::Equal,
            true => return Ordering::Greater,
            false if rhs.is_empty() => return Ordering::Less,
            false => {}
        }
        let lhs = self.as_str().split('.');
        let mut rhs = rhs.as_str().split('.');
        for lhs in lhs {
            let rhs = match rhs.next() {
                None => return Ordering::Greater,
                Some(rhs) => rhs,
            };
            let string_cmp = || Ord::cmp(lhs, rhs);
            let is_ascii_digit = |b: u8| b.is_ascii_digit();
            let ordering = match (
                lhs.bytes().all(is_ascii_digit),
                rhs.bytes().all(is_ascii_digit),
            ) {
                (true, true) => Ord::cmp(&lhs.len(), &rhs.len()).then_with(string_cmp),
                (true, false) => return Ordering::Less,
                (false, true) => return Ordering::Greater,
                (false, false) => string_cmp(),
            };
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        if rhs.next().is_none() { Ordering::Equal } else { Ordering::Less }
    }
}
impl Ord for BuildMetadata {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let lhs = self.as_str().split('.');
        let mut rhs = rhs.as_str().split('.');
        for lhs in lhs {
            let rhs = match rhs.next() {
                None => return Ordering::Greater,
                Some(rhs) => rhs,
            };
            let is_ascii_digit = |b: u8| b.is_ascii_digit();
            let ordering = match (
                lhs.bytes().all(is_ascii_digit),
                rhs.bytes().all(is_ascii_digit),
            ) {
                (true, true) => {
                    let lhval = lhs.trim_start_matches('0');
                    let rhval = rhs.trim_start_matches('0');
                    Ord::cmp(&lhval.len(), &rhval.len())
                        .then_with(|| Ord::cmp(lhval, rhval))
                        .then_with(|| Ord::cmp(&lhs.len(), &rhs.len()))
                }
                (true, false) => return Ordering::Less,
                (false, true) => return Ordering::Greater,
                (false, false) => Ord::cmp(lhs, rhs),
            };
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        if rhs.next().is_none() { Ordering::Equal } else { Ordering::Less }
    }
}
impl FromIterator<Comparator> for VersionReq {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Comparator>,
    {
        let comparators = Vec::from_iter(iter);
        VersionReq { comparators }
    }
}
#[cfg(test)]
mod tests_llm_16_43 {
    use super::*;
    use crate::*;
    use std::cmp::Ordering;
    #[test]
    fn test_cmp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let meta1 = BuildMetadata::new(rug_fuzz_0).unwrap();
        let meta2 = BuildMetadata::new(rug_fuzz_1).unwrap();
        let meta3 = BuildMetadata::new(rug_fuzz_2).unwrap();
        let meta4 = BuildMetadata::new(rug_fuzz_3).unwrap();
        let meta5 = BuildMetadata::new(rug_fuzz_4).unwrap();
        let meta6 = BuildMetadata::new(rug_fuzz_5).unwrap();
        let meta7 = BuildMetadata::new(rug_fuzz_6).unwrap();
        let meta8 = BuildMetadata::new(rug_fuzz_7).unwrap();
        let meta9 = BuildMetadata::new(rug_fuzz_8).unwrap();
        debug_assert_eq!(meta1.cmp(& meta2), Ordering::Equal);
        debug_assert_eq!(meta1.cmp(& meta3), Ordering::Less);
        debug_assert_eq!(meta3.cmp(& meta1), Ordering::Greater);
        debug_assert_eq!(meta1.cmp(& meta4), Ordering::Less);
        debug_assert_eq!(meta5.cmp(& meta6), Ordering::Less);
        debug_assert_eq!(meta7.cmp(& meta7), Ordering::Equal);
        debug_assert_eq!(meta7.cmp(& meta9), Ordering::Less);
        debug_assert_eq!(meta8.cmp(& meta9), Ordering::Less);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use crate::Prerelease;
    use std::cmp::Ordering;
    #[test]
    fn test_prerelease_cmp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let examples = vec![
            (rug_fuzz_0, rug_fuzz_1, Ordering::Equal), ("", "1.0.0", Ordering::Less),
            ("1.0.0", "", Ordering::Greater), ("alpha", "alpha", Ordering::Equal),
            ("alpha.1", "alpha.2", Ordering::Less), ("alpha.2", "alpha.1",
            Ordering::Greater), ("alpha", "alpha.1", Ordering::Less), ("alpha.2",
            "alpha.11", Ordering::Less), ("alpha.11", "alpha.2", Ordering::Greater),
            ("alpha.2", "alpha.a", Ordering::Less), ("beta", "alpha", Ordering::Greater),
            ("beta.2", "alpha", Ordering::Greater), ("alpha", "beta", Ordering::Less),
            ("0.3", "11", Ordering::Less), ("11", "0.3", Ordering::Greater), ("1.2.3",
            "1.2.3.4", Ordering::Less), ("1.2.3.5", "1.2.3.4", Ordering::Greater)
        ];
        for (a, b, expected) in examples {
            let a_pr = a.parse::<Prerelease>().expect(rug_fuzz_2);
            let b_pr = b.parse::<Prerelease>().expect(rug_fuzz_3);
            debug_assert_eq!(a_pr.cmp(& b_pr), expected, "Comparing {} and {}", a, b);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;
    use crate::*;
    use std::cmp::Ordering;
    #[test]
    fn test_partial_cmp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let metadata1 = BuildMetadata::new(rug_fuzz_0).unwrap();
        let metadata2 = BuildMetadata::new(rug_fuzz_1).unwrap();
        let metadata3 = BuildMetadata::new(rug_fuzz_2).unwrap();
        let metadata4 = BuildMetadata::new(rug_fuzz_3).unwrap();
        let metadata5 = BuildMetadata::new(rug_fuzz_4).unwrap();
        let metadata6 = BuildMetadata::new(rug_fuzz_5).unwrap();
        let metadata7 = BuildMetadata::new(rug_fuzz_6).unwrap();
        let metadata8 = BuildMetadata::new(rug_fuzz_7).unwrap();
        let metadata9 = BuildMetadata::new(rug_fuzz_8).unwrap();
        debug_assert_eq!(metadata1.partial_cmp(& metadata2), Some(Ordering::Equal));
        debug_assert_eq!(metadata1.partial_cmp(& metadata3), Some(Ordering::Less));
        debug_assert_eq!(metadata3.partial_cmp(& metadata1), Some(Ordering::Greater));
        debug_assert_eq!(metadata4.partial_cmp(& metadata5), Some(Ordering::Equal));
        debug_assert_eq!(metadata4.partial_cmp(& metadata6), Some(Ordering::Less));
        debug_assert_eq!(metadata6.partial_cmp(& metadata4), Some(Ordering::Greater));
        debug_assert_eq!(metadata7.partial_cmp(& metadata8), Some(Ordering::Equal));
        debug_assert_eq!(metadata7.partial_cmp(& metadata9), Some(Ordering::Less));
        debug_assert_eq!(metadata9.partial_cmp(& metadata8), Some(Ordering::Greater));
        debug_assert_eq!(metadata1.partial_cmp(& metadata4), Some(Ordering::Greater));
        debug_assert_eq!(metadata4.partial_cmp(& metadata1), Some(Ordering::Less));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_46 {
    use super::*;
    use crate::*;
    use std::cmp::Ordering;
    #[test]
    fn test_partial_cmp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let prerelease1 = Prerelease::new(rug_fuzz_0).unwrap();
        let prerelease2 = Prerelease::new(rug_fuzz_1).unwrap();
        let prerelease3 = Prerelease::new(rug_fuzz_2).unwrap();
        let prerelease4 = Prerelease::new(rug_fuzz_3).unwrap();
        let prerelease5 = Prerelease::new(rug_fuzz_4).unwrap();
        let prerelease6 = Prerelease::new(rug_fuzz_5).unwrap();
        debug_assert_eq!(prerelease1.partial_cmp(& prerelease2), Some(Ordering::Less));
        debug_assert_eq!(
            prerelease2.partial_cmp(& prerelease1), Some(Ordering::Greater)
        );
        debug_assert_eq!(prerelease2.partial_cmp(& prerelease3), Some(Ordering::Less));
        debug_assert_eq!(prerelease3.partial_cmp(& prerelease4), Some(Ordering::Less));
        debug_assert_eq!(prerelease4.partial_cmp(& prerelease5), Some(Ordering::Less));
        debug_assert_eq!(prerelease5.partial_cmp(& prerelease6), Some(Ordering::Equal));
        debug_assert_eq!(
            prerelease6.partial_cmp(& prerelease1), Some(Ordering::Greater)
        );
        debug_assert_eq!(prerelease1.partial_cmp(& prerelease1), Some(Ordering::Equal));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_47_llm_16_47 {
    use crate::Identifier;
    use std::default::Default;
    #[test]
    fn test_default_identifier_is_empty() {
        let _rug_st_tests_llm_16_47_llm_16_47_rrrruuuugggg_test_default_identifier_is_empty = 0;
        let default_identifier: Identifier = Default::default();
        debug_assert!(default_identifier.is_empty());
        let _rug_ed_tests_llm_16_47_llm_16_47_rrrruuuugggg_test_default_identifier_is_empty = 0;
    }
    #[test]
    fn test_default_identifier_equality() {
        let _rug_st_tests_llm_16_47_llm_16_47_rrrruuuugggg_test_default_identifier_equality = 0;
        let default_identifier1: Identifier = Default::default();
        let default_identifier2: Identifier = Default::default();
        if default_identifier1 != default_identifier2 {
            panic!("Default identifiers are not equal");
        }
        let _rug_ed_tests_llm_16_47_llm_16_47_rrrruuuugggg_test_default_identifier_equality = 0;
    }
    #[test]
    fn test_default_identifier_hash() {
        let _rug_st_tests_llm_16_47_llm_16_47_rrrruuuugggg_test_default_identifier_hash = 0;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let default_identifier: Identifier = Default::default();
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        default_identifier.hash(&mut hasher1);
        default_identifier.hash(&mut hasher2);
        debug_assert_eq!(hasher1.finish(), hasher2.finish());
        let _rug_ed_tests_llm_16_47_llm_16_47_rrrruuuugggg_test_default_identifier_hash = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_48_llm_16_48 {
    use super::*;
    use crate::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    #[test]
    fn hash_empty_identifier() {
        let _rug_st_tests_llm_16_48_llm_16_48_rrrruuuugggg_hash_empty_identifier = 0;
        let identifier = Identifier::default();
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        identifier.hash(&mut hasher1);
        Identifier::default().hash(&mut hasher2);
        debug_assert_eq!(hasher1.finish(), hasher2.finish());
        let _rug_ed_tests_llm_16_48_llm_16_48_rrrruuuugggg_hash_empty_identifier = 0;
    }
    #[test]
    fn hash_non_empty_identifier() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let identifier1 = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        let identifier2 = unsafe { Identifier::new_unchecked(rug_fuzz_1) };
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        let mut hasher3 = DefaultHasher::new();
        identifier1.hash(&mut hasher1);
        identifier1.clone().hash(&mut hasher2);
        identifier2.hash(&mut hasher3);
        debug_assert_eq!(hasher1.finish(), hasher2.finish());
        debug_assert_ne!(hasher1.finish(), hasher3.finish());
             }
});    }
    #[test]
    fn hash_consistent_with_equality() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let identifier1 = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        let identifier2 = unsafe { Identifier::new_unchecked(rug_fuzz_1) };
        let identifier3 = unsafe { Identifier::new_unchecked(rug_fuzz_2) };
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        let mut hasher3 = DefaultHasher::new();
        identifier1.hash(&mut hasher1);
        identifier2.hash(&mut hasher2);
        identifier3.hash(&mut hasher3);
        let identifier1_bytes = identifier1.as_str().as_bytes();
        let identifier2_bytes = identifier2.as_str().as_bytes();
        let identifier3_bytes = identifier3.as_str().as_bytes();
        debug_assert_eq!(hasher1.finish(), hasher2.finish());
        debug_assert_ne!(hasher1.finish(), hasher3.finish());
        debug_assert_eq!(identifier1_bytes, identifier2_bytes);
        debug_assert_ne!(identifier1_bytes, identifier3_bytes);
             }
});    }
    #[test]
    fn hash_handles_inline_and_heap_allocated_identifiers() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let identifier_inline = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        let identifier_heap = unsafe { Identifier::new_unchecked(rug_fuzz_1) };
        let mut hasher_inline = DefaultHasher::new();
        let mut hasher_heap = DefaultHasher::new();
        identifier_inline.hash(&mut hasher_inline);
        identifier_heap.hash(&mut hasher_heap);
        debug_assert_ne!(hasher_inline.finish(), hasher_heap.finish());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_49 {
    use super::*;
    use crate::*;
    use crate::{Comparator, Op, VersionReq, Version, Prerelease};
    #[test]
    fn from_iter_creates_version_req_with_given_comparators() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v1 = Comparator {
            op: Op::GreaterEq,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let v2 = Comparator {
            op: Op::Less,
            major: rug_fuzz_3,
            minor: None,
            patch: None,
            pre: Prerelease::EMPTY,
        };
        let comparators = vec![v1, v2];
        let version_req = VersionReq::from_iter(comparators.clone());
        debug_assert_eq!(version_req.comparators, comparators);
             }
});    }
    #[test]
    fn from_iter_creates_star_version_req_for_empty_iter() {
        let _rug_st_tests_llm_16_49_rrrruuuugggg_from_iter_creates_star_version_req_for_empty_iter = 0;
        let comparators: Vec<Comparator> = Vec::new();
        let version_req = VersionReq::from_iter(comparators);
        debug_assert_eq!(version_req.comparators, Vec::new());
        let _rug_ed_tests_llm_16_49_rrrruuuugggg_from_iter_creates_star_version_req_for_empty_iter = 0;
    }
    #[test]
    fn from_iter_supports_iterator_chain() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, u64, u64, u64, u64, u64, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let comparators = vec![
            Comparator { op : Op::Tilde, major : rug_fuzz_0, minor : Some(rug_fuzz_1),
            patch : Some(rug_fuzz_2), pre : Prerelease::EMPTY, }, Comparator { op :
            Op::Caret, major : 2, minor : Some(0), patch : Some(0), pre :
            Prerelease::EMPTY, }
        ];
        let version_req = comparators
            .into_iter()
            .chain(
                std::iter::once(Comparator {
                    op: Op::Exact,
                    major: rug_fuzz_3,
                    minor: Some(rug_fuzz_4),
                    patch: Some(rug_fuzz_5),
                    pre: Prerelease::EMPTY,
                }),
            )
            .collect::<VersionReq>();
        debug_assert!(version_req.matches(& Version::parse(rug_fuzz_6).unwrap()));
        debug_assert!(version_req.matches(& Version::parse(rug_fuzz_7).unwrap()));
        debug_assert!(version_req.matches(& Version::parse(rug_fuzz_8).unwrap()));
        debug_assert!(! version_req.matches(& Version::parse(rug_fuzz_9).unwrap()));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_51_llm_16_51 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    use std::ops::Deref;
    #[test]
    fn prerelease_deref_returns_identifier_as_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert_eq!(& * prerelease, "alpha.1");
             }
});    }
    #[test]
    fn prerelease_deref_returns_identifier_as_str_with_multiple_parts() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert_eq!(& * prerelease, "beta.2.3");
             }
});    }
    #[test]
    fn prerelease_deref_empty() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert_eq!(& * prerelease, "");
             }
});    }
    #[test]
    fn prerelease_deref_with_hyphens() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert_eq!(& * prerelease, "alpha-1");
             }
});    }
    #[test]
    fn prerelease_deref_with_mixed_characters() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert_eq!(& * prerelease, "rc-123.x.y");
             }
});    }
    #[test]
    fn prerelease_deref_numeric() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert_eq!(& * prerelease, "12345");
             }
});    }
    #[test]
    fn prerelease_deref_with_leading_zeros() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
        debug_assert_eq!(& * prerelease, "00123");
             }
});    }
    #[test]
    #[should_panic(expected = "IllegalCharacter")]
    fn prerelease_deref_with_illegal_characters() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
             }
});    }
    #[test]
    #[should_panic(expected = "IllegalCharacter")]
    fn prerelease_deref_with_empty_parts() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _prerelease = Prerelease::from_str(rug_fuzz_0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use std::ops::Deref;
    use crate::BuildMetadata;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = BuildMetadata::new(rug_fuzz_0).unwrap();
        debug_assert_eq!(p0.deref(), "1.5.0");
             }
});    }
}
