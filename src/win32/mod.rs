#[inline]
pub fn rand_bytes(bytes: &mut [u8]) {
    let status = unsafe {
        windows_sys::Win32::Security::Cryptography::ProcessPrng(
            core::ptr::from_mut::<[u8]>(bytes).cast(),
            bytes.len(),
        )
    };
    debug_assert_ne!(status, 0);
}
