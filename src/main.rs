#![no_std]
#![no_main]

use esp_idf_sys as _;

#[no_mangle]
fn main() {
    //esp_idf_sys::link_patches();

    unsafe {
        let part_name = core::ffi::CStr::from_bytes_with_nul_unchecked(b"\0");
        let _ = esp_idf_sys::nvs_flash_init_partition(part_name.as_ptr());
    }
}
