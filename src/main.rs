use my_parser_practice_3::*;

pub fn main() {
      assert_eq!(list_parser::list("[1,1,2,3,5,8]"), Ok(vec![1, 1, 2, 3, 5, 8]));

      println!("{:?}", list_parser::list("[1,1,2,3,5,8]"));
  }