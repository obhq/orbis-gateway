SECTIONS {
    .text 0 : ALIGN(0x4000) {
        *(.text.entry)
    }

    .dynamic : ALIGN(0x4000) {
        *(.dynamic)
    }

    .text : ALIGN(0x4000) {
        *(.text.*)
    }

    .rodata : ALIGN(0x4000) {
        *(.rela.dyn)
    }

    /DISCARD/ : {
        *(.dynsym)
        *(.gnu.hash)
        *(.hash)
        *(.dynstr)
    }
}
