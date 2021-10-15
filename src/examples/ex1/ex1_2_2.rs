// derive的Debug方式不能自定义输出
// 自定义输出需要实现fmt::Display方法
use std::fmt;

#[allow(dead_code)]
pub fn run() {
  struct Structure(i32);
  // 通过impl和for实现相关Display trait，实现自定义输出
  impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "fmt = {}", self.0)
    }
  }
  println!("{}", Structure(125));

  // 泛型默认没有提供Display的实现，只有Debug，但是可以自己实现
  #[derive(Debug)]
  struct MinMax(i32, i32);
  // 实现MinMax的Display，和自带的Debug进行比较
  impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{min} - {max}", min=self.0, max=self.1)
    }
  }
  println!("Display: \n{}", MinMax(0, 256));
  println!("Debug: \n{:#?}", MinMax(0, 65535));
}