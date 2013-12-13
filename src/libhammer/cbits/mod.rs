pub mod cbits {
  use std::libc::{c_uint,c_char,size_t,FILE};

  pub struct HParser;
  pub struct HTokenData;
  pub struct HTokenType;
  pub struct HArena;

  pub struct HParsedToken {
    token_type: HTokenType,
    token_data: HTokenData,  
    index: size_t,
    bit_offset: char,
  }

  pub struct HParseResult {
    ast: *HParsedToken,
    bit_length: u64,
    arena: *HArena,
  }

  extern {
    pub fn h_token(str: *c_char, len: size_t) -> *HParser;
    pub fn h_ch(char: c_char) -> *HParser;
    pub fn h_ch_range(char: c_char, lower: c_char, upper: c_char) -> *HParser;
    pub fn h_int8() -> *HParser;

    pub fn h_ignore(p: *HParser) -> *HParser;
    pub fn h_in(chars: *c_char) -> *HParser;
    pub fn h_left(p: *HParser, q: *HParser) -> *HParser;
    pub fn h_right(p: *HParser, q: *HParser) -> *HParser;
    pub fn h_middle(p: *HParser, x: *HParser, q: *HParser) -> *HParser;
    pub fn h_sepBy1(p: *HParser, sep: *HParser) -> *HParser;
    pub fn h_many1(p: *HParser) -> *HParser;
    pub fn h_repeat_n(p: *HParser) -> *HParser;

    pub fn h_parse(parser: *HParser, t: *c_char, len: size_t) -> *HParseResult;

    pub fn h_pprint(file: *FILE, ast: *HParsedToken, indent: c_uint, delta: c_uint);
    pub fn h_parse_result_free(result: *HParseResult);
  }
}
