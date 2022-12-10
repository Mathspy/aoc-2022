mod day09;

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let mut args = std::env::args();
    let _ = args.next();
    let output = match (args.next().as_deref(), args.next().as_deref()) {
        (Some("09"), Some("1")) => day09::part1()?,
        (Some("09"), Some("2")) => day09::part2()?,
        (arg1, arg2) => bail!("Invalid arguments {arg1:?} {arg2:?}"),
    };

    println!("{output}");

    Ok(())
}
