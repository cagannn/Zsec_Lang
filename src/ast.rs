use std::arch::is_aarch64_feature_detected;
#[derive(Debug,PartialEq)]
pub enum ASTType{
    VarDecl,
    NumDecl,
    Identifier,
    BinaryExpression,
    Program,
    PrintStmt,
    NullType,
}
#[derive(Debug)]
pub struct ASTNode{
    pub ast_type:ASTType,
    pub values:String,
    pub childeren: Vec<ASTNode>,
}

impl ASTNode{
    pub fn new(AstType:ASTType, values: String,childeren:Vec<ASTNode>)->Self{
        Self {ast_type:AstType,values:values,childeren:childeren}
    }
}
impl Default for ASTNode{
    fn default()->Self{
        ASTNode{
            ast_type: ASTType::Program,
            values:String::new(),
            childeren:Vec::new()
        }
    }
}
