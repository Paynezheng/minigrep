## minigrep

 `cargo run A B`.
 其中A为搜索的字段，B为待搜索文件。
 `cargo run A B > output.txt`.
 结果(标准输出)保留在文件中，而其他信息(标准错误)输出到命令行.
 设置环境变量`CASE_INSENSITIVE=1`，`minigrep`会忽略大小写执行搜索.
