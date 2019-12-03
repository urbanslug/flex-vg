use std::fs::File;
use std::io::{BufRead, BufReader};
use vcf;
use std::io::Cursor;


/// A slice exists in relation to a reference.
/// This is the index and the length for which it runs
struct Slice {
    index: u64,
    len: u64,
}

// Would it work better as a tuple struct?
/// We parse VCF files to get VCFRecords and specifically want to store slices
/// so that we can easily split references.
pub struct Variation {
    vcf_record: vcf::VCFRecord,
    kmer: Slice,
}

// This iteration is slow
pub fn gen_variations(vcf_reader: &mut vcf::VCFReader<BufReader<File>>) -> Vec<Variation> {
    let start = 0;
    let mut v: Vec<Variation> = vec![];
    for l in vcf_reader {
        let record = l.unwrap();
        let index = start;
        let len = record.position - start;

        let slice = Slice { index, len };

        v.push(Variation {
            vcf_record: record,
            kmer: slice,
        });
    }
    v
}

pub fn open_vcf(fp: &str) -> vcf::VCFReader<BufReader<File>> {
    let f = File::open(fp).unwrap();
    vcf::VCFReader::new(f).unwrap()
}

pub fn handle_vcf(fp: &str) -> Vec<Variation> {
    let mut vcf_reader = open_vcf(fp);
    let variations = gen_variations(&mut vcf_reader);
    variations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_vcf() {
        let data = Cursor::new("
##fileformat=VCFv4.2
##FORMAT=<ID=GT,Number=1,Type=Integer,Description=\"Genotype\">
##FORMAT=<ID=GP,Number=G,Type=Float,Description=\"Genotype Probabilities\">
##FORMAT=<ID=PL,Number=G,Type=Float,Description=\"Phred-scaled Genotype Likelihoods\">
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO	FORMAT	SAMP001	SAMP002
19	1006608 rs84825 C	T,G 	.	PASS	.	GT:GP	0/1:.	0/1:0.03,0.97,0
19	1100608 rs84825 C	ATCT	.	PASS	.	GT:GP	0/1:.	0/1:0.03,0.97,0
20	1291018	rs11449	G	A	    .	PASS	.	GT	0/0	0/1
20	2300608 rs84825 C	T	    .	PASS	.	GT:GP	0/1:.	0/1:0.03,0.97,0
20	2301308 rs84823 T	G	    .	PASS	.	GT:PL	./.:.	1/1:10,5,0
");

        let mut  vcf_reader = vcf::VCFReader::new(data).unwrap();


        let samples = &vcf_reader.header().samples;
        let items = &vcf_reader.header().items;

        let expected_samples = vec!["SAMP001", "SAMP002"];

        // Can these be compared any better?
        // We don't care about the ordering of the samples
        for sample in samples {
            let sample = sample.as_str();
            assert!(match expected_samples.binary_search(&sample) {
                Ok(_) => true,
                _ => false,
            });
        }

        // Records
        
        let record: &mut vcf::VCFRecord = &mut vcf_reader.iter().next().unwrap().unwrap();

        
        assert_eq!(record.chromosome, "19");
    }

    #[test]
    #[ignore]
    fn late() {
        let fp =
            "/Users/mmwaniki/data/RSV/fake_H_3801_22_04.freebayes.vcf";

        let mut vcf_reader = open_vcf(fp);
        // let variations = gen_variations(&mut vcf_reader);

        // assert!(variations.len() > 0);
    }
}
