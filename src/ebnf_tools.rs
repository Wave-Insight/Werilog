use parser_rust_simple::prelude::*;

macro_rules! head {
    () => {
        //(identifier().map(|x| format!("macro_rules! {} {{\n    () => {{(", x))
        (DontConsume(ParseRegex(r".*\n").map(|x| format!("\n/// {}", x))) *
        (identifier().map(|x| format!("pub fn {}() -> impl Parser<Out = String> {{\n", x))
            << whitespace() << Token("::=") << whitespace())
        ).map(|x| format!("{}{}", x.0, x.1))
    };
}

macro_rules! things {
    () => {(
        (Many(
            (identifier().map(|x| format!("{}()", x)) |
            Token("|").map(|_| " | ".to_string()) |
            Token("{").map(|_| "Many(".to_string()) |
            Token("}").map(|_| ")".to_string()) |
            Token("[").map(|_| "Try(".to_string()) |
            Token("]").map(|_| ")".to_string())) |
            ParseRegex(r"\S*").map(|x| format!("Token(\"{}\")", x)),
            Some(" ")
        ).map(|x|
            x.into_iter().reduce(|a,b| a+&b).unwrap_or_else(|| "error at things".to_string()))
        * Try(Token("\n") >> whitespace() >> Token("| ") >>
            Many(
                Many(
                    (identifier().map(|x| format!("{}()", x)) |
                    Token("|").map(|_| " | ".to_string()) |
                    Token("{").map(|_| "Many(".to_string()) |
                    Token("}").map(|_| ")".to_string()) |
                    Token("[").map(|_| "Try(".to_string()) |
                    Token("]").map(|_| ")".to_string())) |
                    ParseRegex(r"\S*").map(|x| format!("Token(\"{}\")", x)),
                    Some(" ")
                ).map(|x|
                    x.into_iter().reduce(|a,b| a+&b).unwrap_or_else(|| "error at things".to_string())),
                Some("\n | ")                
            ).map(|x| x.into_iter().reduce(|a,b| a+"\n.or("+&b).unwrap_or_else(|| "error at things".to_string()))
        )).map(|(a,b)| a+&b.unwrap_or_else(|| "".to_string()))
    )};
}

pub fn ebnf(input: &str) -> Result<String, Location> {
    let item = 
        (whitespace() >> (head!()
        * things!()))
        //.map(|x| format!("{}\n        {}\n    )}};\n}}\n", x.0, x.1));
        .map(|x| format!("{}    {}\n}}\n", x.0, x.1));
    Many(
        item,
        Some("\n")
    ).map(|x| x.into_iter().reduce(|a,b| a+&b).unwrap_or_else(|| "error at final".to_string()))
    .run(input)
}

fn identifier() -> impl Parser<Out = String> {
    ParseRegex(r"[a-zA-Z_][a-zA-Z_0-9]*")
}

#[test]
fn test() {
    let input = "
delay3 ::=
 # delay_value
 | # ( mintypmax_expression [ , mintypmax_expression [ , mintypmax_expression ] ] )
delay2 ::=
 # delay_value
 | # ( mintypmax_expression [ , mintypmax_expression ] )
delay_value ::=
 unsigned_number
 | real_number
 | identifier
    ";
    let parser = ebnf(input);
    //println!("{:?}", parser);
    println!("{}", parser.unwrap())
}
