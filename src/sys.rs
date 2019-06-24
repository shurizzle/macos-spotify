#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use libc::{c_char, c_long, c_short};
pub use libc::{c_void, strlen};
use std::{fmt, slice, str};

pub type OSStatus = i32;
pub type OSErr = c_short;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ResType(pub u32);
pub type AEEventID = ResType;
pub type AEEventClass = ResType;
pub type DescType = ResType;
pub type AEKeyword = ResType;

impl ResType {
    pub fn new(value: &str) -> ResType {
        ResType::from_str(value)
    }

    pub fn from_str(value: &str) -> ResType {
        if !value.is_ascii() || value.len() != 4 {
            panic!(
                "ResType value must be a String of 4 ASCII characters, {} given.",
                value
            );
        }

        ResType::from_int(unsafe { *(value.as_ptr() as *const u32) })
    }

    pub fn from_int(value: u32) -> ResType {
        ResType(value)
    }

    #[cfg(target_endian = "little")]
    pub fn to_string(&self) -> String {
        let s =
            unsafe { slice::from_raw_parts(&self.0.swap_bytes() as *const u32 as *const u8, 4) };
        str::from_utf8(s).unwrap().to_string()
    }

    #[cfg(target_endian = "big")]
    pub fn to_string(&self) -> String {
        let s = unsafe { slice::from_raw_parts(&self.0 as *const u32 as *const u8, 4) };
        str::from_utf8(s).unwrap().to_string()
    }

    pub fn to_int(&self) -> u32 {
        self.0
    }

    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for ResType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'", self.to_string())
    }
}

impl From<&str> for ResType {
    fn from(value: &str) -> ResType {
        ResType::from_str(value)
    }
}

impl From<u32> for ResType {
    fn from(value: u32) -> ResType {
        ResType::from_int(value)
    }
}

impl fmt::Debug for ResType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResType({:?})", self.to_string())
    }
}

#[repr(C)]
pub struct AEDesc {
    pub(crate) descriptorType: DescType,
    pub(crate) dataHandle: *const c_void,
}

impl AEDesc {
    pub fn is_null(&self) -> bool {
        self.descriptorType.is_null() || self.descriptorType == typeNull
    }
}

pub type AEDescList = AEDesc;
pub type AERecord = AEDescList;
pub type AppleEvent = AERecord;
pub type AEBuildErrorCode = u32;

#[derive(Debug)]
#[repr(C)]
pub struct AEBuildError {
    pub fError: AEBuildErrorCode,
    pub fErrorPos: u32,
}

pub type Ptr = *mut c_char;
pub type Handle = *mut Ptr;

pub const kAECoreSuite: ResType = ResType(0x636f7265);
pub const kAEGetData: ResType = ResType(0x67657464);
pub const kAESetData: ResType = ResType(0x73657464);

pub const keyDirectObject: ResType = ResType(0x2d2d2d2d);

pub const typeApplicationBundleID: ResType = ResType(0x62756e64);
pub const typeNull: ResType = ResType(0x6e756c6c);
pub const typeBoolean: ResType = ResType(0x626f6f6c);
pub const typeUnicodeText: ResType = ResType(0x75747874);
pub const typeChar: ResType = ResType(0x54455854);
pub const typeVersion: ResType = ResType(0x76657273);
pub const typeWildCard: ResType = ResType(0x2a2a2a2a);
pub const typeApplSignature: ResType = ResType(0x7369676e);
pub const typeEnumerated: ResType = ResType(0x656e756d);
pub const typeIEEE64BitFloatingPoint: ResType = ResType(0x646f7562);
pub const typeFloat: ResType = typeIEEE64BitFloatingPoint;
pub const typeLongFloat: ResType = typeIEEE64BitFloatingPoint;
pub const typeSInt16: ResType = ResType(0x73686f72);
pub const typeSMInt: ResType = typeSInt16;
pub const typeShortInteger: ResType = typeSInt16;
pub const typeSInt32: ResType = ResType(0x6c6f6e67);
pub const typeInteger: ResType = typeSInt32;
pub const typeLongInteger: ResType = typeSInt32;
pub const typeSInt64: ResType = ResType(0x636f6d70);
pub const typeComp: ResType = typeSInt64;
pub const typeType: ResType = ResType(0x74797065);

pub const kAutoGenerateReturnID: i16 = -1;
pub const kAnyTransactionID: i32 = 0;

pub const kAEDefaultTimeout: c_long = -1;
pub const kNoTimeOut: c_long = -2;

pub const kAENoReply: i32 = 0x00000001;
pub const kAEQueueReply: i32 = 0x00000002;
pub const kAEWaitReply: i32 = 0x00000003;
pub const kAEDontReconnect: i32 = 0x00000080;
pub const kAEWantReceipt: i32 = 0x00000200;
pub const kAENeverInteract: i32 = 0x00000010;
pub const kAECanInteract: i32 = 0x00000020;
pub const kAEAlwaysInteract: i32 = 0x00000030;
pub const kAECanSwitchLayer: i32 = 0x00000040;
pub const kAEDontRecord: i32 = 0x00001000;
pub const kAEDontExecute: i32 = 0x00002000;
pub const kAEProcessNonReplyEvents: i32 = 0x00008000;
pub const kAEDoNotAutomaticallyAddAnnotationsToEvent: i32 = 0x00010000;

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    pub fn AEBuildAppleEvent(
        theClass: AEEventClass,
        theID: AEEventID,
        addressType: DescType,
        addressData: *const c_void,
        addressLength: usize,
        returnID: i16,
        transactionID: i32,
        result: *mut AppleEvent,
        error: *mut AEBuildError,
        paramsFmt: *const u8,
        ...
    ) -> OSStatus;
    pub fn AEDisposeDesc(theAEDesc: *const AEDesc) -> OSErr;
    pub fn AESendMessage(
        event: *const AppleEvent,
        reply: *mut AppleEvent,
        sendMode: i32,
        timeOutInTicks: c_long,
    ) -> OSStatus;
    pub fn AESizeOfParam(
        theAppleEvent: *const AppleEvent,
        theAEKeyword: AEKeyword,
        typeCode: *mut DescType,
        dataSize: *mut usize,
    ) -> OSErr;
    pub fn AEGetParamPtr(
        theAppleEvent: *const AppleEvent,
        theAEKeyword: AEKeyword,
        desiredType: DescType,
        actualType: *mut DescType,
        dataPtr: *mut c_void,
        maximumSize: usize,
        actualSize: *mut usize,
    ) -> OSErr;
    pub fn AEGetParamDesc(
        theAppleEvent: *const AppleEvent,
        theAEKeyword: AEKeyword,
        desiredType: DescType,
        result: *mut AEDesc,
    ) -> OSErr;
    pub fn AEBuildDesc(dst: *mut AEDesc, error: *mut AEBuildError, src: *const u8, ...) -> OSErr;
    pub fn AEPrintDescToHandle(desc: *const AEDesc, result: *mut Handle) -> OSStatus;
    pub fn DisposeHandle(h: Handle);
}

impl Drop for AEDesc {
    fn drop(&mut self) {
        if !self.dataHandle.is_null() {
            unsafe { AEDisposeDesc(self) };
            self.dataHandle = std::ptr::null();
        }
    }
}

impl Default for ResType {
    fn default() -> Self {
        ResType(0)
    }
}

impl Default for AEDesc {
    fn default() -> Self {
        AEDesc {
            descriptorType: typeNull,
            dataHandle: std::ptr::null(),
        }
    }
}

use std::ffi::CStr;

impl fmt::Debug for AEDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut handle: Handle = std::ptr::null_mut();
            if AEPrintDescToHandle(self, &mut handle) == 0 {
                let res = write!(f, "{}", CStr::from_ptr(*handle).to_str().unwrap());
                DisposeHandle(handle);
                res
            } else {
                Err(fmt::Error)
            }
        }
    }
}

impl Default for AEBuildError {
    fn default() -> Self {
        AEBuildError {
            fError: 0,
            fErrorPos: 0,
        }
    }
}
