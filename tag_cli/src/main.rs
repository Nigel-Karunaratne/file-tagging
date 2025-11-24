use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use std::{fs::File, path::PathBuf};

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
    /// Opens a workspace
    Open {
        #[arg(short,long)]
        name: String

    },

    /// Creates a workspace, if it doesn't exist
    Create {
        #[arg(short,long)]
        name: String
    },

    /// Adds listed tags to a file
    Add {

    },

    /// Removes specified tags from files
    Remove {

    },

    /// Show all tags on a file
    Show {

    },

    /// Search by tags
    Search {

        
    }
}

fn main() {
    let _workspace = load_workspace_from_storage();
    let cli = Cli::parse();

    match cli.command {
        Commands::Open { name } => set_open_workspace_file(&name),
        Commands::Create { name  } => create_set_workspace_file(&name),
        Commands::Add {  } => todo!(),
        Commands::Remove {  } => todo!(),
        Commands::Show {  } => todo!(),
        Commands::Search {  } => todo!(),
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
    return match Workspace::open_or_create_workspace(dir.to_path_buf(), file_name.to_string()) {
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
    let binding = workspace.get_path_to_file();
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
    match Workspace::open_or_create_workspace(PathBuf::from("."), name.clone()) {
        Ok(_) => (),
        Err(_x) => {
            // HANDLE ERROR
            println!("ERROR [open]: Could not open workspace");
            return;
        },
    }
}

fn create_set_workspace_file(name: &String) {
    let workspace = match Workspace::open_or_create_workspace(PathBuf::from("."), name.clone()) {
        Ok(w) => w,
        Err(_x) => {
            // HANDLE ERROR
            println!("ERROR [open]: Could not create workspace");
            return;
        },
    };
    save_workspace_path_to_storage(&workspace);
}