use std::path::Path;

mod bytes;

fn main() {
    let files = Path::new("tests/test_files/simple.nex");
    bytes::read_nexus_bytes(files).unwrap();

    println!("Fasta:");
    let fasta = Path::new("tests/test_files/simple.fas");
    bytes::read_fasta_bytes(fasta).unwrap();

    println!("Iterator fasta:");
    bytes::iter_fasta_bytes(fasta).unwrap();
}
