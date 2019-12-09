# clippy工作原理

clippy的仓库在 https://github.com/rust-lang/rust-clippy
看根目录下的Cargo.toml可知，该工程编译之后主要是两个bin(cargo-clippy/clippy-driver)。

cargo-clippy只是最外层一个命令入口。将clippy-driver作为rustc wrapper传递给cargo。
clippy-driver实现为一个rustc_driver。这是rustc的一种扩展机制，可以为编译器写插件，实现一些功能。
当然clippy这里主要是lint功能。rustc内置了一些lint功能，也是通过rustc_driver的形式来实现的。

主要是实现了rustc_driver::Callbacks这个trait。

```
struct ClippyCallbacks;

impl rustc_driver::Callbacks for ClippyCallbacks {
    fn after_parsing(&mut self, compiler: &interface::Compiler) -> bool {
```

然后在after_parsing函数中注册各种lint的插件。

```
clippy_lints::register_plugins(&**mut **registry, &conf);
```

所以有用的代码其实主要在clippy_lints目录下。
可以在clippy_lints/src/[lib.rs](http://lib.rs/)中找到上述注册函数。
里面会将每个lint pass注册进来。
实际增加lint pass的时候不用手工修改这个文件。
util/dev 脚本可以自动做这个事情。

这里的pass又分两类：EarlyLintPass 和 LateLintPass。
区别是前者是在解析AST(抽象语法树)之后调用的，只能访问AST的信息；后者是在类型检查之后调用的，可以访问类型信息。

Lint规则本身的书写还是比较简单的，就像正则表达式一样，匹配各种模式，然后给出对应的告警或者建议信息。
当然比正则表达式还是要复杂一些的，用到了很多编译器内部的数据结构，需要参考rustc的代码。
从clippy入手是一个学习Rust比较好的途径。
参见 https://llogiq.github.io/2015/06/04/workflows.html

感觉现在Lint规则的书写人为因素太大。很多建议也不是很靠谱。
考虑一种方式，先编译现有代码到ir，然后反编译出生成同样ir的最简洁的代码。
将反编译出的代码作为建议代码给用户。
