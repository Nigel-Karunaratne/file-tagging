use std::{
    collections::{HashMap, HashSet}, fs::File, io::{Read, Write}, path::{Path, PathBuf}
};
use serde::{Serialize, Deserialize};

use crate::{errors::TagFileError, tag::Tag};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagFile {
    #[serde(skip)]
    pub full_path_to_tagfile: PathBuf,

    pub mapping: HashMap<String, Vec<Tag>>
}

impl TagFile {
    /// Loads a TagFile from disk and returns it.
    pub fn from_file_in_dir(path_to_tagfile_file: &Path) -> Result<TagFile, TagFileError> {
        return TagFile::load_tagfile_from_disk(&path_to_tagfile_file);
    }

    /// Creates an empty TagFile and saves it to disk
    pub fn empty(path_to_tagfile_file: PathBuf) -> Result<TagFile, TagFileError> {
        let tf = TagFile {
            full_path_to_tagfile: path_to_tagfile_file,
            mapping: HashMap::new(),
        };

        tf.save_tagfile_to_disk()?;
        return Ok(tf);
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

            if self.mapping[file_name].contains(&tag) {
                // self.mapping.get_mut(file_name).unwrap()
                //     .retain(|vec_tag| vec_tag != tag);
                    // .swap_remove(self.mapping[file_name].iter().position(|x| x == tag).unwrap() );
                
                if let Some(index) = self.mapping[file_name].iter().position(|val: &Tag | val == tag) {
                    self.mapping.get_mut(file_name).unwrap().remove(index);
                }
            }

            if self.mapping[file_name].len() <= 0 {
                self.mapping.remove(file_name);
            }
        }
        else {

        }

        self.save_tagfile_to_disk()
    }

    pub fn get_all_tags_for_filename(&self, file_name: &String) -> Vec<Tag> {
        if let Some(vec_tags) = self.mapping.get(file_name) {
            // vec_tags.iter().for_each(|tag| {
            //     match tag {
            //         Tag::Simple(x) => return_val.insert(x.to_string()),
            //         Tag::KV(k, v) => {
            //             return_val.insert(k.to_string());
            //             return_val.insert(v.to_string())
            //         }
            //     };
            // });
            vec_tags.clone()
        } else {
            Vec::<Tag>::new()
        }
    }

    pub fn get_all_tags_string(&self) -> HashSet<String> {
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

        ret
    }

    pub fn get_mapping_ref(&self) -> &HashMap<String, Vec<Tag>> {
        &self.mapping
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

    fn load_tagfile_from_disk(full_path_to_tagfile_file: &Path) -> Result<TagFile, TagFileError> {
        let mut file: File = match File::open(full_path_to_tagfile_file) {
            Ok(file) => file,
            Err(err) => return Err(TagFileError::Io(err))
        };

        let mut contents: String = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => return Err(TagFileError::Io(err)),
        }

        let mut tf = match toml::from_str::<TagFile>(contents.as_str()) {
            Ok(tf) => tf,
            Err(_) => return Err(TagFileError::Serialize("Cannot Deserailize TagFile".to_string())),
        };

        tf.full_path_to_tagfile = full_path_to_tagfile_file.to_path_buf();
        Ok(tf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_tagfile_from_str() {
        let tf_str: &str = r#"
[mapping]
file1 = ["TODO"]
file2 = [["Character", "Jim"], "TODO", "yoohoo", ["Chara","VonVia"], "bello!"]
        "#;

        let tf: TagFile = toml::from_str::<TagFile>(tf_str).unwrap();

        assert_eq!(tf.mapping.len(), 2);
        assert!(tf.mapping.contains_key("file1"));
        assert_eq!(tf.mapping["file1"].len(), 1);
        assert_eq!(tf.mapping["file1"][0], Tag::Simple("TODO".to_string()));
        
        assert!(tf.mapping.contains_key("file2"));
        assert_eq!(tf.mapping["file2"].len(), 5);
        assert_eq!(tf.mapping["file2"][0], Tag::KV("Character".to_string(), "Jim".to_string()));
        assert_eq!(tf.mapping["file2"][3], Tag::KV("Chara".to_string(), "VonVia".to_string()));
    }

    #[test]
    fn serialize_tagfile_to_str() {
        let tf: TagFile = TagFile {
            full_path_to_tagfile: PathBuf::from("."),
            mapping: HashMap::from(
                [
                    (
                        "file1".to_string(),
                        vec![Tag::Simple("TODO".to_string())]
                    ),
                    (
                        "file2".to_string(),
                        vec![Tag::KV("Due".to_string(), "Today".to_string()), Tag::Simple("Hi".to_string())]
                    )
                ]
            )
        };

        let str = toml::to_string(&tf).unwrap();
        // NOTE - Due to HashMap, order that files are listed is random.
        let possible_string_1 = r#"[mapping]
file1 = ["TODO"]
file2 = [["Due", "Today"], "Hi"]
"#;

        let possible_string_2: &str = r#"[mapping]
file2 = [["Due", "Today"], "Hi"]
file1 = ["TODO"]
"#;

        assert!(str == possible_string_1 || str == possible_string_2);
    }

    #[test]
    fn from_file_in_dir() {
        use tempdir::TempDir;
        let test_dir = TempDir::new("test").unwrap();

        std::fs::write(test_dir.path().join(".tag_test"), r#"[mapping]
file1 = ["TODO"]
file2 = [["Due", "Today"], "Hi"]
"#).unwrap();

        let tf: TagFile = TagFile::from_file_in_dir(&test_dir.path().join(".tag_test")).unwrap();

        assert_eq!(tf.mapping.len(), 2);
        assert!(tf.mapping.contains_key("file1"));
        assert_eq!(tf.mapping["file1"].len(), 1);
        assert_eq!(tf.mapping["file1"][0], Tag::Simple("TODO".to_string()));
        
        assert!(tf.mapping.contains_key("file2"));
        assert_eq!(tf.mapping["file2"].len(), 2);
        assert_eq!(tf.mapping["file2"][0], Tag::KV("Due".to_string(), "Today".to_string()));
        assert_eq!(tf.mapping["file2"][1], Tag::Simple("Hi".to_string()));

        assert_eq!(tf.full_path_to_tagfile, test_dir.path().join(".tag_test").to_path_buf());
    }

    #[test]
    fn save_tagfile_to_disk() {
        use tempdir::TempDir;
        let test_dir = TempDir::new("test").unwrap();
        let path_to_tf = test_dir.into_path().join(".tag_test").to_path_buf();

        let tf: TagFile = TagFile {
            full_path_to_tagfile: path_to_tf.clone(),
            mapping: HashMap::from(
                [
                    (
                        "file1".to_string(),
                        vec![Tag::Simple("TODO".to_string())]
                    ),
                    (
                        "file2".to_string(),
                        vec![Tag::KV("Due".to_string(), "Today".to_string()), Tag::Simple("Hi".to_string())]
                    )
                ]
            )
        };

        tf.save_tagfile_to_disk().unwrap();
        assert!(path_to_tf.exists());
        let msg = std::fs::read_to_string(path_to_tf).unwrap();

        // NOTE - HashMap means multiple strs are possible
        let possible_string_1 = r#"[mapping]
file1 = ["TODO"]
file2 = [["Due", "Today"], "Hi"]
"#;

        let possible_string_2 = r#"[mapping]
file2 = [["Due", "Today"], "Hi"]
file1 = ["TODO"]
"#;
        assert!(msg == possible_string_1 || msg == possible_string_2);
    }

    #[test]
    fn load_tagfile_from_disk() {
        use tempdir::TempDir;
        let test_dir = TempDir::new("test").unwrap();

        std::fs::write(test_dir.path().join(".tag_test"), r#"[mapping]
file1 = ["TODO"]
file2 = [["Due", "Today"], "Hi"]
"#).unwrap();

        let tf: TagFile = TagFile::load_tagfile_from_disk(&test_dir.path().join(".tag_test")).unwrap();

        assert_eq!(tf.mapping.len(), 2);
        assert!(tf.mapping.contains_key("file1"));
        assert_eq!(tf.mapping["file1"].len(), 1);
        assert_eq!(tf.mapping["file1"][0], Tag::Simple("TODO".to_string()));
        
        assert!(tf.mapping.contains_key("file2"));
        assert_eq!(tf.mapping["file2"].len(), 2);
        assert_eq!(tf.mapping["file2"][0], Tag::KV("Due".to_string(), "Today".to_string()));
        assert_eq!(tf.mapping["file2"][1], Tag::Simple("Hi".to_string()));

        assert_eq!(tf.full_path_to_tagfile, test_dir.path().join(".tag_test").to_path_buf());
    }

    #[test]
    fn load_tagfile_from_disk_empty() {
        use tempdir::TempDir;
        let test_dir = TempDir::new("test").unwrap();

        std::fs::write(test_dir.path().join(".tag_test"), "[mapping]\n").unwrap();

        let tf: TagFile = TagFile::load_tagfile_from_disk(&test_dir.path().join(".tag_test")).unwrap();

        assert_eq!(tf.mapping.len(), 0);
    }

    #[test]
    fn tagfile_add_tag_to_file_in_self() {
        use tempdir::TempDir;
        let test_dir = TempDir::new("test").unwrap();
        let path_to_file: PathBuf = test_dir.path().join("file1.txt");

        std::fs::write(test_dir.path().join(".tag_test"), "[mapping]\n").unwrap();

        let path_to_tagfile = &test_dir.path().join(".tag_test");
        let mut tf: TagFile = TagFile::load_tagfile_from_disk(&path_to_tagfile).unwrap();

        tf.add_tag_to_file_in_self(&path_to_file, Tag::Simple("ADDED TAG".to_string())).unwrap();

        assert!(tf.mapping.contains_key("file1.txt"));
        assert_eq!(tf.mapping["file1.txt"].len(), 1);
        assert_eq!(tf.mapping["file1.txt"][0], Tag::Simple("ADDED TAG".to_string()));

        tf.add_tag_to_file_in_self(&path_to_file, Tag::Simple("ADDED TAG".to_string())).unwrap();

        assert!(tf.mapping.contains_key("file1.txt"));
        assert_eq!(tf.mapping["file1.txt"].len(), 1);
        assert_eq!(tf.mapping["file1.txt"][0], Tag::Simple("ADDED TAG".to_string()));

        tf.add_tag_to_file_in_self(&path_to_file, Tag::Simple("SECOND TAG".to_string())).unwrap();

        assert!(tf.mapping.contains_key("file1.txt"));
        assert_eq!(tf.mapping["file1.txt"].len(), 2);
        assert_eq!(tf.mapping["file1.txt"][1], Tag::Simple("SECOND TAG".to_string()));

        tf.add_tag_to_file_in_self(&path_to_file, Tag::KV("Key".to_string(),"value".to_string())).unwrap();

        assert!(tf.mapping.contains_key("file1.txt"));
        assert_eq!(tf.mapping["file1.txt"].len(), 3);
        assert_eq!(tf.mapping["file1.txt"][2], Tag::KV("Key".to_string(),"value".to_string()));

        let path_to_file: PathBuf = test_dir.path().join("secondfile.c");
        tf.add_tag_to_file_in_self(&path_to_file, Tag::KV("DUE".to_string(),"tomorrow".to_string())).unwrap();

        assert!(tf.mapping.contains_key("file1.txt"));
        assert!(tf.mapping.contains_key("secondfile.c"));
        assert_eq!(tf.mapping["secondfile.c"].len(), 1);
        assert_eq!(tf.mapping["secondfile.c"][0], Tag::KV("DUE".to_string(),"tomorrow".to_string()));

        let contents = std::fs::read_to_string(&path_to_tagfile).unwrap();

        // NOTE - Due to HashMap, order that files are listed is random.
        let possible_string_1 = r#"[mapping]
"secondfile.c" = [["DUE", "tomorrow"]]
"file1.txt" = ["ADDED TAG", "SECOND TAG", ["Key", "value"]]
"#;

        let possible_string_2: &str = r#"[mapping]
"file1.txt" = ["ADDED TAG", "SECOND TAG", ["Key", "value"]]
"secondfile.c" = [["DUE", "tomorrow"]]
"#;
        assert!(contents == possible_string_1 || contents == possible_string_2);
    }

    #[test]
    fn tagfile_remove_tag_from_file_in_self() {
        use tempdir::TempDir;
        let test_dir = TempDir::new("test").unwrap();

        std::fs::write(test_dir.path().join(".tag_test"), r#"[mapping]
"file1.txt" = ["TODO", "Blue"]
"file2.c" = [["Due", "Today"], "Hi", ["Color", "Red"]]
"#).unwrap();

        let path_to_tagfile = test_dir.path().join(".tag_test");
        let mut tf: TagFile = TagFile::load_tagfile_from_disk(&path_to_tagfile).unwrap();

        let path_to_file1: PathBuf = test_dir.path().join("file1.txt");
        let path_to_file2: PathBuf = test_dir.path().join("file2.c");
        
        tf.remove_tag_from_file_in_self(&path_to_file1, &Tag::Simple("TODO".to_string())).unwrap();

        assert!(tf.mapping.contains_key("file1.txt"));
        assert_eq!(tf.mapping["file1.txt"].len(), 1);
        assert_eq!(tf.mapping["file1.txt"][0], Tag::Simple("Blue".to_string()));
        assert!(tf.mapping.contains_key("file2.c"));
        assert_eq!(tf.mapping["file2.c"].len(), 3);

        tf.remove_tag_from_file_in_self(&path_to_file2, &Tag::KV("Due".to_string(),"Today".to_string())).unwrap();

        assert!(tf.mapping.contains_key("file1.txt"));
        assert_eq!(tf.mapping["file1.txt"].len(), 1);
        assert!(tf.mapping.contains_key("file2.c"));
        assert_eq!(tf.mapping["file2.c"].len(), 2);
        assert_eq!(tf.mapping["file2.c"][0], Tag::Simple("Hi".to_string()));
        assert_eq!(tf.mapping["file2.c"][1], Tag::KV("Color".to_string(),"Red".to_string()));

        tf.remove_tag_from_file_in_self(&path_to_file1, &Tag::Simple("Blue".to_string())).unwrap();

        assert!(!tf.mapping.contains_key("file1.txt"));
        assert!(tf.mapping.contains_key("file2.c"));

        let contents = std::fs::read_to_string(&path_to_tagfile).unwrap();
        assert_eq!(contents, r#"[mapping]
"file2.c" = ["Hi", ["Color", "Red"]]
"#);
    }
}