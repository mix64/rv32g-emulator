main:
    addi x6, x0, 123
    addi x7, x0, 456
    mul  x10, x6, x7    # x10 = 123*456 = 0xdb18 (56088)
    mulh x11, x6, x7    # x11 = 0x0

    add x6, x0, x10
    add x7, x0, x6
    mul x12, x6, x7     # x12 = 0xdb18*0xdb18 = 0xbb821240
    mulh x13, x6, x7    # x13 = 0

    # 0xbb821240 -> -1149103552
    # hex(-1149103552*-1149103552) = 0x12532440_0a4d1000
    # hex(0xbb821240*0xbb821240)   = 0x895748c0_0a4d1000
    add x6, x0, x12
    add x7, x0, x12
    mul x14, x6, x7     # x14 = 0x0a4d1000
    mulh x15, x6, x7    # x15 = 0x12532440 (signed * signed)
    mulhu x16, x6, x7   # x16 = 0x895748c0 (unsigned * signed)

    addi x6, x0, -123
    addi x7, x0, 456
    mul x17, x6, x7     # x17 = -56088
    mulhsu x18, x6, x7  # x18 = 0xffffffff

    addi x6, x0, -123
    addi x7, x0, 456
    div x18, x6, x7     # x18 = 0
    divu x19, x6, x7    # x19 = 0x008fb823
    rem x20, x6, x7     # x20 = -123
    remu x21, x6, x7    # x21 = 0x12d

    addi x6, x0, -123
    addi x7, x0, 0
    div x22, x6, x7     # x22 = 0xffffffff
    divu x23, x6, x7    # x23 = 0xffffffff
    rem x24, x6, x7     # x24 = -123
    remu x25, x6, x7    # x25 = 0xffffff85
    
    addi x6, x0, 1
    sll x26, x6, 31
    add x6, x0, x26
    addi x7, x0, -1
    div x26, x6, x7     # x26 = 0x80000000
    rem x27, x6, x7     # x27 = 0

