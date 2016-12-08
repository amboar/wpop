#![feature(test)]

extern crate test;
extern crate wpop;


#[cfg(test)]
mod tests {
    use test::Bencher;
    use wpop::radix;
    use wpop::hash;

    #[bench]
    fn bench_radix(b: &mut Bencher) {
        b.iter(|| radix::wpop("apples bananas oranges peaches bananas"));
    }

    #[bench]
    fn bench_hash(b: &mut Bencher) {
        b.iter(|| hash::wpop("apples bananas oranges peaches bananas"));
    }
}
