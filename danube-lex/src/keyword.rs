#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    If,           // if
    Else,         // else
    For,          // for
    While,        // while
    Loop,         // loop
    In,           // in
    Break,        // break
    Continue,     // continue
    Match,        // match
    Return,       // return
    Yield,        // yield
    Where,        // where
    Const,        // const
    Static,       // static
    Let,          // let
    Mut,          // mut
    Function,     // fn
    Trait,        // trait
    Struct,       // struct
    Type,         // type
    Enum,         // enum
    Impl,         // impl
    Module,       // mod
    TypeSelf,     // Self
    VariableSelf, // self
    Public,       // pub
    Async,        // async
    Await,        // await
    Use,          // use
    Super,        // super
    As,           // as
    Placeholder,  // _
}