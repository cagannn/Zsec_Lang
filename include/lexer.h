#ifndef lexer
#define lexer
#include <iostream>
#include <string>


enum tokens{
    LPAR,
    RPAR,
    NUM,
    PLUS,
    MINUS,
    MUL,
    DIV,
    IDENTIFIER,
    STRING,
    SPACE,
    CHAR,
    RARROW,
    EQUL,
    END,

};

int* lexing(std::string& x){

    int count=0;
    int* tokens;
    tokens = new int[x.length()];
    for (int i=0; i<x.length();i++){
    
        if (x[i] == '+') {
            tokens[i]= PLUS;
            
        } 
        else if (x[i] == '-') {
            tokens[i]=MINUS    ;
                         
        } 
        else if (x[i] == '*') {
            tokens[i]=MUL;
             
        } 
        else if (x[i] == '/') {
            tokens[i]=DIV;
             
        } 
        else if (x[i]=='1' || x[i]=='2' || x[i]=='3' || x[i]=='4' || x[i]=='5' || x[i]=='6' || x[i]=='7' || x[i]=='8' || x[i]=='9' || x[i]=='0'){
            tokens[i]=NUM;
             
        }
        else if (x[i]=='('){
            tokens[i]=LPAR;
             
        }
        else if (x[i]==')'){
            tokens[i]=RPAR;
            
        }
        else if (x[i]==' '){
            tokens[i]=SPACE;
        }
        else if (x[i]=='>'){
            tokens[i]=RARROW;
        }
        else if (x[i]==';'){
            tokens[i]=END;
        }
        else if(x[i]=='='){
            tokens[i]=EQUL;
        }
        else {
            tokens[i]=CHAR;

        }
    }
    return tokens;  
    
    }
    
#endif
