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

struct Records {
    id: String,
    seq: String,
}

pub fn read_fasta_bytes(path: &Path) -> Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut recs = Records {
        id: String::new(),
        seq: String::new(),
    };

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

struct Recs {
    id: String,
    seq: String,
}

impl Recs {
    fn new(id: &str, seq: &str) -> Self {
        Recs {
            id: String::from(id),
            seq: String::from(seq),
        }
    }
}

struct FastaReader<R> {
    reader: BufReader<R>,
    id: String,
    seq: String,
}

impl<R: Read> FastaReader<R> {
    fn new(file: R) -> Self {
        Self {
            reader: BufReader::new(file),
            id: String::new(),
            seq: String::new(),
        }
    }

    fn next_seq(&mut self) -> Option<Recs> {
        while let Some(Ok(line)) = self.reader.by_ref().lines().next() {
            if let Some(id) = line.strip_prefix('>') {
                if self.id.is_empty() {
                    self.id = String::from(id);
                    self.seq.clear();
                } else {
                    let recs = self.get_recs(&self.id, &self.seq);
                    self.id = String::from(id);
                    self.seq.clear();
                    return Some(recs);
                }
            } else {
                self.seq.push_str(line.trim());
            }
        }
        if !self.id.is_empty() {
            let recs = self.get_recs(&self.id, &self.seq);
            self.id.clear();
            self.seq.clear();
            Some(recs)
        } else {
            None
        }
    }

    fn get_recs(&self, id: &str, seq: &str) -> Recs {
        Recs::new(id, seq)
    }
}

impl<R: Read> Iterator for FastaReader<R> {
    type Item = Recs;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_seq()
    }
}
