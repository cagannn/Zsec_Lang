#ifndef errors
#define errors
#include <iostream>

void logic_err(){
    std::cout<<"Entered file not found"<<std::endl;
}
void varType_err(){
    std::cout<<"variableType Error"<<std::endl;
}
void syntax_err(){
    std::cout<<"Syntax Error"<<std::endl;
}

#endif
