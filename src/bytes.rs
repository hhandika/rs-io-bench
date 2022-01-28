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

pub fn iter_fasta_bytes(path: &Path) -> Result<()> {
    let file = File::open(path)?;
    let recs = FastaReader::new(file);

    recs.into_iter().for_each(|rec| {
        println!("{}", rec.id);
        println!("{}", rec.seq);
    });

    Ok(())
}

struct Recs {
    id: String,
    seq: String,
}

impl Recs {
    fn new() -> Self {
        Recs {
            id: String::new(),
            seq: String::new(),
        }
    }
}

pub fn read_fasta_bytes(path: &Path) -> Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut recs = Recs::new();

    loop {
        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                if let Some(id) = buffer.strip_prefix('>') {
                    if !recs.id.is_empty() {
                        println!("{}", recs.id.trim());
                        println!("{}", recs.seq);
                        recs.id.clear();
                    }
                    if !buffer.is_empty() {
                        recs.id = String::from(id);
                        recs.seq.clear();
                    }
                    recs.seq.clear();
                } else {
                    recs.seq.push_str(buffer.trim());
                }
                buffer.clear();
            }
            Err(e) => return Err(e),
        }
    }

    // Print last read record
    if !recs.id.is_empty() {
        println!("{}", recs.id.trim());
        println!("{}", recs.seq);
    }
    Ok(())
}

struct FastaReader<R> {
    reader: BufReader<R>,
    buffer: String,
}

impl<R: Read> FastaReader<R> {
    pub fn new(file: R) -> Self {
        Self {
            reader: BufReader::new(file),
            buffer: String::new(),
        }
    }

    fn next_seq(&mut self) -> Option<Recs> {
        let mut recs = Recs::new();
        while let Some(Ok(line)) = self.reader.read_line(&mut self.buffer).next() {
            if line == 0 {
                None
            } else {
                if let Some(id) = self.buffer.strip_prefix('>') {
                    if !recs.id.is_empty() {
                        return Some(recs);
                        // recs.id.clear();
                    }
                    if !self.buffer.is_empty() {
                        recs.id = String::from(id);
                        recs.seq.clear();
                    }
                    recs.id.clear();
                    recs.seq.clear();
                } else {
                    recs.seq.push_str(self.buffer.trim());
                }
                self.buffer.clear();
                continue;
            }
        }

        if !recs.id.is_empty() {
            Some(recs)
        } else {
            None
        }
    }
}
// fn get_recs(&self, id: &str, seq: &str) -> Recs {
//     Recs::new(id, seq)
// }

impl<R: Read> Iterator for FastaReader<R> {
    type Item = Recs;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_seq()
    }
}
