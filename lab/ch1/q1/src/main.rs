use std::fs;

fn main() {
    // match是一种模式匹配语法，有点类似于其他语言的switch，但更强大灵活
    let curr_dir = match fs::read_dir(".") {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("当前目录读取失败！错误详情：{}", err);
            return;
        }
    };

    // rust的`for in `类似java的`for : `
    for item in curr_dir {
        if let Ok(item) = item {
            // 获取文件元信息
            let metadata = fs::metadata(item.path()).expect("获取目标元信息失败！");

            // 判断是目录还是文件
            // 这里的写法显然比其他流行语言如java等更漂亮
            let file_type = if metadata.is_dir() {
                "d"
            } else {
                "-"
            };

            // 获取文件大小（字节数）
            let size = metadata.len();

            // 获取文件名
            let file_name = item.file_name();

            // 左对齐输出
            println!("{:<3} {:<6} {}", file_type, size, file_name.to_string_lossy());
        }
    }
}
