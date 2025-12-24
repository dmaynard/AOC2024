use day_03::part2::{process_a, process_b};
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let result_a = process_a(file).context("process part 2")?;
    println!("a {}", result_a);
    let result_b = process_b(file).context("process part 2")?;
    println!("b {}", result_b);
    Ok(())
}
