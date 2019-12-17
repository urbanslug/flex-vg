use std::str;

// Files
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

// VCF
use vcf::{self, VCFReader, VCFRecord};

use crate::graph::graph;
use crate::io::types::{Buf, Seeker};

// VCF
fn open_vcf(fp: &str) -> VCFReader<BufReader<File>> {
    let f = File::open(fp).unwrap();
    vcf::VCFReader::new(f).unwrap()
}

// Reference
fn read_ref() {
    let f = File::open("test/data/RSV/refererence_and_vcf_file/9465113.fa").unwrap();
    // process_fasta(f);
}

// Check that the sequences in the reference are in the same order as
// they are in the VCF
fn preprocess_vcf_and_fasta<R: Read>(fasta_data: R, vcf_reader: &mut VCFReader<BufReader<R>>) {
    println!("Preprocessing...");

    // Get the order of chromosomes in the VCF
    let mut chromosomes_in_vcf: Vec<String> = Vec::new();

    for result_record in vcf_reader.iter() {
        let record = result_record.unwrap();
        let chr = record.chromosome;
        if !chromosomes_in_vcf.contains(&chr) {
            chromosomes_in_vcf.push(chr);
        }
    }

    // Get the order of sequences in the reference
    let mut sequences_in_fasta: Vec<String> = Vec::new();
    needletail::parse_sequence_reader(
        fasta_data,
        |t| {
            if t != "FASTA" {
                panic!("Preprocessing failed: expected a FASTA file but got {}.", t);
            }
        },
        |seq| {
            let id = std::str::from_utf8(&seq.id).unwrap();
            sequences_in_fasta.push(String::from(id))
        },
    )
    .expect("Preprocessing the reference failed");

    let mut indices: Vec<usize> = Vec::new();
    //let mut positions: Vec<usize> = Vec::new();
    // Check that refererences and variation data are in the same order
    for chr in chromosomes_in_vcf {
        // panic if chr not in the reference
        match sequences_in_fasta.binary_search(&chr) {
            Ok(index) => indices.push(index),
            Err(e) => panic!("Couldn't find {} in reference", chr),
        }
    }

    let mut sorted_indices = indices.clone();
    sorted_indices.sort_unstable();

    if indices == sorted_indices {
        println!("Reference and VCF are in order");
    } else {
        println!(
            "Warning: VCF and refererence aren't in order. Some variation data will be skipped."
        )
    }
}

// File processing
// TODO: rename gen_graph/build_graph look at libhandlegraph
fn process_fasta<R: Read>(fasta_data: R, vcf_reader: &mut VCFReader<BufReader<R>>) -> () {
    // Should we check for record ID in case the VCF and reference don't match?

    // let mut v: Vec<&SequenceRecord> = Vec::new();

    // Temporarily store a VCF record in case the iterator goes forward
    let mut vcf_recrod_buffer: Buf<VCFRecord> = Buf::new();

    // I want this to be an array that stores just one VCF record
    // let mut vcf_record_buf: [VCFRecord; 1];
    // TODO: how to type annotate an array

    // Start seeker at pos 0
    let mut option_seeker: Option<Seeker> = None;

    needletail::parse_sequence_reader(
        fasta_data,
        |_| {},
        |seq| {
            // TODO: use a slice and not a String here
            graph::splitter(seq, vcf_reader, &mut vcf_recrod_buffer, &mut option_seeker);
        },
    )
    .expect("Parsing failed");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn yeild_vcf() -> Cursor<&'static str> {
        Cursor::new(
            "
##fileformat=VCFv4.2
##FORMAT=<ID=GT,Number=1,Type=Integer,Description=\"Genotype\">
##FORMAT=<ID=GP,Number=G,Type=Float,Description=\"Genotype Probabilities\">
##FORMAT=<ID=PL,Number=G,Type=Float,Description=\"Phred-scaled Genotype Likelihoods\">
#CHROM	POS    	ID    	REF	ALT 	QUAL	FILTER	INFO	FORMAT	SAMP001	SAMP002
chr1	10	rs21549	T	C	.	PASS	.	GT	0/0	0/1
chr1	20	rs21549	T	.	.	PASS	.	GT	0/0	0/1
chr2	33	rs34146	G	C	.	PASS	.	GT	0/0	0/1
chr2	40	rs44459	G	TA	.	PASS	.	GT	0/0	0/1
chr3	23	rs71549	A	CA	.	PASS	.	GT	0/0	0/1
chrW	23	rs71549	A	CA	.	PASS	.	GT	0/0	0/1
chrY	70	rs91489	A	T,TAC	.	PASS	.	GT	0/0	0/1
",
        )
    }

    fn yeild_fasta() -> Cursor<&'static str> {
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
>chrW
TCTTGTTCTCAAGACCATGGTGAAATTGCTGAAGCCCTGTGTTGCCTCGC
",
        );
        fasta_data
    }

    #[test]
    #[ignore]
    fn test_can_preprocess_input() {
        let fasta_data = yeild_fasta();
        let mut vcf_reader = vcf::VCFReader::new(yeild_vcf()).unwrap();

        // TODO: fix panic because of out of order reference and VCF
        assert_eq!(preprocess_vcf_and_fasta(fasta_data, &mut vcf_reader), ())
    }

    #[test]
    #[should_panic]
    fn test_panic_when_variation_not_in_reference() {
        let vcf_data: Cursor<&str> = Cursor::new(
            "
##fileformat=VCFv4.2
##FORMAT=<ID=GT,Number=1,Type=Integer,Description=\"Genotype\">
##FORMAT=<ID=GP,Number=G,Type=Float,Description=\"Genotype Probabilities\">
##FORMAT=<ID=PL,Number=G,Type=Float,Description=\"Phred-scaled Genotype Likelihoods\">
#CHROM	POS    	ID    	REF	ALT 	QUAL	FILTER	INFO	FORMAT	SAMP001	SAMP002
chr1	10	rs21549	T	C	.	PASS	.	GT	0/0	0/1
chr1	20	rs21549	T	.	.	PASS	.	GT	0/0	0/1
chr2	33	rs34146	G	C	.	PASS	.	GT	0/0	0/1
chr2	40	rs44459	G	TA	.	PASS	.	GT	0/0	0/1
chr3	23	rs71549	A	CA	.	PASS	.	GT	0/0	0/1
chrX	23	rs71549	A	CA	.	PASS	.	GT	0/0	0/1
chrY	70	rs91489	A	T,TAC	.	PASS	.	GT	0/0	0/1
",
        );

        let mut vcf_reader = vcf::VCFReader::new(vcf_data).unwrap();
        let fasta_data = yeild_fasta();

        preprocess_vcf_and_fasta(fasta_data, &mut vcf_reader)
    }

    #[test]
    #[should_panic]
    fn test_panic_when_reference_is_fastq() {
        let mut vcf_reader = vcf::VCFReader::new(yeild_vcf()).unwrap();
        let fasta_data: Cursor<&str> = Cursor::new(
            "\
        @SRR3951347.1.1 1 length=251
TGTTAGGCTGGTGATTATACATCCCAAGAGGCCCCTTTTCTGCTTT
+
11>AAFFFFFAFGGGGGGGGGGHA01000000A0ABF1BAD21D11
@SRR3951347.2.1 2 length=251
TATCATCAGCGGCTTGCCCGTCTCCGCCCGTAGGGGCCGGGAGATA
+
>>AABFFFFFBBGGGGGGGGGGC2A22AA2E2221100000001B5
@SRR3951347.4.1 4 length=251
TGTCTCACTCATATAGATGCCCACTTCCTATCCCAGACAAAGCAGA
",
        );

        preprocess_vcf_and_fasta(fasta_data, &mut vcf_reader)
    }

    #[test]
    fn test_can_split_ref() {
        let reference = yeild_fasta();
        let mut vcf_reader = VCFReader::new(yeild_vcf()).unwrap();

        process_fasta(reference, &mut vcf_reader);
        // panic!("Intentional panic for print");
    }

    // TODO: Is this test necessary? It tests nothing in *flex-vg* code
    #[test]
    fn test_can_read_vcf() {
        let mut vcf_reader = vcf::VCFReader::new(yeild_vcf()).unwrap();

        // VCF Header
        let samples = &vcf_reader.header().samples;
        let items = &vcf_reader.header().items;
        let expected_samples = vec!["SAMP001", "SAMP002"];

        // We don't care about the ordering of the samples.
        // We only care that they exist.
        // TODO: Can these be compared any better?
        for sample in samples {
            let sample = sample.as_str();

            assert!(match expected_samples.binary_search(&sample) {
                Ok(_) => true,
                _ => false,
            });
        }

        // VCF Records
        let record: &mut vcf::VCFRecord = &mut vcf_reader.iter().next().unwrap().unwrap();

        assert_eq!(record.chromosome, "chr1");
        assert_eq!(record.reference, "T");
        assert_eq!(record.position, 10);
        assert_eq!(record.alternative, vec!["C"]);
    }
}
