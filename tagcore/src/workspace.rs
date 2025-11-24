use std::{
    collections::{HashMap, HashSet}, fs::File, path::{Path, PathBuf}
};

use crate::{
    errors::{WorkspaceError, TagFileError}, tag::Tag, tagfile::TagFile
};

#[derive(Debug)]
pub struct Workspace {
    root_folder: PathBuf,
    /// The name of the workspace
    name: String,
    /// Mapping from DIRECTORY PATHS to in-memory TagFiles. Directory paths ARE CANNONICALIZED
    all_tagfiles: HashMap<PathBuf, TagFile>,
    tags_cache: HashSet<String>
}

// Public functions
impl Workspace {
    /// Attempts to open a workspace given a directory (a folder) and a workspace name. If no workspace exists in the directory, creates one instead. Validates the workspace name
    pub fn open_or_create_workspace(directory: PathBuf, name: String) -> Result<Workspace, WorkspaceError>{
        if !Workspace::is_name_valid(&name) {
            return Err(WorkspaceError::InvalidName(name));
        }

        let tag_file_name = directory.join(Workspace::get_workspace_file_name(&name));

        //If file cannot be opened, return error
        if let Err(_e) = Workspace::open_workspace_file(&tag_file_name) {
            return match tag_file_name.file_name() {
                Some(name) => Err(WorkspaceError::FileUnavailable(name.to_str().unwrap_or("[unknown file name]").to_string())),
                None => Err(WorkspaceError::FileUnavailable("[no file name]".to_string())),
            }
        }

        //Create a workspace
        Ok(Workspace { 
            root_folder: directory,
            name: name,
            all_tagfiles: HashMap::new(),
            tags_cache: HashSet::new()
        }) //move into else up above
    }

    /// Scans for .tag files, starting from the workspace's root directory and recursing into folders.
    pub fn scan_for_tagfiles(&mut self) {
        use walkdir::WalkDir;
        let root_folder: String = self.root_folder.to_str().unwrap().to_string();
        for entry in WalkDir::new(root_folder).into_iter().filter_map(|e| e.ok()) { //Ignores un-owned files
            // Check if TagFile exists in directory
            let full_path = entry.path().join(Workspace::get_tagfile_file_name(&self.name));
            if full_path.exists() {
                //REVIEW - If tag path is IN all_tagfiles hashmap, remove from hasmap and re-serialize it?
                let tf: TagFile = match TagFile::from_file_in_dir(full_path.as_path()) {
                    Ok(tf) => tf,
                    Err(_) => continue, //Cannot create TagFile = Skip
                };

                self.tags_cache.extend(tf.get_all_tags_string());

                //add tagfile to workspace's set. This moves the TagFile.
                self.all_tagfiles.insert(full_path.to_path_buf().canonicalize().unwrap_or(full_path.to_path_buf()), tf);
            }
        }
    }

    /// Adds the given string(s) as a tag to a file.
    pub fn add_tag_to_file(&mut self, path_to_file: PathBuf, tag_1: String, tag_2: Option<String>) -> Result<(), TagFileError> {
        // println!("START: adding tag to a file");
        // println!("This is the pathbuf: {:?}", path_to_file);
        let parent_dir: &Path = path_to_file.parent().ok_or(TagFileError::BadPath("Invalid Path, parent dir".to_string()))?;
        // let full_parent_dir = std::path::absolute(parent_dir).map_err(|_| TagFileError::BadPath("Invalid Path, canonical dir".to_string()))?;
        let full_parent_dir = parent_dir.canonicalize().map_err(|_| TagFileError::BadPath("Invalid Path, canonical dir".to_string()))?;
        
        // let file_name = path_to_file.file_name().ok_or(TagAddError::InvalidPath())?;

        let tag: Tag = if tag_2.is_none() {
            Tag::Simple(tag_1.clone()) //clone so that tag_cache can update with original strings
        }
        else {
            Tag::KV(tag_1.clone(), tag_2.clone().unwrap())
        };

        if let Some(tf) = self.all_tagfiles.get_mut(&full_parent_dir) {
            //takes ownership of the Tag enum. Will return any errors because of '?'
            tf.add_tag_to_file_in_self(&path_to_file, tag)?;
        }
        else {
            let mut tf = TagFile::empty(parent_dir.join(Workspace::get_tagfile_file_name(&self.name)))?;
            tf.add_tag_to_file_in_self(&path_to_file, tag)?;
            self.all_tagfiles.insert(full_parent_dir, tf);
        }

        // Check if tag is in memory-cache, if not, add to cache. Since down here, only add to cache if TagFile open/create was successful
        if !self.tags_cache.contains(&tag_1) {
            self.tags_cache.insert(tag_1);
        }
        if tag_2.as_ref().is_some_and(|t| self.tags_cache.contains(t)) {
            self.tags_cache.insert(tag_2.unwrap());
        }

        Ok(())
    }

    /// Removes the given string(s) as a tag from a file. If the file does not have the tag/any tags, does nothing.
    pub fn remove_tag_from_file(&mut self, path_to_file: PathBuf, tag_1: String, tag_2: Option<String>) -> Result<(), TagFileError> {
        let parent_dir: &Path = path_to_file.parent().ok_or(TagFileError::BadPath("Invalid Path".to_string()))?;
        let tag: Tag = if tag_2.is_none() {
            Tag::Simple(tag_1.clone()) //clone so that tag_cache can update with original strings
        }
        else {
            Tag::KV(tag_1.clone(), tag_2.clone().unwrap())
        };

        // If tagfile exists, attempt to remove tag. If no tagfile, silently do nothing.
        if let Some(tf) = self.all_tagfiles.get_mut(parent_dir) {
            tf.remove_tag_from_file_in_self(&path_to_file, &tag)?;
        }

        // No need to "rebuild" tag cache as tag may need to be used elsewhere... just return
        Ok(())
    }

    pub fn get_tags_for_file_name(&self, full_path_to_file: PathBuf) -> Result<Vec<Tag>, WorkspaceError> {
        let parent_dir: &Path = &full_path_to_file.parent().ok_or(WorkspaceError::InvalidName("Invalid Path, parent dir".to_string()))?;
        let full_parent_dir = parent_dir.canonicalize().map_err(|_| WorkspaceError::InvalidName("Invalid Path, canonical dir".to_string()))?;
        let file_name = full_path_to_file.file_name().ok_or(WorkspaceError::InvalidName("Invalid File Name".to_string()))?;
        let file_name = file_name.to_str().ok_or(WorkspaceError::InvalidName("Invalid File Name".to_string()))?.to_string();

        if let Some(t) = self.all_tagfiles.get(&full_parent_dir.join(Workspace::get_tagfile_file_name(&self.name))) {
            // println!("    GET TAGS FOR FILE NAME: SUPER IN");
            Ok(t.get_all_tags_for_filename(&file_name))
        }
        else {
            Ok(Vec::<Tag>::new())
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn query_fuzzy(&self, text: &str, simple: bool, key: bool, value: bool) -> HashMap<String, Vec<Tag>> {
        let mut rv: HashMap<String, Vec<Tag>> = HashMap::new();
        for (_path, tf) in &self.all_tagfiles {
            for (file_name,tags) in tf.get_mapping_ref() {
                let mut vec: Vec<Tag>= Vec::new();
                for tag in tags {
                    match tag {
                        Tag::Simple(s) => {
                            if simple && s.contains(text) {
                                vec.push(tag.clone());
                            }
                        },
                        Tag::KV(k,v) => {
                            if key && k.contains(text) {
                                vec.push(tag.clone());
                            }
                            if value && v.contains(text) {
                                vec.push(tag.clone());
                            }
                        }
                    }
                }
                if !vec.is_empty() {
                    // TODO - full filepath?
                    rv.insert(file_name.clone(), vec);
                }
            }
        }
        rv
    }

    pub fn query_exact(&self, text: &str, simple: bool, key: bool, value: bool) -> HashMap<String, Vec<Tag>> {
        let mut rv: HashMap<String, Vec<Tag>> = HashMap::new();
        for (_path, tf) in &self.all_tagfiles {
            for (file_name,tags) in tf.get_mapping_ref() {
                let mut vec: Vec<Tag>= Vec::new();
                for tag in tags {
                    match tag {
                        Tag::Simple(s) => {
                            if simple && s == text {
                                vec.push(tag.clone());
                            }
                        },
                        Tag::KV(k,v) => {
                            if key && k == text {
                                vec.push(tag.clone());
                            }
                            if value && v == text {
                                vec.push(tag.clone());
                            }
                        }
                    }
                }
                if !vec.is_empty() {
                    // TODO - full filepath?
                    rv.insert(file_name.clone(), vec);
                }
            }
        }
        rv
    }
    
    // }
}

// Private / Helper Functions
impl Workspace {
    // TODO - fix
    fn is_name_valid(name: &String) -> bool {
        let invalid_workspace_name_chars: &str = "/*<>. ";
        !name.chars().any(|c| invalid_workspace_name_chars.contains(c) || c.is_uppercase())
    }
    
    ///Returns the file name of a workspace file, given a name
    fn get_workspace_file_name(name: &String) -> String {
        format!(".tagwksp_{}",name)
    }

    /// Returns the file name of a tag file, given a workspace name
    fn get_tagfile_file_name(workspace_name: &String) -> String {
        format!(".tag_{}", workspace_name)
    }

    /// Opens a .tagwksp file. The file is just a flag file.
    fn open_workspace_file(full_path_to_file: &Path) -> std::io::Result<()> {
        match File::create(full_path_to_file) {
            Ok(_file) => Ok(()), //ignore the file handle itself
            Err(_error) => Err(_error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_name_valid() {
        assert_eq!(Workspace::is_name_valid(&"helloworld".to_string()), true);
        
        assert_eq!(Workspace::is_name_valid(&"hello world".to_string()), false);
        assert_eq!(Workspace::is_name_valid(&"aHib".to_string()), false);
        assert_eq!(Workspace::is_name_valid(&"/".to_string()), false);
        assert_eq!(Workspace::is_name_valid(&"a/b".to_string()), false);
        assert_eq!(Workspace::is_name_valid(&"<".to_string()), false);
    }

    #[test]
    fn get_workspace_file_name() {
        assert_eq!(Workspace::get_workspace_file_name(&"name".to_string()), ".tagwksp_name".to_string());
        assert_eq!(Workspace::get_workspace_file_name(&"a_b_c".to_string()), ".tagwksp_a_b_c".to_string());
        assert_eq!(Workspace::get_workspace_file_name(&"12345".to_string()), ".tagwksp_12345".to_string());
    }

    #[test]
    fn open_workspace_file() {
        // TODO
        use tempdir::TempDir;
        let root_dir = TempDir::new("test").unwrap();
        let result = Workspace::open_workspace_file(root_dir.path().join("foo/\\bar").as_path());
        assert!(result.is_err());
        // assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidFilename);
        
        // todo!()
    }

    #[test]
    fn workspace_create() {
        use tempdir::TempDir;
        let root_dir = TempDir::new("prefix").unwrap();
        let _workspace_1: Result<Workspace, WorkspaceError> = Workspace::open_or_create_workspace(root_dir.path().to_path_buf(), "hello".to_string());

        assert!(_workspace_1.is_ok());
        assert!(root_dir.path().join(".tagwksp_hello").exists());
    }

    #[test]
    fn workspace_open() {
        use tempdir::TempDir;
        let root_dir: TempDir = TempDir::new("prefix_root").unwrap();
        {
            let _ = Workspace::open_or_create_workspace(root_dir.path().to_path_buf(), "active_space".to_string());
        }
        let workspace_1 = Workspace::open_or_create_workspace(root_dir.path().to_path_buf(), "active_space".to_string());

        assert!(workspace_1.is_ok());
    }

}