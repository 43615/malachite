use integer::Integer;
use malachite_base::num::arithmetic::traits::DivisibleBy;

impl DivisibleBy<Integer> for Integer {
    /// Returns whether an [`Integer`] is divisible by another [`Integer`]; in other words, whether
    /// the first is a multiple of the second. Both [`Integer`]s are taken by value.
    ///
    /// This means that zero is divisible by any [`Integer`], including zero; but a nonzero
    /// [`Integer`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::ZERO.divisible_by(Integer::ZERO), true);
    /// assert_eq!(Integer::from(-100).divisible_by(Integer::from(-3)), false);
    /// assert_eq!(Integer::from(102).divisible_by(Integer::from(-3)), true);
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000000000000000").unwrap()
    ///             .divisible_by(Integer::from_str("1000000000000").unwrap()),
    ///     true
    /// );
    /// ```
    fn divisible_by(self, other: Integer) -> bool {
        self.abs.divisible_by(other.abs)
    }
}

impl<'a> DivisibleBy<&'a Integer> for Integer {
    /// Returns whether an [`Integer`] is divisible by another [`Integer`]; in other words, whether
    /// the first is a multiple of the second. The first [`Integer`] is taken by value and the
    /// second by reference.
    ///
    /// This means that zero is divisible by any [`Integer`], including zero; but a nonzero
    /// [`Integer`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::ZERO.divisible_by(&Integer::ZERO), true);
    /// assert_eq!(Integer::from(-100).divisible_by(&Integer::from(-3)), false);
    /// assert_eq!(Integer::from(102).divisible_by(&Integer::from(-3)), true);
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000000000000000").unwrap()
    ///             .divisible_by(&Integer::from_str("1000000000000").unwrap()),
    ///     true
    /// );
    /// ```
    fn divisible_by(self, other: &'a Integer) -> bool {
        self.abs.divisible_by(&other.abs)
    }
}

impl<'a> DivisibleBy<Integer> for &'a Integer {
    /// Returns whether an [`Integer`] is divisible by another [`Integer`]; in other words, whether
    /// the first is a multiple of the second. The first [`Integer`] is taken by reference and the
    /// second by value.
    ///
    /// This means that zero is divisible by any [`Integer`], including zero; but a nonzero
    /// [`Integer`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Integer::ZERO).divisible_by(Integer::ZERO), true);
    /// assert_eq!((&Integer::from(-100)).divisible_by(Integer::from(-3)), false);
    /// assert_eq!((&Integer::from(102)).divisible_by(Integer::from(-3)), true);
    /// assert_eq!(
    ///     (&Integer::from_str("-1000000000000000000000000").unwrap())
    ///             .divisible_by(Integer::from_str("1000000000000").unwrap()),
    ///     true
    /// );
    /// ```
    fn divisible_by(self, other: Integer) -> bool {
        (&self.abs).divisible_by(other.abs)
    }
}

impl<'a, 'b> DivisibleBy<&'b Integer> for &'a Integer {
    /// Returns whether an [`Integer`] is divisible by another [`Integer`]; in other words, whether
    /// the first is a multiple of the second. Both [`Integer`]s are taken by reference.
    ///
    /// This means that zero is divisible by any [`Integer`], including zero; but a nonzero
    /// [`Integer`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Integer::ZERO).divisible_by(&Integer::ZERO), true);
    /// assert_eq!((&Integer::from(-100)).divisible_by(&Integer::from(-3)), false);
    /// assert_eq!((&Integer::from(102)).divisible_by(&Integer::from(-3)), true);
    /// assert_eq!(
    ///     (&Integer::from_str("-1000000000000000000000000").unwrap())
    ///             .divisible_by(&Integer::from_str("1000000000000").unwrap()),
    ///     true
    /// );
    /// ```
    fn divisible_by(self, other: &'b Integer) -> bool {
        (&self.abs).divisible_by(&other.abs)
    }
}
