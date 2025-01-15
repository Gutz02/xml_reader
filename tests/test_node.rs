use xml_reader::node::{self, Node};
use std::{cell::{Ref, RefCell}, collections::HashMap, f32::consts::E, rc::{Rc, Weak}, vec};

#[test]
fn test_node_creation() {
    let attributes = HashMap::new();
    let node = Node::new("Root".to_string(), attributes, 0);

    assert_eq!(node.get_name(), "Root");
    assert!(node.get_parent().is_none());
    assert_eq!(node.get_children().len(), 0);
}

#[test]
fn test_set_parent() {
    let root = Node::new("Root".to_string(),  HashMap::new(), 0);
    let child = Node::new("Child".to_string(),  HashMap::new(),1);

    child.set_parent(root.clone());
    let parent = child.get_parent().unwrap();
    assert_eq!(&"Root".to_string(), parent.get_name());
}

#[test]
fn test_set_child() {
    let root = Node::new("Root".to_string(),  HashMap::new(),0 );
    let child = Node::new("Child".to_string(),  HashMap::new(), 1);

    root.set_child(child);    
    let children = root.get_children();
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].get_name(), "Child");
}

#[test]
fn test_get_attribute() {
    let mut attributes = HashMap::new();
    attributes.insert("id".to_string(), "123".to_string());

    let node = Node::new("Root".to_string(),  attributes,0 );

    assert_eq!(node.get_attribute("id".to_string()), Some(&"123".to_string()));
    assert_eq!(node.get_attribute("nonexistent".to_string()), None);
}

#[test]
fn test_get_parent_no_parent() {
    let node = Node::new("Orphan".to_string(), HashMap::new(),0 );
    assert!(node.get_parent().is_none());
}

#[test]
fn test_get_children_no_children() {
    let node = Node::new("Root".to_string(),  HashMap::new(), 0);
    let children = node.get_children();
    assert!(children.is_empty());
}
