#![feature(test)]

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use flex_vg;

    #[bench]
    fn bench_handle_vcf(b: &mut Bencher) {

        

        let mut vcf_reader = flex_vg::open_vcf(
            "/Users/mmwaniki/data/mouse_mm10/C57BL/4512-JFI-0333_C57BL_6J_two_lanes_large_svs.vcf",
        );
        b.iter(|| flex_vg::gen_variations(&mut vcf_reader));
    }
}
