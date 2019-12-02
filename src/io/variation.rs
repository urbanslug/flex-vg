use std::fs::File;
use std::io::{BufRead, BufReader};
use vcf;

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

//
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
    fn late() {
        let fp =
            "/Users/mmwaniki/data/mouse_mm10/C57BL/4512-JFI-0333_C57BL_6J_two_lanes_large_svs.vcf";
        assert!(handle_vcf(fp).len() > 0);
    }
}
