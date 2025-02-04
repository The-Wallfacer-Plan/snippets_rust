use slotmap::SecondaryMap;
use slotmap::SlotMap;

fn main() {
    let mut sm = SlotMap::new();
    let foo = sm.insert("foo");
    let bar = sm.insert("bar");
    assert_eq!(sm[foo], "foo");
    assert_eq!(sm[bar], "bar");

    sm.remove(bar);
    let reuse = sm.insert("reuse"); // Space from bar reused.
    assert_eq!(sm.contains_key(bar), false); // After deletion a key stays invalid.

    let mut sec = SecondaryMap::new();
    sec.insert(foo, "noun"); // We provide the key for secondary maps.
    sec.insert(reuse, "verb");

    for (key, val) in sm {
        println!("key={:?}, {} is a {}", key, val, sec[key]);
    }
}
