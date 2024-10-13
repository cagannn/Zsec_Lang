#ifndef syntax
#define syntax
#include <iostream>
#include <string>
#include <sstream>
#include <typeinfo>
#include "errors.h"


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
int syntaxer(std::string& code_,int* lexed, int size_){
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
                            if(lexed[k]==13)
                                break;
                            if(code_[k]=='1' || code_[k]=='2' || code_[k]=='3' || code_[k]=='4' || code_[k]=='5' || code_[k]=='6' || code_[k]=='7' || code_[k]=='8' || code_[k]=='9' || code_[k]=='0')
                                hold_int=hold_int+code_[k];
                            
                            else{
                                varType_err();
                                break;
                                return 0;
                                }
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
            
            for(int j=i+1;j<=size_;j++){
                
                if(lexed[j]==12){
                    for(int k=j+1;k<size_-1;k++){
                      
                        if (lexed[j+1]==14 && lexed[size_-2]==14)
                        {
                            if(lexed[k]!=13 ){
                                count++;
                            }
                        }
                        else{
                            syntax_err();
                            return 0;
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
                    for(int k=j+1;k<size_-3;k++){
                        if(lexed[k]!=13 ){
                            hold_str[k-(j+1)]=code_[k+1];
                            
                        }
                    }
                }
                
            }
            char*str_var=(char* )calloc(count+1,sizeof(char));
            
            for(int l=0;l<count-2;l++){
                str_var[l]=hold_str[l];
            }
            delete hold_str;
            hold_str=nullptr;
            std::cout<<"Bellekte olusturulan deger "<<var_name<<" >> "<<str_var<<std::endl;
           
        }
        else if (command=="float" && lexed[i+1]==4 && lexed[i+2]==11){
            std::cout<<"float olusturulacak"<<std::endl;
        }
        else if(command=="bool" && lexed[i+1]==4 && lexed[i+2]==11){
            int *count =new int;
            *count=0;
            bool* real_var= new bool();
             for(int j=i+1;j<=size_;j++){
                if(lexed[j]==12){
                    for(int k=j+1;k<size_;k++){
                        if(lexed[k]!=13){
                            *count=*count+1;
                        }
                        
                    }
                }
            }
            char* true_false=(char* )calloc(4,sizeof(char));
            char* tamp_code=new char[code_.length()+1];
            for(int n=0;n<code_.length()-1;n++){
                tamp_code[n]=code_[n];
            }
            int* hold_c=new int;
            *hold_c=1;
           
             for(int j=i+1;j<=size_;j++){
                if(lexed[j]==12){
                    for(int k=j+1;k<size_;k++){
                        if(lexed[k]!=13){
                            true_false[*hold_c]=tamp_code[k];
                    
                            *hold_c=*hold_c+1;
                            
                        }
                        
                    }
                }
            }
            if (true_false[1]=='t' && true_false[2]=='r' && true_false[3]=='u' && true_false[4]=='e'){
                *real_var=true;
            }
            else if (true_false[1]=='f' && true_false[2]=='a' && true_false[3]=='l' && true_false[4]=='s' && true_false[5]=='e')
            {
                *real_var=false;;
            }
            else{
                syntax_err();
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

        delete count;
        count=nullptr;
        delete hold_c;
        hold_c=nullptr;
        delete true_false;
        true_false=nullptr;

        std::cout<<"Bellekte olusturulan deger: "<<var_name<<" >> "<<*real_var<<std::endl;
    }
        if(command=="func" && lexed[i+1]==9 && lexed[i+2]==10){
        std::cout<<"fonksiyon olusturulacak"<<std::endl;
        }
    }
}
return 1;
}

#endif
