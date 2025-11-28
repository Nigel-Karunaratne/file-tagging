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
    /// Mapping from directory paths including file names to in-memory TagFiles. Directory paths ARE CANNONICALIZED
    all_tagfiles: HashMap<PathBuf, TagFile>,
    tags_cache: HashSet<String>
}

// Public functions
impl Workspace {
    /// Attempts to open a workspace given a directory (a folder) and a workspace name. If no workspace exists in the directory, errors. Validates the workspace name
    pub fn open_workspace(directory: PathBuf, name: &String) -> Result<Workspace, WorkspaceError> {
        if !Workspace::is_name_valid(&name) {
            return Err(WorkspaceError::InvalidName(name.clone()));
        }

        let workspace_file_name: PathBuf = directory.join(Workspace::get_workspace_file_name(&name));

        Workspace::open_workspace_file(&workspace_file_name).map_err(|_e| WorkspaceError::FileUnavailable("".to_string()))?;

        //Create a workspace instance
        let Ok(cannon_dir) = directory.canonicalize() else {
            return Err(WorkspaceError::FileUnavailable("Cannot get parent directory".to_string()))
        };
        Ok(Workspace { 
            root_folder: cannon_dir,
            name: name.clone(),
            all_tagfiles: HashMap::new(),
            tags_cache: HashSet::new()
        })
    }

    /// Attempts to create a workspace given a directory (a folder) and a workspace name. If a same-named workspace exists in the directory, errors. Validates the workspace name
    pub fn create_workspace(directory: PathBuf, name: &String) -> Result<Workspace, WorkspaceError>{
        if !Workspace::is_name_valid(&name) {
            return Err(WorkspaceError::InvalidName(name.clone()));
        }

        let workspace_file_name: PathBuf = directory.join(Workspace::get_workspace_file_name(&name));

        Workspace::create_workspace_file(&workspace_file_name).map_err(|_e| WorkspaceError::FileUnavailable("".to_string()))?;

        //Create a workspace instance
        let Ok(cannon_dir) = directory.canonicalize() else {
            return Err(WorkspaceError::FileUnavailable("Cannot get parent directory".to_string()))
        };
        Ok(Workspace { 
            root_folder: cannon_dir,
            name: name.clone(),
            all_tagfiles: HashMap::new(),
            tags_cache: HashSet::new()
        })
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

        if let Some(tf) = self.all_tagfiles.get_mut(&full_parent_dir.join(Workspace::get_tagfile_file_name(&self.name))) {
            //takes ownership of the Tag enum. Will return any errors because of '?'
            tf.add_tag_to_file_in_self(&path_to_file, tag)?;
        }
        else {
            let mut tf = TagFile::empty(parent_dir.join(Workspace::get_tagfile_file_name(&self.name)))?;
            tf.add_tag_to_file_in_self(&path_to_file, tag)?;
            self.all_tagfiles.insert(full_parent_dir.join(Workspace::get_tagfile_file_name(&self.name)), tf);
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

        let parent_dir_cannonical = parent_dir.canonicalize().map_err(|_| TagFileError::BadPath("Invalid Path, canonical dir".to_string()))?;
        // If tagfile exists, attempt to remove tag. If no tagfile, silently do nothing.
        if let Some(tf) = self.all_tagfiles.get_mut(&parent_dir_cannonical.join(Workspace::get_tagfile_file_name(&self.name))) {
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

        let path_to_tagfile = &full_parent_dir.join(Workspace::get_tagfile_file_name(&self.name));
        if let Some(t) = self.all_tagfiles.get(path_to_tagfile) {
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

    pub fn get_path_to_workspace_file(&self) -> PathBuf {
        self.root_folder.join(Workspace::get_workspace_file_name(&self.name))
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

    /// Creates a .tagwksp file. The file is just a flag file. Errors if the file exists.
    fn create_workspace_file(full_path_to_file: &Path) -> std::io::Result<()> {
        match File::create_new(full_path_to_file) {
            Ok(_file) => Ok(()),
            Err(_error) => Err(_error)
        }
    }

    /// Opens a .tagwksp file. The file is just a flag file.
    fn open_workspace_file(full_path_to_file: &Path) -> std::io::Result<()> {
        match File::open(full_path_to_file) {
            Ok(_file) => Ok(()),
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
        use tempdir::TempDir;
        let root_dir = TempDir::new("test").unwrap();
        let result = Workspace::open_workspace_file(root_dir.path().join("foo/\\bar<><>").as_path());
        assert!(result.is_err());

        let _workspace_file_supertest = File::create(root_dir.path().join(".tagwksp_supertest"));
        let result = Workspace::open_workspace_file(&root_dir.path().join(".tagwksp_supertest"));
        assert!(result.is_ok());

        let result = Workspace::open_workspace_file(&root_dir.path().join(".tagwksp_dne"));
        assert!(result.is_err());
    }

    #[test]
    fn create_workspace_file() {
        use tempdir::TempDir;
        let root_dir = TempDir::new("test").unwrap();
        let result = Workspace::create_workspace_file(root_dir.path().join("foo/\\bar<><>").as_path());
        assert!(result.is_err());

        let path = root_dir.path().join(".tagwksp_supertest");
        let result = Workspace::create_workspace_file(&path);
        assert!(result.is_ok());
        
        let result = Workspace::create_workspace_file(&path); //same file name = same workspace name
        assert!(result.is_err());
    }

    #[test]
    fn workspace_create() {
        use tempdir::TempDir;
        let root_dir = TempDir::new("prefix").unwrap();
        let _workspace_1: Result<Workspace, WorkspaceError> = Workspace::create_workspace(root_dir.path().to_path_buf(), &"hello".to_string());

        assert!(_workspace_1.is_ok());
        assert!(root_dir.path().join(".tagwksp_hello").exists());
    }

    #[test]
    fn workspace_open() {
        use tempdir::TempDir;
        let root_dir: TempDir = TempDir::new("prefix_root").unwrap();
        {
            let _ = Workspace::create_workspace(root_dir.path().to_path_buf(), &"active_space".to_string());
        }
        let workspace_1 = Workspace::open_workspace(root_dir.path().to_path_buf(), &"active_space".to_string());

        assert!(workspace_1.is_ok());

        let workspace_dne = Workspace::open_workspace(root_dir.path().to_path_buf(), &"does_not_exist".to_string());
        assert!(workspace_dne.is_err());
    }

    #[test]
    fn workspace_open_add_tags_to_file() {
        use tempdir::TempDir;
        use std::fs::File;

        // let root_dir: TempDir = TempDir::new_in(".", "test").unwrap();
        let root_dir: TempDir = TempDir::new("test").unwrap();
        let root_dir_path = root_dir.path().to_path_buf();
        
        let _file1: File = File::create(root_dir_path.join("./file1.txt")).unwrap();
        assert!(root_dir_path.join("file1.txt").exists());

        let mut workspace = Workspace::create_workspace(root_dir.path().to_path_buf(), &"testspace".to_string()).unwrap();

        let result = workspace.add_tag_to_file(root_dir_path.join("./file1.txt"), "Hello".to_string(), None);
        assert!(result.is_ok());
        assert!(root_dir_path.join(".tag_testspace").exists());
        assert_eq!(workspace.all_tagfiles.len(), 1);
        assert!(workspace.all_tagfiles.get(&root_dir_path.canonicalize().unwrap().join(".tag_testspace")).is_some());

        let result = workspace.add_tag_to_file(root_dir_path.join("./file2.txt"), "EndAll".to_string(), None);
        assert!(result.is_ok());
        assert!(root_dir_path.join(".tag_testspace").exists());
        assert_eq!(workspace.all_tagfiles.len(), 1);
        assert!(workspace.all_tagfiles.get(&root_dir_path.canonicalize().unwrap().join(".tag_testspace")).is_some());
        assert_eq!(workspace.all_tagfiles.get(&root_dir_path.canonicalize().unwrap().join(".tag_testspace")).unwrap().mapping.len(), 2);

        std::fs::create_dir(root_dir_path.join("subfolder/") ).unwrap();
        let _file2: File = File::create(root_dir_path.join("subfolder/nested.txt")).unwrap(); 

        let result = workspace.add_tag_to_file(root_dir_path.join("subfolder/nested.txt"), "TODO".to_string(), None);
        assert!(result.is_ok());
        assert!(root_dir_path.join("subfolder/.tag_testspace").exists());
        assert_eq!(workspace.all_tagfiles.len(), 2);

        assert!(workspace.all_tagfiles.get(&root_dir_path.join("subfolder").canonicalize().unwrap().join(".tag_testspace")).is_some());
    }

    #[test]
    fn workspace_get_tags_for_filename() {
        use tempdir::TempDir;
        use std::fs::File;

        let root_dir: TempDir = TempDir::new("test").unwrap();
        let root_dir_path = root_dir.path().to_path_buf();
        
        // Create files
        let _file1: File = File::create(root_dir_path.join("./file1.txt")).unwrap();
        let _file2: File = File::create(root_dir_path.join("./file2.txt")).unwrap();
        std::fs::create_dir(root_dir_path.join("subfolder/") ).unwrap();
        let _file3: File = File::create(root_dir_path.join("subfolder/nested.txt")).unwrap();

        // Add tags to files
        let mut workspace = Workspace::create_workspace(root_dir.path().to_path_buf(), &"testspace".to_string()).unwrap();
        let _ = workspace.add_tag_to_file(root_dir_path.join("./file1.txt"), "Hello".to_string(), None);
        let _ = workspace.add_tag_to_file(root_dir_path.join("./file1.txt"), "Hello".to_string(), Some("World".to_string()));
        let _ = workspace.add_tag_to_file(root_dir_path.join("./file2.txt"), "EndAll".to_string(), None);
        let _ = workspace.add_tag_to_file(root_dir_path.join("subfolder/nested.txt"), "TODO".to_string(), None);

        // Get tags for file names
        let result = workspace.get_tags_for_file_name(root_dir_path.join("./file1.txt"));
        // println!("{:?}",result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Tag::Simple("Hello".to_string()));
        assert_eq!(result[1], Tag::KV("Hello".to_string(), "World".to_string()));

        let result = workspace.get_tags_for_file_name(root_dir_path.join("./file2.txt"));
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Tag::Simple("EndAll".to_string()));
        
        let result = workspace.get_tags_for_file_name(root_dir_path.join("./fileDNE.txt"));
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_empty());

        let result = workspace.get_tags_for_file_name(root_dir_path.join("./subfolder/nested.txt"));
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Tag::Simple("TODO".to_string()));
    }

    #[test]
    fn workspace_open_remove_tags_from_file() {
        use tempdir::TempDir;
        use std::fs::File;

        let root_dir: TempDir = TempDir::new("test").unwrap();
        let root_dir_path = root_dir.path().to_path_buf();
        
        // Create files
        let _file1: File = File::create(root_dir_path.join("./file1.txt")).unwrap();
        let _file2: File = File::create(root_dir_path.join("./file2.txt")).unwrap();
        let _file3: File = File::create(root_dir_path.join("./file3.txt")).unwrap();
        std::fs::create_dir(root_dir_path.join("subfolder/") ).unwrap();
        let _file_nested: File = File::create(root_dir_path.join("subfolder/nested.txt")).unwrap();

        // Add tags to files
        let mut workspace = Workspace::create_workspace(root_dir.path().to_path_buf(), &"testspace".to_string()).unwrap();

        let _ = workspace.add_tag_to_file(root_dir_path.join("./file1.txt"), "Hello".to_string(), None);
        let _ = workspace.add_tag_to_file(root_dir_path.join("./file1.txt"), "Hello".to_string(), Some("World".to_string()));
        let _ = workspace.add_tag_to_file(root_dir_path.join("./file2.txt"), "Hello".to_string(), None);
        let _ = workspace.add_tag_to_file(root_dir_path.join("./file2.txt"), "Hello".to_string(), Some("World".to_string()));
        let _ = workspace.add_tag_to_file(root_dir_path.join("./file3.txt"), "A".to_string(), None);
        let _ = workspace.add_tag_to_file(root_dir_path.join("./file4.txt"), "A".to_string(), Some("B".to_string()));
        let _ = workspace.add_tag_to_file(root_dir_path.join("subfolder/nested.txt"), "TODO".to_string(), None);


        // Remove tags from files (REAL TEST)
        let result = workspace.remove_tag_from_file(root_dir_path.join("./file1.txt"), "Hello".to_string(), None);
        assert!(result.is_ok());
        let result = workspace.get_tags_for_file_name(root_dir_path.join("./file1.txt")).unwrap();
        println!("RESULT: {:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Tag::KV("Hello".to_string(), "World".to_string()));

        let result = workspace.remove_tag_from_file(root_dir_path.join("./file2.txt"), "Hello".to_string(), Some("World".to_string()));
        assert!(result.is_ok());
        let result = workspace.get_tags_for_file_name(root_dir_path.join("./file2.txt")).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Tag::Simple("Hello".to_string()));

        let result = workspace.remove_tag_from_file(root_dir_path.join("./file3.txt"), "A".to_string(), Some("B".to_string()));
        assert!(result.is_ok());
        let result = workspace.get_tags_for_file_name(root_dir_path.join("./file3.txt")).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Tag::Simple("A".to_string()));

        let result = workspace.remove_tag_from_file(root_dir_path.join("./file3.txt"), "A".to_string(), None);
        assert!(result.is_ok());
        let result = workspace.get_tags_for_file_name(root_dir_path.join("./file3.txt")).unwrap();
        assert!(result.is_empty());

        let result = workspace.remove_tag_from_file(root_dir_path.join("./file4.txt"), "A".to_string(), None);
        assert!(result.is_ok());
        let result = workspace.get_tags_for_file_name(root_dir_path.join("./file4.txt")).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Tag::KV("A".to_string(), "B".to_string()));

        let result = workspace.remove_tag_from_file(root_dir_path.join("./file4.txt"), "A".to_string(), Some("B".to_string()));
        assert!(result.is_ok());
        let result = workspace.get_tags_for_file_name(root_dir_path.join("./file4.txt")).unwrap();
        assert!(result.is_empty());

        let result = workspace.remove_tag_from_file(root_dir_path.join("./subfolder/nested.txt"), "TODO".to_string(), None);
        assert!(result.is_ok());
        let result = workspace.get_tags_for_file_name(root_dir_path.join("./subfolder/nested.txt")).unwrap();
        assert!(result.is_empty());
    }

}