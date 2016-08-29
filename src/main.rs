use std::io;
use std::env;
extern crate getopts;
use getopts::Options;

fn encode() {
  // While try to read 3 bytes:
  //   - Ok => Encode 3 bytes
  //   - 1 Byte is read:
  //     - Set remaining 2 bytes to zero
  //     - Encode 3 bytes
  //     - Add '==' to output
  //   - 2 Bytes are read
  //     - Set remaining 1 byte to zero
  //     - Encode 3 bytes
  //     - Add '=' to output
  println!("Encode base64.");
}

fn decode() {
  println!("Decode base64.");
}

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optflag("d", "decode", "decode base64");
  opts.optflag("h", "help", "print this message");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
    print_usage(&program, opts);
    return;
  }

  if matches.opt_present("decode") {
    decode();
  } else {
    encode();
  }
}
