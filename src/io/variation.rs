use std::fs::File;
use std::io::{BufRead, BufReader};
use vcf;

// This is essentially a slice
// should it also contain the string?
// A k-mer from a reference
struct Kmer {
    index: u64,
    len: u64,
}

// This is a VCF record plus a kmer (essentially a Slice).
// Would it work better as a tuple struct?
pub struct Variation {
    vcf_record: vcf::VCFRecord,
    kmer: Kmer,
}

fn gen_variations(vcf_reader: vcf::VCFReader<BufReader<File>>) -> Vec<Variation> {
    let start = 0;
    let mut v: Vec<Variation> = vec![];
    for l in vcf_reader {
        let record = l.unwrap();
        let index = start;
        let len = record.position - start;

        let slice = Kmer { index, len };

        v.push(Variation {
            vcf_record: record,
            kmer: slice,
        });
    }
    v
}



// Why is this test so slow?
pub fn handle_vcf() -> Vec<Variation> {
    let fp = "/Users/mmwaniki/data/mouse_mm10/C57BL/4512-JFI-0333_C57BL_6J_two_lanes_large_svs.vcf";
    let f = File::open(fp).unwrap();
    let vcf_reader = vcf::VCFReader::new(f).unwrap();

    let variations = gen_variations(vcf_reader);

    variations
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn late() {
        assert!(handle_vcf().len() > 0);
    }
}
