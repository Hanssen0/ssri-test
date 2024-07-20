use alloc::borrow::Cow;

use crate::error::Error;

pub fn name() -> Result<Cow<'static, [u8]>, Error> {
    Ok(Cow::from("Test".as_bytes()))
}
