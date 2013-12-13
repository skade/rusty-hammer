pub mod hammer {
  use cbits::cbits::{h_many1,h_repeat_n,h_left,h_right,h_middle,h_sepBy1,h_token,h_ch,h_ch_range,h_int8,h_parse,h_pprint,h_parse_result_free,h_ignore,HParser,HParseResult};
  use std::libc::{STDOUT_FILENO, c_int, fdopen};
  use std::cast::transmute;

  pub struct Parser {
    parser: *HParser,
  }

  pub struct ParseResult {
    result: *HParseResult,
  }

  pub fn token(str: ~str) -> Parser {
    str.with_c_str(|c_string| {
      unsafe { Parser { parser: h_token(c_string, str.len().to_u64().unwrap()) } }
    })
  }

  pub fn within(str: ~str) -> Parser {
    str.with_c_str(|c_string| {
      unsafe { Parser { parser: h_token(c_string, str.len().to_u64().unwrap()) } }
    })
  }

  pub fn left(p: Parser, q: Parser) -> Parser {
    unsafe { Parser { parser: h_left(p.parser, q.parser) } }
  }

  pub fn right(p: Parser, q: Parser) -> Parser {
    unsafe { Parser { parser: h_right(p.parser, q.parser) } }
  }

  pub fn middle(p: Parser, x: Parser, q: Parser) -> Parser {
    unsafe { Parser { parser: h_middle(p.parser, x.parser, q.parser) } }
  }

  pub fn sepBy1(p: Parser, sep: Parser) -> Parser {
    unsafe { Parser { parser: h_sepBy1(p.parser, sep.parser) } }
  }

  pub fn repeat_n(p: Parser) -> Parser {
    unsafe { Parser { parser: h_repeat_n(p.parser) } }
  }

  pub fn many1(p: Parser) -> Parser {
    unsafe { Parser { parser: h_many1(p.parser) } }
  }

  pub fn ch(char: i8) -> Parser {
    unsafe { Parser { parser: h_ch(char) } }
  }

  pub fn ch_range(char: i8, lower: i8, upper: i8) -> Parser {
    unsafe { Parser { parser: h_ch_range(char, lower, upper) } }
  }

  pub fn int8() -> Parser {
    unsafe { Parser { parser: h_int8() } }
  }

  impl Parser {
    pub fn parse(&self, t: ~[u8]) -> Option<ParseResult> {
      unsafe {
        t.with_c_str(|c_string| {
          let result = h_parse(self.parser, c_string, t.len().to_u64().unwrap());
          
          if result.is_null() {
            None
          } else {
            Some(ParseResult { result: result })
          }
        })
      }
    }
  }

  impl ParseResult {
    pub fn print(&self, indent: u32, delta: u32) {
      unsafe {
        let mode = "w";
        let stdout = fdopen(STDOUT_FILENO as c_int, transmute(&mode[0]));

        let result = *self.result;

        h_pprint(stdout, result.ast, indent, delta);
      }
    }
  }

  impl Drop for ParseResult {
    fn drop(&mut self) {
      unsafe { h_parse_result_free(self.result) }
    }
  }

  #[cfg(test)]
  mod tests {
    use super::{int8,within,token,sepBy1,repeat_n,many1};
    use std::io::{File,Path};

    #[test]
    fn simple_integer_parser() {
      let parser = int8();
      let str = ~"3";
      let result = parser.parse(str.into_bytes());
      result.unwrap().print(0, 0);
    }

    #[test]
    fn parse_delimiter() {
      let parser = token(~", ");
      let str = ~", ";
      let result = parser.parse(str.into_bytes());

      result.unwrap().print(0, 0);
    }

    #[test]
    fn parse_name() {
      let chars = within(~"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz$");
      let parser = many1(chars);
      let str = ~"A";
      let result = parser.parse(str.into_bytes());

      result.unwrap().print(0, 0);
    }

    #[test]
    fn parse_actorname() {
      let name = within(~"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz$");
      let sep = token(~", ");

      let parser = sepBy1(name, sep);
      let str = ~"$, Claw";
      let result = parser.parse(str.into_bytes());

      result.unwrap().print(0, 0);
    }
  }
}
