#![feature(test)]
extern crate test;

extern crate bytecount;

fn my_bytecount1(arr: &[u8]) -> usize {
    const v: u8 = 0;
    let mut size: usize = 0;
    for b in arr {
        if *b != v {
            size += 1;
        }
    }
    size
}

fn my_bytecount2(arr: &[u8], v: u8) -> usize {
    let mut size: usize = 0;
    for b in arr {
        if *b != v {
            size += 1;
        }
    }
    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytecount() {
        let bytes = [3; 65536];
        let size = bytecount::count(&bytes, b' ');
        assert_eq!(size, 0usize);
    }

    #[bench]
    fn bench_bytecount(b: &mut test::Bencher) {
        let bytes = [3; 65536];
        b.iter(|| bytecount::count(&bytes, 0u8));
    }

    #[bench]
    fn bench_my_bytecount1(b: &mut test::Bencher) {
        let bytes = [3; 65536];
        b.iter(|| my_bytecount1(&bytes));
    }

    #[bench]
    fn bench_bytecount_naive(b: &mut test::Bencher) {
        let bytes = [3; 65536];
        b.iter(|| bytecount::naive_count(&bytes, 0u8));
    }

    #[bench]
    fn bench_my_bytecount2(b: &mut test::Bencher) {
        let bytes = [3; 65536];
        b.iter(|| my_bytecount2(&bytes, 0u8));
    }
}