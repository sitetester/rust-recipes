use std::error::Error;

type I32OrErrResult = Result<i32, Box<dyn Error>>;

fn parse_demo(s: &str) -> I32OrErrResult {
    let n = s.parse::<i32>()?;
    Ok(n)
}

fn main() {
    let handle_match = |a: I32OrErrResult| {
        match a {
            Ok(n) => println!("{}", n),
            Err(e) => println!("{}", e),
        }
    };

    handle_match(
        parse_demo("123")
    );

    handle_match(
        parse_demo("blah")
    );
}