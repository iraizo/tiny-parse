use std::collections::HashMap;
use std::fs::read_to_string;
use anyhow::Result;
use anyhow::anyhow;
use serde::Serialize;
use serde::Deserialize;

#[cfg(test)]
#[path = "../test/parse.rs"]
mod tests;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JavaClass {
    pub name: String,
    pub intermediate: String,
    pub methods: Vec<JavaMethod>,
    pub fields: Vec<JavaField>,
    //methods: Vec<JavaMethods>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JavaProperty {
    pub name: String,
    pub key: i8, /* im not entirely sure what this is supposed to be, but this is the position in which the constructor is getting called */
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JavaField {
    pub type_descriptor: String,
    pub name: String,
    pub intermediate: String,
}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JavaMethod {
    pub intermediate: String,
    pub name: String,
    pub intermediate_descriptor: String,
    pub properties: Vec<JavaProperty>
}



#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub major_version: i8,
    pub minor_version: i8,
    pub namespace_a: String,
    pub namespace_b: String
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TinyMap {
    pub header: Header,
    pub classes: Vec<JavaClass>,

    /* contains all descriptors where K is intermediate and V is named, this will be used to resolve them at the bindgen stage later on */
    pub descriptors: HashMap<String, String>
}

impl TinyMap {
    pub fn new(path: &str) -> Result<Self> {
        let mut map = TinyMap {
            classes: Vec::new(),
            header: Header {
                major_version: 0,
                minor_version: 0,
                namespace_a: String::new(),
                namespace_b: String::new(),
            },
            descriptors: HashMap::new(),
        };

        // buffer for the current class
        let mut class = JavaClass {
            name: String::new(),
            intermediate: String::new(),
            fields: Vec::new(),
            methods: Vec::new(),
        };

        let mut method = JavaMethod {
            name: String::new(),
            intermediate: String::new(),
            intermediate_descriptor: String::new(),
            properties: Vec::new(),
        };
        

        for line in read_to_string(path)?.lines() {
            let lexemens: Vec<&str> = line.split_ascii_whitespace().collect();
            match lexemens[0] {
                "tiny" => {
                    map.header = Header {
                        major_version: lexemens[1].parse::<i8>()?,
                        minor_version: lexemens[2].parse::<i8>()?,
                        namespace_a: lexemens[3].to_string(),
                        namespace_b: lexemens[4].to_string(),
                    }
                }

                "c" => {
                    // skip comments, this is out of scope for this project
                    if lexemens.len() != 3 {
                      //  log::info!("Skipped comment: {:?}", line);
                        continue;
                    }


                    /* push last added class onto the map */
                    if !class.name.is_empty() {
                        map.classes.push(class);

                        class = JavaClass {
                            name: String::new(),
                            intermediate: String::new(),
                            fields: Vec::new(),
                            methods: Vec::new(),
                        };
                    }

                  //  log::info!("{:?}", line);
                    class = JavaClass {
                        name: lexemens[2].to_string(),
                        intermediate: lexemens[1].to_string(),
                        fields: Vec::new(),
                        methods: Vec::new(),
                    };

                    map.descriptors.insert(class.intermediate.clone(), class.name.clone());
                }

                "m" => {
                    if !method.name.is_empty() {
                        class.methods.push(method);
                        method = JavaMethod {
                            name: String::new(),
                            intermediate: String::new(),
                            intermediate_descriptor: String::new(),
                            properties: Vec::new(),
                        };
                    }

                    method = JavaMethod {
                        name: lexemens[3].to_string(),
                        intermediate: lexemens[2].to_string(),
                        intermediate_descriptor: lexemens[1].to_string(),
                        properties: Vec::new(),
                    };
                }

                "f" => {
                    class.fields.push(JavaField {
                        type_descriptor: lexemens[1].to_string(),
                        name: lexemens[3].to_string(),
                        intermediate: lexemens[2].to_string(),
                    });
                }

                "p" => {
                    method.properties.push(JavaProperty {
                        name: lexemens[2].to_string(),
                        key: lexemens[1].parse::<i8>()?,
                    });
                }

                _ => {
                    log::info!("Dumped map: {:#?}", map);
                    return Err(anyhow!("found unknown lexeme in tiny map: {}", lexemens[0]))
                }
            }
        }
        //Err(anyhow!("found unknown lexeme in tiny map"))

       // log::info!("Done, dumping map: {:#?}", map);
        Ok(map)
    }

    pub fn intermediate_to_named(&self, named: &str) -> Option<&String> {
        let desc = self.descriptors.get(named);
        desc
    }
}
