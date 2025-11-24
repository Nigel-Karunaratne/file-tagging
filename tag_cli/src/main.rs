use std::path::PathBuf;

use tagcore::Workspace;

fn main() {
    println!("Hello, world!");
    let mut _wkspc: Workspace = Workspace::open_or_create_workspace(PathBuf::from("."), "test".to_string()).unwrap();

    println!("{:?}", PathBuf::from("./src/main.rs"));

    match _wkspc.add_tag_to_file(PathBuf::from("./src/main.rs"), "Hello".to_string(), None) {
        Ok(_) => println!("Done successfully"),
        Err(e) => println!("ERROR: {}", e.to_string()),
    }
}
