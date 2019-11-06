#![allow(unused_imports)]
use needletail::{parse_sequence_path, Sequence, SequenceRecord};
use std::borrow::Cow;
use std::str;
use vcf::*;
use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io;

fn process_fasta(x: SequenceRecord) -> () {
    let y = str::from_utf8(&x.id).unwrap();
    println!("{}", y);
    
}

const VCF_FILE: &str = "/Users/urbanslug/src/racket/graphite/data/RSV/refererence_and_vcf_file/H_3801_22_04.freebayes.vcf";
const FASTA_FILE: &str = "/Users/urbanslug/src/racket/graphite/data/RSV/refererence_and_vcf_file/9465113.fa";
//const FASTA_FILE: &str = "/Users/urbanslug/src/racket/graphite/data/test/tiny_fake_ref.fa";



fn read_vcf() {
    let p = File::open(VCF_FILE).unwrap();
    let mut vcf_reader = VCFReader::new(p).unwrap();

    for one in vcf_reader {
        let record = one.unwrap();
        // process a record
        println!("{}",  record.chromosome);
    }
}

fn read_fasta() {
    let filename = FASTA_FILE;

    parse_sequence_path(
        filename,
        |_| {},
        |seq| {
            process_fasta(seq);
        },
    )
    .expect("parsing failed");
}

fn main() {
    //read_vcf();
    read_fasta();
}
