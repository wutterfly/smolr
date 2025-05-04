#[inline]
pub fn rand_bytes(bytes: &mut [u8]) {
    let status = unsafe { libc::getrandom(bytes as *mut [u8] as *mut _, bytes.len(), 4) };
    assert_ne!(status, 0);
}
