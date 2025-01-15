use std::{cell::RefCell, collections::HashMap, fs, result, vec};



pub fn read_xml_file(file_name: &str) -> Result<String, String> {
    match fs::read_to_string(file_name) {
        Ok(data) => Ok(data),
        Err(_) => Err("No Such File Found".to_string()),
    }
}

pub fn make_vec_list(file_contents : &String) -> Vec<String> {
    file_contents.lines().map(|line| line.to_string()).collect()
}

pub fn is_prolog(prolog: &String) -> bool {
    prolog.contains("<?") & prolog.contains("?>")
}

pub fn is_comment(comment: &String) -> bool{
    comment.contains("<!--") & comment.contains("-->")
}

pub fn is_text(contents: &String) -> bool{
    !contents.is_empty() & !contents.contains("<")
}

fn calculate_indentation(line: &String) -> usize {
    line.find('<')
        .map(|index| line[..index].chars().filter(|&c| c == ' ').count())
        .unwrap_or(0) / 4
}


pub fn line_extractor(line : &mut String) -> Vec<String>{
    let mut result : Vec<String> = Vec::new();

    while !line.is_empty(){
        let start = line.find("<").unwrap();
        let end = line.find(">").unwrap()+1;

        if start != 0 {
            result.push(String::from(
                &line[0..start]
            ));
            line.replace_range(0..start, "");
        }
        
        else {
            let slit = &line[start..end];
            result.push(String::from(slit));
            line.replace_range(start..end, "");
        }

    }
    result
}

pub fn line_formatter(indentation: usize, mut formatted_elements: Vec<String>) -> Vec<String> {
    
    let mut indent_level = indentation;
    let mut no_inc = false;

    for i in 0..formatted_elements.len() {
        if i != 0 {
            let line = &formatted_elements[i];
            if line.starts_with("</") {
                indent_level -= 1;
                no_inc = true;
            } else if line.contains("/>") {
                no_inc = true;
            } else if line.starts_with("<") {
                if !no_inc {
                    indent_level += 1;
                }else{
                    no_inc = false;
                }
            } else if no_inc == false {
                indent_level += 1;
                no_inc = true;
            }

        }
        let num_spaces = "\t".repeat(indent_level);
        formatted_elements[i] = format!("{num_spaces}{}", formatted_elements[i]);

    }

    formatted_elements
}



pub fn file_formatter(file_contents : &String) -> Vec<String>{
    let current_contents = make_vec_list(file_contents);

    let mut formatted_contents: Vec<String> = Vec::new();
    let mut line_indentation : usize = 0;

    for mut line in current_contents{
        if is_prolog(&line){
            formatted_contents.push(line);
        }else if is_comment(&line) {
            formatted_contents.push(line);
        }else if is_text(&line){
            line_indentation = line_indentation + 1; 
            line = line.trim().to_string();
            let num_spaces = "\t".repeat(line_indentation);
            line = format!("{num_spaces}{}", line);
            formatted_contents.push(line);
        }
        else {
            line_indentation = calculate_indentation(&line);
            line = line.trim().to_string();
            let extracted_elements = line_extractor(&mut line);

            let formatted_elemtns = line_formatter(line_indentation, extracted_elements);
            formatted_contents.extend(formatted_elemtns);
        }
    }

    formatted_contents
}