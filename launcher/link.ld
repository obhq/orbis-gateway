SECTIONS {
    .text 0 : {
        *(.text.entry)
    }

    .dynamic 0x20 : {
        *(.dynamic)
    }

    .rodata : ALIGN(8) {
        *(.rela.dyn)
    }

    .text : {
        *(.text.*)
    }

    /DISCARD/ : {
        *(.dynsym)
        *(.gnu.hash)
        *(.hash)
        *(.dynstr)
    }
}
