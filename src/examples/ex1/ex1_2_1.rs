// 格式化打印通过trait实现
// 标准库中所有类型都有相应的trait，可以直接打印
// derive推导宏#[derive(Debug)]，可以自动推导std中所有debug输出{:?}, 是std::fmt::Debug的实现
#[allow(dead_code)]
pub fn run() {
  // 结构体不经处理不能直接输出
  #[allow(dead_code)]
  struct UnPrintable(i32);
  // println!("{}", UnPrintable(5));

  #[derive(Debug)]
  struct DebugPrintable(i32);
  println!("{:?}", DebugPrintable(125));
  
  // 嵌套debug输出结构体
  #[derive(Debug)]
  struct Inner<'a> {
    name: &'a str,
    age: i32
  }
  #[derive(Debug)]
  struct Outer<'a>(Inner<'a>);

  println!("{:#?} \n{:?}", Outer(Inner {
    name: "test-1111",
    age: 12,
  }), Inner { name: "123", age: 12 });
}