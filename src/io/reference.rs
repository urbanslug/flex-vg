use std::str;

// Files
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};

// VCF
use vcf::{self, VCFReader, VCFRecord};

// Reference
use needletail::{self, SequenceRecord};
use std::borrow::Cow;
use std::io::Cursor;
// use stringreader::StringReader;
use std::collections::HashMap;

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
            splitter(seq, vcf_reader, &mut vcf_recrod_buffer, &mut option_seeker);
        },
    )
    .expect("Parsing failed");
}



/*
Split the reference based on variation data.

Read the records (values) in `vcf_reader` and split the
reference based on the chromosome/id & the positions.

A VCF record is a data lines that contains marker and genotype data (one variant per line).

For our purposes, the following will be (somewhat) synonymous:
 - sequence & reference
 - locus & position & index
 - record and VCF record

*Loop* through the variation data and split there reference file/data
based on the variations.

BUG:
Skips chromosomes when the order of chromosomes on the reference
isn't the same as the order of chromosomes in the VCF
 */
fn splitter<R: Read>(
    seq: SequenceRecord,
    vcf_reader: &mut VCFReader<BufReader<R>>,
    vcf_record_buffer: &mut Buf<VCFRecord>,
    opt_seeker: &mut Option<Seeker>,
) -> () {
    /*
    Name and/or a unique identifier for the sequence
    Most times refers to a chromosome
     */
    let sequence_id = str::from_utf8(&seq.id).unwrap();
    println!("Processing sequence with ID {}", sequence_id);

    let process_record = |record: &VCFRecord| {
        println!(
            "Processing record for sequence {} and chromosome {}",
            sequence_id, record.chromosome
        );
    };

    /*
    Don't iterate before checking whether there's a record in the `vcf_record_buffer`
    because the iterator may skip the current value in the buffer and lose some records
    This is flex's way of moving the cursor one record back to the previous variation value.
     */
    if vcf_record_buffer.has_value() {
        let buffered_record = vcf_record_buffer.read().unwrap();

        /*
        If the chromosome value of the VCF record in the buffer doesn't match the `sequence_id`
        Put it back in the buffer else process the record.
         */
        if buffered_record.chromosome == sequence_id {
            process_record(&buffered_record);
        } else {
            vcf_record_buffer.write(buffered_record);
            return
        }
    }

    /*
    Loop through the variation data and take a split the reference if the
    chromosome matches the value of the sequence.

    Splitting the reference is based on acquiring slices of the reference/sequence.

    We create a slice of the previous variation position and the current variation position.
    A slice refers to an index and a length.

    This slice is going to be considered some sort of conserved region.
    This will make a node that doesn't have a "vertically adjacent node"
    The locus that contains the variation is then going to make an alternative node
    to the reference node.
     */
    for opt_record in vcf_reader.iter() {
        let vcf_record = opt_record.unwrap();

        if vcf_record.chromosome == sequence_id {
            process_record(&vcf_record)
        } else {
            // store the VCF record in a buffer
            // use it when we start to read that part of the reference
            println!("Moving to another sequence because of chromosome {} id {}",
                     vcf_record.chromosome,
                     sequence_id);
            println!("Storing record with ID {} \n\n", vcf_record.chromosome);
            vcf_record_buffer.write(vcf_record);
            break;
        }
    }
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
chr1	10	rs21549	T	.	.	PASS	.	GT	0/0	0/1
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
>ChrW
TCTTGTTCTCAAGACCATGGTGAAATTGCTGAAGCCCTGTGTTGCCTCGC
",
        );
        fasta_data
    }

    #[test]
    fn can_split_ref() {
        let reference = yeild_fasta();
        let mut vcf_reader = VCFReader::new(yeild_vcf()).unwrap();

        process_fasta(reference, &mut vcf_reader);
        panic!("Intentional panic for test");
    }

    #[test]
    fn test_fasta_processing() {
        let data = yeild_fasta();
        //process_fasta(data);
    }

    #[test]
    fn can_read_vcf() {
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

        assert_eq!(record.chromosome, "12");
        assert_eq!(record.reference, "G");
        assert_eq!(record.position, 91018);
        assert_eq!(record.alternative, vec!["A", "T"]);
    }
}
