/*
    (3.1.6.1) Privilege and Global Interrupt-Enable Stack

    Interrupts for lower-privilege modes, w<x, are always globally "disabled"
    regardless of the setting of the lower-privilege mode’s global wIE bit.

    Interrupts for higher-privilege modes, y>x, are always globally "enabled"
    regardless of the setting of the higher-privilege mode’s global yIE bit.

    When a trap is taken from privilege mode y into privilege mode x,
    xPIE is set to the value of xIE; xIE is set to 0; and xPP is set to y.

    When executing an xRET instruction, supposing xPP holds the value y,
    xIE is set to xPIE; the privilege mode is changed to y; xPIE is set to 1; and xPP is set to U.
*/
// Interrupt Enable
pub const MSTATUS_UIE: u32 = 0;
pub const MSTATUS_SIE: u32 = 1;
pub const MSTATUS_MIE: u32 = 3;
// Previous Interrupt Enable
pub const MSTATUS_UPIE: u32 = 4;
pub const MSTATUS_SPIE: u32 = 5;
pub const MSTATUS_MPIE: u32 = 7;
// Previous Privilege mode
// The xPP fields can only hold privilege modes up to x.
pub const MSTATUS_SPP: u32 = 8;
pub const MSTATUS_MPP: u32 = 11;

/*
    (3.1.6.3) Memory Privilege

    The MPRV bit modifies the privilege level at which loads and stores execute in all privilege modes.
    When MPRV=0, loads and stores behave as normal, using the translation and protection mechanisms of
    the current privilege mode.
    When MPRV=1, load and store memory addresses are translated and protected as though the current
    privilege mode were set to MPP.
    Instruction address-translation and protection are unaffected by the setting of MPRV.

    The SUM bit modifies the privilege with which S-mode loads and stores access virtual memory.
    When SUM=0, S-mode memory accesses to pages that are accessible by U-mode (U=1) will fault.
    When SUM=1, these accesses are permitted.
    SUM has no effect when page-based virtual memory is not in effect.
    Note that, while SUM is ordinarily ignored when not executing in S-mode, it is in effect when MPRV=1 and MPP=S.

    The MXR bit modifies the privilege with which loads access virtual memory.
    When MXR=0, only loads from pages marked readable (R=1 in Figure 4.15) will succeed.
    When MXR=1, loads from pages marked either readable or executable (R=1 or X=1) will succeed.
*/
// Modify PRiVilege
pub const MSTATUS_MPRV: u32 = 17;
// permit Supervisor User Memory access
pub const MSTATUS_SUM: u32 = 18;
// Make eXecutable Readable
pub const MSTATUS_MXR: u32 = 19;

/*
    (3.1.6.4) Virtualization Support

    The TVM bit supports intercepting supervisor virtual-memory management operations.
    When TVM=1, attempts to read or write the satp CSR or execute the SFENCE.VMA instruction
    while executing in S-mode will raise an illegal instruction exception.
    When TVM=0, these operations are permitted in S-mode.

    The TW bit supports intercepting the WFI instruction.
    When TW=0, the WFI instruction may execute in lower privilege modes when not prevented for some other reason.
    When TW=1, then if WFI is executed in any less-privileged mode, and it does not complete within
    an implementation-specific, bounded time limit, the WFI instruction causes an illegal instruction exception.
    The time limit may always be 0, in which case WFI always causes an illegal instruction exception in
    less-privileged modes when TW=1.
    When S-mode is implemented, then executing WFI in U-mode causes an illegal instruction exception,
    unless it completes within an implementation-specific, bounded time limit.

    The TSR (Trap SRET) bit supports intercepting the supervisor exception return instruction, SRET.
    When TSR=1, attempts to execute SRET while executing in S-mode will raise an illegal instruction exception.
    When TSR=0, this peration is permitted in S-mode.
*/
// Trap Virtual Memory
pub const MSTATUS_TVM: u32 = 20;
// Timeout Wait
pub const MSTATUS_TW: u32 = 21;
// Trap SRet
pub const MSTATUS_TSR: u32 = 22;

/*
    (3.1.6.5) Extension Context Status

    Status  FS Meaning  XS Meaning
    0       Off         All off
    1       Initial     None dirty or clean, some on
    2       Clean       None dirty, some clean
    3       Dirty       Some dirty

    The FS field encodes the status of the floating-point unit, including the CSR fcsr and floating-point data registers f0–f31,
    while the XS field encodes the status of additional user-mode extensions and associated state.
    These fields can be checked by a context switch routine to quickly determine whether a state save or restore is required.
    If a save or restore is required, additional instructions and CSRs are typically required to effect and optimize the process.
    In systems that do not implement S-mode and do not have a floating-point unit, the FS field is hardwired to zero.
    In systems without additional user extensions requiring new state, the XS field is hardwired to zero.

    The SD bit is a read-only bit that summarizes whether either the FS field or XS field signals the presence of some dirty state
    that will require saving extended user context to memory.
*/
pub const MSTATUS_FS: u32 = 13; // WARL
pub const MSTATUS_XS: u32 = 15; // read-only
pub const MSTATUS_SD: u32 = 31; // read-only
