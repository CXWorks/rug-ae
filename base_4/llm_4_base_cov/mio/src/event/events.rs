use crate::event::Event;
use crate::sys;
use std::fmt;
/// A collection of readiness events.
///
/// `Events` is passed as an argument to [`Poll::poll`] and will be used to
/// receive any new readiness events received since the last poll. Usually, a
/// single `Events` instance is created at the same time as a [`Poll`] and
/// reused on each call to [`Poll::poll`].
///
/// See [`Poll`] for more documentation on polling.
///
/// [`Poll::poll`]: ../struct.Poll.html#method.poll
/// [`Poll`]: ../struct.Poll.html
///
/// # Examples
///
#[cfg_attr(feature = "os-poll", doc = "```")]
#[cfg_attr(not(feature = "os-poll"), doc = "```ignore")]
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use mio::{Events, Poll};
/// use std::time::Duration;
///
/// let mut events = Events::with_capacity(1024);
/// let mut poll = Poll::new()?;
/// #
/// # assert!(events.is_empty());
///
/// // Register `event::Source`s with `poll`.
///
/// poll.poll(&mut events, Some(Duration::from_millis(100)))?;
///
/// for event in events.iter() {
///     println!("Got an event for {:?}", event.token());
/// }
/// #     Ok(())
/// # }
/// ```
pub struct Events {
    inner: sys::Events,
}
/// [`Events`] iterator.
///
/// This struct is created by the [`iter`] method on [`Events`].
///
/// [`Events`]: struct.Events.html
/// [`iter`]: struct.Events.html#method.iter
///
/// # Examples
///
#[cfg_attr(feature = "os-poll", doc = "```")]
#[cfg_attr(not(feature = "os-poll"), doc = "```ignore")]
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use mio::{Events, Poll};
/// use std::time::Duration;
///
/// let mut events = Events::with_capacity(1024);
/// let mut poll = Poll::new()?;
///
/// // Register handles with `poll`.
///
/// poll.poll(&mut events, Some(Duration::from_millis(100)))?;
///
/// for event in events.iter() {
///     println!("Got an event for {:?}", event.token());
/// }
/// #     Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    inner: &'a Events,
    pos: usize,
}
impl Events {
    /// Return a new `Events` capable of holding up to `capacity` events.
    ///
    /// # Examples
    ///
    /// ```
    /// use mio::Events;
    ///
    /// let events = Events::with_capacity(1024);
    /// assert_eq!(1024, events.capacity());
    /// ```
    pub fn with_capacity(capacity: usize) -> Events {
        Events {
            inner: sys::Events::with_capacity(capacity),
        }
    }
    /// Returns the number of `Event` values that `self` can hold.
    ///
    /// ```
    /// use mio::Events;
    ///
    /// let events = Events::with_capacity(1024);
    /// assert_eq!(1024, events.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }
    /// Returns `true` if `self` contains no `Event` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use mio::Events;
    ///
    /// let events = Events::with_capacity(1024);
    /// assert!(events.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Returns an iterator over the `Event` values.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "os-poll", doc = "```")]
    #[cfg_attr(not(feature = "os-poll"), doc = "```ignore")]
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mio::{Events, Poll};
    /// use std::time::Duration;
    ///
    /// let mut events = Events::with_capacity(1024);
    /// let mut poll = Poll::new()?;
    ///
    /// // Register handles with `poll`.
    ///
    /// poll.poll(&mut events, Some(Duration::from_millis(100)))?;
    ///
    /// for event in events.iter() {
    ///     println!("Got an event for {:?}", event.token());
    /// }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn iter(&self) -> Iter<'_> {
        Iter { inner: self, pos: 0 }
    }
    /// Clearing all `Event` values from container explicitly.
    ///
    /// # Notes
    ///
    /// Events are cleared before every `poll`, so it is not required to call
    /// this manually.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "os-poll", doc = "```")]
    #[cfg_attr(not(feature = "os-poll"), doc = "```ignore")]
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mio::{Events, Poll};
    /// use std::time::Duration;
    ///
    /// let mut events = Events::with_capacity(1024);
    /// let mut poll = Poll::new()?;
    ///
    /// // Register handles with `poll`.
    ///
    /// poll.poll(&mut events, Some(Duration::from_millis(100)))?;
    ///
    /// // Clear all events.
    /// events.clear();
    /// assert!(events.is_empty());
    /// #     Ok(())
    /// # }
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    /// Returns the inner `sys::Events`.
    pub(crate) fn sys(&mut self) -> &mut sys::Events {
        &mut self.inner
    }
}
impl<'a> IntoIterator for &'a Events {
    type Item = &'a Event;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a Event;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.inner.inner.get(self.pos).map(Event::from_sys_event_ref);
        self.pos += 1;
        ret
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.inner.inner.len();
        (size, Some(size))
    }
    fn count(self) -> usize {
        self.inner.inner.len()
    }
}
impl fmt::Debug for Events {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use crate::event::events::Events;
    use std::iter::IntoIterator;
    #[test]
    fn test_events_into_iter() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_test_events_into_iter = 0;
        let rug_fuzz_0 = 4;
        let mut events = Events::with_capacity(rug_fuzz_0);
        let mut iter = (&events).into_iter();
        debug_assert_eq!(iter.size_hint(), (0, Some(4)));
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_test_events_into_iter = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2 {
    use crate::event::events::Events;
    use std::iter::Iterator;
    #[test]
    fn count_zero_when_empty() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_count_zero_when_empty = 0;
        let rug_fuzz_0 = 10;
        let events = Events::with_capacity(rug_fuzz_0);
        let count = events.iter().count();
        debug_assert_eq!(count, 0);
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_count_zero_when_empty = 0;
    }
    #[test]
    fn count_non_zero_when_not_empty() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_count_non_zero_when_not_empty = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 0;
        let mut events = Events::with_capacity(rug_fuzz_0);
        let count_before_clear = events.iter().count();
        debug_assert!(count_before_clear > rug_fuzz_1);
        events.clear();
        let count_after_clear = events.iter().count();
        debug_assert_eq!(count_after_clear, 0);
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_count_non_zero_when_not_empty = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_23 {
    use super::*;
    use crate::*;
    #[test]
    fn events_capacity_correct() {
        let _rug_st_tests_llm_16_23_rrrruuuugggg_events_capacity_correct = 0;
        let rug_fuzz_0 = 0;
        let capacities = vec![rug_fuzz_0, 1, 10, 1024, 65536];
        for &capacity in &capacities {
            let events = Events::with_capacity(capacity);
            debug_assert_eq!(capacity, events.capacity());
        }
        let _rug_ed_tests_llm_16_23_rrrruuuugggg_events_capacity_correct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_25 {
    use super::*;
    use crate::*;
    #[test]
    fn events_is_empty_with_new_instance() {
        let _rug_st_tests_llm_16_25_rrrruuuugggg_events_is_empty_with_new_instance = 0;
        let rug_fuzz_0 = 10;
        let events = Events::with_capacity(rug_fuzz_0);
        debug_assert!(events.is_empty());
        let _rug_ed_tests_llm_16_25_rrrruuuugggg_events_is_empty_with_new_instance = 0;
    }
    #[test]
    fn events_is_empty_after_clear() {
        let _rug_st_tests_llm_16_25_rrrruuuugggg_events_is_empty_after_clear = 0;
        let rug_fuzz_0 = 10;
        let mut events = Events::with_capacity(rug_fuzz_0);
        events.clear();
        debug_assert!(events.is_empty());
        let _rug_ed_tests_llm_16_25_rrrruuuugggg_events_is_empty_after_clear = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_27 {
    use crate::Events;
    use std::fmt;
    #[test]
    fn test_events_sys() {
        let _rug_st_tests_llm_16_27_rrrruuuugggg_test_events_sys = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = '[';
        let rug_fuzz_2 = ']';
        let mut events = Events::with_capacity(rug_fuzz_0);
        let inner_sys_events = events.sys();
        inner_sys_events.clear();
        debug_assert!(inner_sys_events.is_empty());
        let debug_str = format!("{:?}", events);
        debug_assert!(
            debug_str.starts_with(rug_fuzz_1) && debug_str.ends_with(rug_fuzz_2)
        );
        let _rug_ed_tests_llm_16_27_rrrruuuugggg_test_events_sys = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_28 {
    use crate::Events;
    #[test]
    fn events_with_capacity() {
        let _rug_st_tests_llm_16_28_rrrruuuugggg_events_with_capacity = 0;
        let rug_fuzz_0 = 1024;
        let capacity = rug_fuzz_0;
        let events = Events::with_capacity(capacity);
        debug_assert_eq!(
            events.capacity(), capacity,
            "Capacity should be equal to the value passed to with_capacity"
        );
        debug_assert!(events.is_empty(), "Events should be initially empty");
        debug_assert_eq!(
            format!("{:?}", events), "[]", "Debug representation should be an empty list"
        );
        let _rug_ed_tests_llm_16_28_rrrruuuugggg_events_with_capacity = 0;
    }
}
