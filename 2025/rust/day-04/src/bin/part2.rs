use day_04::part2::process_a;
use day_04::part2::process_b;
use day_04::part2::process_c;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process_a(file).context("process part 2")?;
    println!("a {}", result);
    let result = process_b(file).context("process part 2")?;
    println!("b {}", result);
    let result = process_c(file).context("process part 2")?;
    println!("b {}", result);

    Ok(())
}