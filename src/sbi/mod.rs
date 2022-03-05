//! [RISC-V Supervisor Binary Interface (SBI)](https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc)

use core::arch::asm;

pub mod base;
pub mod legacy;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct SBIReturn {
    error: SBIError,
    value: usize,
}

impl SBIReturn {
    fn unwrap(self) -> usize {
        assert_eq!(self.error, SBIError::Success);
        self.value
    }
}

/// The error type which is returned from SBI.
#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum SBIError {
    Success = 0,
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
}

/// The type returned by SBI functions.
pub type SBIResult<T = ()> = Result<T, SBIError>;

impl From<SBIReturn> for SBIResult<usize> {
    fn from(ret: SBIReturn) -> Self {
        match ret.error {
            SBIError::Success => Ok(ret.value),
            err => Err(err),
        }
    }
}

#[inline(always)]
fn sbi_call(ext_id: usize, func_id: usize, arg0: usize, arg1: usize, arg2: usize) -> SBIReturn {
    let error: isize;
    let value;
    unsafe {
        asm!(
            "ecall",
            lateout("a0") error,
            lateout("a1") value,
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a6") func_id,
            in("a7") ext_id,
            options(nostack)
        );
    }
    let error = match error {
        0 => SBIError::Success,
        -1 => SBIError::Failed,
        -2 => SBIError::NotSupported,
        -3 => SBIError::InvalidParam,
        -4 => SBIError::Denied,
        -5 => SBIError::InvalidAddress,
        -6 => SBIError::AlreadyAvailable,
        error => panic!("invalid error value: {}", error),
    };
    SBIReturn { error, value }
}
