#[cfg(target_arch = "riscv64")]
use core::arch::asm;

use ckb_std::ckb_constants::SYS_VM_VERSION;

#[cfg(target_arch = "riscv64")]
#[allow(clippy::too_many_arguments)]
pub unsafe fn syscall(
    mut a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> u64 {
    asm!(
        "ecall",
        inout("a0") a0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        in("a5") a5,
        in("a6") a6,
        in("a7") a7
    );
    a0
}

#[cfg(not(target_arch = "riscv64"))]
#[allow(clippy::too_many_arguments)]
pub unsafe fn syscall(
    _a0: u64,
    _a1: u64,
    _a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
    _a6: u64,
    _a7: u64,
) -> u64 {
    u64::MAX
}

pub fn vm_version() -> u64 {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYS_VM_VERSION) }
}
