use crate::{hstring, ErrorCode, Guid, RawPtr};

#[link(name = "kernel32")]
extern "system" {
    pub fn GetProcessHeap() -> RawPtr;
    pub fn HeapAlloc(heap: RawPtr, flags: u32, bytes: usize) -> RawPtr;
    pub fn HeapFree(heap: RawPtr, flags: u32, ptr: RawPtr) -> i32;
}

#[link(name = "onecore")]
extern "system" {
    // TODO: get rid of these (not available on Windows 7) - we'll load these dynamically
    pub fn CoIncrementMTAUsage(cookie: *mut RawPtr) -> ErrorCode;
    pub fn RoGetActivationFactory(
        hstring: *mut hstring::Header,
        interface: &Guid,
        result: *mut RawPtr,
    ) -> ErrorCode;
}
