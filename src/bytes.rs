use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};
use std::path::Path;

pub fn read_nexus_bytes(path: &Path) -> Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    loop {
        match reader.read_until(b';', &mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                println!("{}", std::str::from_utf8(&buffer).unwrap().trim());
                buffer.clear();
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

struct Recs {
    id: String,
    seq: String,
}

pub fn read_fasta_bytes(path: &Path) -> Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut recs = Recs {
        id: String::new(),
        seq: String::new(),
    };

    loop {
        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                if let Some(id) = buffer.strip_prefix('>') {
                    if !recs.id.is_empty() {
                        println!("{}", recs.id);
                        println!("{}", recs.seq);
                        recs.id.clear();
                        recs.seq.clear();
                    } else {
                        recs.id = String::from(id);
                        recs.seq.clear();
                    }
                } else {
                    recs.seq.push_str(buffer.trim());
                }
                buffer.clear();
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
