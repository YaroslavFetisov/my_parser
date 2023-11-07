use pest_derive::Parser;
pub use pest::Parser;


peg::parser!{
    pub grammar list_parser() for str {
      rule number() -> u32
        = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }
  
      pub rule list() -> Vec<u32>
        = "[" l:(number() ** ",") "]" { l }
    }
  }

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[cfg(test)]
mod tests {

    use anyhow::anyhow;
    use super::*;

    #[test]
    fn field_test() -> anyhow::Result< () > {
    
        let pair = Grammar::parse(Rule::field, "131.13")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
        assert_eq!( pair.as_str(), "131.13" );
        assert_eq!( pair.as_span().start(), 0 );
        assert_eq!( pair.as_span().end(), 6 );
    
        let pair = Grammar::parse(Rule::field, "x");
        assert!( pair.is_err() );
    
        let pair = Grammar::parse(Rule::field, "");
        assert!( pair.is_err() );
    
        // dbg!(pair);
        Ok( () )
    }



    #[test]
    fn record_test() -> anyhow::Result< () > {
    
        let pair = Grammar::parse(Rule::record, "131.13,99")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
        assert_eq!( pair.as_str(), "131.13,99" );
        assert_eq!( pair.as_span().start(), 0 );
        assert_eq!( pair.as_span().end(), 9 );
    
        let pair = Grammar::parse(Rule::record, "");
        assert!( pair.is_err() );
    
        Ok( () )
    }

}