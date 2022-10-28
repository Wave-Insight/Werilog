use parser_rust_simple::prelude::*;

/*
{} {{}} Concatenation, replication
unary + unary - Unary operators
+ - * / ** Arithmetic
% Modulus
> >= < <= Relational
! Logical negation
&& Logical and
|| Logical or
== Logical equality
!= Logical inequality
=== Case equality
!== Case inequality
~ Bitwise negation
& Bitwise and
| Bitwise inclusive or
^ Bitwise exclusive or
^~ or ~^ Bitwise equivalence
& Reduction and
~& Reduction nand
| Reduction or
~| Reduction nor
^ Reduction xor
~^ or ^~ Reduction xnor
<< Logical left shift
>> Logical right shift
<<< Arithmetic left shift
>>> Arithmetic right shift
? : Conditional
*/

pub enum Operator {
    Concatenation,// {}
    Replication,// {{}}
    Unary(Unary),// + -
    Arithmetic(Arithmetic),// + - * / % **
    Relational(Relational),// > >= < <=
    Equality(Equality),// === !== == !=
    Logical(Logical),// && ||
    Bitwise(Bitwise),// & ~& | ~| ^ (~^ or ^~) ~
    Shift(Shift),// << >> <<< >>>
    LogicalNegation,// !
    Conditional,// ? :
}

pub fn operator() -> impl Parser<Out = Operator> {
    shift().map(Operator::Shift) |
    logical().map(Operator::Logical) |
    arithmetic().map(Operator::Arithmetic) |
    relational().map(Operator::Relational) |
    equality().map(Operator::Equality) |
    bitwise().map(Operator::Bitwise)
    //TODO
}

pub enum Unary {
    Plus,// +
    Minus,// -
}

fn unary() -> impl Parser<Out = Unary> {
    Token("+").map(|_| Unary::Plus) |
    Token("-").map(|_| Unary::Minus)
}

pub enum Arithmetic {
    Add,// +
    Sub,// -
    Mul,// *
    Div,// /
    Mod,// %
    Pow,// **
}

fn arithmetic() -> impl Parser<Out = Arithmetic> {
    Token("**").map(|_| Arithmetic::Pow) |
    Token("+").map(|_| Arithmetic::Add) |
    Token("-").map(|_| Arithmetic::Sub) |
    Token("*").map(|_| Arithmetic::Mul) |
    Token("/").map(|_| Arithmetic::Div) |
    Token("%").map(|_| Arithmetic::Mod)
}

pub enum Relational {
    Less,// <
    Greater,// >
    LessEqual,// <=
    GreaterEqual,// >=
}

fn relational() -> impl Parser<Out = Relational> {
    Token("<=").map(|_| Relational::LessEqual) |
    Token(">=").map(|_| Relational::GreaterEqual) |
    Token("<").map(|_| Relational::Less) |
    Token(">").map(|_| Relational::Greater)
}

pub enum Equality {
    CaseEqual,// ===
    CaseInequal,// !==
    LogicEqual,// ==
    LogicInequal,// !=
}

fn equality() -> impl Parser<Out = Equality> {
    Token("===").map(|_| Equality::CaseEqual) |
    Token("!==").map(|_| Equality::CaseInequal) |
    Token("==").map(|_| Equality::LogicEqual) |
    Token("!=").map(|_| Equality::LogicInequal)
}

pub enum Logical {
    And,// &&
    Or,// ||
}

fn logical() -> impl Parser<Out = Logical> {
    Token("&&").map(|_| Logical::And) |
    Token("||").map(|_| Logical::Or)
}

pub enum Bitwise {
    And,// &
    Nand,// ~&
    Or,// |
    Nor,// ~|
    Xor,// ^
    Xnor,// ^~ or ~^
    Neg,// ~
}

fn bitwise() -> impl Parser<Out = Bitwise> {
    Token("&").map(|_| Bitwise::And) |
    Token("~&").map(|_| Bitwise::Nand) |
    Token("|").map(|_| Bitwise::Or) |
    Token("~|").map(|_| Bitwise::Nor) |
    Token("^").map(|_| Bitwise::Xor) |
    (Token("^~") | Token("~^")).map(|_| Bitwise::Xnor) |
    Token("~").map(|_| Bitwise::Neg)
}

pub enum Shift {
    LogicalLeft,// <<
    LogicalRight,// >>
    ArithmeticLeft,// <<<
    ArithmeticRight,// >>>
}

fn shift() -> impl Parser<Out = Shift> {
    Token("<<<").map(|_| Shift::ArithmeticLeft) |
    Token(">>>").map(|_| Shift::ArithmeticRight) |
    Token("<<").map(|_| Shift::LogicalLeft) |
    Token(">>").map(|_| Shift::LogicalRight)
}
