use std::{cell::{Ref, RefCell}, collections::HashMap, fmt, rc::{Rc, Weak}, vec};

pub struct Node{
    name : String,
    contents : RefCell<Option<String>>,
    attributes : HashMap<String,String>,
    parent: RefCell<Option<Weak<Node>>>, 
    children : RefCell<Vec<Rc<Node>>>,
    id : usize
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_id = self.get_parent().map_or("None".to_string(), |parent| parent.get_id().to_string());

        let children_ids: Vec<String> = self
            .get_children()
            .iter()
            .map(|child| child.get_id().to_string())
            .collect();

        let attributes: Vec<String> = self
            .attributes
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect();

        write!(
            f,
            "ID :: {} || Name :: {} || Attributes :: [ {} ] || Parent :: {} || Children :: Vec![ {} ]",
            self.id,
            self.name,
            attributes.join(", "),
            parent_id,
            children_ids.join(", ")
        )
    }
}


impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}



impl Node {
    
    pub fn new(name : String, attributes: HashMap<String,String>, id : usize) -> Rc<Self> {
        Rc::new(
            Node{
                name,
                contents : RefCell::new(None),
                attributes,
                parent: RefCell::new(None),
                children : RefCell::new(vec![]),
                id
            }
        )
    }

    pub fn get_name(&self) -> &String{
        &self.name
    }

    pub fn get_contents(&self) -> Ref<Option<String>>{
        self.contents.borrow()
    }

    pub fn set_content(&self, content : &String){
        *self.contents.borrow_mut() = Some(content.clone());
    }

    pub fn set_parent(&self, parent : Rc<Node>) {
        *self.parent.borrow_mut() = Some(Rc::downgrade(&parent));
    }

    pub fn set_child(&self, child : Rc<Node>){
        self.children.borrow_mut().push(child);
    }

    pub fn get_parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().as_ref()?.upgrade()
    }

    pub fn get_children(&self) -> Ref<Vec<Rc<Node>>>{
        self.children.borrow()
    }

    pub fn get_attribute(&self, id : String) -> Option<&String>{
        self.attributes.get(&id)
    }

    pub fn get_id(&self) -> usize{
        self.id
    }

}


