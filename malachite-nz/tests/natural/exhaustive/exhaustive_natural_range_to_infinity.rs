use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::exhaustive::exhaustive_natural_range_to_infinity;
use malachite_nz::natural::Natural;

fn exhaustive_natural_range_to_infinity_helper(a: Natural, values: &str) {
    let xs = exhaustive_natural_range_to_infinity(a)
        .take(20)
        .collect_vec()
        .to_debug_string();
    assert_eq!(xs, values);
}

#[test]
fn test_exhaustive_natural_range_to_infinity() {
    exhaustive_natural_range_to_infinity_helper(
        Natural::ZERO,
        "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]",
    );
    exhaustive_natural_range_to_infinity_helper(
        Natural::exact_from(10),
        "[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]",
    );
}
