use pars::*;

#[derive(Debug, Clone)]
enum Bracketed {
    Item(String),
    Bracketed(Box<Bracketed>),
}

fn main() {
    let brackets: Uninit<Bracketed> = Uninit::new();
    let secret = Literal("secret").map(|s| Bracketed::Item(s));

    brackets.init(
        Literal("(")
            .tup(brackets.weak())
            .tup(Literal(")"))
            .map(|((_, inner), _)| Bracketed::Bracketed(Box::new(inner)))
            .or(secret),
    );

    let to_parse ="((((((((((secret))))))))))";

    println!(
        "{:?}",
        *(brackets.pars(&to_parse).unwrap())
    );
}
