pub use anyhow::anyhow;
use my_parser_yaroslav_fetisov::*;

pub fn main() -> anyhow::Result<()> {
    println!("{:?}", list_parser::list("[1,1,2,3,5,8]"));
    let pair = Grammar::parse(Rule::field, "131.13")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    dbg!(pair);
    Ok(())
}
