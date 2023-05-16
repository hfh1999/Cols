use clap::{
    arg,
    builder::{TypedValueParser, ValueParserFactory},
    value_parser, Command,
};
use regex::Regex;
//use log::debug;
use std::io;
#[derive(Debug, PartialEq, Clone)]
struct Span {
    inner: Vec<usize>,
}
impl ValueParserFactory for Span {
    type Parser = SpanParser;
    fn value_parser() -> Self::Parser {
        SpanParser
    }
}

#[derive(Clone)]
struct SpanParser;

impl TypedValueParser for SpanParser {
    type Value = Span;
    fn parse_ref(
        &self,
        _: &crate::Command,
        _: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value_str = value.to_str().unwrap();
        parse_to_span(value_str)
    }
}

fn parse_to_span(value_str: &str) -> Result<Span, clap::Error> {
    let continuous_pattern = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    let uncontinu_pattern = Regex::new(r"^(\d+)(,\d+)*$").unwrap();
    if let Some(capture) = continuous_pattern.captures(value_str) {
        let cap1 = capture.get(1).unwrap();
        let cap2 = capture.get(2).unwrap();
        let start_num = cap1.as_str().parse::<usize>().unwrap();
        let end_num = cap2.as_str().parse::<usize>().unwrap();
        if start_num > end_num {
            return Err(clap::Error::new(clap::error::ErrorKind::ValueValidation));
        }
        return Ok(Span {
            inner: (start_num..end_num + 1).collect(),
        });
    }
    if let Some(capture) = uncontinu_pattern.captures(value_str) {
        let uncontinuous_str = capture.get(0).unwrap().as_str();
        let data:Vec<usize> = uncontinuous_str.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        Ok(Span { inner: data })
    } else {
        Err(clap::Error::new(clap::error::ErrorKind::ValueValidation))
    }
}
fn main() {
    //env_logger::builder()
    //    .filter_level(log::LevelFilter::Debug)
    //    .init();

    let app = Command::new("Cols")
        .version("1.0")
        .author("hfh1999 <2350827470@qq.com>")
        .about("Choose Text by cols.")
        .arg(
            arg!( -r --rownum <ROWNUM>)
                .value_parser(value_parser!(usize))
                .default_value("1"),
        )
        .arg(arg!(<COLNUM>).value_parser(value_parser!(Span)));

    let matches = app.get_matches();
    let rows_num = matches.get_one::<usize>("rownum").unwrap();
    let cols_num = matches.get_one::<Span>("COLNUM").unwrap().inner[0];

    //debug!("rows_num = {}, cols_num = {}",rows_num, cols_num);
    println!("rows_num = {}, cols_num = {}", rows_num, cols_num);

    let instream = io::stdin();
    let lines = instream.lines().map(|l| l.unwrap());
    for (_, line) in lines.enumerate() {
        let words: Vec<&str> = line.split(' ').collect();
        if words.len() - 1 < cols_num {
            println!(" ");
        } else {
            println!("{}", words[cols_num]);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parse_to_span;
    use crate::Span;
    #[test]
    fn parse_to_span_continuous1() {
        let v1 = parse_to_span("1-3").unwrap();
        assert_eq!(v1,Span{inner:(1..4).collect()});
    }
    #[test]
    fn parse_to_span_continuous2() {
        let v1 = parse_to_span("1-1").unwrap();
        assert_eq!(v1,Span{inner:(1..2).collect()});
    }
    #[test]
    #[should_panic]
    fn parse_to_span_continuous_leftbig_error() {
        let _ = parse_to_span("5-1").unwrap();
    }

    #[test]
    fn parse_to_span_uncontinuous1()
    {
        let v1 = parse_to_span("1,2,3").unwrap();
        assert_eq!(v1,Span{inner:vec![1,2,3]});
    }
    
    #[test]
    fn parse_to_span_uncontinuous2()
    {
        let v1 = parse_to_span("3").unwrap();
        assert_eq!(v1,Span{inner:vec![3]});
    }

    #[test]
    #[should_panic]
    fn parse_to_span_uncontinuous_error1()
    {
        let _ = parse_to_span("3,").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_to_span_uncontinuous_error2()
    {
        let _ = parse_to_span(",3").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_to_span_error1()
    {
        let _ = parse_to_span("3434e").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_to_span_error2()
    {
        let _ = parse_to_span("-32432").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_to_span_error3()
    {
        let _ = parse_to_span("8888-").unwrap();
    }

}
