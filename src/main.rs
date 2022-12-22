extern crate libc;

use std::env;
use std::process::exit;
use std::ffi::CString;
use std::collections::HashMap;
use nix::unistd::execvp;


fn print_help() {
    let message = r#"
Usage:
tagrun [--tag PROCESS_NAME_TAG] [--prefix PROCESS_NAME_PREFIX] COMMAND [ARG] ...
tagrun --help

Run COMMAND, and rename the process(i.e., argv[0] of the command line). 

    --tag       Rename the command with PROCESS_NAME_TAG.
    --prefix    Prepend the PROCESS_NAME_PREFIX to the command line.

Example:

    tagrun --tag awake --prefix [test] sleep 1000

Above command will be displayed in the result of `ps aux` like

    [test]awake 1000

And you can use `pgrep -f -x -a '\[test\]awake 1000'` to match the process.
"#;
    println!("{}", message);
}


fn main() {
    let origin_args: Vec<String> = env::args().collect();    
    if origin_args.len() > 1 && origin_args[1] == "--help" {
        print_help();
        exit(1);
    }

    let mut ctrl_params = HashMap::new();
    let mut split_index = 1;
    if origin_args.len() >= 3 && (origin_args[1] == "--tag" || origin_args[1] == "--prefix") {
       ctrl_params.insert(origin_args[1].clone(), origin_args[2].clone());
       split_index = 3;
       if origin_args.len() >= 5 && (origin_args[3] == "--tag" || origin_args[3] == "--prefix") {
           ctrl_params.insert(origin_args[3].clone(), origin_args[4].clone());
           split_index = 5;
        }
    }

    let mut _left: &[String];
    let right: &[String];
    (_left, right) = origin_args.split_at(split_index);
    if right.len() < 1 {
        print_help();
        exit(1);
    }

    let (target_cmd, target_params) = right.split_at(1);
    let mut tagged_cmd = target_cmd[0].clone();
    if ctrl_params.get("--tag") != None {
        tagged_cmd = ctrl_params.get("--tag").unwrap().clone()
    }
    if ctrl_params.get("--prefix") != None {
        tagged_cmd = ctrl_params.get("--prefix").unwrap().clone() + tagged_cmd.as_str();
    }
        
    let mut new_args = Vec::<String>::new();
    new_args.push(tagged_cmd);
    new_args.append(&mut target_params.iter().map(|x| x.to_owned()).collect::<Vec<String>>());
    let c_string_new_args: Vec<CString> = new_args.iter().map(|arg| CString::new(arg.as_str()).unwrap().to_owned()).collect();
    let tagged_cmd_path : CString = CString::new(target_cmd[0].as_str()).unwrap();
    match execvp(&tagged_cmd_path, c_string_new_args.as_slice()) {
        Ok(_) => todo!(),
        Err(e) => {
            println!("{}", e);
        }        
    }
}
