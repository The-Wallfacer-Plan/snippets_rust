extern crate memmap;

fn main() {
    use std::io::Write;
    use memmap::{Mmap, Protection};

    let file_mmap = Mmap::open_path("README.md", Protection::Read).unwrap();
    let bytes: &[u8] = unsafe { file_mmap.as_slice() };
    assert_eq!(b"# memmap", &bytes[0..8]);

    let mut anon_mmap = Mmap::anonymous(4096, Protection::ReadWrite).unwrap();
    unsafe { anon_mmap.as_mut_slice() }.write_all(b"foo").unwrap();
    assert_eq!(b"foo\0\0", unsafe { &anon_mmap.as_slice()[0..5] });

}