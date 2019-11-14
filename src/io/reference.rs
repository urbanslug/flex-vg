use needletail::{self, SequenceRecord};
use std::borrow::Cow;
use std::io::Cursor;
// use stringreader::StringReader;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str;

// https://github.com/onecodex/needletail

fn splitter(seq: SequenceRecord) -> () {
    let g = str::from_utf8(&seq.id).unwrap();
    println!("{}", g);
}

fn other_splitter<'a>(seq: &'a Cow<str>, graph: &mut HashMap<u64, &'a str>) -> () {
    let x: &'a str = &seq[0..10];
    graph.insert(10, x);
    ()
}

fn old_main() {
    let seq: String = String::from(
        "\
        ACAACAAACTTGCGTAAACCAAAAAAATGGGGC\
        AAATAAGAATTTGATAAGTACCACTTAAATTTA\
        ACTCCTTTGGTTAGAGATGGGCAGCAACTCATT\

CCTGGGACACTCTCAATCATCTATTATTCATATC\
ATCGTGCTTATACAAGTTAAATCTTAAATCTATA\
GCACAAATCACATTATCTATTTTGGCAATGATAA\
TCTCAACCTCACGAGTATGATAGCACAAATCAGT\
",
    );

    let mut graph: HashMap<u64, &str> = HashMap::new();

    // Cow<str> is a borrow, so it will be &str internally
    let cow_seq: Cow<str> = Cow::Borrowed(&seq);
    other_splitter(&cow_seq, &mut graph);
    //println!("{}", first_ten);
    let v = 10u64;
    println!("{}", graph.get(&v).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn return_fasta_data() -> Cursor<&'static str> {
        let fasta_data: Cursor<&str> = Cursor::new(
            "\
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
",
        );
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

struct Slice {
    index: u64,
    len: u64,
}

struct SlicedVCFRecord {
    vcf_record: vcf::VCFRecord,
    slice: Slice,
}

fn process_vcf() -> Vec<SlicedVCFRecord> {
    let f = File::open("test/data/RSV/refererence_and_vcf_file/fake_H_3801_22_04.freebayes.vcf")
        .unwrap();
    let vcf_reader = vcf::VCFReader::new(f).unwrap();

    println!("{:?}", *vcf_reader.header());

    let start = 0;
    let mut v: Vec<SlicedVCFRecord> = vec![];
    for l in vcf_reader {
        let record = l.unwrap();
        let index = start;
        let len = record.position - start;

        let slice = Slice { index, len };

        v.push(SlicedVCFRecord {
            vcf_record: record,
            slice,
        });
    }
    v
}
