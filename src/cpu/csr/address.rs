// User Trap Setup
// User status register.
pub const USTATUS: usize = 0x000;
// User interrupt-enable register.
pub const UIE: usize = 0x004;
// User trap handler base address.
pub const UTVEC: usize = 0x005;

// User Trap Handling
// Scratch register for user trap handlers.
pub const USCRATCH: usize = 0x040;
// User exception program counter.
pub const UEPC: usize = 0x041;
// User trap cause.
pub const UCAUSE: usize = 0x042;
// User bad address or instruction
pub const UTVAL: usize = 0x043;
// User interrupt pending.
pub const UIP: usize = 0x044;

// User Floating-Point CSRs
// Floating-Point Accrued Exceptions.
pub const FFLAGS: usize = 0x001;
// Floating-Point Dynamic Rounding Mode.
pub const FRM: usize = 0x002;
// Floating-Point Control and Status Register (frm+fflags)
pub const FCSR: usize = 0x003;

// User Counter/Timers (Read-Only)
// Cycle counter for RDCYCLE instruction.
pub const CYCLE: usize = 0xC00;
// Timer for RDTIME instruction.
pub const TIME: usize = 0xC01;
// Instructions-retired counter for RDINSTRET instruction.
pub const INSTRET: usize = 0xC02;
// Performance-monitoring counter.
pub const HPMCOUNTER3: usize = 0xC03;
// hpmcounter[3,31] = [0xC03,0xC1F]
pub const HPMCOUNTER31: usize = 0xC1F;
// Upper 32bits of Counter/Timers, RV32I only.
pub const CYCLEH: usize = 0xC80;
pub const TIMEH: usize = 0xC81;
pub const INSTRETH: usize = 0xC82;
pub const HPMCOUNTER3H: usize = 0xC83;
// hpmcounter[3,31]h = [0xC83,0xC9F]
pub const HPMCOUNTER31H: usize = 0xC9F;

// Supervisor Trap Setup
// Supervisor status register.
pub const SSTATUS: usize = 0x100;
// Supervisor exception delegation register.
pub const SEDELEG: usize = 0x102;
// Supervisor interrupt delegation register.
pub const SIDELEG: usize = 0x103;
// Supervisor interrupt-enable register.
pub const SIE: usize = 0x104;
// Supervisor trap handler base address.
pub const STVEC: usize = 0x105;
// Supervisor counter enable.
pub const SCOUNTEREN: usize = 0x106;

// Supervisor Trap Handling
// Scratch register for supervisor trap handlers.
pub const SSCRATCH: usize = 0x140;
// Supervisor exception program counter.
pub const SEPC: usize = 0x141;
// Supervisor trap cause.
pub const SCAUSE: usize = 0x142;
// Supervisor bad address or instruction.
pub const STVAL: usize = 0x143;
// Supervisor interrupt pending.
pub const SIP: usize = 0x144;

// Supervisor Protection and Translation
// Supervisor address translation and protection.
pub const SATP: usize = 0x180;

// Machine Information Registers
// Vendor ID.
pub const MVENDORID: usize = 0xF11;
// Architecture ID.
pub const MARCHID: usize = 0xF12;
// Implementation ID.
pub const MIMPID: usize = 0xF13;
// Hardware thread ID.
pub const MHARTID: usize = 0xF14;

// Machine Trap Setup
// Machine status register.
// Restricted views of the "mstatus" appear as the "sstatus" and "ustatus" (3.1.6)
pub const MSTATUS: usize = 0x300;
// ISA and extensions.
pub const MISA: usize = 0x301;
// Machine exception delegation register.
pub const MEDELEG: usize = 0x302;
// Machine interrupt delegation register.
pub const MIDELEG: usize = 0x303;
// Machine interrupt-enable register.
// Restricted views of the "mie" appear as the "sie" and "uie" (3.1.6)
pub const MIE: usize = 0x304;
// Software Interrupt Enable
pub const MIE_USIE: u32 = 0b1;
pub const MIE_SSIE: u32 = 0b1 << 1;
pub const MIE_MSIE: u32 = 0b1 << 3;
// Timer Interrupt Enable
pub const MIE_UTIE: u32 = 0b1 << 4;
pub const MIE_STIE: u32 = 0b1 << 5;
pub const MIE_MTIE: u32 = 0b1 << 7;
// External Interrupt Enable
pub const MIE_UEIE: u32 = 0b1 << 8;
pub const MIE_SEIE: u32 = 0b1 << 9;
pub const MIE_MEIE: u32 = 0b1 << 11;
/*
    (3.1.7) Machine trap-handler base address.

    31     2 1     0
     | BASE | MODE |
     MODE    Name        Description
       0    Direct      All exceptions set pc to BASE.
       1    Vectored    Asynchronous interrupts set pc to BASE+4×cause.
*/
pub const MTVEC: usize = 0x305;
// Machine counter enable.
pub const MCOUNTEREN: usize = 0x306;

// Machine Trap Handling
// Scratch register for machine trap handlers.
pub const MSCRATCH: usize = 0x340;
/*
    (3.1.15) Machine exception program counter. (WARL)

    On implementations that support only IALIGN=32, the two low bits (mepc[1:0]) are always zero.
    When a trap is taken into M-mode, mepc is written with the virtual address of the instruction
    that was interrupted or that encountered the exception.
    Otherwise, mepc is never written by the implementation, though it may be explicitly written by software.
*/
pub const MEPC: usize = 0x341;
/*
    (3.1.16) Machine Cause Register

    When a trap is taken into M-mode, mcause is written with a code indicating the event that caused the trap.
    Otherwise, mcause is never written by the implementation, though it may be explicitly written by software.

    The Interrupt bit in the mcause register is set if the trap was caused by an interrupt.
    The Exception Code field contains a code identifying the last exception.
    The Exception Code is a WLRL field, so is only guaranteed to hold supported exception codes.

    Priority of Exception (Table 3.7)
*/
// Machine trap cause.
pub const MCAUSE: usize = 0x342;
// Machine bad address or instruction.
pub const MTVAL: usize = 0x343;
/*
    (3.1.9) Machine Interrupt Registers

    Only the bits corresponding to lower-privilege software interrupts (USIP, SSIP), timer interrupts (UTIP, STIP),
    and external interrupts (UEIP, SEIP) in mip are writable through this CSR address; the remaining bits are read-only.

    The machine-level MSIP bits are written by accesses to memory-mapped control registers, which are used by
    remote harts to provide machine-mode interprocessor interrupts.
    Interprocessor interrupts for lower privilege levels are implemented through implementation-specific mechanisms,
    e.g., via calls to an AEE or SEE, which might ultimately result in a machine-mode write to the receiving hart’s MSIP bit.

    By default, M-mode interrupts are globally enabled if the hart’s current privilege mode is less than M,
    or if the current privilege mode is M and the MIE bit in the mstatus register is set.
    If bit i in mideleg is set, however, interrupts are considered to be globally enabled if the hart’s
    current privilege mode equals the delegated privilege mode (S or U) and that mode’s interrupt enable bit (SIE or UIE) is set,
    or if the current privilege mode is less than the delegated privilege mode.

    Multiple simultaneous interrupts destined for the same privilege mode are handled in the following decreasing priority order:
    MEI, MSI, MTI, SEI, SSI, STI, UEI, USI, UTI.
    Synchronous exceptions are of lower priority than all interrupts.
*/
// Machine interrupt pending.
// Restricted views of the "mip" appear as the "sip" and "uip" (3.1.6)
pub const MIP: usize = 0x344;
// Software Interrupt (rw-rw-ro)
pub const MIP_USIP: u32 = 0b1;
pub const MIP_SSIP: u32 = 0b1 << 1;
pub const MIP_MSIP: u32 = 0b1 << 3;
// Timer Interrupt (rw-rw-ro)
pub const MIP_UTIP: u32 = 0b1 << 4;
pub const MIP_STIP: u32 = 0b1 << 5;
pub const MIP_MTIP: u32 = 0b1 << 7;
// External Interrupt
pub const MIP_UEIP: u32 = 0b1 << 8;
pub const MIP_SEIP: u32 = 0b1 << 9;
pub const MIP_MEIP: u32 = 0b1 << 11;

// Machine Memory Protection
// Physical memory protection configration
pub const PMPCFG0: usize = 0x3A0;
pub const PMPCFG1: usize = 0x3A1;
pub const PMPCFG2: usize = 0x3A2;
pub const PMPCFG3: usize = 0x3A3;
// Physical memory protection address register.
pub const PMPADDR0: usize = 0x3B0;
pub const PMPADDR1: usize = 0x3B1;
pub const PMPADDR2: usize = 0x3B2;
pub const PMPADDR3: usize = 0x3B3;
pub const PMPADDR4: usize = 0x3B4;
pub const PMPADDR5: usize = 0x3B5;
pub const PMPADDR6: usize = 0x3B6;
pub const PMPADDR7: usize = 0x3B7;
pub const PMPADDR8: usize = 0x3B8;
pub const PMPADDR9: usize = 0x3B9;
pub const PMPADDR10: usize = 0x3BA;
pub const PMPADDR11: usize = 0x3BB;
pub const PMPADDR12: usize = 0x3BC;
pub const PMPADDR13: usize = 0x3BD;
pub const PMPADDR14: usize = 0x3BE;
pub const PMPADDR15: usize = 0x3BF;

// Machine Counter/Timers
// Machine cycle counter
pub const MCYCLE: usize = 0xB00;
// Machine instructions-retired counter.
pub const MINSTRET: usize = 0xB02;
// Machine performance-monitoring counter.
pub const MHPMCOUNTER3: usize = 0xB03;
// mhpmcounter[3,31] = [0xB03,0xB1F]
pub const MHPMCOUNTER31: usize = 0xB1F;
// Upper 32 bits of Machine Counter/Timers, RV32I only.
pub const MCYCLEH: usize = 0xB80;
pub const MINSTRETH: usize = 0xB82;
pub const MHPMCOUNTER3H: usize = 0xB83;
// mhpmcounter[3,31]h = [0xB83,0xB9F]
pub const MHPMCOUNTER31H: usize = 0xB9F;

// Machine Counter Setup
// Machine counter-inhibit register.
pub const MCOUNTINHIBIT: usize = 0x320;
// Machine performance-monitoring event selector.
pub const MHPMEVENT3: usize = 0x323;
// mhpmevent[3,31] = [0x323,0x33F]
pub const MHPMEVENT31: usize = 0x33F;

// Debug/Trace Registers (shared with Debug Mode)
// Debug/Trace trigger register select.
pub const TSELECT: usize = 0x7A0;
// Debug/Trace trigger data register.
pub const TDATA1: usize = 0x7A1;
pub const TDATA2: usize = 0x7A2;
pub const TDATA3: usize = 0x7A3;

// Debug Mode Registers
// Debug control and status register.
pub const DCSR: usize = 0x7B0;
// Debug PC.
pub const DPC: usize = 0x7B1;
// Debug scratch register.
pub const DSCRATCH0: usize = 0x7B2;
pub const DSCRATCH1: usize = 0x7B3;
