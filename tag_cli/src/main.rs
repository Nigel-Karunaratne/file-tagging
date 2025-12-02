use clap::{ArgAction, Parser, Subcommand};
use directories::ProjectDirs;
use std::{collections::HashMap, fs::File, path::PathBuf};

use tagcore::Workspace;

#[derive(Parser)]
#[command(name = "tag-cli")]
#[command(about = "A CLI app for interfacing with file tags", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Opens a workspace from the current directory")]
    Open {
        #[arg(help = "The name of the workspace to open")]
        name: String
    },

    #[command(about = "Creates a workspace in the current directory, if it doesn't exist")]
    Create {
        #[arg(help = "The name of the workspace to create")]
        name: String
    },

    #[command(about = "Adds listed tags to a file")]
    Add {
        #[arg(required = true, help = "The file to add tags to")]
        file_name: String,

        #[arg(required = false, num_args = 1, help = "Tags to add (defaults to Simple tags)")]
        simple: Vec<String>,

        #[arg(short, long, required = false, num_args = 2, value_names = &["KEY", "VALUE"], help = "Specified next two entries are a Key-Value pair")]
        kv: Vec<String>
    },

    #[command(about = "Removes specified tags from files")]
    Remove {
        #[arg(required = true, help = "The file to remove tags from")]
        file_name: String,

        #[arg(short, action = ArgAction::SetTrue, required = false, help = "If specified, removes all tags from the file")]
        all_remove: bool,

        #[arg(required = false, num_args = 1, help = "Tags to remove (defaults to Simple tags)")]
        simple: Vec<String>,

        #[arg(short, long, required = false, num_args = 2, value_names = &["KEY", "VALUE"], help = "Specified next two entries are a Key-Value pair")]
        kv: Vec<String>
    },

    #[command(about = "Show all tags on a file, as a comma-separated list")]
    Show {
        #[arg(required = true, help = "The file to show all tags for")]
        file_name: String
    },

    #[command(about = "Search by tags. If no flags are given, searches all flag types")]
    Search(SearchArgs),

    #[command(about = "Outputs the name of the current-open workspace")]
    Name {

    }
}

#[derive(Parser)]
struct SearchArgs {
    #[arg(short, help="Search mode is exact (default)")]
    exact: bool,

    #[arg(short, help="Search mode is fuzzy")]
    fuzzy: bool,

    #[arg(short, help="Include Simple tags")]
    simple_on: bool,

    #[arg(short, help="Include Keys from Key-Value tags")]
    key_on: bool,

    #[arg(short, help="Include Values from Key-Value tags")]
    values_on: bool,

    #[arg(required = true)]
    tags: Vec<String>
}

impl SearchArgs {
    fn normalize(mut self) -> Self {
        if !(self.simple_on || self.key_on || self.values_on) {
            self.simple_on = true;
            self.key_on = true;
            self.key_on = true;
        }
        self
    }
}

fn main() {
    let mut workspace = load_workspace_from_storage();
    if let Some(ref mut w) = workspace {
        w.scan_for_tagfiles();
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Open { name } => set_open_workspace_file(&name),
        Commands::Create { name  } => create_set_workspace_file(&name),
        Commands::Add { file_name, simple, kv } => add_tags_to_file(&mut workspace, &file_name, &simple, &kv),
        Commands::Remove { file_name, all_remove, simple, kv } => remove_tags_from_file(&mut workspace, all_remove, &file_name, &simple, &kv),
        Commands::Show { file_name  } => show_tags_for_file(&workspace, &file_name),
        Commands::Search(search_args) => {
            let search_args: SearchArgs = search_args.normalize();
            search(&workspace, &search_args);
        },
        Commands::Name {  } => show_workspace_name(&workspace),
    };
}

fn load_workspace_from_storage() -> Option<Workspace> {
    //Open storage file in config dir
    let Some(dir) = ProjectDirs::from("com", "filetags", "cli") else {
        return None;
    };
    let path_to_storage_file = dir.config_local_dir().join("WorkspaceFilePath.txt");
    if !path_to_storage_file.exists() {
        // Create empty file
        let Some(parent_dir) = path_to_storage_file.parent() else {
            println!("ERROR: cannot create config file as no parent dir was specified");
            return None;
        };
        let Ok(_) = std::fs::create_dir_all(parent_dir) else {
            println!("ERROR: cannot create config file as cannot create parent dir ");
            return None;
        };
        match File::create(&path_to_storage_file) {
            Ok(_) => (),
            Err(e) => {
                println!("ERROR: cannot create config file at location '{:?}'", path_to_storage_file);
                println!("err is {:?}", e);
            },
        };
        return None;
    }
    let Ok(path) = std::fs::read_to_string(path_to_storage_file) else {
        return None;
    };
    
    //Using the read path to workspace file, open the workspace file
    let dir: PathBuf = PathBuf::from(path);
    let Some(file_name) = dir.file_name() else {
        return None;
    };
    let Some(dir) = dir.parent() else {
        return None;
    };
    let Some(file_name) = file_name.to_str() else {
        return None;
    };
    return match Workspace::open_workspace(dir.to_path_buf(), &file_name[9..].to_string()) {
        Ok(w) => Some(w),
        Err(_) => None,
    };
}

fn save_workspace_path_to_storage(workspace: &Workspace) -> bool {
    //Open storage file in config dir
    let Some(dir) = ProjectDirs::from("com", "filetags", "cli") else {
        return false;
    };
    let path_to_storage_file = dir.config_local_dir().join("WorkspaceFilePath.txt");
    if !path_to_storage_file.exists() {
        // Create empty file
        let Some(parent_dir) = path_to_storage_file.parent() else {
            println!("ERROR: cannot create config file as no parent dir was specified");
            return false;
        };
        let Ok(_) = std::fs::create_dir_all(parent_dir) else {
            println!("ERROR: cannot create config file as cannot create parent dir ");
            return false;
        };
        match File::create(&path_to_storage_file) {
            Ok(_) => (),
            Err(e) => {
                println!("ERROR: cannot create config file at location '{:?}'", path_to_storage_file);
                println!("err is {:?}", e);
            },
        };
        return false;
    }

    //Write workspace file's path to file
    let binding = workspace.get_path_to_workspace_file();
    let Some(file_path) = &binding.to_str() else {
        println!("ERROR: cannot create config file at location '{:?}'", path_to_storage_file);
        return false;
    };
    return match std::fs::write(path_to_storage_file, file_path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn set_open_workspace_file(name: &String) -> () {
    // let Ok(_cwd) = env::current_dir() else {
    //     println!("ERROR [open]: current directory invalid");
    //     return;
    // };
    match Workspace::open_workspace(PathBuf::from("."), &name.clone()) {
        Ok(_) => (),
        Err(_x) => {
            // HANDLE ERROR
            println!("ERROR [open]: Could not open workspace");
            return;
        },
    }
}

fn create_set_workspace_file(name: &String) {
    let workspace = match Workspace::create_workspace(PathBuf::from("."), &name.clone()) {
        Ok(w) => w,
        Err(_x) => {
            // HANDLE ERROR
            println!("ERROR [open]: Could not create workspace");
            return;
        },
    };
    save_workspace_path_to_storage(&workspace);
}

fn add_tags_to_file(workspace: &mut Option<Workspace>, file_name: &String, simple: &Vec<String>, kv: &Vec<String>) {
    let Some(workspace) = workspace else {
        // TODO - error out
        return;
    };
    let path = PathBuf::from(file_name);
    // TODO - validate
    for simple_tag in simple {
        match workspace.add_tag_to_file(path.clone(), simple_tag.clone(), None) {
            Ok(_) => (),
            Err(error) => {
                println!("ERROR when adding tag: {}", error.to_string());
            }
        }
    }

    for chunk in kv.chunks(2) {
        if chunk.len() == 2 {
            let k = &chunk[0];
            let v = &chunk[1];

            match workspace.add_tag_to_file(path.clone(), k.clone(), Some(v.clone())) {
                Ok(_) => (),
                Err(error) => {
                    println!("ERROR when adding tag: {}", error.to_string());
                }
            }
        }
    }
}

fn remove_tags_from_file(workspace: &mut Option<Workspace>, all_remove: bool, file_name: &String, simple: &Vec<String>, kv: &Vec<String>) {
    let Some(workspace) = workspace else {
        // TODO - error out
        return;
    };
    let path = PathBuf::from(".").join(file_name).canonicalize().unwrap();
    // TODO - validate

    if all_remove {
        let Ok(vec) = workspace.get_tags_for_file_name(path.clone()) else {
            return; //TODO - error out
        };

        for tag in vec {
            let result = match tag {
                tagcore::Tag::Simple(s) => workspace.remove_tag_from_file(path.clone(), s.clone(), None),
                tagcore::Tag::KV(k, v) => workspace.remove_tag_from_file(path.clone(), k.clone(), Some(v.clone())),
            };

            match result {
                Ok(_) => (),
                Err(error) => println!("ERROR when removing tag: {}", error.to_string()),
            };
        }
        return;
    }

    
    for simple_tag in simple {
        println!("REUSLT IS {:?}", path);
        match workspace.remove_tag_from_file(path.clone(), simple_tag.clone(), None) {
            Ok(_) => (),
            Err(error) => {
                println!("ERROR when removing tag: {}", error.to_string());
            }
        }
    }

    for chunk in kv.chunks(2) {
        if chunk.len() == 2 {
            let k = &chunk[0];
            let v = &chunk[1];

            match workspace.remove_tag_from_file(path.clone(), k.clone(), Some(v.clone())) {
                Ok(_) => (),
                Err(error) => {
                    println!("ERROR when adding tag: {}", error.to_string());
                }
            }
        }
    }
}

fn show_tags_for_file(workspace: &Option<Workspace>, file_name: &String) {
    let Some(workspace) = workspace else {
        // TODO - error out
        return;
    };

    let path = PathBuf::from(file_name);
    // TODO - validate

    let Ok(tags) = workspace.get_tags_for_file_name(path.clone()) else {
        // TODO - error out
        return;
    };

    let str_vec: Vec<String> = tags.iter().map(|tag| {
        match tag {
            tagcore::Tag::Simple(s) => s.to_owned(),
            tagcore::Tag::KV(k,v) => k.to_string() + ": " + v,
        }
    }).collect();

    println!("{}", str_vec.join(", "));
}

// TODO - fix query problem...
fn search(workspace: &Option<Workspace>, search_args: &SearchArgs) {
    let Some(workspace) = workspace else {
        // TODO - error out
        return;
    };

    let mut map = HashMap::<String, Vec<tagcore::Tag>>::new();
    if search_args.exact {
        for arg in &search_args.tags {
            let result = workspace.query_exact(&arg, search_args.simple_on, search_args.key_on, search_args.values_on);
            map.extend(result);
        }
    }
    else {
        for arg in &search_args.tags {
            let result = workspace.query_fuzzy(&arg, search_args.simple_on, search_args.key_on, search_args.values_on);
            map.extend(result);
        }
    }

    let mut keys: Vec<String> = map.keys().cloned().collect();
    keys.sort();

    for file_path in keys {
        let str_vec: Vec<String> = map.get(&file_path).unwrap().iter().map(|tag| {
            match tag {
                tagcore::Tag::Simple(s) => s.to_owned(),
                tagcore::Tag::KV(k,v) => k.to_string() + ": " + v,
            }
        }).collect();

        println!("{} -- {}", file_path, str_vec.join(", "));
    }
}

fn show_workspace_name(workspace: &Option<Workspace>) {
    match workspace {
        Some(w) => println!("{}", w.get_name()),
        None => println!("No workspace opened"),
    }
}