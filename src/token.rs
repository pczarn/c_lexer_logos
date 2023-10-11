// use std::num::ParseIntError;

use internship::IStr;
use logos::Logos;

use crate::error;

/// Number representation of parsed number
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Number {
    /// Whole part of number
    pub integer: u32,
    /// Decimal part of number
    pub decimal: u32,
    /// Number behind E / e (exponent)
    pub exponent: i64,
    /// base of number
    pub radix: u8,
}

impl Number {
    /// Create instance of js representation of number
    #[inline]
    pub fn new(integer: u32, decimal: u32, exponent: i64, radix: u8) -> Self {
        Self {
            integer,
            decimal,
            exponent,
            radix,
        }
    }

    fn from_str(input: &str) -> Result<Self, error::Error> {
        if input.chars().all(|ch| ch.is_ascii_digit()) {
            input.parse().map(|integer| Number { integer, decimal: 0, exponent: 0, radix: 10 })
        } else if input.starts_with("0x") || input.starts_with("0X") {
            u32::from_str_radix(&input[2..], 16).map(|integer| Number { integer, decimal: 0, exponent: 0, radix: 10 })
        } else {
            input.parse().map(|_x: u32| unreachable!())
        }.map_err(|parse_int_err| parse_int_err.into())
    }
}

#[derive(PartialEq, Clone, Debug, Logos)]
#[logos(error = error::Error)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Assign,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("-")]
    Minus,
    #[token("~")]
    Tilde,
    #[token("!")]
    Exclamation,
    #[token("+")]
    Plus,
    #[token("*")]
    Multi,
    #[token("/")]
    Slash,
    #[token(":")]
    Colon,
    #[token("?")]
    QuestionMark,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("&")]
    SingleAnd,
    #[token("|")]
    InclusiveOr,
    #[token("^")]
    ExclusiveOr,
    #[token("%")]
    Mod,
    #[regex(r"[a-zA-Z_][0-9a-zA-Z_]*", |lex| IStr::new(lex.slice()))]
    Identifier(IStr),
    #[regex(r"[0-9]([0-9a-fA-Fe\.xb][0-9a-fA-Fe\.]*)?", |lex| Number::from_str(lex.slice()))]
    NumericLiteral(Number),
    #[regex(r#"'([^'\\]|\\.)*'|"([^"\\]|\\.)*""#, |lex| lex.slice().to_string())]
    StringLiteral(String),
    #[token("__func__")]
    FuncName,
    #[token("sizeof")]
    SIZEOF,
    #[token("->")]
    PtrOp,
    #[token("++")]
    IncOp,
    #[token("--")]
    DecOp,
    #[token("<<")]
    LeftOp,
    #[token(">>")]
    RightOp,
    #[token("<=")]
    LeOp,
    #[token(">=")]
    GeOp,
    #[token("==")]
    EqOp,
    #[token("!=")]
    NeOp,
    #[token("&&")]
    AndOp,
    #[token("||")]
    OrOp,
    #[token("*=")]
    MulAssign,
    #[token("/=")]
    DivAssign,
    #[token("%=")]
    ModAssign,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubAssign,
    #[token("&=")]
    AndAssign,
    #[token("^=")]
    XorAssign,
    #[token("|=")]
    OrAssign,
    #[token("<<=")]
    LeftAssign,
    #[token(">>=")]
    RightAssign,
    // TODO: this should be done when we found this is a typedef name,
    //       typedef LL int, then LL is typedef_name
    TypedefName,
    #[token("...")]
    ELLIPSIS,
    EnumerationConstant(String), // TODO: add check
    #[token("\n")]
    LineTerminator,
    EOF,

    #[token("typedef")]
    TYPEDEF,
    #[token("extern")]
    EXTERN,
    #[token("static")]
    STATIC,
    #[token("auto")]
    AUTO,
    #[token("register")]
    REGISTER,
    #[token("inline")]
    INLINE,
    #[token("const")]
    CONST,
    #[token("restrict")]
    RESTRICT,
    #[token("volatile")]
    VOLATILE,
    #[token("bool")]
    BOOL,
    #[token("char")]
    CHAR,
    #[token("short")]
    SHORT,
    #[token("int")]
    INT,
    #[token("long")]
    LONG,
    #[token("signed")]
    SIGNED,
    #[token("unsigned")]
    UNSIGNED,
    #[token("float")]
    FLOAT,
    #[token("double")]
    DOUBLE,
    #[token("void")]
    VOID,
    #[token("complex")]
    COMPLEX,
    #[token("imaginary")]
    IMAGINARY,
    #[token("struct")]
    STRUCT,
    #[token("union")]
    UNION,
    #[token("enum")]
    ENUM,
    #[token("case")]
    CASE,
    #[token("default")]
    DEFAULT,
    #[token("if")]
    IF,
    #[token("else")]
    ELSE,
    #[token("switch")]
    SWITCH,
    #[token("while")]
    WHILE,
    #[token("do")]
    DO,
    #[token("for")]
    FOR,
    #[token("goto")]
    GOTO,
    #[token("continue")]
    CONTINUE,
    #[token("break")]
    BREAK,
    #[token("return")]
    RETURN,
    #[token("alignas")]
    ALIGNAS,
    #[token("alignof")]
    ALIGNOF,
    #[token("atomic")]
    ATOMIC,
    #[token("generic")]
    GENERIC,
    #[token("noreturn")]
    NORETURN,
    #[token("static_assert")]
    StaticAssert,
    #[token("thread_local")]
    ThreadLocal,
}
