use std::{
    collections::{HashMap, HashSet}, fs::File, path::{Path, PathBuf}
};

use crate::{
    errors::{WorkspaceError, TagFileError}, tag::Tag, tagfile::TagFile
};

#[derive(Debug)]
pub struct Workspace {
    root_folder: PathBuf,
    name: String,
    all_tagfiles: HashMap<PathBuf, TagFile>, //Mapping from DIRECTORY PATHS to in-memory TagFiles
    tags_cache: HashSet<String>
}

// Public functions
impl Workspace {
    /// Attempts to open a workspace given a directory (a folder) and a workspace name. If no workspace exists in the directory, creates one instead
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
            let full_path = entry.path().join(format!(".tag_{}", self.name));
            if full_path.exists() {
                //REVIEW - If tag path is IN all_tagfiles hashmap, remove from hasmap and re-serialize it?
                
                let tf: TagFile = TagFile::from_file_in_dir(entry.path());
                self.tags_cache.extend(tf.get_all_tags());

                //add tagfile to workspace's set. This moves the TagFile.
                self.all_tagfiles.insert(full_path.to_path_buf(), tf);
            }
        }
    }

    pub fn add_tag_to_file(&mut self, path_to_file: PathBuf, tag_1: String, tag_2: Option<String>) -> Result<(), TagFileError> {
        let parent_dir: &Path = path_to_file.parent().ok_or(TagFileError::BadPath("Invalid Path".to_string()))?;
        // let file_name = path_to_file.file_name().ok_or(TagAddError::InvalidPath())?;

        let tag: Tag = if tag_2.is_none() {
            Tag::Simple(tag_1.clone()) //clone so that tag_cache can update with original strings
        }
        else {
            Tag::KV(tag_1.clone(), tag_2.clone().unwrap())
        };

        if let Some(tf) = self.all_tagfiles.get_mut(parent_dir) {
            //takes ownership of the Tag enum. Will return any errors because of '?'
            tf.add_tag_to_file_in_self(&path_to_file, tag)?;
        }
        else {
            // TODO - Create file
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

    // Creates a workspace using the current directory.
    // pub fn discover_nearest_workspace_above(path: &Path) -> Option<Self> {
    //     let mut current_path = Some(path);
    //     let mut levels_left = 5;

    //     while let Some(p) = current_path {
    //         levels_left -= 1;
    //         if levels_left == 0 {
    //             break;
    //         }

    //         let tag_file = p.join(".tags");
    //         if tag_file.exists() {
    //             return Some(Self {
    //                 root: p.to_path_buf(),
    //                 active_tag_file: None
    //             });
    //         }
    //         current_path = p.parent();
    //     }

    //     //if nothing found, return None. TODO - max limit?
    //     None
    
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
        
        todo!()
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