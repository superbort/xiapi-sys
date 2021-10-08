#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::os::raw::c_char;
    use std::ptr::null_mut;
    use crate::*;
    use serial_test::serial;
    use std::mem::size_of;

    macro_rules! assert_ok {
        ($x:expr) => {
            assert_eq!($x, XI_RET::XI_OK as XI_RETURN)
        }
    }

    #[test]
    #[serial]
    fn open_close() {
        unsafe {
            let mut xiH: HANDLE = null_mut();
            assert_ok!(xiOpenDevice(0, &mut xiH));
            assert_ok!(xiCloseDevice(xiH));
        }
    }

    #[test]
    #[serial]
    fn set_param() {
        unsafe {
            let mut xiH: HANDLE = null_mut();
            assert_ok!(xiOpenDevice(0, &mut xiH));
            assert_ok!(xiSetParamInt(xiH, XI_PRM_EXPOSURE.as_ptr() as *const c_char, 1000));
            assert_ok!(xiCloseDevice(xiH));
        }
    }

    #[test]
    #[serial]
    fn xi_sample() {
        unsafe {
            let mut image = XI_IMG::default();
            image.size = size_of::<XI_IMG>() as DWORD;

            let mut xiH: HANDLE = null_mut();

            assert_ok!(xiOpenDevice(0, &mut xiH));
            
            assert_ok!(xiSetParamInt(xiH, XI_PRM_EXPOSURE.as_ptr() as *const c_char, 10000));
            
            assert_ok!(xiStartAcquisition(xiH));

            for i in 0..100 {
                assert_ok!(xiGetImage(xiH, 50000, &mut image));
                let pixel = *(image.bp as *const u8);
                println!("Image {} ({}x{}) received from camera. First pixel value: {}", i, image.width, image.height, pixel);
            }

            assert_ok!(xiStopAcquisition(xiH));

            assert_ok!(xiCloseDevice(xiH));
        }

    }
}
