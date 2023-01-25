use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = read_input()?;
    println!("{}", input);
    Ok(())
}

fn read_input() -> color_eyre::Result<String> {
    let input = std::fs::read_to_string("src/input.txt").wrap_err("reading src/input.txt")?;
    Ok(input)
}
