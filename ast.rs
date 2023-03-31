pub struct Node {
    pub start: usize,
    pub end: usize,

}

impl Node {
    pub fn new(start: usize, end: usize) -> Self {
        Self {start, end}
    }
}

pub struct Program {
    pub node : Node,
    pub body: Vec<Statement>,
}

pub enum Statement {
    VariableDeclarationStatement(VariableDeclaration),
    FunctionDeclarationStatement(FunctionDeclaration),
    ExpressionStatement(Expression)

}

pub struct VariableDeclaration{
    pub node: Node,
    pub declarations : Vec<VariableDeclarator>,
}

pub struct VariableDeclarator {
    pub node: Node,
    pub id: BindingIdentifier,
    pub init: Option<Expression>,
}

pub struct BindingIdentifier {
    pub node: Node,
    pub name: String,
}

pub struct FunctionDeclaration {
    node: Node,
    id: BindingIdentifier,
    params: Vec<String>,
    body: BlockStatement,
}

pub enum Expression {
    NullLiteral(Box<NullLiteral>),
    StringLiteral(Box<StringLiteral>),
    BooleanLiteral(Box<BooleanLiteral>),
    NumberLiteral(Box<NumberLiteral>),


    Identifier(String),

    ArrayExpression(Box<ArrayExpression>),
    BinaryExpression(Box<BinaryExpression>),
    UnaryExpression(Box<UnaryExpression>),
    LogicalExpression(Box<LogicalExpression>)
}

pub struct  IfStatement {
    pub node: Node,

    pub test: Box<Expression>,

    pub consequent: Box<Statement>,
    
    pub alternate: Box<Statement>
   
}

pub struct BlockStatement {
    pub node: Node,
    pub body: Vec<Statement>
}

pub struct NullLiteral {
    pub node: Node
}

pub struct StringLiteral {
    node: Node,
    value: String
}

pub struct BooleanLiteral {
    node: Node,
    value: bool
}

pub struct NumberLiteral  {
    node: Node,
    value: f64
}

pub struct ArrayExpression {
    elements: Vec<Expression>,
    node: Node
}

pub struct UnaryExpression{
    node: Node,
}

pub struct BinaryExpression {
    node: Node,
    left: Expression,
    operator: BinaryOperator,
    right: Expression
}

pub struct LogicalExpression {
    node: Node,
    left: Expression,
    operator: LogicalOperator,
    right: Expression
}

pub enum BinaryOperator {
    Equality,
    InEquality,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    BitwiseAnd,
    BitwiseOR,
    BitwiseXOR,
    Remainder,
}

pub enum LogicalOperator {
    And,
    Or
}