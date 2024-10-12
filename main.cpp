#include <iostream>
#include <fstream>
#include <string>
#include <typeinfo>
#include "include/lexer.h"
#include "include/syntax.h"



int main(int argc, char** argv){
    int* array;
    std::string satir;

    for (int i = 0; i < argc; ++i) {
        std::cout << argv[i] << "\n";
    }
    
    std::ifstream dosyaOku(argv[1], std::ios::in);
            
    if(dosyaOku.is_open()){
        while(getline(dosyaOku,satir)){
            array=lexing(satir);
            syntaxer(satir,array, satir.length());
        }
       
        dosyaOku.close();
    }
    return 0;
}
