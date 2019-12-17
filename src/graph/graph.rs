// Reference
use needletail::{self, SequenceRecord};
use std::borrow::Cow;

// Files
use std::io::BufReader;
use std::io::Read;

// VCF
use vcf::{self, VCFReader, VCFRecord};

use std::str;

use crate::graph::types::{self, Node};
use crate::io::types::{Buf, Seeker};
/*
Split the reference based on variation data.

Read the records (values) in `vcf_reader` and split the
reference based on the chromosome/id & the positions.

A VCF record is a data lines that contains marker and genotype data (one variant per line).

For our purposes, the following will be (somewhat) synonymous:
 - sequence & reference
 - locus, position & index
 - record and VCF record

*Loop* through the variation data and split there reference file/data
based on the variations.

BUG:
Skips chromosomes when the order of chromosomes on the reference
isn't the same as the order of chromosomes in the VCF.
 */
pub fn splitter<R: Read>(
    seq_record: SequenceRecord,
    vcf_reader: &mut VCFReader<BufReader<R>>,
    vcf_record_buffer: &mut Buf<VCFRecord>,
    opt_seeker: &mut Option<Seeker>,
) -> () {
    /*
    Name and/or a unique identifier for the sequence
    Most times refers to a chromosome
     */
    let sequence_id = str::from_utf8(&seq_record.id).unwrap();
    println!("Processing sequence with ID {}", sequence_id);

    //
    let mut process_record = |record: &VCFRecord, seq_record: &SequenceRecord| {
        let seq: &Cow<[u8]> = &seq_record.seq;
        let reference = str::from_utf8(&seq_record.id).unwrap();
        let record_pos = record.position as usize; // end

        let start = match opt_seeker {
            // If there's a seeker and it matches the chromosome use it.
            Some(seeker) => {
                if seeker.chromosome() == record.chromosome {
                    seeker.position() as usize
                } else {
                    // this is the first split for this sequence so again start at 0
                    0
                }
            }
            // If there isn't a seeker it means this is the first split so we start at 0
            _ => 0,
        };

        // Slice the sequence
        let p = &seq[start..record_pos];
        let p = str::from_utf8(p).unwrap();
        let n = Node::new(p, record_pos, reference, Vec::new(), Vec::new());
        println!("Node: {}", n);

        // Update the seeker
        let new_seeker = Seeker::new(record.chromosome.clone(), record.position);
        opt_seeker.replace(new_seeker);
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
            process_record(&buffered_record, &seq_record);
        } else {
            vcf_record_buffer.write(buffered_record);
            return;
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
            process_record(&vcf_record, &seq_record)
        } else {
            // store the VCF record in a buffer
            // use it when we start to read that part of the reference
            println!(
                "Moving to another sequence because of chromosome {} id {}",
                vcf_record.chromosome, sequence_id
            );
            println!("Storing record with ID {} \n\n", vcf_record.chromosome);
            vcf_record_buffer.write(vcf_record);
            break;
        }
    }
}
