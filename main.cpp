#include <iostream>
#include <fstream>
#include <string>
#include <typeinfo>
#include "include/lexer.h"
#include "include/syntax.h"

bool end_with(std::string file_name){
    std::string _end="";
    for(int i=0;i<file_name.length();i++){
    if(file_name[i]=='.'){
        for (int j=i+1; j<file_name.length();j++){
            _end=_end+file_name[j];
        }
        break;
    }
    }
    if(_end=="zsec")
        return true;
    else
        return false;
}

int main(int argc, char** argv){
    if(end_with(argv[1]))
    {
    int* array;
    std::string satir;
    std::string file_name;
    file_name=argv[1];  
    std::ifstream dosyaOku(argv[1], std::ios::in);
   
   if(dosyaOku.is_open()){
        while(getline(dosyaOku,satir)){
            //x[i]=kar;
            
            array=lexing(satir);
            syntaxer(satir,array, satir.length());
        }
       
        dosyaOku.close();
    }}
    else
    std::cout<<"Error";
    return 0;
}
