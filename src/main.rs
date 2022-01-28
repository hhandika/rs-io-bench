use std::path::Path;
use std::time::Instant;

mod bytes;

fn main() {
    let files = Path::new("tests/test_files/simple.nex");
    let fasta = Path::new("tests/test_files/simple.fas");
    bytes::read_nexus_bytes(files).unwrap();

    println!("Iterator fasta:");
    let time_2 = Instant::now();
    bytes::iter_fasta_bytes(fasta).unwrap();
    println!("Execution time: {:?}", time_2.elapsed());

    println!("Fasta:");
    let time = Instant::now();
    bytes::read_fasta_bytes(fasta).unwrap();
    println!("Execution time: {:?}", time.elapsed());
}
