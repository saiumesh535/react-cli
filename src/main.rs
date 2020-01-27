use crate::helpers::fs_helper::write_file_src;
use std::env;
use crate::helpers::string_helper::string_replace;

mod helpers;

const DIR_KEY: &str = "DIR_NAME";
const FILE_NAME: &str = "FN";

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn read_env(key: &str) -> String {
    match env::var(key) {
        Ok(value) => value,
        Err(_e) => panic!(format!("{key} cannot be empty", key = key)),
    }
}

fn write_component(dir_name: &String, file_name: &String) {
    let mut content = format!("{}", String::from_utf8_lossy(include_bytes!("static/cli.component.tsx")));
    let component = format!("{}.tsx", file_name);
    let component_name = format!("{file_name}Component", file_name = file_name);
    content = string_replace("ReactComponent", uppercase_first_letter(component_name.as_str()).as_str(), 2, content);
    match write_file_src(dir_name, &component, content) {
        Ok(()) => println!("Component has been written"),
        Err(e) => panic!("Error {:?}", e),
    };
}

fn write_effect(dir_name: &String, file_name: &String) {
    let mut content = format!("{}", String::from_utf8_lossy(include_bytes!("static/cli.effects.ts")));
    let component = format!("{}.effects.ts", file_name);
    content = string_replace("%file_name%", file_name.as_str(), 3, content);
    match write_file_src(dir_name, &component, content) {
        Ok(()) => println!("Effects has been written"),
        Err(e) => panic!("Error {:?}", e),
    };
}

fn write_actions(dir_name: &String, file_name: &String) {
    let mut content = format!("{}", String::from_utf8_lossy(include_bytes!("static/cli.actions.ts")));
    let component = format!("{}.actions.ts", file_name);
    content = string_replace("%file_name%", file_name.as_str(), 2, content);
    content = string_replace("%file_name_cap%", file_name.to_uppercase().as_str(), 2, content);
    match write_file_src(dir_name, &component, content) {
        Ok(()) => println!("Actions has been written"),
        Err(e) => panic!("Error {:?}", e),
    };
}

fn write_reducer(dir_name: &String, file_name: &String) {
    let mut content = format!("{}", String::from_utf8_lossy(include_bytes!("static/cli.reducer.ts")));
    let component = format!("{}.reducer.ts", file_name);
    content = string_replace("%file_name%", file_name.as_str(), 6, content);
    content = string_replace("%c_file_name%", uppercase_first_letter(file_name.as_str()).as_str(), 3, content);
    match write_file_src(dir_name, &component, content) {
        Ok(()) => println!("Reducer has been written"),
        Err(e) => panic!("Error {:?}", e),
    };
}




fn main() {
    let dir_name = read_env(DIR_KEY);
    let file_name = read_env(FILE_NAME);
    write_component(&dir_name, &file_name);
    write_effect(&dir_name, &file_name);
    write_actions(&dir_name, &file_name);
    write_reducer(&dir_name, &file_name);
}
