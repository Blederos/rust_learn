fn main() {
    print!("hello ");                // 不带换行打印
    let s1 = String::from("world");
    let s2 = String::from("!");
    eprintln!("error message");       // 打印到标准错误
    let message = format!("{} {}", s1, s2);  // 格式化字符串，不打印
    println!("{}", message);          // 打印拼接后的字符串
}