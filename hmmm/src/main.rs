use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

// dont worry about any of the code below, it's not cursed or anything
// only 100% beautiful RESF-approved code here 👍

fn main() {
    let program = "#include <stdio.h>
int main(void)
{ printf(\"Hello world\\n\"); }";
    let dir = tempdir().unwrap();
    let source_path = dir.path().join("helloworld.c");
    let output_path = dir.path().join("a.out");
    let mut source = File::create(source_path.clone()).unwrap();
    writeln!(source, "{}", program);

    Command::new("clang")
        .arg(source_path.clone())
        .arg("-o")
        .arg(output_path.clone())
        .output()
        .expect("failed to execute clang");

    let output = Command::new(output_path)
        .output()
        .expect("failed to execute helloworld");

    println!("{}", String::from_utf8(output.stdout).unwrap());
}
