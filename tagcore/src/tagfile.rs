use std::{
    collections::{HashMap, HashSet}, fs::File, io::Write, path::{Path, PathBuf}
};
use serde::{Serialize, Deserialize};

use crate::{errors::TagFileError, tag::Tag};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagFile {
    #[serde(skip)]
    full_path_to_tagfile: PathBuf,

    mapping: HashMap<String, Vec<Tag>>
}

impl TagFile {
    pub fn from_file_in_dir(_path: &Path) -> TagFile {
        todo!()
    }

    // TODO - assumes path to file is valid
    pub fn add_tag_to_file_in_self(&mut self, path_to_file: &Path, tag: Tag) -> Result<(), TagFileError> {
        // Add tag in memory
        let Some(file_name) = path_to_file.file_name().unwrap().to_str() else {
            return Err(TagFileError::BadPath("Invalid File Name".to_string()));
        };

        // If file is mapped, only add tag to vec if tag not already in vec. If file unmapped, create a mapping w/ tag
        if self.mapping.contains_key(file_name) {
            if !self.mapping[file_name].contains(&tag) {
                self.mapping.get_mut(file_name).unwrap().push(tag);
            }
        } else {
            self.mapping.insert(file_name.to_string(), vec![tag]);
        }

        // Write to disk
        self.save_tagfile_to_disk()
    }

    pub fn remove_tag_from_file_in_self(&mut self, path_to_file: &Path, tag: &Tag) -> Result<(), TagFileError> {
        // Remove tag in memory
        let Some(file_name) = path_to_file.file_name().unwrap().to_str() else {
            return Err(TagFileError::BadPath("Invalid File Name".to_string()));
        };

        // IF mapping exists, only remove tag from vec if tag not already in vec. If mapping DNE do nothing?
        if self.mapping.contains_key(file_name) {
            
            if !self.mapping[file_name].contains(&tag) {
                self.mapping.get_mut(file_name).unwrap()
                    .retain(|vec_tag| vec_tag != tag);
                    // .swap_remove(self.mapping[file_name].iter().position(|x| x == tag).unwrap() );
            }
        }
        else {

        }

        self.save_tagfile_to_disk()
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

impl TagFile {
    fn save_tagfile_to_disk(&self) -> Result<(), TagFileError> {
        let contents = match toml::to_string(&self) {
            Ok(contents) => contents,
            Err(_) => return Err(TagFileError::Serialize("Cannot Serialize TagFile".to_string()))
        };

        let mut file = match File::create(&self.full_path_to_tagfile) {
            Ok(file) => file,
            Err(err) => return Err(TagFileError::Io(err))
        };

        match file.write_all(contents.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(TagFileError::Io(err)),
        }

        Ok(())
    }
}