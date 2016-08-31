#include <iostream>
#include <string>
#include <iterator>

inline
unsigned char encode_byte_0(const unsigned char* mapping, const unsigned char* input_buf) {
  return mapping[ ( input_buf[0] >> 2 ) ];
}

inline
unsigned char encode_byte_1(const unsigned char* mapping, const unsigned char* input_buf) {
  // return mapping[ ( ((input_buf[0] & 0b0000_00_11) << 4) | (input_buf[1] >> 4) ) ];
  return mapping[ ( ((input_buf[0] & 0x03) << 4) | (input_buf[1] >> 4) ) ];
}

inline
unsigned char encode_byte_2(const unsigned char* mapping, const unsigned char* input_buf) {
  // return mapping[ ( ((input_buf[1] & 0b0000_11_11) << 2) | (input_buf[2] >> 6) ) ];
  return mapping[ ( ((input_buf[1] & 0x0f) << 2) | (input_buf[2] >> 6) ) ];
}

inline
unsigned char encode_byte_3(const unsigned char* mapping, const unsigned char* input_buf) {
  // return mapping[ ( input_buf[2] & 0b00_11_1111 ) ];
  return mapping[ ( input_buf[2] & 0x3f ) ];
}


void encode(std::istream& from, std::ostream& to, const unsigned char* mapping) {
  unsigned char input_buf[3];
  unsigned char output_buf[4];
  size_t input_index = 0;

  // std::istreambuf_iterator<char> it(from);
  // std::istreambuf_iterator<char> end = std::istreambuf_iterator<char>();

  char c;
  while (from.get(c)) {
    input_buf[input_index] = static_cast<unsigned char>(c);
    input_index += 1;
    if (input_index == 3) {
      output_buf[0] = encode_byte_0(mapping, &input_buf[0]);
      output_buf[1] = encode_byte_1(mapping, &input_buf[0]);
      output_buf[2] = encode_byte_2(mapping, &input_buf[0]);
      output_buf[3] = encode_byte_3(mapping, &input_buf[0]);
      to.write((char *)&output_buf[0], 4);
      input_index = 0;
    }
  }

  if (input_index == 1) {
    input_buf[1] = 0;
    output_buf[0] = encode_byte_0(mapping, &input_buf[0]);
    output_buf[1] = encode_byte_1(mapping, &input_buf[0]);
    output_buf[2] = static_cast<unsigned char>('=');
    output_buf[3] = static_cast<unsigned char>('=');
    to.write((char *)&output_buf[0], 4);
  } else if (input_index == 2) {
    input_buf[2] = 0;
    output_buf[0] = encode_byte_0(mapping, &input_buf[0]);
    output_buf[1] = encode_byte_1(mapping, &input_buf[0]);
    output_buf[2] = encode_byte_2(mapping, &input_buf[0]);
    output_buf[3] = static_cast<unsigned char>('=');
    to.write((char *)&output_buf[0], 4);
  }

  to.put('\n');
}

int main(int argc, char* argv[]) {
  static const unsigned char mapping[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  encode(std::cin, std::cout, mapping);
  return 0;
}
