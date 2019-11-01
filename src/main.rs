use needletail::{parse_sequence_path, Sequence};
use std::env;

fn main() {
    let filename: String = env::args().nth(1).unwrap();

    let mut n_bases = 0;
    let mut n_valid_kmers = 0;
    parse_sequence_path(
	filename,
        |_| {},
        |seq| {
            // seq.id is the name of the record
            // seq.seq is the base sequence
            // seq.qual is an optional quality score

            // keep track of the total number of bases
            n_bases += seq.seq.len();

            // normalize to make sure all the bases are consistantly capitalized
            let norm_seq = seq.normalize(false);
            // we make a reverse complemented copy of the sequence first for
            // `canonical_kmers` to draw the complemented sequences from.
            let rc = norm_seq.reverse_complement();
            // now we keep track of the number of AAAAs (or TTTTs via
            // canonicalization) in the file; note we also get the postion (i.0;
            // in the event there were `N`-containing kmers that were skipped)
            // and whether the sequence was complemented (i.2) in addition to
            // the canonical kmer (i.1)
            for (_, kmer, _) in norm_seq.canonical_kmers(4, &rc) {
                if kmer == b"AAAA" {
                    n_valid_kmers += 1;
                }
            }
        },
    )
    .expect("parsing failed");
    println!("There are {} bases in your file.", n_bases);
    println!("There are {} AAAAs in your file.", n_valid_kmers);
}
