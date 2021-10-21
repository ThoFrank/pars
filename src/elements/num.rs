use crate::{ParseElement, ParseError, ParseOk};

pub struct Integer;

impl ParseElement for Integer {
    type ParseOut = i32;

    fn pars(&self, input: &str) -> crate::ParseResult<Self::ParseOut> {
        let mut last_parse = Err(ParseError::new());
        for i in 1..=input.len() {
            let val = input[0..i].parse::<i32>();
            if let Ok(val) = val {
                last_parse = Ok(ParseOk {
                    bytes_parsed: i,
                    result: val,
                });
            } else {
                break;
            }
        }
        return last_parse;
    }
}

#[test]
fn int() {
    let parser = Integer;

    let result = parser.pars("");
    assert!(result.is_err());

    let result = parser.pars("1");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(*result, 1);
    assert_eq!(result.bytes_parsed, 1);

    let result = parser.pars("12345");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(*result, 12345);
    assert_eq!(result.bytes_parsed, 5);
}
