use crate::serialize::{Mask, MaskedSerializer};
use serde::Serialize;
use serde_json::Result;

// This is lifted directly from serde_json for consistency

// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

// end of lifted section

#[inline]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn to_writer<W, T>(mask: Mask, writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + Serialize,
{
    let mut serializer = serde_json::Serializer::new(writer);
    let mut masked_serializer = MaskedSerializer::new(&mut serializer, mask);
    value.serialize(&mut masked_serializer)
}

#[inline]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn to_writer_pretty<W, T>(mask: Mask, writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + Serialize,
{
    let mut serializer = serde_json::Serializer::pretty(writer);
    let mut masked_serializer = MaskedSerializer::new(&mut serializer, mask);
    value.serialize(&mut masked_serializer)
}

#[inline]
pub fn to_vec<T>(mask: Mask, value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    tri!(to_writer(mask, &mut writer, value));
    Ok(writer)
}

#[inline]
pub fn to_vec_pretty<T>(mask: Mask, value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    tri!(to_writer_pretty(mask, &mut writer, value));
    Ok(writer)
}

#[inline]
pub fn to_string<T>(mask: Mask, value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = tri!(to_vec(mask, value));
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

#[inline]
pub fn to_string_pretty<T>(mask: Mask, value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = tri!(to_vec_pretty(mask, value));
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}
