use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::random::get_random_natural_with_up_to_bits;

fn get_random_natural_with_up_to_bits_helper(bits: u64, out: &str) {
    assert_eq!(
        get_random_natural_with_up_to_bits(&mut random_primitive_ints(EXAMPLE_SEED), bits)
            .to_string(),
        out
    );
}

#[test]
fn test_get_random_natural_with_up_to_bits() {
    get_random_natural_with_up_to_bits_helper(0, "0");
    get_random_natural_with_up_to_bits_helper(1, "1");
    get_random_natural_with_up_to_bits_helper(10, "881");
    get_random_natural_with_up_to_bits_helper(100, "976558340558744279591984426865");
    get_random_natural_with_up_to_bits_helper(
        1000,
        "451401255738005197898854278327293891946634633305720512960911550410462452070718179114814511\
        6162200520751727277816528182754455803315977010218275020882256456155559081083019035440674861\
        0630885749971038892758146521810654725671239516399344653265522969157198643030481343955853985\
        19592187655255240600759693169",
    );
}
