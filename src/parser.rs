
use crate::ast::*;
use crate::lexer::*;
use std::process;
#[derive(Debug)]
pub struct Parser{
    pub tokens:Vec<Token>,
    pub current_token:usize,
    pub line:i32,
}
impl Parser {
    pub fn new(tokens:Vec<Token>, current_token:usize,line:i32)->Self{
        Self{
            tokens,
            current_token,
            line,

        }

    }
    pub fn parser(&mut self)-> ASTNode{

        let mut programNode:ASTNode=ASTNode{
            ast_type:ASTType::Program,
            values:"".to_string(),
            childeren:Vec::new(),
        };

        while self.current_token<self.tokens.len(){
            let token = &self.tokens[self.current_token];


            match token.token_type {
                TokenType::INT | TokenType::FLOAT => {
                    match self.parseVarDecl() {
                        Ok(stmt) => programNode.childeren.push(stmt),
                        Err(e) => {
                            eprintln!("Parse hatası: {}", e);
                            self.current_token += 1;
                        }
                    }
                }
                TokenType::PRINT => {
                    match self.parsePrint() {
                        Ok(stmt) => programNode.childeren.push(stmt),
                        Err(e) => {
                            eprintln!("Print hatası: {}", e);
                            self.current_token += 1;
                        }
                    }
                }
                TokenType::END =>{
                    self.current_token += 1;
                    continue;
                }
                _ => {
                    eprintln!("Syntax Error -> Unknown keyword, {:?} : {}", self.tokens[self.current_token].values, self.line); //Tanınmayan token: {:?
                    process::exit(1);
                }
            }
            /*match self.parseVarDecl() {
                Ok(stmt)=> programNode.childeren.push(stmt),
                Err(e)=>{
                    eprintln!("Parse hatası: {}", e);
                    self.current_token += 1;
                }
            }
            match self.parsePrint() {
                Ok(stmt)=> programNode.childeren.push(stmt),
                Err(e)=> {
                    eprintln!("Print hatasoı {}",e);
                    self.current_token += 1;
                }
            }

             */

        }


       programNode
    }
    pub fn parseVarDecl(&mut self) -> Result<ASTNode, String> {

        let type_token = if self.check(TokenType::INT) {
            "int"
        } else if self.check(TokenType::FLOAT) {
            "float"
        } else {
            eprintln!("Type Error-> there is no type as {}: {}",self.tokens.get(self.current_token).unwrap().values, self.line); //Beklenen veri tipi (int, float, auto) yok."
            process::exit(1);
        };

        if !self.check(TokenType::RARROW) {
            eprintln!("Syntax Error -> expected allocator ':>': {}", self.line);
            process::exit(1);
        }


        let name_token = match self.tokens.get(self.current_token) {
            Some(Token { token_type: TokenType::IDENTIFIER, values }) => {
                self.current_token += 1;
                values.clone()
            }
            _ => {
                eprintln!("Syntax Error -> no indentifier : {}", self.line);//Beklenen identifier (değişken ismi) yok.
                process::exit(1);
            }
        };


        if !self.check(TokenType::EQUL) {
            eprintln!("Syntax Error-> expected '=' initializer: {}", self.line);
            process::exit(1);
        }

        let value_token = match self.tokens.get(self.current_token) {
            Some(Token { token_type: TokenType::NUMINT, values }) |
            Some(Token { token_type: TokenType::NUMFLOAT, values }) => {
                self.current_token += 1;
                values.clone()
            }
            _ => {
                eprintln!("Syntax Error -> no matching type: {}", self.line);//Beklenen bir sayı değeri yok.
                process::exit(1);
            }
        };

        if !self.check(TokenType::END) {
            eprintln!("Syntax Error -> waiting ';': {}", self.line); //Satır sonunda ';' bekleniyor.
            process::exit(1);
        }


        let mut var_decl = ASTNode::default();
        var_decl.ast_type = ASTType::VarDecl;

        let mut type_node = ASTNode::default();
        type_node.ast_type = ASTType::Identifier;
        type_node.values = type_token.to_string();

        let mut name_node = ASTNode::default();
        name_node.ast_type = ASTType::Identifier;
        name_node.values = name_token;

        let mut value_node = ASTNode::default();
        value_node.ast_type = ASTType::NumDecl;
        value_node.values = value_token;


        var_decl.childeren.push(type_node);
        var_decl.childeren.push(name_node);
        var_decl.childeren.push(value_node);

        Ok(var_decl)
    }
    pub fn parsePrint(&mut self) -> Result<ASTNode, String> {
        let mut ast = ASTNode::default();
        let mut key="789425614";
        ast.ast_type = ASTType::PrintStmt;
        if self.tokens[self.current_token].token_type == TokenType::PRINT{
            if self.tokens[self.current_token+1].token_type == TokenType::LPAR && self.tokens[self.current_token+2].token_type == TokenType::QUOL{
                if self.tokens[self.current_token+4].token_type == TokenType::QUOL && self.tokens[self.current_token+5].token_type == TokenType::RPAR {
                    ast.values=self.tokens[self.current_token+3].values.to_string()+"\"";
                    self.current_token+=6;
                    return Ok(ast);
                }
                else {
                    self.current_token+=6;
                    eprintln!("Sytanx Error -> expected 'print' initializer: {}", self.line);
                    process::exit(1);
                }

            }
            else if self.tokens[self.current_token+1].token_type == TokenType::LPAR && self.tokens[self.current_token+2].token_type == TokenType::IDENTIFIER{
                if self.tokens[self.current_token+3].token_type == TokenType::RPAR{
                    ast.values=self.tokens[self.current_token+2].values.to_string();
                    self.current_token+=4;
                    return Ok(ast);
                }
                else{
                     eprintln!("Syntax Error -> expected 'print' initializer{}", self.line);
                    process::exit(1);
                }

            }

        }
        else {
            eprintln!("Syntax Error -> expected 'print' initializer: {}", self.line);
            process::exit(1);
        }
        println!("{}",self.tokens[self.current_token].values.clone().to_string());
        eprintln!("Invalid print statement: {}", self.line);
        process::exit(1);
    }
    pub fn check(&mut self,mut t:TokenType)->bool{


        if self.current_token<self.tokens.len() && self.tokens[self.current_token].token_type==t {
            self.current_token +=1;
            return true;
        }
        else {
            return false;
        }

    }

}
