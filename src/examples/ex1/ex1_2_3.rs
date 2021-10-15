// 通过?处理所有值的Display，而不是一个个处理
// ？是try! trait的简写，当发生错误时可以抛出错误。否则继续执行
use std::fmt;

#[allow(dead_code)]
pub fn run() {
  struct List(Vec<i32>);

  impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let vec = &self.0;
      write!(f, "[")?;
      for (count, v) in vec.iter().enumerate() {
        if count != 0 {
          write!(f, ", ")?;
        }
        write!(f, "{}", v)?;
      }

      write!(f, "]")
    }
  }

  let v = List(vec![12, 35, 68]);
  println!("{}", v);
}