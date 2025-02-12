use std::collections::HashMap;

#[derive(Debug)]
struct BaseEntity {
    key: String,
    name: String,
    multiplicity: u16,
    attributes: HashMap<String, BaseAttribute>,
}

impl BaseEntity {

    fn new(key: String, name: String, multiplicity: u16, attribute: HashMap<String, BaseAttribute>) -> BaseEntity{
        if self.attributes == None {
            self.attributes = HashMap::new()
        }
        self.attributes.insert(key , attribute);
        BaseEntity {key: key, name : name, multiplicity : multiplicity, attributes: todo!() }
    }

    fn add_attribute(&mut self, attribute: HashMap<String, BaseAttribute>){
        // self.attributes.in
    }
}

#[derive(Debug)]
struct BaseAttribute {
    key: String,
    name: String,
    multiplicity: u16,
}

impl BaseAttribute {
    
}

fn main() {
    let base_entity = BaseEntity = {

    }


    println!("Hello, world!");
}
