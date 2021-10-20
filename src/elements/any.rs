use crate::{ParseElement, ParseError, ParseOk};

pub struct Any;

impl ParseElement for Any{
    type ParseOut = String;

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        if let Some(c) =input.chars().next(){
            Ok(ParseOk{
                bytes_parsed: c.len_utf8(),
                result: c.to_string()
            })
        } else {
            Err(ParseError{})
        }
    }
}

#[test]
fn any(){
    let parser = Any;
    let result = parser.pars("🦀");
    assert!(result.is_ok());
    let ParseOk { bytes_parsed, result, .. } = result.unwrap();
    assert_eq!(bytes_parsed, "🦀".len());
    assert_eq!(result, String::from("🦀"));
}