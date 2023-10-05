use std::fs::File;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn main() {

    // sleep() 睡眠
    sleep(Duration::from_secs(5));

    // println!() 打印字符串
    // 这里的println!()中出现了!，说明这里是一个宏调用
    let message = "Hello World!";
    println!("{}", message);

    // 写入文件
    // .expect()方法是一个处理错误的简便方法
    // .expect()作用是在一个可能会产生错误的操作之后，立即检查该操作是否成功：如果操作成功，则.expect()方法将返回成功的结果；如果操作失败时就触发panic，并打印指定的错误信息
    // Rust变量默认是不可变的；使用mut修饰表示它是mutable，可变的
    let mut file = File::create("output.txt").expect("创建文件失败！");
    file.write_all(message.as_bytes()).expect("写入文件失败！");

}