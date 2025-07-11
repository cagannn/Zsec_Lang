
use crate::ast::*;
use std::process;
pub struct Interpreter{
    pub variables:std::collections::HashMap<String, f64>,

}
impl Interpreter{

    pub fn interpret(&mut self,root:ASTNode, line:i32)->(){
       if root.ast_type != ASTType::Program{
           println!("interpret(): Kök düğüm Program türünde olmalı.");
       }
        for i in root.childeren{

                    self.eval(&i,line);

        }

    }
    pub fn eval (&mut self, node:&ASTNode,line:i32)->f64{
        if node.ast_type == ASTType::VarDecl {
            if let (Some(type_node), Some(name_node), Some(value_node)) = (
                node.childeren.get(0),
                node.childeren.get(1),
                node.childeren.get(2),

            )
            {
                let value: f64 = self.eval(value_node,line);
                let name = &name_node.values;

                self.variables.insert(name.clone(), value);


                return value;
            }

            return 0_f64;

        }
        else if node.ast_type == ASTType::NumDecl{
            let mut mut_node= &node.values;
            return mut_node.parse().unwrap();
        }
        else if node.ast_type == ASTType::PrintStmt{

            let mut a = &node.values;
            let bytes:Vec<u8>=a.bytes().collect();
            let mut last_w=String::new();
            let mut is_string=false;
            for mut i in 0..bytes.len(){
                if bytes[i]==55 && i+8<= bytes.len() { //7
                    if bytes[i+1]==56 && bytes[i+2]==57 && bytes[i+3]==52 && bytes[i+4]==50 && bytes[i+5]==53 && bytes[i+6]==54 && bytes[i+7]==49 && bytes[i+8]==52 {
                        is_string=true;
                    }
                }
                if is_string{
                    break;
                }
                else{
                    last_w.push(bytes[i] as char);
                }
            }
            if self.variables.get(a)!=None{
                println!("{:?}",self.variables.get(a).unwrap());
            }
            else if self.variables.get(a)==None && is_string==false{
                eprintln!("There is no variable as {}: {}", a, line);
                process::exit(1);
            }
            else if self.variables.get(a)==None && is_string==true{
                println!("{}",last_w);
            }

            return 0_f64;
        }
        else {
            println!("Uyarı");
            return 0_f64;
        }
    }
}
