use std::{fs::File, io::Write, process::Command};

use proc_macro::TokenStream;

#[proc_macro]
pub fn zig_unsafe(_item: TokenStream) -> TokenStream {
    // write token stream to zig file to compile
    println!("{}", _item);
    let mut zig_file = File::create("zig_unsafe_output.zig").unwrap();
    let content = format!("{}", _item);
    let res = zig_file.write_all(content.as_bytes()).unwrap();

    Command::new("zig")
        .args(["build-lib", "zig_unsafe_output.zig"])
        .output()
        .expect("failed to execute zig build");

    "".parse().unwrap()
}
