mod bytecode;
mod module;

use std::fs::File;
use std::io::Read;
use crate::bytecode::Bytecode;

fn main() {
    let mut path = std::env::current_dir().unwrap();
    path.push("./code.wasm");
    let bin = read_file(&path.to_str().unwrap().to_string()).unwrap();

    let bytecode = Bytecode::new(bin);
    
}

fn read_file(path: &String) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut out = vec![];
    file.read(&mut out)?;

    Ok(out)
}
