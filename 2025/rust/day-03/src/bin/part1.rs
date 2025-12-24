    use day_03::part1::process_a;
    use day_03::part1::process_b;
    use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result_a = process_a(file).context("process part 1")?;
    println!(" a {}", result_a);
    let result_b = process_b(file).context("process part 1")?;
    println!(" b {}", result_b);
    
    Ok(())
}