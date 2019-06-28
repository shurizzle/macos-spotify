#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use libc::{c_char, c_long, c_short};
pub use libc::{c_void, strlen};
use std::fmt;

pub use four_char_code::*;

pub type OSStatus = i32;
pub type OSErr = c_short;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ResType(pub FourCharCode);
pub type AEEventID = ResType;
pub type AEEventClass = ResType;
pub type DescType = ResType;
pub type AEKeyword = ResType;

impl ResType {
    #[inline]
    pub fn new(value: u32) -> ResType {
        ResType(FourCharCode::new(value))
    }

    #[inline]
    pub fn to_u32(&self) -> u32 {
        self.0.to_u32()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn is_null(&self) -> bool {
        self.to_u32() == 0
    }
}

impl fmt::Display for ResType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'", self.to_string())
    }
}

impl From<FourCharCode> for ResType {
    fn from(value: FourCharCode) -> ResType {
        ResType(value)
    }
}

// impl<T> From<T> for ResType where T: Into<FourCharCode> {
//     fn from(value: T) -> ResType {
//         ResType(value.into())
//     }
// }

impl fmt::Debug for ResType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResType({:?})", self.to_string())
    }
}

#[macro_export]
macro_rules! res_type {
    ( $value:literal ) => {
        $crate::sys::ResType($crate::sys::four_char_code!($value))
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

pub const kAECoreSuite: ResType = res_type!("core");
pub const kAEGetData: ResType = res_type!("getd");
pub const kAESetData: ResType = res_type!("setd");

pub const keyDirectObject: ResType = res_type!("----");

pub const typeApplicationBundleID: ResType = res_type!("bund");
pub const typeNull: ResType = res_type!("null");
pub const typeBoolean: ResType = res_type!("bool");
pub const typeUnicodeText: ResType = res_type!("utxt");
pub const typeChar: ResType = res_type!("TEXT");
pub const typeVersion: ResType = res_type!("vers");
pub const typeWildCard: ResType = res_type!("****");
pub const typeApplSignature: ResType = res_type!("sign");
pub const typeEnumerated: ResType = res_type!("enum");
pub const typeIEEE64BitFloatingPoint: ResType = res_type!("doub");
pub const typeFloat: ResType = typeIEEE64BitFloatingPoint;
pub const typeLongFloat: ResType = typeIEEE64BitFloatingPoint;
pub const typeSInt16: ResType = res_type!("shor");
pub const typeSMInt: ResType = typeSInt16;
pub const typeShortInteger: ResType = typeSInt16;
pub const typeSInt32: ResType = res_type!("long");
pub const typeInteger: ResType = typeSInt32;
pub const typeLongInteger: ResType = typeSInt32;
pub const typeSInt64: ResType = res_type!("comp");
pub const typeComp: ResType = typeSInt64;
pub const typeType: ResType = res_type!("type");

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
        ResType::new(0)
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
