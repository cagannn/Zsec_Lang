use log::error;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LPAR,       // (
    RPAR,       // )

    
    PLUS,       // +
    MINUS,      // -
    MUL,        // *
    DIV,        // /
    EQUL,       // =
    RARROW,     // :> 
    COMMA,      // , 

   
    IDENTIFIER,
    INT,
    FLOAT,
    AUTO,// int, float, auto, vs.
    NUM,        // 123, 3.14
    STRING,     // "metin"
    BOOLEAN,    // true, false
    NUMINT,
    NUMFLOAT,
   
    QUOL,       // " (
    END,        // ; 


    COMMENT,     // // yorum
    ROWEND,
    PRINT,
    EOF
}
#[derive(Debug, Clone)]
pub struct Token {

    pub token_type: TokenType,
    pub values: String,
}

impl Token {
    fn new(token_type: TokenType, values: String) -> Token {
        Token { token_type, values }
    }
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let  chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        let mut count = 0;
        while i <chars.len() {

            if chars[i]==' ' {
                i +=1;
                continue;
            }
            else if chars[i].is_digit(10) {
                tokens.push(Self::lexNumber(&chars,&mut i,input.len(),input))
            }
            else if chars[i].is_alphabetic() {

                tokens.push(Self::lexIdentifierOrKeyword(&chars,&mut i));
            }
            else if i+1<chars.len() && chars[i]==':' && chars[i+1]=='>'{
                tokens.push(Token::new(TokenType::RARROW, ":>".to_string()));
            }
            else if chars[i]==';' {
                tokens.push(Token::new(TokenType::END, ";".to_string()));

            }
            else if chars[i]=='=' {
                tokens.push(Token::new(TokenType::EQUL, "=".to_string()));
            }
            else if chars[i]=='+' {
                tokens.push(Token::new(TokenType::PLUS, "+".to_string()));
            }
            else if chars[i]=='-' {
                tokens.push(Token::new(TokenType::MINUS, "-".to_string()));
            }
            else if chars[i]=='*' {
                tokens.push(Token::new(TokenType::MUL, "*".to_string()));
            }
            else if chars[i]=='/' {
                tokens.push(Token::new(TokenType::DIV, "/".to_string()));
            }
            else if chars[i]=='(' {
                tokens.push(Token::new(TokenType::LPAR, "(".to_string()));
            }
                else if chars[i]==')' {
                    tokens.push(Token::new(TokenType::RPAR, ")".to_string()));
                }
             else if chars[i]=='"'{
                 tokens.push(Token::new(TokenType::QUOL,'"'.to_string()));
                 tokens.push(Self::afterQual(&chars,&mut i));
                 tokens.push(Token::new(TokenType::QUOL,'"'.to_string()));

             }
            else{
                i+=1;
                continue;
            }
            i+=1;
        }
        tokens
    }

    pub fn lexNumber(chars: &Vec<char>, i:&mut usize,inputLen:usize,input:&str) -> Token {
        let mut token:Token=Self::new(TokenType::EOF,"".to_string());
        let mut a=inputLen-*i;
        let mut isThereDot:bool=false;
        for j in *i..inputLen{
            if !chars[j].is_digit(10) {
                if chars[j]=='.'{
                    isThereDot=true;
                    continue;
                }
                else{
                    a=j-*i;
                    break;
                }
            }
        }

        if isThereDot {
            let slice: String = chars[*i..*i+a].iter().collect();
            token.token_type = TokenType::NUMFLOAT;
            token.values = slice;



        }
        else{
            let slice: String = chars[*i..*i+a].iter().collect();
            token.token_type = TokenType::NUMINT;
            token.values = slice;

        }
        *i=*i+(a-1);
        token
    }
    pub fn lexIdentifierOrKeyword(chars: &Vec<char>, i:&mut usize) -> Token {
        let mut token:Token=Token::new(TokenType::EOF,chars[0].to_string());
        let mut deger:String=String::new();
        let mut a=0;
        for j in *i..chars.len(){
            if !chars[j].is_alphabetic(){
                a=j-*i;
                break;
            }
        }
        let slice: String = chars[*i..(*i + a)].iter().collect();
        deger=deger + &slice;
        if deger =="int"{
            token.token_type = TokenType::INT;
            token.values = deger;
        }
        else if deger =="float"{
            token.token_type = TokenType::FLOAT;
            token.values = deger;
        }
        else if deger =="bool"{
            token.token_type=TokenType::BOOLEAN;
            token.values = deger;
        }
        else if deger =="auto"{
            token.token_type = TokenType::AUTO;
            token.values = deger;
        }
        else if deger=="print"{
            token.token_type = TokenType::PRINT;
            token.values = deger;
        }
        else {
            token.token_type = TokenType::IDENTIFIER;
            token.values = deger;
        }
        *i=*i+(a-1);
        token
    }
    pub fn afterQual(chars: &Vec<char>, i:&mut usize) -> Token {
        let mut token:Token=Token::new(TokenType::EOF,"".to_string());
        let mut deger:String=String::new();
        *i=*i+1;
        while chars[*i] != '"'{
            deger.push(chars[*i]);
            *i+=1;
        }
        token.values = deger;
        token.token_type = TokenType::IDENTIFIER;
        return token;

    }
}
