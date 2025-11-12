use std::{
    collections::{HashMap, HashSet}, fs::File, path::{Path, PathBuf}
};

use crate::{errors::WorkspaceError, tagfile::TagFile};
use crate::errors::TagAddError;

#[derive(Debug)]
pub struct Workspace {
    pub root_folder: PathBuf,
    pub name: String,
    pub all_tagfiles: HashMap<PathBuf, TagFile>,
    pub tags_cache: HashSet<String>
}

// Public functions
impl Workspace {
    pub fn open_or_create_workspace(directory: PathBuf, name: String) -> Result<Workspace, WorkspaceError>{
        if !Workspace::is_name_valid(&name) {
            return Err(WorkspaceError::InvalidName(name));
        }

        let tag_file_name = directory.join(format!(".tag_wrkspc_{}",name));

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

    pub fn add_tag_to_file(&mut self, path_to_file: PathBuf, tag: String) -> Result<(), TagAddError> {
        // TODO - Check if directory has a .tag file, if not create the file
        let parent_dir: &Path = path_to_file.parent().ok_or(TagAddError::InvalidPath())?;

        //if let Some((exist_path, exist_tagfile)) = self.all_tagfiles.iter().find(|(exist_path,exist_tagfile)| *exist_path == parent_dir) {

        if self.all_tagfiles.contains_key(parent_dir) {
          // Open the File

        } else {
            // Create the file
        }

        // FIXME - not implemented yet
        // let tag_file: TagFile = if parent_dir.exists() && parent_dir.join(self.get_tagfile_name()).exists() {
        //     // get_tag_file(path_to_file)
        // } else {
        //     // create_tag_file(path_to_file)
        // };

        // Check if tag is in memory-cache, if not, add to cache
        if !self.tags_cache.contains(&tag) {
            self.tags_cache.insert(tag);
        }
        // TODO - Add tag to the TagFile.

        // tag_file.add_tag(&tag);
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
        !name.chars().any(|c| invalid_workspace_name_chars.contains(c))
    }

    
    fn get_tagfile_name(&self) -> String {
        format!(".tag_{}",self.name)
    }

    //Opens a .tag_wks file. The file is just a flag file.
    fn open_workspace_file(full_path_to_file: &Path) -> std::io::Result<()> {
        match File::create(full_path_to_file) {
            Ok(_file) => Ok(()), //ignore the file handle itself
            Err(_error) => Err(_error)
        }
    }
}
