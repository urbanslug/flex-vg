#![feature(test)]

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use graphite;

    #[bench]
    fn bench_handle_vcf(b: &mut Bencher) {
        b.iter(|| graphite::handle_vcf());
    }
}
