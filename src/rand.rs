#[cfg(target_os = "linux")]
use super::linux as plat;

#[cfg(target_os = "windows")]
use super::win32 as plat;

pub use plat::rand_bytes;

#[inline]
#[must_use]
pub fn rand_u8() -> u8 {
    let mut buf = [0u8; core::mem::size_of::<u8>()];

    rand_bytes(&mut buf);

    u8::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_u16() -> u16 {
    let mut buf = [0u8; core::mem::size_of::<u16>()];

    rand_bytes(&mut buf);

    u16::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_u32() -> u32 {
    let mut buf = [0u8; core::mem::size_of::<u32>()];

    rand_bytes(&mut buf);

    u32::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_u64() -> u64 {
    let mut buf = [0u8; core::mem::size_of::<u64>()];

    rand_bytes(&mut buf);

    u64::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_usize() -> usize {
    let mut buf = [0u8; core::mem::size_of::<usize>()];

    rand_bytes(&mut buf);

    usize::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_i8() -> i8 {
    let mut buf = [0u8; core::mem::size_of::<i8>()];

    rand_bytes(&mut buf);

    i8::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_i16() -> i16 {
    let mut buf = [0u8; core::mem::size_of::<i16>()];

    rand_bytes(&mut buf);

    i16::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_i32() -> i32 {
    let mut buf = [0u8; core::mem::size_of::<i32>()];

    rand_bytes(&mut buf);

    i32::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_i64() -> i64 {
    let mut buf = [0u8; core::mem::size_of::<i64>()];

    rand_bytes(&mut buf);

    i64::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_isize() -> isize {
    let mut buf = [0u8; core::mem::size_of::<isize>()];

    rand_bytes(&mut buf);

    isize::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_f32() -> f32 {
    let mut buf = [0u8; core::mem::size_of::<f32>()];

    rand_bytes(&mut buf);

    f32::from_be_bytes(buf)
}

#[inline]
#[must_use]
pub fn rand_f64() -> f64 {
    let mut buf = [0u8; core::mem::size_of::<f64>()];

    rand_bytes(&mut buf);

    f64::from_be_bytes(buf)
}
