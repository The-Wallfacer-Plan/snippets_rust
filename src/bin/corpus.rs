#[macro_use]
extern crate nom;
extern crate regex;

use nom::*;
use std::str;
use std::str::FromStr;
use std::u8;

named!(key< (String, Option<u8>) >,
      map! (re_bytes_capture!(r"([[:alpha:]][[:word:]]*)(?:(?:@)(\d+))?"),
      |v| {
          let key_raw = String::from(str::from_utf8(v[1]).unwrap());
          let level = if v.len() == 3 {
              Some(u8::from_str(str::from_utf8(v[2]).unwrap()).unwrap())
          } else {
              None
          };
          (key_raw, level)
      }
      )
);

named!(
    hex_part< Vec<u8> >, 
    many1!(map_res!(map_res!(preceded!(tag!("\\x"), take!(2)), str::from_utf8), |src| u8::from_str_radix(src, 16)))
    );

named!(char_part< Vec<u8> >,
    map!(
        call!(alpha),
        |s|{
            let mut v = Vec::new();
            v.extend_from_slice(s);
            v
        }
       ));

named!(value < Vec<u8> >,
      delimited!(
          tag!("\""),
          fold_many1!( alt!(hex_part | char_part), Vec::new(), |mut acc: Vec<_>, ref item: Vec<_>|{
           acc.extend_from_slice(&item);
           acc
       } ),
       tag!("\"")
      ));

named!(kv <&[u8], (String, Option<u8>, Vec<u8>)  >, 
       do_parse!(
           k: call!(key) >>
           ws!(tag!("=")) >>
           v: ws!(call!(value)) >>
           ((k.0, k.1, v))
                )
       );

named!(
    comment< &str >,
    map_res!(preceded!(tag!("#"), ws!(call!(not_line_ending))), str::from_utf8)
);

fn test_hex_str() {

    let k = key(&b"aA@11"[..]);
    println!("k={:?}", k);
    let k2 = key(&b"aA"[..]);
    println!("k2={:?}", k2);

    let h = hex_part(&b"\\xff\\x01"[..]);
    println!("h={:?}", h);

    let c = comment(&b"# fdfsdfdf 11"[..]);
    println!("c={:?}", c);

    let ch = char_part(&b"ab\\x"[..]);
    println!("ch={:?}", ch);

    let v = value(&"\"\\x01\\xffABC\\x0a\\x1a\\x0a\"".as_bytes()[..]);
    println!("v={:?}", v);

    let kv = kv(
        &r#"header_png@2 = "\x89PNG\x0d\x0a\x1a\x0a""#.as_bytes()[..],
    );
    println!("kv={:?}", kv);
}

fn main() {
    test_hex_str();
}
