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
            int *int_var=new int();
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
            hold >> *int_var;            
    }

}
    for (int p=i+1;p<size_;p++){
        if(lexed[p]==11){
             for (int n=p+1;n<size_;n++){
                   if(lexed[n]==12){
                           break;
                    }
                 var_name=var_name+code_[n];                        
            }
        }
    }
       std::cout<<"Bellekte olusturulan deger: "<<var_name<<" >> "<<*int_var<<std::endl;
            
        }
        else if(command=="string" && lexed[i+1]==4 && lexed[i+2]==11){
            int count=0;
            
            for(int j=i+1;j<size_;j++){
                if(lexed[j]==12){
                    for(int k=j+2;k<size_-1;k++){
                        if(lexed[k]!=13 ){
                            
                            count++;
                        }
                    }
                }
                if(lexed[j]==11){
                    for (int n=j+1;n<size_;n++){
                        if(lexed[n]==12){
                            break;
                            }
                        var_name=var_name+code_[n];
                        
                    }
                
                }
            }
            char* hold_str=new char(count+1);
                  for(int j=i+1;j<size_;j++){
                if(lexed[j]==12){
                    for(int k=j+1;k<size_;k++){
                        if(lexed[k]!=13 ){
                            hold_str[k-(j+1)]=code_[k+1];
                            
                        }
                    }
                }
                
            }
            char*str_var=(char* )calloc(count+1,sizeof(char));
            
            for(int l=0;l<count-1;l++){
                str_var[l]=hold_str[l];
            }
            std::cout<<"Bellekte olusturulan deger "<<var_name<<" >> "<<str_var<<std::endl;
           
        }
        else if (command=="float" && lexed[i+1]==4 && lexed[i+2]==11){
            std::cout<<"float olusturulacak"<<std::endl;
        }
        else if(command=="bool" && lexed[i+1]==4 && lexed[i+2]==11){
            std::cout<<"bool olusturulacak"<<std::endl;
        }
        if(command=="func" && lexed[i+1]==9 && lexed[i+2]==10){
        std::cout<<"fonksiyon olusturulacak"<<std::endl;
        }
    }
}
return 0;
}

#endif
