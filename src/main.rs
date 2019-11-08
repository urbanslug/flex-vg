#![allow(dead_code, unused_imports, unused_variables, unused_imports)]
use std::borrow::Cow;



fn splitter<'a>(seq: &'a Cow<str>) -> &'a str {
    &seq[0..10]
}

fn main() {
    let seq: String = String::from("\
ACAACAAACTTGCGTAAACCAAAAAAATGGGGC\
AAATAAGAATTTGATAAGTACCACTTAAATTTA\
ACTCCTTTGGTTAGAGATGGGCAGCAACTCATT\
CCTGGGACACTCTCAATCATCTATTATTCATATC\
ATCGTGCTTATACAAGTTAAATCTTAAATCTATA\
GCACAAATCACATTATCTATTTTGGCAATGATAA\
TCTCAACCTCACGAGTATGATAGCACAAATCAGT\
");
    // Cow<str> is a borrow, so it will be &str internally
    let cow_seq: Cow<str> = Cow::Borrowed(&seq);
    let first_ten = splitter(&cow_seq);
    println!("{}", first_ten);
}
