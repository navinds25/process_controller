use std::error::Error;
use std::io::prelude::*;
use std::process::{Command,Stdio};

fn main() {
    let scriptname = String::from("/home/joker/Rust/test_data/bash_script1.sh");
    println!("Starting process {} with streaming from stdout", scriptname);
    start_process_stream_stdout(scriptname);
    //println!("Starting process {} output on completion", scriptname);
    //start_process_output_on_req(scriptname);
}

fn start_process_stream_stdout(commandname: String) {
    let child = Command::new(commandname)
        .stdout(Stdio::inherit())
        .spawn() 
        .expect("failed to spawn child");

    child
        .wait_with_output()
        .expect("failed to wait on child");
 }

// WIP
//fn start_process_output_on_req(commandname: String) {
//     let child = Command::new(commandname)
//        .stdout(Stdio::piped())
//        .spawn() 
//        .expect("failed to spawn child");
//
//    child
//        .wait_with_output()
//        .expect("failed to wait on child");
//
//    let mut s = String::new();
//    child.stdout.unwrap().read_to_string(&mut s) {
//            Err(why) => panic!("couldn't read child process stdout: {}", why.description()),
//            Ok(_) => print!("child process responded with: \n{}", s),
//        }
//    
// }
