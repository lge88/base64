use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::result;
use std::env;
extern crate getopts;
use getopts::Options;

fn encode_byte_0(mapping: &[u8], input_buf: &[u8; 3]) -> u8 {
  mapping[ ( input_buf[0] >> 2 ) as usize ]
}

fn encode_byte_1(mapping: &[u8], input_buf: &[u8; 3]) -> u8 {
  mapping[ ( ((input_buf[0] << 6) >> 2) | (input_buf[1] >> 4) ) as usize ]
}

fn encode_byte_2(mapping: &[u8], input_buf: &[u8; 3]) -> u8 {
  mapping[ ( ((input_buf[1] << 4) >> 2) | (input_buf[2] >> 6) ) as usize ]
}

fn encode_byte_3(mapping: &[u8], input_buf: &[u8; 3]) -> u8 {
  mapping[ ( input_buf[2] & 0b00_11_1111 ) as usize ]
}

fn encode<From: Read, To: Write>(
  mut from: From, mut to: To, mapping: &[u8]) -> std::io::Result<()> {
  // Foreach 3 bytes read:
  //   - Encode these 3 bytes to 4 base64 bytes
  //   - Write to stdout
  //
  // If there are bytes remaining in input buffer:
  //   - 1 Byte was read:
  //     - Set remaining 2 bytes to zero
  //     - Encode these 3 bytes
  //     - Add '==' to output
  //   - 2 Bytes were read
  //     - Set remaining 1 byte to zero
  //     - Encode these 3 bytes
  //     - Add '=' to output
  let mut input_buf: [u8; 3] = [0; 3];
  let mut output_buf: [u8; 4] = [0; 4];
  let mut input_index: usize = 0;
  let pad_one = "=".as_bytes();
  let pad_two = "==".as_bytes();
  let new_line = "\n".as_bytes();

  for byte in from.bytes() {
    input_buf[input_index] = byte.unwrap();
    input_index += 1;
    if input_index == 3 {
      output_buf[0] = encode_byte_0(mapping, &input_buf);
      output_buf[1] = encode_byte_1(mapping, &input_buf);
      output_buf[2] = encode_byte_2(mapping, &input_buf);
      output_buf[3] = encode_byte_3(mapping, &input_buf);
      try!(to.write(&output_buf));
      input_index = 0;
    }
  }

  // https://tools.ietf.org/html/rfc4648#section-4
  if input_index == 1 {
    input_buf[1] = 0;
    output_buf[0] = encode_byte_0(mapping, &input_buf);
    output_buf[1] = encode_byte_1(mapping, &input_buf);
    try!(to.write(&output_buf[0..2]));
    try!(to.write(&pad_two));
  } else if input_index == 2 {
    input_buf[2] = 0;
    output_buf[0] = encode_byte_0(mapping, &input_buf);
    output_buf[1] = encode_byte_1(mapping, &input_buf);
    output_buf[2] = encode_byte_2(mapping, &input_buf);
    output_buf[3] = encode_byte_3(mapping, &input_buf);
    try!(to.write(&output_buf[0..3]));
    try!(to.write(&pad_one));
  }

  try!(to.write(&new_line));
  Ok(())
}

fn decode() {
  println!("Decode base64.");
}

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

fn main() {
  let mut base64_mapping: [u8; 64] = [0; 64];
  base64_mapping[..]
    .clone_from_slice("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes());

  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optflag("d", "decode", "decode base64");
  opts.optflag("u", "url-safe", "use url safe mapping");
  opts.optflag("h", "help", "print this message");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(err) => { panic!(err.to_string()) }
  };

  if matches.opt_present("h") {
    print_usage(&program, opts);
    return;
  }

  if matches.opt_present("url-safe") {
    base64_mapping[62] = '-' as u8;
    base64_mapping[63] = '_' as u8;
  }

  if matches.opt_present("decode") {
    decode();
  } else {
    encode(stdin(), stdout(), &base64_mapping);
  }
}
