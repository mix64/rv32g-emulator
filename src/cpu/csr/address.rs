// User Trap Setup
// User status register.
pub const USTATUS: u16 = 0x000;
// User interrupt-enable register.
pub const UIE: u16 = 0x004;
// User trap handler base address.
pub const UTVEC: u16 = 0x005;

// User Trap Handling
// Scratch register for user trap handlers.
pub const USCRATCH: u16 = 0x040;
// User exception program counter.
pub const UEPC: u16 = 0x041;
// User trap cause.
pub const UCAUSE: u16 = 0x042;
// User bad address or instruction
pub const UTVAL: u16 = 0x043;
// User interrupt pending.
pub const UIP: u16 = 0x044;

// User Floating-Point CSRs
// Floating-Point Accrued Exceptions.
pub const FFLAGS: u16 = 0x001;
// Floating-Point Dynamic Rounding Mode.
pub const FRM: u16 = 0x002;
// Floating-Point Control and Status Register (frm+fflags)
pub const FCSR: u16 = 0x003;

// User Counter/Timers (Read-Only)
// Cycle counter for RDCYCLE instruction.
pub const CYCLE: u16 = 0xC00;
// Timer for RDTIME instruction.
pub const TIME: u16 = 0xC01;
// Instructions-retired counter for RDINSTRET instruction.
pub const INSTRET: u16 = 0xC02;
// Performance-monitoring counter.
pub const HPMCOUNTER3: u16 = 0xC03;
// hpmcounter[3,31] = [0xC03,0xC1F]
pub const HPMCOUNTER31: u16 = 0xC1F;
// Upper 32bits of Counter/Timers, RV32I only.
pub const CYCLEH: u16 = 0xC80;
pub const TIMEH: u16 = 0xC81;
pub const INSTRETH: u16 = 0xC82;
pub const HPMCOUNTER3H: u16 = 0xC83;
// hpmcounter[3,31]h = [0xC83,0xC9F]
pub const HPMCOUNTER31H: u16 = 0xC9F;

// Supervisor Trap Setup
// Supervisor status register.
pub const SSTATUS: u16 = 0x100;
// Supervisor exception delegation register.
pub const SEDELEG: u16 = 0x102;
// Supervisor interrupt delegation register.
pub const SIDELEG: u16 = 0x103;
// Supervisor interrupt-enable register.
pub const SIE: u16 = 0x104;
// Supervisor trap handler base address.
pub const STVEC: u16 = 0x105;
// Supervisor counter enable.
pub const SCOUNTEREN: u16 = 0x106;

// Supervisor Trap Handling
// Scratch register for supervisor trap handlers.
pub const SSCRATCH: u16 = 0x140;
// Supervisor exception program counter.
pub const SEPC: u16 = 0x141;
// Supervisor trap cause.
pub const SCAUSE: u16 = 0x142;
// Supervisor bad address or instruction.
pub const STVAL: u16 = 0x143;
// Supervisor interrupt pending.
pub const SIP: u16 = 0x144;

// Supervisor Protection and Translation
// Supervisor address translation and protection.
pub const SATP: u16 = 0x180;

// Machine Information Registers
// Vendor ID.
pub const MVENDORID: u16 = 0xF11;
// Architecture ID.
pub const MARCHID: u16 = 0xF12;
// Implementation ID.
pub const MIMPID: u16 = 0xF13;
// Hardware thread ID.
pub const MHARTID: u16 = 0xF14;

// Machine Trap Setup
// Machine status register.
// Restricted views of the "mstatus" appear as the "sstatus" and "ustatus" (3.1.6)
pub const MSTATUS: u16 = 0x300;
// ISA and extensions.
pub const MISA: u16 = 0x301;
// Machine exception delegation register.
pub const MEDELEG: u16 = 0x302;
// Machine interrupt delegation register.
pub const MIDELEG: u16 = 0x303;
// Machine interrupt-enable register.
pub const MIE: u16 = 0x304;
// Machine trap-handler base address.
pub const MTVEC: u16 = 0x305;
// Machine counter enable.
pub const MCOUNTEREN: u16 = 0x306;

// Machine Trap Handling
// Scratch register for machine trap handlers.
pub const MSCRATCH: u16 = 0x340;
// Machine exception program counter.
pub const MEPC: u16 = 0x341;
// Machine trap cause.
pub const MCAUSE: u16 = 0x342;
// Machine bad address or instruction.
pub const MTVAL: u16 = 0x343;
// Machine interrupt pending.
pub const MIP: u16 = 0x344;

// Machine Memory Protection
// Physical memory protection configration
pub const PMPCFG0: u16 = 0x3A0;
pub const PMPCFG1: u16 = 0x3A1;
pub const PMPCFG2: u16 = 0x3A2;
pub const PMPCFG3: u16 = 0x3A3;
// Physical memory protection address register.
pub const PMPADDR0: u16 = 0x3B0;
pub const PMPADDR1: u16 = 0x3B1;
pub const PMPADDR2: u16 = 0x3B2;
pub const PMPADDR3: u16 = 0x3B3;
pub const PMPADDR4: u16 = 0x3B4;
pub const PMPADDR5: u16 = 0x3B5;
pub const PMPADDR6: u16 = 0x3B6;
pub const PMPADDR7: u16 = 0x3B7;
pub const PMPADDR8: u16 = 0x3B8;
pub const PMPADDR9: u16 = 0x3B9;
pub const PMPADDR10: u16 = 0x3BA;
pub const PMPADDR11: u16 = 0x3BB;
pub const PMPADDR12: u16 = 0x3BC;
pub const PMPADDR13: u16 = 0x3BD;
pub const PMPADDR14: u16 = 0x3BE;
pub const PMPADDR15: u16 = 0x3BF;

// Machine Counter/Timers
// Machine cycle counter
pub const MCYCLE: u16 = 0xB00;
// Machine instructions-retired counter.
pub const MINSTRET: u16 = 0xB02;
// Machine performance-monitoring counter.
pub const MHPMCOUNTER3: u16 = 0xB03;
// mhpmcounter[3,31] = [0xB03,0xB1F]
pub const MHPMCOUNTER31: u16 = 0xB1F;
// Upper 32 bits of Machine Counter/Timers, RV32I only.
pub const MCYCLEH: u16 = 0xB80;
pub const MINSTRETH: u16 = 0xB82;
pub const MHPMCOUNTER3H: u16 = 0xB83;
// mhpmcounter[3,31]h = [0xB83,0xB9F]
pub const MHPMCOUNTER31H: u16 = 0xB9F;

// Machine Counter Setup
// Machine counter-inhibit register.
pub const MCOUNTINHIBIT: u16 = 0x320;
// Machine performance-monitoring event selector.
pub const MHPMEVENT3: u16 = 0x323;
// mhpmevent[3,31] = [0x323,0x33F]
pub const MHPMEVENT31: u16 = 0x33F;

// Debug/Trace Registers (shared with Debug Mode)
// Debug/Trace trigger register select.
pub const TSELECT: u16 = 0x7A0;
// Debug/Trace trigger data register.
pub const TDATA1: u16 = 0x7A1;
pub const TDATA2: u16 = 0x7A2;
pub const TDATA3: u16 = 0x7A3;

// Debug Mode Registers
// Debug control and status register.
pub const DCSR: u16 = 0x7B0;
// Debug PC.
pub const DPC: u16 = 0x7B1;
// Debug scratch register.
pub const DSCRATCH0: u16 = 0x7B2;
pub const DSCRATCH1: u16 = 0x7B3;
