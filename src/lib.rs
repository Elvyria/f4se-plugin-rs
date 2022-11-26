mod bindings;

use std::cell::UnsafeCell;

use bindings::*;

use region::Protection;

static PLUGIN_NAME: &[u8] = b"F4SE Rust Plugin\0";

// TL;DR `static mut` is bad. SyncUnsafeCell is not stable.
// [PR 95438](https://github.com/rust-lang/rust/pull/95438)
// [Issue 53639](https://github.com/rust-lang/rust/issues/53639)
thread_local!
{
    pub static PLUGIN_HANDLE: UnsafeCell<PluginHandle> = UnsafeCell::new(0);
    pub static BASE_ADDRESS: UnsafeCell<usize> = UnsafeCell::new(0);
}

fn write<T>(addr: usize, src: T) {
    unsafe {
        let _protection = region::protect_with_handle(addr as *const T, std::mem::size_of::<T>(), Protection::READ_WRITE_EXECUTE).unwrap();

        (addr as *mut T).write_unaligned(src);
    }
}

#[no_mangle]
pub extern "C" fn F4SEPlugin_Query(f4se: &F4SEInterface, info: &mut PluginInfo) -> bool {
    if f4se.isEditor != 0 { return false }

    info.infoVersion = PluginInfo_kInfoVersion as _;
    info.name = PLUGIN_NAME.as_ptr() as _;
    info.version = 1;

    PLUGIN_HANDLE.with(|f| unsafe {
        *f.get() = f4se.GetPluginHandle.unwrap()() as PluginHandle
    });

    true
}

#[no_mangle]
pub extern "C" fn F4SEPlugin_Load(f4se: &F4SEInterface) -> bool {
    let base = unsafe { RelocationManager_s_baseAddr };

    BASE_ADDRESS.with(|f| unsafe { *f.get() = base; });

    true
}
