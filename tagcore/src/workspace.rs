use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf}
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

impl Workspace {
    fn is_name_valid(name: &String) -> bool {
        !name.contains("/")
    }

    fn get_tagfile_name(&self) -> String {
        format!(".tag_{}",self.name)
    }

    pub fn open_or_create_workspace(directory: PathBuf, name: String) -> Result<Workspace, WorkspaceError>{
        //TODO - validate name !
        if !Workspace::is_name_valid(&name) {
            return Err(WorkspaceError::InvalidName(name));
        }

        let tag_file_name = directory.join(format!(".tag_wrkspc_{}",name));

        if tag_file_name.exists() {
            // TODO - open workspace info from disk
        }
        else {
            // TODO - create workspace info and serialize
        }

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

        // FIXME - not implemented yet
        let tag_file: TagFile = if parent_dir.exists() && parent_dir.join(self.get_tagfile_name()).exists() {
            // get_tag_file(path_to_file)
        } else {
            // create_tag_file(path_to_file)
        };

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
