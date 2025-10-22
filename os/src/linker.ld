OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

# 这样一来，_start的地址就是0x80200000


# 后面的每一个位置都给出具体地址
SECTIONS
{
    . = BASE_ADDRESS;  # BASE_ADDRESS: 0x80200000
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)          # entry code
        *(.text .text.*)        # normal code
        # .text.* means all sections whose name starts with .text
        # 也就是说我们指定.entry一定在开头，然后其他的子程序我们允许它自己调度如何跳转，对吗？
    }

    . = ALIGN(4K);              # 对齐到4K
    etext = .;                   
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}