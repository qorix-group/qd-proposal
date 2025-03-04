use std::collections::HashMap;

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)]
struct BaseAttribute {
    // identity elements
    key: String,
    name: String,
    multiplicity: u16,
    att_type: String,
    required: bool,

    // non-identity elements
    value: String,
}

#[allow(dead_code)]
impl BaseAttribute {
    fn new(key: String, name: String, multiplicity: u16, att_type: String) -> BaseAttribute {
        BaseAttribute {
            key: key,
            name: name,
            multiplicity: multiplicity,
            att_type: att_type,
            ..Default::default()
        }
    }

    fn set_value(&mut self, value: String) {
        self.value.clear();
        self.value.push_str(&value);
    }

    fn set_type(&mut self, value: String) {
        self.att_type.clear();
        self.att_type.push_str(&value);
    }

    fn set_required(&mut self, required: bool) {
        self.required = required;
    }

    fn get_key(&self) -> String {
        self.key.to_string()
    }

    fn get_type(&self) -> String {
        self.att_type.to_string()
    }

    fn validate(&self, input_attr: &BaseAttribute) -> bool {
        self == input_attr
    }
}

impl PartialEq for BaseAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.name == other.name
            && self.multiplicity == other.multiplicity
            && self.att_type == other.att_type
            && self.required == other.required
            && self.value == other.value
    }
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)]
struct BaseEntity {
    key: String,
    name: String,
    attributes: HashMap<String, BaseAttribute>,
}

#[allow(dead_code)]
impl BaseEntity {
    fn new(key: String, name: String) -> BaseEntity {
        BaseEntity {
            key: key,
            name: name,
            attributes: HashMap::new(),
        }
    }

    fn add_attribute(
        &mut self, // must be mutable
        key: &str,
        node: BaseAttribute,
    ) {
        self.attributes.insert(key.to_string(), node);
    }

    fn validate(&self, input_entity: &BaseEntity) -> bool {
        self == input_entity
    }
}

impl PartialEq for BaseEntity {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.name == other.name && self.attributes == other.attributes
    }
}

use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// #[derive(Deserialize, Debug)]
// struct User {
//     fingerprint: String,
//     location: String,
// }

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<BaseEntity, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: BaseEntity = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

// fn main() {
//     let u = read_user_from_file("test.json").unwrap();
//     println!("{:#?}", u);
// }

fn main() {
    // Rest Entity
    let mut my_rest_configuration: BaseEntity = BaseEntity::new(
        "Rest_R1".to_owned(),
        "My Rest API configuration".to_string(),
    );

    // Attributes of REST API
    let method_type = BaseAttribute::new(
        "A1".to_owned(),
        "methodType".to_owned(),
        1,
        "String".to_owned(),
    );
    let url = BaseAttribute::new("A2".to_owned(), "url".to_owned(), 1, "String".to_owned());
    let mut port = BaseAttribute::new("A3".to_owned(), "port".to_owned(), 1, "int".to_owned());
    port.set_value("80".to_string());

    my_rest_configuration.add_attribute(&method_type.get_key(), method_type);
    my_rest_configuration.add_attribute(&url.get_key(), url);
    my_rest_configuration.add_attribute(&port.get_key(), port);

    // Rest Entity
    let mut my_rest_configuration_1: BaseEntity = BaseEntity::new(
        "Rest_R1".to_owned(),
        "My Rest API configuration".to_string(),
    );

    // Attributes of REST API
    let method_type = BaseAttribute::new(
        "A1".to_owned(),
        "methodType".to_owned(),
        1,
        "String".to_owned(),
    );
    let url_1 = BaseAttribute::new("A2".to_owned(), "url".to_owned(), 1, "String".to_owned());
    let mut port_1 = BaseAttribute::new("A3".to_owned(), "port".to_owned(), 1, "int".to_owned());
    // port_1.set_required(true);
    port_1.set_value("80".to_string());

    // println!("port_1 ---- {:?}", port_1);

    my_rest_configuration_1.add_attribute(&method_type.get_key(), method_type);
    my_rest_configuration_1.add_attribute(&url_1.get_key(), url_1);
    my_rest_configuration_1.add_attribute(&port_1.get_key(), port_1);

    // let res = my_rest_configuration.validate(&my_rest_configuration_1);

    println!("Original ------ {:#?}", my_rest_configuration);
    // println!("\nNew -----------{:#?}", my_rest_configuration_1);

    

    let my_rest_configuration_2: BaseEntity = read_user_from_file("test1.json").unwrap();
    println!("{:#?}", my_rest_configuration_2);

    let res = my_rest_configuration.validate(&my_rest_configuration_2);
    println!("\nValidation between entity - {}", res);

}
