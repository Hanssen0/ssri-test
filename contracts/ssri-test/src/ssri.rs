use ckb_std::syscalls::set_content;
use crate::syscall::vm_version;

use crate::error::Error;

pub fn ssri() -> Result<(), Error> {
    if vm_version() != u64::MAX {
        return Err(Error::InvalidVmVersion);
    }

    set_content("hello world".as_bytes())?;
    Ok(())
}
