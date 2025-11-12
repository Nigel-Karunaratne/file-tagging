use std::{collections::HashMap, collections::HashSet, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};

use crate::tag::Tag;

//HAS LOTS OF WARNINGS!!!!, NOT IMPLEMENTED!!!!!

#[derive(Debug, Serialize, Deserialize)]
pub struct TagFile {
    workspace_name: String, //Workspace lives as long as the TagFile in-memory
    #[serde(skip)]
    full_file_path: PathBuf,

    mapping: HashMap<String, Vec<Tag>>
}

impl TagFile {
    pub fn from_file_in_dir(path: &Path) -> TagFile {
        todo!()
    }

    pub fn add_tag_to_file_in_self(&mut self, file_name: &Path, tag: &String) {
        todo!()
    }

    pub fn remove_tag_from_file_in_self(&mut self, file_name: &Path, tag: &String) {
        todo!()
    }

    pub fn get_all_tags(&self) -> HashSet<String> {
        let mut ret: HashSet<String> = HashSet::<String>::new();
        self.mapping.values().for_each(|vec | vec.iter().for_each(|tag| {
            match tag {
                Tag::Simple(x) => ret.insert(x.to_string()),
                Tag::KV(k, v) => {
                    ret.insert(k.to_string());
                    ret.insert(v.to_string())
                }
            };
        }));

        ret.clone()
    }
}
