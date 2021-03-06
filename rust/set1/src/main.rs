/// Convert hex to base64
/// The string:
///
///     49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
///
/// Should produce:
///
///     SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
///
/// So go ahead and make that happen. You'll need to use this code for the rest of the exercises.
///
/// Cryptopals Rule
/// Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.

use std::os;
use std::fmt;

static HEXMAP: &'static str = "0123456789ABCDEF";
static BASE64MAP: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
  let inp = os::args();
  let hexstr = inp[1].as_slice();
  let bs = ByteString::from_hex(hexstr);
  let result = bs.to_b64();

  if hexstr ==
    "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d" {
      assert_eq!(result,
      String::from_str("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
  }

  println!("Result:  {}", result)
}

#[deriving(PartialEq)]
struct Byte {
  value: int,
}

impl Byte {
  fn to_hex(&self) -> (char, char) {
    let lv = self.value >> 4;
    let rv = self.value & 0b00001111;
    let hmap = HEXMAP.as_slice();
    let lc = hmap.char_at(lv as uint);
    let rc = hmap.char_at(rv as uint);
    (lc, rc)
  }

  fn from_hex(inchar: (char, char)) -> Byte {
    let lc = inchar.val0().to_uppercase();
    let rc = inchar.val1().to_uppercase();
    let hmap = HEXMAP.as_slice();
    let lv = hmap.find(lc);
    let rv = hmap.find(rc);
    let result = (lv.unwrap() << 4) + (rv.unwrap());
    Byte { value: result as int }
  }
}

impl fmt::Show for Byte {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.value.fmt(f)
  }
}

#[deriving(PartialEq, Show)]
struct ByteString {
  bytes: Vec<Byte>,
}

impl ByteString {
  fn to_hex(&self) -> String {
    let mut hex_string = String::new();
    for b in self.bytes.iter() {
      let (l, r) = b.to_hex();
      hex_string.push(l);
      hex_string.push(r);
    }

    hex_string
  }

  fn from_hex(hexstr: &str) -> ByteString {
    let mut result = vec!();

    for pair in hexstr.as_bytes().chunks(2) {
      let val1:char = pair[0] as char;
      let val2:char = pair[1] as char;
      let val = Byte::from_hex((val1, val2));
      result.push(val)
    }

    ByteString { bytes: result }
  }

  fn to_b64(&self) -> String {
    let mut result = String::new();

    for trip in self.bytes.as_slice().chunks(3) {
      let t1 = trip[0].value;
      let t2 = trip[1].value;
      let t3 = trip[2].value;

      let v1 = t1 >> 2;
      let v2 = ((t1 & 0b00000011) << 4) + (t2 >> 4);
      let v3 = ((t2 & 0b00001111) << 2) + (t3 >> 6);
      let v4 = t3 & 0b00111111;

      let bmap = BASE64MAP.as_slice();
      result.push(bmap.char_at(v1 as uint));
      result.push(bmap.char_at(v2 as uint));
      result.push(bmap.char_at(v3 as uint));
      result.push(bmap.char_at(v4 as uint));
    }

    result
  }
}


/// Automated tests
/// Use 'cargo test' to run them

#[test]
fn test_hexmap_size() {
  if HEXMAP.len() != 16 {
    fail!("HEXMAP is the wrong size!");
  }
}

#[test]
fn test_b64map_size() {
  if BASE64MAP.len() != 64 {
    fail!("BASE64MAP is the wrong size!");
  }
}

#[test]
fn test_byte_fromhex_upcase() {
  let test = Byte::from_hex(('A', 'B'));
  if test.value != 171 {
    fail!("Byte::from_hex(('A', 'B')) should be value(171), not {}",
    test.value)
  }
}

#[test]
fn test_byte_fromhex_downcase() {
  let test = Byte::from_hex(('a', 'b'));
  if test.value != 171 {
    fail!("Byte::from_hex(('a', 'b')) should be value(171), not {}",
    test.value)
  }
}

#[test]
fn test_byte_fromhex_mixcase() {
  let test = Byte::from_hex(('A', 'b'));
  if test.value != 171 {
    fail!("Byte::from_hex(('A', 'b')) should be value(171), not {}",
    test.value)
  }
}

#[test]
fn test_to_hex_low() {
  let test_val = Byte { value: 9 };
  if test_val.to_hex() != ('0','9') {
    fail!("Byte{9}.to_hex() Should be ('0', '9')");
  }
}

#[test]
fn test_to_hex_high() {
  let test_val = Byte { value: 123 };
  if test_val.to_hex() != ('7','B') {
    fail!("Byte{123}.to_hex() Should be ('7', 'B')");
  }
}

#[test]
fn test_bs_to_hex() {
  let a = Byte { value: 171 };
  let b = Byte { value: 205 };
  let c = Byte { value: 239 };
  let test_bstring = ByteString {bytes: vec!(a, b, c)};
  let results = test_bstring.to_hex();
  if results != String::from_str("ABCDEF") {
    fail!("ByteString(171, 205, 239).to_hex() should be \"ABCDEF\", not {}",
    results);
  }
}

#[test]
fn test_bs_from_hex() {
  let teststr = "ABC123";
  let a = Byte { value: 171 };
  let b = Byte { value: 193 };
  let c = Byte { value: 35 };
  let expected = ByteString {bytes: vec!(a, b, c)};
  let results = ByteString::from_hex(teststr);
  if results != expected {
    fail!("hex 'ABC123' should result in ByteString(171, 193, 35)");
  }
}

#[test]
fn test_bs_tob64() {
  let a = Byte { value: 171 };
  let b = Byte { value: 193 };
  let c = Byte { value: 35 };
  let d = Byte { value: 171 };
  let e = Byte { value: 193 };
  let f = Byte { value: 35 };
  let test_bstring = ByteString {bytes: vec!(a, b, c, d, e, f)};
  let results = test_bstring.to_b64();
  if results != String::from_str("q8Ejq8Ej") {
    fail!("ByteString to Base64 should be \"q8Ejq8Ej\", not {}", results)
  }
}

