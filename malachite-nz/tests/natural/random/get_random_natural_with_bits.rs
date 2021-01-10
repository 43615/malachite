use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::random::get_random_natural_with_bits;

fn get_random_natural_with_bits_helper(bits: u64, out: &str) {
    assert_eq!(
        get_random_natural_with_bits(&mut random_primitive_ints(EXAMPLE_SEED), bits).to_string(),
        out
    );
}

#[test]
fn test_get_random_natural_with_bits() {
    get_random_natural_with_bits_helper(0, "0");
    get_random_natural_with_bits_helper(1, "1");
    get_random_natural_with_bits_helper(10, "881");
    get_random_natural_with_bits_helper(100, "976558340558744279591984426865");
    get_random_natural_with_bits_helper(
        1000,
        "987155559331138858373066802857294797227337039158487316682786744595637977633186240361413701\
        0240679811389700642404293916880391529744438580436267309669605743557526364970431150933385398\
        3656197605680428663668911754185572635377576510237140480985253354471771499012869776666692287\
        34508013967448659203593727857"
    );
}
