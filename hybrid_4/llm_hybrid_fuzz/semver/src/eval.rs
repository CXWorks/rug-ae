use crate::{Comparator, Op, Version, VersionReq};
pub(crate) fn matches_req(req: &VersionReq, ver: &Version) -> bool {
    for cmp in &req.comparators {
        if !matches_impl(cmp, ver) {
            return false;
        }
    }
    if ver.pre.is_empty() {
        return true;
    }
    for cmp in &req.comparators {
        if pre_is_compatible(cmp, ver) {
            return true;
        }
    }
    false
}
pub(crate) fn matches_comparator(cmp: &Comparator, ver: &Version) -> bool {
    matches_impl(cmp, ver) && (ver.pre.is_empty() || pre_is_compatible(cmp, ver))
}
fn matches_impl(cmp: &Comparator, ver: &Version) -> bool {
    match cmp.op {
        Op::Exact | Op::Wildcard => matches_exact(cmp, ver),
        Op::Greater => matches_greater(cmp, ver),
        Op::GreaterEq => matches_exact(cmp, ver) || matches_greater(cmp, ver),
        Op::Less => matches_less(cmp, ver),
        Op::LessEq => matches_exact(cmp, ver) || matches_less(cmp, ver),
        Op::Tilde => matches_tilde(cmp, ver),
        Op::Caret => matches_caret(cmp, ver),
        #[cfg(no_non_exhaustive)]
        Op::__NonExhaustive => unreachable!(),
    }
}
fn matches_exact(cmp: &Comparator, ver: &Version) -> bool {
    if ver.major != cmp.major {
        return false;
    }
    if let Some(minor) = cmp.minor {
        if ver.minor != minor {
            return false;
        }
    }
    if let Some(patch) = cmp.patch {
        if ver.patch != patch {
            return false;
        }
    }
    ver.pre == cmp.pre
}
fn matches_greater(cmp: &Comparator, ver: &Version) -> bool {
    if ver.major != cmp.major {
        return ver.major > cmp.major;
    }
    match cmp.minor {
        None => return false,
        Some(minor) => {
            if ver.minor != minor {
                return ver.minor > minor;
            }
        }
    }
    match cmp.patch {
        None => return false,
        Some(patch) => {
            if ver.patch != patch {
                return ver.patch > patch;
            }
        }
    }
    ver.pre > cmp.pre
}
fn matches_less(cmp: &Comparator, ver: &Version) -> bool {
    if ver.major != cmp.major {
        return ver.major < cmp.major;
    }
    match cmp.minor {
        None => return false,
        Some(minor) => {
            if ver.minor != minor {
                return ver.minor < minor;
            }
        }
    }
    match cmp.patch {
        None => return false,
        Some(patch) => {
            if ver.patch != patch {
                return ver.patch < patch;
            }
        }
    }
    ver.pre < cmp.pre
}
fn matches_tilde(cmp: &Comparator, ver: &Version) -> bool {
    if ver.major != cmp.major {
        return false;
    }
    if let Some(minor) = cmp.minor {
        if ver.minor != minor {
            return false;
        }
    }
    if let Some(patch) = cmp.patch {
        if ver.patch != patch {
            return ver.patch > patch;
        }
    }
    ver.pre >= cmp.pre
}
fn matches_caret(cmp: &Comparator, ver: &Version) -> bool {
    if ver.major != cmp.major {
        return false;
    }
    let minor = match cmp.minor {
        None => return true,
        Some(minor) => minor,
    };
    let patch = match cmp.patch {
        None => {
            if cmp.major > 0 {
                return ver.minor >= minor;
            } else {
                return ver.minor == minor;
            }
        }
        Some(patch) => patch,
    };
    if cmp.major > 0 {
        if ver.minor != minor {
            return ver.minor > minor;
        } else if ver.patch != patch {
            return ver.patch > patch;
        }
    } else if minor > 0 {
        if ver.minor != minor {
            return false;
        } else if ver.patch != patch {
            return ver.patch > patch;
        }
    } else if ver.minor != minor || ver.patch != patch {
        return false;
    }
    ver.pre >= cmp.pre
}
fn pre_is_compatible(cmp: &Comparator, ver: &Version) -> bool {
    cmp.major == ver.major && cmp.minor == Some(ver.minor)
        && cmp.patch == Some(ver.patch) && !cmp.pre.is_empty()
}
#[cfg(test)]
mod tests_llm_16_19_llm_16_19 {
    use super::*;
    use crate::*;
    use crate::{Comparator, Op, Prerelease, Version, BuildMetadata};
    #[test]
    fn test_matches_caret_major() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let cmp = Comparator {
            op: Op::Caret,
            major: rug_fuzz_0,
            minor: None,
            patch: None,
            pre: Prerelease::EMPTY,
        };
        let ver = Version::new(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(matches_caret(& cmp, & ver));
        let ver = Version::new(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert!(! matches_caret(& cmp, & ver));
             }
});    }
    #[test]
    fn test_matches_caret_minor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let cmp = Comparator {
            op: Op::Caret,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: None,
            pre: Prerelease::EMPTY,
        };
        let ver = Version::new(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        debug_assert!(matches_caret(& cmp, & ver));
        let ver = Version::new(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        debug_assert!(! matches_caret(& cmp, & ver));
        let ver = Version::new(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10);
        debug_assert!(matches_caret(& cmp, & ver));
             }
});    }
    #[test]
    fn test_matches_caret_patch() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let cmp = Comparator {
            op: Op::Caret,
            major: rug_fuzz_0,
            minor: Some(rug_fuzz_1),
            patch: Some(rug_fuzz_2),
            pre: Prerelease::EMPTY,
        };
        let ver = Version::new(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert!(matches_caret(& cmp, & ver));
        let ver = Version::new(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert!(! matches_caret(& cmp, & ver));
        let ver = Version::new(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert!(matches_caret(& cmp, & ver));
             }
});    }
    #[test]
    fn test_matches_caret_pre() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(&str, u64, u64, u64, u64, u64, u64, &str, u64, u64, u64, &str, u64, u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let pre = Prerelease::new(rug_fuzz_0).unwrap();
        let cmp = Comparator {
            op: Op::Caret,
            major: rug_fuzz_1,
            minor: Some(rug_fuzz_2),
            patch: Some(rug_fuzz_3),
            pre,
        };
        let ver = Version {
            major: rug_fuzz_4,
            minor: rug_fuzz_5,
            patch: rug_fuzz_6,
            pre: Prerelease::new(rug_fuzz_7).unwrap(),
            build: BuildMetadata::EMPTY,
        };
        debug_assert!(matches_caret(& cmp, & ver));
        let ver = Version {
            major: rug_fuzz_8,
            minor: rug_fuzz_9,
            patch: rug_fuzz_10,
            pre: Prerelease::new(rug_fuzz_11).unwrap(),
            build: BuildMetadata::EMPTY,
        };
        debug_assert!(! matches_caret(& cmp, & ver));
        let ver = Version {
            major: rug_fuzz_12,
            minor: rug_fuzz_13,
            patch: rug_fuzz_14,
            pre: Prerelease::new(rug_fuzz_15).unwrap(),
            build: BuildMetadata::EMPTY,
        };
        debug_assert!(matches_caret(& cmp, & ver));
             }
});    }
    #[test]
    fn test_matches_caret_build() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u64, u64, u64, u64, &str, u64, u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let cmp = Comparator {
            op: Op::Caret,
            major: rug_fuzz_0,
            minor: None,
            patch: None,
            pre: Prerelease::EMPTY,
        };
        let ver = Version {
            major: rug_fuzz_1,
            minor: rug_fuzz_2,
            patch: rug_fuzz_3,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::new(rug_fuzz_4).unwrap(),
        };
        debug_assert!(matches_caret(& cmp, & ver));
        let ver = Version {
            major: rug_fuzz_5,
            minor: rug_fuzz_6,
            patch: rug_fuzz_7,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::new(rug_fuzz_8).unwrap(),
        };
        debug_assert!(! matches_caret(& cmp, & ver));
             }
});    }
    #[test]
    fn test_matches_caret_zero_major() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let cmp = Comparator {
            op: Op::Caret,
            major: rug_fuzz_0,
            minor: None,
            patch: None,
            pre: Prerelease::EMPTY,
        };
        let ver = Version::new(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(matches_caret(& cmp, & ver));
        let ver = Version::new(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert!(! matches_caret(& cmp, & ver));
        let ver = Version::new(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9);
        debug_assert!(matches_caret(& cmp, & ver));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_23_llm_16_23 {
    use super::*;
    use crate::*;
    use crate::{Op, Version, Comparator, Prerelease, BuildMetadata};
    use std::str::FromStr;
    #[test]
    fn matches_impl_exact() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, u64, u64, u64, u64, u64, &str, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let version = Version::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let comparator = Comparator {
            op: Op::Exact,
            major: rug_fuzz_3,
            minor: Some(rug_fuzz_4),
            patch: Some(rug_fuzz_5),
            pre: Prerelease::new(rug_fuzz_6).unwrap(),
        };
        debug_assert!(matches_impl(& comparator, & version));
        let version = Version::new(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9);
        debug_assert!(! matches_impl(& comparator, & version));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_24 {
    use crate::{
        Version, Prerelease, eval::{self, matches_less},
        Comparator, Op,
    };
    #[test]
    fn matches_less_major() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ver = Version::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let cmp = Comparator {
            op: Op::Less,
            major: rug_fuzz_3,
            minor: None,
            patch: None,
            pre: Prerelease::EMPTY,
        };
        debug_assert!(matches_less(& cmp, & ver));
             }
});    }
    #[test]
    fn matches_less_major_eq() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ver = Version::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let cmp = Comparator {
            op: Op::Less,
            major: rug_fuzz_3,
            minor: Some(rug_fuzz_4),
            patch: None,
            pre: Prerelease::EMPTY,
        };
        debug_assert!(matches_less(& cmp, & ver));
             }
});    }
    #[test]
    fn matches_less_minor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ver = Version::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let cmp = Comparator {
            op: Op::Less,
            major: rug_fuzz_3,
            minor: Some(rug_fuzz_4),
            patch: None,
            pre: Prerelease::EMPTY,
        };
        debug_assert!(matches_less(& cmp, & ver));
             }
});    }
    #[test]
    fn matches_less_minor_eq() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ver = Version::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let cmp = Comparator {
            op: Op::Less,
            major: rug_fuzz_3,
            minor: Some(rug_fuzz_4),
            patch: Some(rug_fuzz_5),
            pre: Prerelease::EMPTY,
        };
        debug_assert!(matches_less(& cmp, & ver));
             }
});    }
    #[test]
    fn matches_less_patch() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ver = Version::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let cmp = Comparator {
            op: Op::Less,
            major: rug_fuzz_3,
            minor: Some(rug_fuzz_4),
            patch: Some(rug_fuzz_5),
            pre: Prerelease::EMPTY,
        };
        debug_assert!(matches_less(& cmp, & ver));
             }
});    }
    #[test]
    fn matches_less_pre_release_newer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u64, u64, u64, &str, u64, u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ver = Version {
            major: rug_fuzz_0,
            minor: rug_fuzz_1,
            patch: rug_fuzz_2,
            pre: Prerelease::new(rug_fuzz_3).unwrap(),
            build: Default::default(),
        };
        let cmp = Comparator {
            op: Op::Less,
            major: rug_fuzz_4,
            minor: Some(rug_fuzz_5),
            patch: Some(rug_fuzz_6),
            pre: Prerelease::new(rug_fuzz_7).unwrap(),
        };
        debug_assert!(matches_less(& cmp, & ver));
             }
});    }
    #[test]
    fn matches_less_pre_release_older() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u64, u64, u64, &str, u64, u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ver = Version {
            major: rug_fuzz_0,
            minor: rug_fuzz_1,
            patch: rug_fuzz_2,
            pre: Prerelease::new(rug_fuzz_3).unwrap(),
            build: Default::default(),
        };
        let cmp = Comparator {
            op: Op::Less,
            major: rug_fuzz_4,
            minor: Some(rug_fuzz_5),
            patch: Some(rug_fuzz_6),
            pre: Prerelease::new(rug_fuzz_7).unwrap(),
        };
        debug_assert!(! matches_less(& cmp, & ver));
             }
});    }
    #[test]
    fn matches_less_no_minor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ver = Version::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let cmp = Comparator {
            op: Op::Less,
            major: rug_fuzz_3,
            minor: None,
            patch: None,
            pre: Prerelease::EMPTY,
        };
        debug_assert!(! matches_less(& cmp, & ver));
             }
});    }
    #[test]
    fn matches_less_no_patch() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ver = Version::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let cmp = Comparator {
            op: Op::Less,
            major: rug_fuzz_3,
            minor: Some(rug_fuzz_4),
            patch: None,
            pre: Prerelease::EMPTY,
        };
        debug_assert!(! matches_less(& cmp, & ver));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_25_llm_16_25 {
    use super::*;
    use crate::*;
    use crate::{Version, Prerelease, BuildMetadata, VersionReq};
    #[test]
    fn matches_req_with_empty_prerelease() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver = Version::new(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(matches_req(& req, & ver));
             }
});    }
    #[test]
    fn matches_req_with_prerelease() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, u64, u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let mut ver = Version::new(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        ver.pre = Prerelease::new(rug_fuzz_4).unwrap();
        debug_assert!(matches_req(& req, & ver));
             }
});    }
    #[test]
    fn matches_req_with_post_release() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let ver = Version::new(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(matches_req(& req, & ver));
             }
});    }
    #[test]
    fn matches_req_with_prerelease_and_build() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, u64, u64, u64, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let mut ver = Version::new(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        ver.pre = Prerelease::new(rug_fuzz_4).unwrap();
        ver.build = BuildMetadata::new(rug_fuzz_5).unwrap();
        debug_assert!(matches_req(& req, & ver));
             }
});    }
    #[test]
    fn does_not_match_with_incompatible_prerelease() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, u64, u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let mut ver = Version::new(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        ver.pre = Prerelease::new(rug_fuzz_4).unwrap();
        debug_assert!(! matches_req(& req, & ver));
             }
});    }
    #[test]
    fn prerelease_does_not_satisfy_plain_version_req() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, u64, u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let req = VersionReq::parse(rug_fuzz_0).unwrap();
        let mut ver = Version::new(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        ver.pre = Prerelease::new(rug_fuzz_4).unwrap();
        debug_assert!(! matches_req(& req, & ver));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_26 {
    use super::*;
    use crate::*;
    use crate::{Version, Prerelease, Comparator, Op};
    #[test]
    fn test_matches_tilde() {
        fn comp(
            major: u64,
            minor: Option<u64>,
            patch: Option<u64>,
            pre: &str,
        ) -> Comparator {
            Comparator {
                op: Op::Tilde,
                major,
                minor,
                patch,
                pre: Prerelease::new(pre).unwrap(),
            }
        }
        fn ver(major: u64, minor: u64, patch: u64, pre: &str) -> Version {
            Version {
                major,
                minor,
                patch,
                pre: Prerelease::new(pre).unwrap(),
                build: BuildMetadata::EMPTY,
            }
        }
        assert!(matches_tilde(& comp(1, None, None, ""), & ver(1, 0, 0, "")));
        assert!(matches_tilde(& comp(1, Some(0), None, ""), & ver(1, 0, 0, "")));
        assert!(matches_tilde(& comp(1, Some(0), Some(0), ""), & ver(1, 0, 0, "")));
        assert!(matches_tilde(& comp(1, Some(0), Some(0), ""), & ver(1, 0, 0, "alpha")));
        assert!(matches_tilde(& comp(1, Some(0), Some(0), ""), & ver(1, 0, 1, "alpha")));
        assert!(
            matches_tilde(& comp(1, Some(0), None, "alpha"), & ver(1, 0, 0, "alpha"))
        );
        assert!(
            matches_tilde(& comp(1, Some(0), Some(0), "alpha"), & ver(1, 0, 0, "alpha"))
        );
        assert!(! matches_tilde(& comp(1, None, None, ""), & ver(2, 0, 0, "")));
        assert!(! matches_tilde(& comp(1, Some(0), None, ""), & ver(1, 1, 0, "")));
        assert!(! matches_tilde(& comp(1, Some(0), Some(0), ""), & ver(1, 0, 1, "")));
        assert!(
            ! matches_tilde(& comp(1, Some(0), None, "alpha"), & ver(1, 0, 0, "beta"))
        );
        assert!(
            ! matches_tilde(& comp(1, Some(0), Some(0), "alpha"), & ver(1, 0, 0, "beta"))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_27 {
    use super::*;
    use crate::*;
    use crate::eval::pre_is_compatible;
    use crate::{Comparator, Version, Op, Prerelease};
    #[test]
    fn test_pre_is_compatible() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u64, u64, u64, &str, u64, u64, u64, u64, u64, u64, &str, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let true_cases = vec![
            (Comparator { op : Op::Exact, major : rug_fuzz_0, minor : Some(rug_fuzz_1),
            patch : Some(rug_fuzz_2), pre : Prerelease::new(rug_fuzz_3).unwrap(), },
            Version::new(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),)
        ];
        for (cmp, ver) in true_cases {
            debug_assert!(pre_is_compatible(& cmp, & ver));
        }
        let false_cases = vec![
            (Comparator { op : Op::Exact, major : rug_fuzz_7, minor : Some(rug_fuzz_8),
            patch : Some(rug_fuzz_9), pre : Prerelease::new(rug_fuzz_10).unwrap(), },
            Version::new(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13),), (Comparator { op :
            Op::Exact, major : 1, minor : Some(0), patch : Some(1), pre :
            Prerelease::new("alpha.1").unwrap(), }, Version::new(1, 0, 0),), (Comparator
            { op : Op::Exact, major : 1, minor : Some(0), patch : Some(0), pre :
            Prerelease::new("beta.1").unwrap(), }, Version::new(1, 0, 0),), (Comparator {
            op : Op::Exact, major : 1, minor : Some(0), patch : Some(0), pre :
            Prerelease::EMPTY, }, Version::new(1, 0, 0),)
        ];
        for (cmp, ver) in false_cases {
            debug_assert!(! pre_is_compatible(& cmp, & ver));
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_3 {
    use crate::{Comparator, Version};
    #[test]
    fn test_matches_comparator() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Comparator::parse(rug_fuzz_0).expect(rug_fuzz_1);
        let mut p1 = Version::new(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        debug_assert_eq!(crate ::eval::matches_comparator(& p0, & p1), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_4 {
    use crate::{Comparator, Version};
    #[test]
    fn test_matches_exact() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Comparator::parse(rug_fuzz_0).expect(rug_fuzz_1);
        let mut p1 = Version::new(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        let result = crate::eval::matches_exact(&p0, &p1);
        debug_assert!(result, "Comparator should exactly match the version");
             }
});    }
}
#[cfg(test)]
mod tests_rug_5 {
    use crate::{Comparator, Version};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Comparator::parse(rug_fuzz_0).expect(rug_fuzz_1);
        let mut p1 = Version::new(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4);
        debug_assert!(crate ::eval::matches_greater(& p0, & p1));
             }
});    }
}
