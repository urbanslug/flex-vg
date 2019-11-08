// #![feature(cow_is_borrowed)]
#![allow(dead_code, unused_imports, unused_variables, unused_imports)]
use needletail::{self, SequenceRecord};
use std::borrow::Cow;
use std::io::Cursor;
// use stringreader::StringReader;
use std::str;
use std::fs::File;
use std::io::Read;

// https://github.com/onecodex/needletail


fn splitter(seq: SequenceRecord) -> () {
    let g = str::from_utf8(&seq.id).unwrap();
    println!("{}", g);
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    fn return_fasta_data() -> Cursor<&'static str> {
        let fasta_data: Cursor<&str> =  Cursor::new("\
            >chr1
GATCATCGGTTACACAGCTAAGTTTGACCGGCGCAGAAATGCCATATAAA
>chr2
TCTTGTTCTCAAGACCATGGTGAAATTGCTGAAGCCCTGTGTTGCCTCGC
>chr3
ACTAGGGTGCCAGGACAGTTACAAGTCTGAGAGACTGCAGACAATCTAAC
CCTTTGGTTGGTTGGAGGTGTGTGGGCGGGGTTGGGGGCGGTCTCTTGCT
>chr4
CCTTTGGTTGGTTGGAGGTGTGTGGGCGGGGTTGGGGGCGGTCTCTTGCT
ACTAGGGTGCCAGGACAGTTACAAGTCTGAGAGACTGCAGACAATCTAAC
>chrY
CCTTTGGTTGGTTGGAGGTGTGTGGGCGGGGTTGGGGGCGGTCTCTTGCT
ACTAGGGTGCCAGGACAGTTACAAGTCTGAGAGACTGCAGACAATCTAAC
");
        fasta_data
    }

    #[test]
    fn test_fasta_processing() {
        let data = return_fasta_data();
        process_fasta(data);
    }

}

fn process_fasta<R: Read>(fasta_data: R) -> () {
    // let mut v: Vec<&SequenceRecord> = Vec::new();
    needletail::parse_sequence_reader(
        fasta_data,
        |_| {},
        |seq| {
            splitter(seq);
        },
    )
        .expect("parsing failed");

    let chr1 = vec![99, 104, 114, 111, 109, 111, 115, 111, 109, 101, 49];
    let chr1_cow: Cow<[u8]> = Cow::Borrowed(&chr1);
    //println!("{}", chr1_cow.is_borrowed());
}

fn read_ref() {
    let f = File::open("test/data/RSV/refererence_and_vcf_file/9465113.fa").unwrap();
    process_fasta(f);
}

struct Slice{
    index: u64,
    len: u64,
}


struct SlicedVCFRecord {
    vcf_record: vcf::VCFRecord,
    slice: Slice
}



fn process_vcf() -> Vec<SlicedVCFRecord> {
    let f = File::open("test/data/RSV/refererence_and_vcf_file/fake_H_3801_22_04.freebayes.vcf").unwrap();
    let vcf_reader = vcf::VCFReader::new(f).unwrap();

    println!("{:?}", *vcf_reader.header());

    let start = 0;
    let mut v: Vec<SlicedVCFRecord> = vec![];
    for l in vcf_reader {
        let record = l.unwrap();
        let index = start;
        let len = record.position - start;

        let slice = Slice {
            index,
            len
        };

        v.push(
            SlicedVCFRecord {
                vcf_record: record,
                slice,
            }
        );
    }
    v
}
