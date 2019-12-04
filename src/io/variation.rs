use std::fs::File;
use std::io::Cursor;
use std::io::{BufRead, BufReader};
use vcf;

fn open_vcf(fp: &str) -> vcf::VCFReader<BufReader<File>> {
    let f = File::open(fp).unwrap();
    vcf::VCFReader::new(f).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn yeild_vcf() -> Cursor<&'static str> {
        Cursor::new(
            "
##fileformat=VCFv4.2
##FORMAT=<ID=GT,Number=1,Type=Integer,Description=\"Genotype\">
##FORMAT=<ID=GP,Number=G,Type=Float,Description=\"Genotype Probabilities\">
##FORMAT=<ID=PL,Number=G,Type=Float,Description=\"Phred-scaled Genotype Likelihoods\">
#CHROM	POS    	ID    	REF	ALT 	QUAL	FILTER	INFO	FORMAT	SAMP001	SAMP002
12	91018	rs10549	G	A,T	.	PASS	.	GT	0/0	0/1
15	300608 rs10825 C	GTCTA	.	PASS	.	GT:GP	0/1:.	0/1:0.03,0.97,0
19	1401308 rs20823 T	G	.	PASS	.	GT:PL	./.:.	1/1:10,5,0
20	1991018	rs31449	G	CCTA	.	PASS	.	GT	0/0	0/1
20	2300608 rs84825 C	T	.	PASS	.	GT:GP	0/1:.	0/1:0.03,0.97,0
20	2301308 rs84823 T	G	.	PASS	.	GT:PL	./.:.	1/1:10,5,0
",
        )
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
