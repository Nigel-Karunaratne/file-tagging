use std::{
    collections::{HashMap, HashSet}, fs::File, path::{Path, PathBuf}
};

use crate::{errors::WorkspaceError, tagfile::TagFile};
use crate::errors::TagAddError;

#[derive(Debug)]
pub struct Workspace {
    root_folder: PathBuf,
    name: String,
    all_tagfiles: HashMap<PathBuf, TagFile>,
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

    pub fn add_tag_to_file(&mut self, path_to_file: PathBuf, tag: String) -> Result<(), TagAddError> {
        let parent_dir: &Path = path_to_file.parent().ok_or(TagAddError::InvalidPath())?;
        // let file_name = path_to_file.file_name().ok_or(TagAddError::InvalidPath())?;

        if let Some(tf) = self.all_tagfiles.get_mut(parent_dir) {
            tf.add_tag_to_file_in_self(&path_to_file, &tag);
        }
        else {
            // TODO - Create file
        }

        // Check if tag is in memory-cache, if not, add to cache. only add to cache if TagFile open/create was successful
        if !self.tags_cache.contains(&tag) {
            self.tags_cache.insert(tag);
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
    
    fn get_workspace_file_name(name: &String) -> String {
        format!(".tagwksp_{}",name)
    }

    //Opens a .tagwksp file. The file is just a flag file.
    fn open_workspace_file(full_path_to_file: &Path) -> std::io::Result<()> {
        match File::create(full_path_to_file) {
            Ok(_file) => Ok(()), //ignore the file handle itself
            Err(_error) => Err(_error)
        }
    }
}
