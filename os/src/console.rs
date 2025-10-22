//! SBI console driver, for text output
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout; 
// 这是什么用法：
// 实现了 fmt::Write trait 之后，Stdout 就可以作为一个输出目标
// 空结构体 Stdout 代表标准输出设备
// 一般而言在Rust中，空结构体用于表示没有数据但具有某种行为或特性的类型

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}
// s是一个字符串切片（&str），表示要写入标准输出的内容。s.chars() 方法将字符串切片转换为一个字符迭代器，允许我们逐个访问字符串中的每个字符。
// Result<(), fmt::Error> 是 fmt 模块中定义的一个类型别名，表示写入操作的结果。如果写入成功，返回 Ok(())；如果发生错误，返回 Err(fmt::Error)。
// Write trait 定义了一个接口，允许我们将数据写入某个输出目标。在这个实现中，我们实现了 write_str 方法，该方法接受一个字符串切片，并将其逐字符写入标准输出设备。
// Write是从core::fmt模块引入的一个trait，定义了写入字符串的方法。通过实现这个trait，我们可以将Stdout作为一个输出目标，用于格式化输出。
// core::fmt模块提供了格式化输出的功能，包括各种宏和trait，用于实现自定义的输出行为。
// 我们会在下列情况使用core::fmt模块：
// 实现自定义的输出目标，例如文件、网络连接或控制台。
// 创建自定义的格式化输出宏。

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
    // unwrap() 用于处理可能的错误情况。如果写入过程中发生错误，程序会在这里panic。
    // 在内核开发中，通常假设写入操作不会失败，因此使用unwrap()来简化错误处理逻辑。
    // 具体来说，unwrap通过检查Result类型的值：
    // 如果是Ok变体，unwrap会返回其中包含的值。
    // 如果是Err变体，unwrap会引发panic，终止程序的执行
    // write_fmt是fmt::Write trait中的一个方法，用于将格式化的字符串写入输出目标。
    // 这也是我们使用Write trait的原因，因为它提供了write_fmt方法，可以方便地进行格式化输出。
    // fmt::Arguments是一个结构体，表示格式化输出的参数。它包含了要输出的字符串以及相关的格式化信息。
    // 这样我们的使用就类似于标准库中的println!宏，但这里是针对内核环境下的控制台输出实现的。
}

/// Print! to the host console using the format string and arguments.
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}
// $fmt: literal 表示宏的第一个参数是一个字符串字面量，代表格式化字符串。
// $(, $($arg: tt)+)? 是一个可选的参数匹配模式

/// Println! to the host console using the format string and arguments.
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}

// 这个宏与print!宏类似，但它在格式化字符串的末尾添加了一个换行符。
// concat!宏用于将多个字符串字面量连接在一起，这里将用户提供的格式化字符串与换行符连接。
// 这样，println!宏在输出内容后会自动换行，类似于标准库中的println!宏。
