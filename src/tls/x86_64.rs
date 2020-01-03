use core::mem::transmute;
use core::ptr;

use crate::tls::ThreadLocalStorage;
static mut TLS: *mut ThreadLocalStorage = ptr::null_mut();

pub(crate) unsafe fn get_tls<'a>() -> *mut ThreadLocalStorage<'a> {
    transmute::<*mut ThreadLocalStorage<'static>, *mut ThreadLocalStorage<'a>>(TLS)
}

pub(crate) unsafe fn set_tls<'a>(t: *mut ThreadLocalStorage<'a>) {
    TLS = transmute::<*mut ThreadLocalStorage<'a>, *mut ThreadLocalStorage<'static>>(t);
}

#[test]
fn has_fs_gs_base_instructions() {
    env_logger::init();
    let cpuid = x86::cpuid::CpuId::new();
    assert!(cpuid
        .get_extended_feature_info()
        .map_or(false, |ef| ef.has_fsgsbase()));

    let p = "asdf";
    set_tls(p.as_bytes().as_mut_ptr() as *mut ThreadLocalStorage);

    assert_eq!(
        p.as_bytes.as_mut_ptr(),
        get_tls(p.as_bytes().as_mut_ptr() as *mut ThreadLocalStorage)
    );
}
