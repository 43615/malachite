use crate::rounding_modes::RoundingMode;
use core::fmt::{Debug, Display, Formatter, Result};

impl Display for RoundingMode {
    /// Converts a [`RoundingMode`] to a [`String`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode;
    ///
    /// assert_eq!(RoundingMode::Down.to_string(), "Down");
    /// assert_eq!(RoundingMode::Up.to_string(), "Up");
    /// assert_eq!(RoundingMode::Floor.to_string(), "Floor");
    /// assert_eq!(RoundingMode::Ceiling.to_string(), "Ceiling");
    /// assert_eq!(RoundingMode::Nearest.to_string(), "Nearest");
    /// assert_eq!(RoundingMode::Exact.to_string(), "Exact");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}
