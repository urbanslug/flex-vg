#![allow(dead_code, unused_imports, unused_variables, unused_imports)]
use std::borrow::Cow;
use std::collections::HashMap;


fn splitter<'a>(seq: &'a Cow<str>, graph: &mut HashMap<u64, &'a str>) -> () {


    let x: &'a str =  &seq[0..10];
    graph.insert(10, x);

    ()
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

    let mut graph: HashMap<u64, &str> = HashMap::new();

    // Cow<str> is a borrow, so it will be &str internally
    let cow_seq: Cow<str> = Cow::Borrowed(&seq);
    splitter(&cow_seq, &mut graph);
    //println!("{}", first_ten);
    let v = 10u64;
    println!("{}", graph.get(&v).unwrap());

}
