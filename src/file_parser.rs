use std::{collections::HashMap, rc::Rc};
use crate::node::{self, Node};

pub fn trim_line(line: &String) -> String {
    line.replace("?", "")
        .replace("/>", "")
        .replace("</", "")
        .replace("<", "") 
        .replace(">", "")
        .to_string()
}
pub fn detect_node_name(line : &String) -> String{
    let line = trim_line(line);
    match line.find(" ") {
        Some(name_end_i) => String::from(&line[0..name_end_i]),
        None => {
            if line.len() > 0{
                line
            }else {
                "ERROR".to_string()
            }
        }
    }
}

pub fn extract_attributes(line: &String, name : &String) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    let mut processed_line: String = trim_line(line);
    processed_line = processed_line.replace(name, "");
    
    while processed_line.len() >= 5{
        let equal_i: usize = processed_line.find('=').unwrap();
        let key: String = processed_line[1..equal_i].to_string().replace(" ", "");
        processed_line.replace_range(..equal_i+2, "");

        let value_i : usize = processed_line.find('"').unwrap();
        let value : String = processed_line[..value_i].to_string();
        processed_line.replace_range(..value_i+1, "");
        result.insert(key.to_string(), value.to_string());
    }

    result
}


pub fn parse_contents(contents : &Vec<String>) -> Vec<Rc<Node>>{

    let mut result : Vec<Rc<Node>> = Vec::new();
    let mut unclosed_tags : Vec<usize> = vec![];
    let mut node_id : usize = 0;

    for line in contents{
        let line = &String::from(line.trim_start());
        if !line.is_empty() && !line.trim_start().starts_with('<'){// text stuff
            let unclosed_tag = result.iter().find(|node| node.get_id() == *unclosed_tags.last().unwrap()).unwrap();
            unclosed_tag.set_content(line);
        }else{
            let node_name = detect_node_name(line);
            if line.starts_with("</") {// closing tag <node/>
                unclosed_tags.pop();
            }else{
                let attributes: HashMap<String, String> = extract_attributes(line, &node_name);
                if line.ends_with("/>"){ // self closing tag </node>
                    result.push(
                        Node::new(node_name,  attributes, node_id)
                    );
                }else{// Opening tag <node>
                    let current_node = Node::new(node_name,  attributes, node_id);
                    if node_id != 0{ // this means that it is not the first node
                        let my_parrent = result.iter().find(|node| node.get_id() == *unclosed_tags.last().unwrap()).unwrap().clone();
                        my_parrent.set_child(current_node.clone());
                        current_node.set_parent(my_parrent);
                    }
                    unclosed_tags.push(node_id);
                    result.push(current_node);
                }
                node_id += 1;
            }
        }

    }

    result
}