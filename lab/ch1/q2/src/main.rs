use std::backtrace::Backtrace;
use std::env;

fn main() {
    // 参见 https://doc.rust-lang.org/std/backtrace/index.html#environment-variables
    // 需要将环境变量RUST_LIB_BACKTRACE设置为非0，否则Backtrace::capture将永远不会捕获回溯
    env::set_var("RUST_LIB_BACKTRACE", "1");
    outer()
}

fn inner() {
    // 捕获调用栈链
    let backtrace = Backtrace::capture();
    println!("{:?}", backtrace);
}

fn middle() {
    inner()
}

fn outer() {
    middle()
}
