//! New public FFI reference, not the private Malphas ABI or rendering engine.
//! One Rust-owned frame, borrowed synchronously by Dart. No dependencies.
#![deny(unsafe_op_in_unsafe_fn)]
const WIDTH: usize = 64;
const HEIGHT: usize = 64;
const BYTES: usize = WIDTH * HEIGHT * 4;

// A chosen alignment for this experiment, not a claim about every CPU's cache.
#[repr(C, align(64))]
pub struct Frame {
    pixels: [u8; BYTES],
}

fn paint(pixels: &mut [u8; BYTES], tick: u32) {
    for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH) as u8;
        let y = (i / WIDTH) as u8;
        pixel.copy_from_slice(&[x.wrapping_add(tick as u8), y, x ^ y, 255]);
    }
}

#[no_mangle]
pub extern "C" fn sample_frame_new() -> *mut Frame {
    Box::into_raw(Box::new(Frame { pixels: [0; BYTES] }))
}

#[no_mangle]
pub extern "C" fn sample_frame_len() -> usize { BYTES }

/// # Safety
/// `frame` must be null or a live allocation returned by sample_frame_new.
/// No concurrent mutation or free is allowed. The returned memory is borrowed,
/// read-only for the caller, and must not be used after sample_frame_free.
#[no_mangle]
pub unsafe extern "C" fn sample_frame_data(frame: *const Frame) -> *const u8 {
    if frame.is_null() { return std::ptr::null(); }
    // SAFETY: the caller guarantees a live, aligned allocation for this read.
    unsafe { (*frame).pixels.as_ptr() }
}

/// # Safety
/// `frame` must be null or a unique live allocation from sample_frame_new.
/// No Dart view may be read while this synchronous write runs.
#[no_mangle]
pub unsafe extern "C" fn sample_frame_render(frame: *mut Frame, tick: u32) -> i32 {
    if frame.is_null() { return -1; }
    // SAFETY: the caller guarantees exclusive access during this call.
    paint(unsafe { &mut (*frame).pixels }, tick);
    0
}

/// # Safety
/// Null is accepted. Otherwise free exactly once with no outstanding uses or
/// concurrent calls. Invalid pointers and double-free cannot be validated here.
#[no_mangle]
pub unsafe extern "C" fn sample_frame_free(frame: *mut Frame) {
    if !frame.is_null() {
        // SAFETY: ownership of this exact Box allocation is returned by caller.
        unsafe { drop(Box::from_raw(frame)); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alignment_and_length_are_explicit() {
        assert_eq!(std::mem::align_of::<Frame>(), 64);
        assert_eq!(sample_frame_len(), 16384);
    }
    #[test]
    fn null_contract_is_defined() {
        unsafe {
            assert!(sample_frame_data(std::ptr::null()).is_null());
            assert_eq!(sample_frame_render(std::ptr::null_mut(), 0), -1);
            sample_frame_free(std::ptr::null_mut());
        }
    }
    #[test]
    fn allocation_is_reused_and_pattern_is_deterministic() {
        let frame = sample_frame_new();
        unsafe {
            let ptr = sample_frame_data(frame);
            assert_eq!((ptr as usize) % 64, 0);
            assert_eq!(sample_frame_render(frame, 0), 0);
            let bytes = std::slice::from_raw_parts(ptr, BYTES);
            assert_eq!(&bytes[..8], &[0,0,0,255,1,0,1,255]);
            assert!(bytes.chunks_exact(4).all(|pixel| pixel[3] == 255));
            // The immutable borrow ends before the next mutating call.
            assert_eq!(sample_frame_render(frame, 1), 0);
            assert_eq!(sample_frame_data(frame), ptr);
            assert_eq!(*ptr, 1);
            sample_frame_free(frame);
        }
    }
    #[test]
    fn tick_wrap_is_deliberate() {
        let mut bytes = [0; BYTES];
        paint(&mut bytes, u32::MAX);
        assert_eq!(&bytes[..8], &[255,0,0,255,0,0,1,255]);
    }
}
