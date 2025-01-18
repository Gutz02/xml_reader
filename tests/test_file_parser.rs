use std::borrow::Borrow;
use std::rc::Rc;
use xml_reader::file_reader::*;
use xml_reader::file_parser::*;
use xml_reader::node::Node;

#[test]
fn test_tree(){
    let simulated_file_1: String = read_xml_file("tests\\to_parse_example.xml").unwrap();
    let formated_file: Vec<String> =  make_vec_list(&simulated_file_1);
    let parse_file : Vec<Rc<Node>> = parse_contents(&formated_file);

    assert_eq!(parse_file.len(), 7);
    assert_eq!(parse_file[3].get_contents().as_ref().unwrap(), "This is a ");
    assert_eq!(*parse_file[3].get_children()[1].as_ref(), *parse_file[4]);
    assert_eq!(*parse_file[4].get_parent().as_ref().unwrap(), parse_file[3]);
    assert_eq!(parse_file[4].get_contents().as_ref().unwrap(), "Formatted File?");
    assert_eq!(parse_file[5].get_contents().as_ref().unwrap(), "Or maybe parsed?");
    assert_eq!(*parse_file[2].get_children()[1].as_ref(), *parse_file[5]);

}

#[test]
fn test_extract_attributes(){
    let line1 = &"<title lang=\"en\" charset=\"UTF-8\" >".to_string();
    let line2 = &"<meta charset=\"UTF-8\" date=\"1-14-2025\"/>".to_string();

    let attributes1 = extract_attributes(line1, &"title".to_string());
    let attributes2 = extract_attributes(line2, &"meta".to_string());

    assert_eq!(attributes1.get("lang").unwrap(), "en");
    assert_eq!(attributes1.get("charset").unwrap(), "UTF-8");
    assert_eq!(attributes2.get("charset").unwrap(), "UTF-8");
    assert_eq!(attributes2.get("date").unwrap(), "1-14-2025");
}