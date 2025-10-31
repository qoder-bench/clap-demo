# 新需求

为`main.rs`增加`login`子命令，主要用于用户登录，要求如下：

- 子命令名称为`login`
- 需要两个必填参数：`username`和`password`
- 子命令匹配后，打印出`User {username} logged in with password {password}`


# 编译优化

这是一个命令行程序，我希望编译后的可执行文件尽可能小，
请帮我优化`Cargo.toml`和`main.rs`，以减少最终生成的二进制文件大小。
