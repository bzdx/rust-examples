pub fn run() {
  // 占位符
  println!("{} 天", 30i64);
  // 索引参数
  println!("param0 = {0}; param1 = {1}", "test0", "test1");
  // 命名参数
  println!("name:{name}, \nage: {age}", name="bzdx", age="35");
  // 指定数字格式
  println!("八进制：{:o};\n二进制：{:b}", 12, 5);
  // 指定对齐宽度
  println!("{:>width$}", 1, width=5);
  println!("{number:>width$}", number=2, width=5);
  println!("{num:>0width$}", num=3, width=5);

  // 结构体不能直接输出
  #[allow(dead_code)] // 去掉提示
  struct Structure(i32);
  // println!("结构体不能直接输出: {}", Structure(3))
}