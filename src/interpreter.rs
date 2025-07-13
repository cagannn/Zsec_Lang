
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
            let mut is_string=false;
            if node.values.get(node.values.len()-1..node.values.len()) == Some("\""){
                is_string=true;
            }
            if self.variables.get(&node.values)!=None{
                println!("{:?}",self.variables.get(&node.values).unwrap());
            }
            else if self.variables.get(&node.values)==None && is_string==false{
                eprintln!("There is no variable as {}: {}", node.values, line);
                process::exit(1);
            }
            else if self.variables.get(&node.values)==None && is_string==true{
                println!("{}",node.values.get(0..node.values.len()-1).unwrap());
            }

            return 0_f64;
        }
        else {
            println!("Uyarı");
            return 0_f64;
        }
    }
}
