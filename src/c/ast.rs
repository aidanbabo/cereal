use crate::S;

#[derive(Debug)]
pub enum Type {
    Int,
}

#[derive(Debug)]
pub enum Literal<'s> {
    Numeric(S<'s, i32>),
}

#[derive(Debug)]
pub enum UnaryType {
    Negate,
    Plus,
    BitNot,
}

#[derive(Debug)]
pub struct Unary<'s> {
    pub ty: S<'s, UnaryType>,
    pub expr: Box<Expression<'s>>,
}

#[derive(Debug)]
pub enum BinaryType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitXor,
    BitOr,
}

#[derive(Debug)]
pub struct Binary<'s> {
    pub left: Box<Expression<'s>>,
    pub ty: S<'s, BinaryType>,
    pub right: Box<Expression<'s>>,
}

#[derive(Debug)]
pub enum AssignmentType {
    Regular,
}

#[derive(Debug)]
pub struct Assignment<'s> {
    pub left: Box<Expression<'s>>,
    pub ty: S<'s, AssignmentType>,
    pub right: Box<Expression<'s>>,
}

#[derive(Debug)]
pub struct Comma<'s> {
    pub left: Box<Expression<'s>>,
    pub right: Box<Expression<'s>>,
}

#[derive(Debug)]
pub enum ExpressionType<'s> {
    Literal(Literal<'s>),
    Unary(Unary<'s>),
    Binary(Binary<'s>),
    Assignment(Assignment<'s>),
    Variable(&'s str),
    Comma(Comma<'s>),
}

#[derive(Debug)]
pub struct Expression<'s> {
    pub ty: ExpressionType<'s>,
    pub expr_ty: Option<Type>,
}

#[derive(Debug)]
pub struct Return<'s> {
    pub expr: Option<Expression<'s>>,
}

#[derive(Debug)]
pub enum StatementType<'s> {
    Return(S<'s, Return<'s>>),
    Expression(Expression<'s>),
}

#[derive(Debug)]
pub struct Statement<'s> {
    pub ty: StatementType<'s>,
}

#[derive(Debug)]
pub struct Declaration<'s> {
    pub ty: S<'s, Type>,
    pub names: Vec<(usize, &'s str)>, // number of indirections and name
}

#[derive(Debug)]
pub struct Procedure<'s> {
    pub args: Vec<S<'s, (Type, &'s str)>>,
    pub name: S<'s, &'s str>,
    pub return_type: Option<S<'s, Type>>,
    pub declarations: Vec<Declaration<'s>>,
    pub body: Vec<Statement<'s>>,
}

#[derive(Debug)]
pub enum TopLevelType<'s> {
    Procedure(Procedure<'s>),
}

#[derive(Debug)]
pub struct TopLevel<'s> {
    pub ty: TopLevelType<'s>,
}

