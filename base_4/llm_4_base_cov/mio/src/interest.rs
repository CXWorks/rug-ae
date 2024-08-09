use std::num::NonZeroU8;
use std::{fmt, ops};
/// Interest used in registering.
///
/// Interest are used in [registering] [`event::Source`]s with [`Poll`], they
/// indicate what readiness should be monitored for. For example if a socket is
/// registered with [readable] interests and the socket becomes writable, no
/// event will be returned from a call to [`poll`].
///
/// [registering]: struct.Registry.html#method.register
/// [`event::Source`]: ./event/trait.Source.html
/// [`Poll`]: struct.Poll.html
/// [readable]: struct.Interest.html#associatedconstant.READABLE
/// [`poll`]: struct.Poll.html#method.poll
#[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Interest(NonZeroU8);
const READABLE: u8 = 0b0001;
const WRITABLE: u8 = 0b0010;
const AIO: u8 = 0b0100;
const LIO: u8 = 0b1000;
const PRIORITY: u8 = 0b10000;
impl Interest {
    /// Returns a `Interest` set representing readable interests.
    pub const READABLE: Interest = Interest(unsafe {
        NonZeroU8::new_unchecked(READABLE)
    });
    /// Returns a `Interest` set representing writable interests.
    pub const WRITABLE: Interest = Interest(unsafe {
        NonZeroU8::new_unchecked(WRITABLE)
    });
    /// Returns a `Interest` set representing AIO completion interests.
    #[cfg(
        any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos",
        )
    )]
    pub const AIO: Interest = Interest(unsafe { NonZeroU8::new_unchecked(AIO) });
    /// Returns a `Interest` set representing LIO completion interests.
    #[cfg(target_os = "freebsd")]
    pub const LIO: Interest = Interest(unsafe { NonZeroU8::new_unchecked(LIO) });
    /// Returns a `Interest` set representing priority completion interests.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub const PRIORITY: Interest = Interest(unsafe {
        NonZeroU8::new_unchecked(PRIORITY)
    });
    /// Add together two `Interest`.
    ///
    /// This does the same thing as the `BitOr` implementation, but is a
    /// constant function.
    ///
    /// ```
    /// use mio::Interest;
    ///
    /// const INTERESTS: Interest = Interest::READABLE.add(Interest::WRITABLE);
    /// # fn silent_dead_code_warning(_: Interest) { }
    /// # silent_dead_code_warning(INTERESTS)
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub const fn add(self, other: Interest) -> Interest {
        Interest(unsafe { NonZeroU8::new_unchecked(self.0.get() | other.0.get()) })
    }
    /// Removes `other` `Interest` from `self`.
    ///
    /// Returns `None` if the set would be empty after removing `other`.
    ///
    /// ```
    /// use mio::Interest;
    ///
    /// const RW_INTERESTS: Interest = Interest::READABLE.add(Interest::WRITABLE);
    ///
    /// // As long a one interest remain this will return `Some`.
    /// let w_interest = RW_INTERESTS.remove(Interest::READABLE).unwrap();
    /// assert!(!w_interest.is_readable());
    /// assert!(w_interest.is_writable());
    ///
    /// // Removing all interests from the set will return `None`.
    /// assert_eq!(w_interest.remove(Interest::WRITABLE), None);
    ///
    /// // Its also possible to remove multiple interests at once.
    /// assert_eq!(RW_INTERESTS.remove(RW_INTERESTS), None);
    /// ```
    pub fn remove(self, other: Interest) -> Option<Interest> {
        NonZeroU8::new(self.0.get() & !other.0.get()).map(Interest)
    }
    /// Returns true if the value includes readable readiness.
    pub const fn is_readable(self) -> bool {
        (self.0.get() & READABLE) != 0
    }
    /// Returns true if the value includes writable readiness.
    pub const fn is_writable(self) -> bool {
        (self.0.get() & WRITABLE) != 0
    }
    /// Returns true if `Interest` contains AIO readiness.
    pub const fn is_aio(self) -> bool {
        (self.0.get() & AIO) != 0
    }
    /// Returns true if `Interest` contains LIO readiness.
    pub const fn is_lio(self) -> bool {
        (self.0.get() & LIO) != 0
    }
    /// Returns true if `Interest` contains priority readiness.
    pub const fn is_priority(self) -> bool {
        (self.0.get() & PRIORITY) != 0
    }
}
impl ops::BitOr for Interest {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.add(other)
    }
}
impl ops::BitOrAssign for Interest {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = (*self | other).0;
    }
}
impl fmt::Debug for Interest {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut one = false;
        if self.is_readable() {
            if one {
                write!(fmt, " | ")?
            }
            write!(fmt, "READABLE")?;
            one = true;
        }
        if self.is_writable() {
            if one {
                write!(fmt, " | ")?
            }
            write!(fmt, "WRITABLE")?;
            one = true;
        }
        #[cfg(
            any(
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "ios",
                target_os = "macos",
                target_os = "tvos",
                target_os = "watchos",
            )
        )]
        {
            if self.is_aio() {
                if one {
                    write!(fmt, " | ")?
                }
                write!(fmt, "AIO")?;
                one = true;
            }
        }
        #[cfg(any(target_os = "freebsd"))]
        {
            if self.is_lio() {
                if one {
                    write!(fmt, " | ")?
                }
                write!(fmt, "LIO")?;
                one = true;
            }
        }
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            if self.is_priority() {
                if one {
                    write!(fmt, " | ")?
                }
                write!(fmt, "PRIORITY")?;
                one = true;
            }
        }
        debug_assert!(one, "printing empty interests");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_5_llm_16_5 {
    use crate::Interest;
    use std::ops::BitOr;
    #[test]
    fn test_bitor() {
        let _rug_st_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_bitor = 0;
        let a = Interest::READABLE;
        let b = Interest::READABLE;
        let combined = a | b;
        debug_assert!(combined.is_readable());
        debug_assert!(! combined.is_writable());
        let a = Interest::WRITABLE;
        let b = Interest::WRITABLE;
        let combined = a | b;
        debug_assert!(! combined.is_readable());
        debug_assert!(combined.is_writable());
        let a = Interest::READABLE;
        let b = Interest::WRITABLE;
        let combined = a | b;
        debug_assert!(combined.is_readable());
        debug_assert!(combined.is_writable());
        let a = Interest::WRITABLE;
        let b = Interest::READABLE;
        let combined = a | b;
        debug_assert!(combined.is_readable());
        debug_assert!(combined.is_writable());
        let _rug_ed_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_bitor = 0;
    }
    #[cfg(
        any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos",
        )
    )]
    #[test]
    fn test_bitor_with_aio() {
        let _rug_st_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_bitor_with_aio = 0;
        let a = Interest::READABLE;
        let b = Interest::AIO;
        let combined = a | b;
        debug_assert!(combined.is_readable());
        debug_assert!(combined.is_aio());
        let _rug_ed_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_bitor_with_aio = 0;
    }
    #[cfg(target_os = "freebsd")]
    #[test]
    fn test_bitor_with_lio() {
        let _rug_st_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_bitor_with_lio = 0;
        let a = Interest::READABLE;
        let b = Interest::LIO;
        let combined = a | b;
        debug_assert!(combined.is_readable());
        debug_assert!(combined.is_lio());
        let _rug_ed_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_bitor_with_lio = 0;
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn test_bitor_with_priority() {
        let _rug_st_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_bitor_with_priority = 0;
        let a = Interest::READABLE;
        let b = Interest::PRIORITY;
        let combined = a | b;
        debug_assert!(combined.is_readable());
        debug_assert!(combined.is_priority());
        let _rug_ed_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_bitor_with_priority = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    use std::num::NonZeroU8;
    const READABLE: u8 = 0b0000_0001;
    const WRITABLE: u8 = 0b0000_0010;
    const AIO: u8 = 0b0000_0100;
    const LIO: u8 = 0b0000_1000;
    const PRIORITY: u8 = 0b0001_0000;
    #[test]
    fn bitor_assign_readable_with_writable() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_bitor_assign_readable_with_writable = 0;
        let mut interest = Interest(NonZeroU8::new(READABLE).unwrap());
        let writable = Interest(NonZeroU8::new(WRITABLE).unwrap());
        interest |= writable;
        debug_assert!(interest.is_readable());
        debug_assert!(interest.is_writable());
        debug_assert_eq!(interest.0.get(), READABLE | WRITABLE);
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_bitor_assign_readable_with_writable = 0;
    }
    #[test]
    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn bitor_assign_readable_with_priority() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_bitor_assign_readable_with_priority = 0;
        let mut interest = Interest(NonZeroU8::new(READABLE).unwrap());
        let priority = Interest(NonZeroU8::new(PRIORITY).unwrap());
        interest |= priority;
        debug_assert!(interest.is_readable());
        debug_assert!(interest.is_priority());
        debug_assert_eq!(interest.0.get(), READABLE | PRIORITY);
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_bitor_assign_readable_with_priority = 0;
    }
    #[test]
    #[cfg(
        any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos",
        )
    )]
    fn bitor_assign_writable_with_aio() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_bitor_assign_writable_with_aio = 0;
        let mut interest = Interest(NonZeroU8::new(WRITABLE).unwrap());
        let aio = Interest(NonZeroU8::new(AIO).unwrap());
        interest |= aio;
        debug_assert!(interest.is_writable());
        debug_assert!(interest.is_aio());
        debug_assert_eq!(interest.0.get(), WRITABLE | AIO);
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_bitor_assign_writable_with_aio = 0;
    }
    #[test]
    #[cfg(target_os = "freebsd")]
    fn bitor_assign_aio_with_lio() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_bitor_assign_aio_with_lio = 0;
        let mut interest = Interest(NonZeroU8::new(AIO).unwrap());
        let lio = Interest(NonZeroU8::new(LIO).unwrap());
        interest |= lio;
        debug_assert!(interest.is_aio());
        debug_assert!(interest.is_lio());
        debug_assert_eq!(interest.0.get(), AIO | LIO);
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_bitor_assign_aio_with_lio = 0;
    }
    #[test]
    fn bitor_assign_self_with_self() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_bitor_assign_self_with_self = 0;
        let mut interest = Interest(NonZeroU8::new(READABLE).unwrap());
        interest |= interest;
        debug_assert!(interest.is_readable());
        debug_assert_eq!(interest.0.get(), READABLE);
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_bitor_assign_self_with_self = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_29 {
    use super::*;
    use crate::*;
    #[test]
    fn test_add_readable_and_writable() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_add_readable_and_writable = 0;
        let a = Interest::READABLE;
        let b = Interest::WRITABLE;
        let combined = a.add(b);
        debug_assert!(combined.is_readable());
        debug_assert!(combined.is_writable());
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_add_readable_and_writable = 0;
    }
    #[test]
    fn test_add_readable_and_readable() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_add_readable_and_readable = 0;
        let a = Interest::READABLE;
        let combined = a.add(a);
        debug_assert!(combined.is_readable());
        debug_assert!(! combined.is_writable());
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_add_readable_and_readable = 0;
    }
    #[test]
    fn test_add_writable_and_writable() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_add_writable_and_writable = 0;
        let a = Interest::WRITABLE;
        let combined = a.add(a);
        debug_assert!(combined.is_writable());
        debug_assert!(! combined.is_readable());
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_add_writable_and_writable = 0;
    }
    #[test]
    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn test_add_readable_and_priority() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_add_readable_and_priority = 0;
        let a = Interest::READABLE;
        let b = Interest::PRIORITY;
        let combined = a.add(b);
        debug_assert!(combined.is_readable());
        debug_assert!(combined.is_priority());
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_add_readable_and_priority = 0;
    }
    #[test]
    #[cfg(target_os = "freebsd")]
    fn test_add_writable_and_lio() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_add_writable_and_lio = 0;
        let a = Interest::WRITABLE;
        let b = Interest::LIO;
        let combined = a.add(b);
        debug_assert!(combined.is_writable());
        debug_assert!(combined.is_lio());
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_add_writable_and_lio = 0;
    }
    #[test]
    #[cfg(
        any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos"
        )
    )]
    fn test_add_aio_and_aio() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_add_aio_and_aio = 0;
        let a = Interest::AIO;
        let combined = a.add(a);
        debug_assert!(combined.is_aio());
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_add_aio_and_aio = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_30 {
    use super::*;
    use crate::*;
    #[test]
    #[cfg(
        any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos",
        )
    )]
    fn interest_is_aio() {
        let _rug_st_tests_llm_16_30_rrrruuuugggg_interest_is_aio = 0;
        debug_assert!(Interest::AIO.is_aio(), "Interest::AIO should be aio");
        debug_assert!(
            ! Interest::READABLE.is_aio(), "Interest::READABLE should not be aio"
        );
        debug_assert!(
            ! Interest::WRITABLE.is_aio(), "Interest::WRITABLE should not be aio"
        );
        let _rug_ed_tests_llm_16_30_rrrruuuugggg_interest_is_aio = 0;
    }
    #[test]
    #[cfg(
        any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos",
        )
    )]
    fn interest_add_aio_and_readable() {
        let _rug_st_tests_llm_16_30_rrrruuuugggg_interest_add_aio_and_readable = 0;
        let interest = Interest::AIO | Interest::READABLE;
        debug_assert!(
            interest.is_aio(), "Interest::AIO | Interest::READABLE should include aio"
        );
        debug_assert!(
            interest.is_readable(),
            "Interest::AIO | Interest::READABLE should include readable"
        );
        let _rug_ed_tests_llm_16_30_rrrruuuugggg_interest_add_aio_and_readable = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use super::*;
    use crate::*;
    #[test]
    fn priority_is_set_correctly() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_priority_is_set_correctly = 0;
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            let priority_interest = Interest::PRIORITY;
            debug_assert!(priority_interest.is_priority());
        }
        let readable_interest = Interest::READABLE;
        debug_assert!(! readable_interest.is_priority());
        let writable_interest = Interest::WRITABLE;
        debug_assert!(! writable_interest.is_priority());
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            let combined_interest = readable_interest | Interest::PRIORITY;
            debug_assert!(combined_interest.is_priority());
        }
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_priority_is_set_correctly = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_33_llm_16_33 {
    use crate::interest::Interest;
    use std::num::NonZeroU8;
    const READABLE: u8 = 0b0000_0001;
    const WRITABLE: u8 = 0b0000_0010;
    #[test]
    fn test_is_readable_true() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_is_readable_true = 0;
        let readable_interest = Interest(NonZeroU8::new(READABLE).unwrap());
        debug_assert!(readable_interest.is_readable());
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_is_readable_true = 0;
    }
    #[test]
    fn test_is_readable_false() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_is_readable_false = 0;
        let writable_interest = Interest(NonZeroU8::new(WRITABLE).unwrap());
        debug_assert!(! writable_interest.is_readable());
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_is_readable_false = 0;
    }
    #[test]
    fn test_is_readable_combined() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_is_readable_combined = 0;
        let combined_interest = Interest(NonZeroU8::new(READABLE | WRITABLE).unwrap());
        debug_assert!(combined_interest.is_readable());
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_is_readable_combined = 0;
    }
    #[test]
    fn test_is_readable_empty() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_is_readable_empty = 0;
        let empty_interest = Interest(NonZeroU8::new(WRITABLE | READABLE).unwrap())
            .remove(Interest(NonZeroU8::new(WRITABLE | READABLE).unwrap()))
            .unwrap_or(Interest(NonZeroU8::new(READABLE).unwrap()));
        debug_assert!(! empty_interest.is_readable());
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_is_readable_empty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_34 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_writable_with_writable() {
        let _rug_st_tests_llm_16_34_rrrruuuugggg_test_is_writable_with_writable = 0;
        let writable_interest = Interest::WRITABLE;
        debug_assert!(writable_interest.is_writable());
        let _rug_ed_tests_llm_16_34_rrrruuuugggg_test_is_writable_with_writable = 0;
    }
    #[test]
    fn test_is_writable_with_readable() {
        let _rug_st_tests_llm_16_34_rrrruuuugggg_test_is_writable_with_readable = 0;
        let readable_interest = Interest::READABLE;
        debug_assert!(! readable_interest.is_writable());
        let _rug_ed_tests_llm_16_34_rrrruuuugggg_test_is_writable_with_readable = 0;
    }
    #[test]
    fn test_is_writable_with_both() {
        let _rug_st_tests_llm_16_34_rrrruuuugggg_test_is_writable_with_both = 0;
        let both_interest = Interest::READABLE | Interest::WRITABLE;
        debug_assert!(both_interest.is_writable());
        let _rug_ed_tests_llm_16_34_rrrruuuugggg_test_is_writable_with_both = 0;
    }
    #[test]
    fn test_is_writable_with_none() {
        let _rug_st_tests_llm_16_34_rrrruuuugggg_test_is_writable_with_none = 0;
        let no_interest = Interest::READABLE.remove(Interest::READABLE).unwrap();
        debug_assert!(! no_interest.is_writable());
        let _rug_ed_tests_llm_16_34_rrrruuuugggg_test_is_writable_with_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;
    use crate::*;
    #[test]
    fn interest_remove_readable() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_interest_remove_readable = 0;
        let rw_interests = Interest::READABLE | Interest::WRITABLE;
        let w_interest = rw_interests.remove(Interest::READABLE).unwrap();
        debug_assert!(! w_interest.is_readable());
        debug_assert!(w_interest.is_writable());
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_interest_remove_readable = 0;
    }
    #[test]
    fn interest_remove_writable() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_interest_remove_writable = 0;
        let rw_interests = Interest::READABLE | Interest::WRITABLE;
        let r_interest = rw_interests.remove(Interest::WRITABLE).unwrap();
        debug_assert!(r_interest.is_readable());
        debug_assert!(! r_interest.is_writable());
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_interest_remove_writable = 0;
    }
    #[test]
    fn interest_remove_all_results_in_none() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_interest_remove_all_results_in_none = 0;
        let rw_interests = Interest::READABLE | Interest::WRITABLE;
        debug_assert_eq!(
            rw_interests.remove(Interest::READABLE | Interest::WRITABLE), None
        );
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_interest_remove_all_results_in_none = 0;
    }
    #[test]
    fn interest_remove_non_existing_interest_keeps_original() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_interest_remove_non_existing_interest_keeps_original = 0;
        let r_interest = Interest::READABLE;
        let result_interest = r_interest.remove(Interest::WRITABLE).unwrap();
        debug_assert_eq!(result_interest, Interest::READABLE);
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_interest_remove_non_existing_interest_keeps_original = 0;
    }
    #[test]
    fn interest_remove_from_empty_returns_none() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_interest_remove_from_empty_returns_none = 0;
        let no_interest = Interest::READABLE.remove(Interest::READABLE).unwrap();
        debug_assert_eq!(no_interest.remove(Interest::WRITABLE), None);
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_interest_remove_from_empty_returns_none = 0;
    }
}
