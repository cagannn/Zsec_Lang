#ifndef syntax
#define syntax
#include <iostream>
#include <string>
#include <sstream>


/*enum tokens{
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
    EQUAL
    END,
    };
*/
int syntaxer(std::string code_,int* lexed, int size_){
    std::string command="";
    std::string var_name="";
 for(int i=0;i<size_;i++){
    if (lexed[i]==10){
    command=command+code_[i];
        if(command=="int" && lexed[i+1]==4 && lexed[i+2]==11){
            int *p=new int();
            int *int_var = (int* )calloc(1,sizeof(int));
            std::stringstream hold;
            std::string hold_int="";
            for (int j=i+3;j<size_;j++){
        
                if (lexed[j]==12){
                    for (int k=j+1;k<size_;k++){
                        if(lexed[k]!=13 ){
                            hold_int=hold_int+code_[k];
                        }
                    }
                hold << hold_int;
                hold >> *p;            
            }
        var_name=var_name+code_[j];
}
            *int_var=*p;
            std::cout<<"Bellekte olusturulan deger:"<<*int_var<<std::endl;
            delete p;
            p=nullptr;
        }
        if(command=="string" && lexed[i+1]==4 && lexed[i+2]==11){
            std::cout<<"string olusturulacak"<<std::endl;
        }
        if(command=="func" && lexed[i+1]==9 && lexed[i+2]==10){
        std::cout<<"fonksiyon olusturulacak"<<std::endl;
        }
    }
}
return 0;
}

#endif
