use std::env::var;
use snafu::{ ResultExt };
use crate::errors::*;
use crate::errors::Result;
use crate::utils::{string_replace, uppercase_first_letter};
use crate::fs_helpers::write_file_src;


const CN: &str = "CN";
const DP: &str = "DP";

pub fn get_env(env: &str) -> Result<String> {
    Ok(var(env).context(EnvErrorConfig{
        message: format!("{} should be passed", env)
    })?)
}

fn write_component(file_name: &String, dir_path: &String) -> Result<()> {
    let mut content = format!("{}", String::from_utf8_lossy(include_bytes!("static/cli.component.tsx")));
    let component = format!("{}.tsx", file_name);
    let component_name = format!("{file_name}Component", file_name = file_name);
    content = string_replace("ReactComponent", uppercase_first_letter(component_name.as_str()).as_str(), 2, content);
    write_file_src(&component, content, dir_path)?;
    Ok(())
}

fn write_effect(file_name: &String, dir_path: &String) -> Result<()> {
    let mut content = format!("{}", String::from_utf8_lossy(include_bytes!("static/cli.effects.ts")));
    let component = format!("{}.effects.ts", file_name);
    content = string_replace("%file_name%", file_name.as_str(), 3, content);
    write_file_src(&component, content, dir_path)?;
    Ok(())
}

fn write_actions(file_name: &String, dir_path: &String) -> Result<()> {
    let mut content = format!("{}", String::from_utf8_lossy(include_bytes!("static/cli.actions.ts")));
    let component = format!("{}.actions.ts", file_name);
    content = string_replace("%file_name%", file_name.as_str(), 2, content);
    content = string_replace("%file_name_cap%", file_name.to_uppercase().as_str(), 2, content);
    write_file_src(&component, content, dir_path)?;
    Ok(())
}

fn write_reducer(file_name: &String, dir_path: &String) -> Result<()> {
    let mut content = format!("{}", String::from_utf8_lossy(include_bytes!("static/cli.reducer.ts")));
    let component = format!("{}.reducer.ts", file_name);
    content = string_replace("%file_name%", file_name.as_str(), 6, content);
    content = string_replace("%c_file_name%", uppercase_first_letter(file_name.as_str()).as_str(), 3, content);
    write_file_src(&component, content, dir_path)?;
    Ok(())
}

fn print_message(message: &str) {
    println!("{}", message);
}

pub fn run_cli() -> Result<()>{
    let component_name = get_env(CN)?;
    let dir_path = var(DP).unwrap_or(String::from(format!("src/components/{component_name}", component_name = component_name)));
    write_component(&component_name, &dir_path)
        .and_then(|_| Ok(print_message("component has been written")))?;
    write_effect(&component_name, &dir_path)
        .and_then(|_| Ok(print_message("effect has been written")))?;
    write_actions(&component_name, &dir_path)
        .and_then(|_| Ok(print_message("actions has been written")))?;
    write_reducer(&component_name, &dir_path)
        .and_then(|_| Ok(print_message("reducer has been written")))?;
    Ok(())
}