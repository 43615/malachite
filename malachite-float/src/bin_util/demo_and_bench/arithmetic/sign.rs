use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::generators::float_gen_var_2;
use malachite_float::ComparableFloat;
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_sign);
    register_demo!(runner, demo_float_sign_debug);

    register_bench!(runner, benchmark_float_sign);
}

fn demo_float_sign(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_2().get(gm, config).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{n} is negative"),
            Ordering::Equal => println!("{n} is zero"),
            Ordering::Greater => println!("{n} is positive"),
        }
    }
}

fn demo_float_sign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_2().get(gm, config).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{:#x} is negative", ComparableFloat(n)),
            Ordering::Equal => println!("{:#x} is zero", ComparableFloat(n)),
            Ordering::Greater => println!("{:#x} is positive", ComparableFloat(n)),
        }
    }
}

fn benchmark_float_sign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.sign()",
        BenchmarkType::Single,
        float_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(n.sign()))],
    );
}
