use day_04::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1a() {
    part1::process_a(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}
#[divan::bench]
fn part1b() {
    part1::process_b(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1c() {
    part1::process_c(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}


#[divan::bench]
fn part2a() {
    part2::process_a(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}
#[divan::bench]
fn part2b() {
    part2::process_b(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2c() {
    part2::process_c(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}