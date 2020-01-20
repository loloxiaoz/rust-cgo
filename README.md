# Golang调用Rust代码

将rust中组合子示例函数封装成动态库，并在golang中调用

## 1 Rust 生成c语言头文件和动态库  

为 Rust 库编写[FFI](https://doc.rust-lang.org/nomicon/ffi.html)(Foreign Function Interface)

### 包装Rust库

由于rust类型无法直接导出成c语言识别的数据类型, 因此需要将数据类型和函数声明重新包装

```Rust
#[repr(C)]
pub struct PairStr{
    left : *const c_char,
    right : *const c_char,
}

#[no_mangle]
pub extern fn space_find_export(input: *const c_char) -> PairStr {

}
```

### 使用cbindgen自动导出h文件

使用[cbindgen](https://github.com/eqrion/cbindgen)工具根据rust代码自动生成c语言头文件
> cbindgen has a simple but effective strategy. It walks through your crate looking for:  
1、#[no_mangle] pub extern fn ("functions")  
2、#[no_mangle] pub static ("globals")  
3、pub const ("constants")  

### 使用步骤

1. yum install clang  
2. cargo install --force cbindgen
3. 在Cargo.toml中添加依赖

```shell
[build-dependencies]
cbindgen = "0.12.2"
```

4. 在项目中添加cbindgen.toml文件

```shell
include_guard = "HYPER_H"
autogen_warning = "/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */"
language = "C"
```

5. 添加build.rs脚本，用于生成c语言头文件hyper.h, 执行cargo build命令，Rust的OUT_DIR将会包含hyper.h  

## 2 Go 调用 C

go语言自带[cgo](https://chai2010.cn/advanced-go-programming-book/ch2-cgo/readme.html)工具支持c语言的函数调用，但需要安装gcc，并且开启CGO_ENABLED  
在go的源代码中把C代码作为注释来写，并标明依赖的库文件和路径，最后使用import "C"

```golang
import "C"
/*
#cgo LDFLAGS: -L./lib -lhyper
#include "./lib/hyper.h"
*/
func main() {
    C.space_find_export(C.CString("hello world"))
}
```

## 3 运行

编译rust程序  + 生成c头文件和动态库 + 编译运行go程序

```shell
make run
```

## 注意事项

1. Rust中所有权问题
2. 内存安全, Go语言和C语言内存模型存在差异
