#![allow(dead_code)]

use crate::sys::*;
pub use crate::sys::{AEDesc, ResType};
use encoding::all::{ASCII, UTF_16LE};
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use libc::{c_char, c_uint, c_void, strlen};
use std::ffi::CString;
use std::fmt;
use std::io::{Error, ErrorKind, Result};

pub trait AutoPropertyType: Sized {
    fn read(reader: EventPropertyReader) -> Result<Option<Self>>;

    fn to_desc(&self) -> Result<AEDesc> {
        unimplemented!();
    }
}

#[macro_export]
macro_rules! build_desc {
    ( $type:ident, $( $params:expr ),+ ) => {{
        let mut event: $crate::sys::AppleEvent = std::default::Default::default();
        let mut err: $crate::sys::AEBuildError = std::default::Default::default();
        match std::ffi::CString::new(format!("{}(@)", $type)) {
            Ok(format) => {
                let res = unsafe {
                    $crate::sys::AEBuildDesc(&mut event, &mut err, format.as_ptr() as *const _, $( $params ),+)
                };

                if res == 0 {
                    Ok(event)
                } else {
                    Err($crate::events::EventBuildError::new(err))
                }
            },
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
        }


    }}
}

#[macro_export]
macro_rules! call {
    ( $obj:ident, $type:ident ) => {{
        let query = b"'----':'null'()\0" as *const u8;

        let mut event: $crate::sys::AppleEvent = std::default::Default::default();
        let mut err: $crate::sys::AEBuildError = std::default::Default::default();

        let bundle_id = $crate::events::EventedObject::bundle_id($obj);

        let res = unsafe {
            $crate::sys::AEBuildAppleEvent(
                $crate::events::EventedObject::signature($obj),
                $type,
                $crate::sys::typeApplicationBundleID,
                bundle_id as *const $crate::sys::c_void,
                $crate::sys::strlen(bundle_id),
                $crate::sys::kAutoGenerateReturnID,
                $crate::sys::kAnyTransactionID,
                &mut event,
                &mut err,
                query
            )
        };

        if res == 0 {
            let res = unsafe {
                $crate::sys::AESendMessage(
                    &event,
                    std::ptr::null_mut(),
                    $crate::sys::kAENoReply | $crate::sys::kAENeverInteract,
                    $crate::sys::kAEDefaultTimeout,
                )
            };

            if res == 0 {
                Ok(())
            } else {
                Err(std::io::Error::from_raw_os_error(res))
            }
        } else {
            Err($crate::events::EventBuildError::new(err))
        }
    }};

    ( $obj:ident, $type:ident, $param:expr ) => {{
        let query = b"'----':@\0" as *const u8;

        let mut event: $crate::sys::AppleEvent = std::default::Default::default();
        let mut err: $crate::sys::AEBuildError = std::default::Default::default();

        let bundle_id = $crate::events::EventedObject::bundle_id($obj);

        let res = unsafe {
            $crate::sys::AEBuildAppleEvent(
                $crate::events::EventedObject::signature($obj),
                $type,
                $crate::sys::typeApplicationBundleID,
                bundle_id as *const $crate::sys::c_void,
                $crate::sys::strlen(bundle_id),
                $crate::sys::kAutoGenerateReturnID,
                $crate::sys::kAnyTransactionID,
                &mut event,
                &mut err,
                query,
                &$crate::events::EventPropertyType::to_desc($param)?
            )
        };

        if res == 0 {
            let res = unsafe {
                $crate::sys::AESendMessage(
                    &event,
                    std::ptr::null_mut(),
                    $crate::sys::kAENoReply | $crate::sys::kAENeverInteract,
                    $crate::sys::kAEDefaultTimeout,
                )
            };

            if res == 0 {
                Ok(())
            } else {
                Err(std::io::Error::from_raw_os_error(res))
            }
        } else {
            Err($crate::events::EventBuildError::new(err))
        }
    }};

    ( $obj:ident, $type:ident, $param:expr, $( $ts:ident : $pars:expr ),+ ) => {{
        let query = std::ffi::CString::new(vec!["'----':@".to_string(), $( format!("{}:@", $ts) ),+].join(", "))?;

        let mut event: $crate::sys::AppleEvent = std::default::Default::default();
        let mut err: $crate::sys::AEBuildError = std::default::Default::default();

        let bundle_id = $crate::events::EventedObject::bundle_id($obj);

        let res = unsafe {
            $crate::sys::AEBuildAppleEvent(
                $crate::events::EventedObject::signature($obj),
                $type,
                $crate::sys::typeApplicationBundleID,
                bundle_id as *const $crate::sys::c_void,
                $crate::sys::strlen(bundle_id),
                $crate::sys::kAutoGenerateReturnID,
                $crate::sys::kAnyTransactionID,
                &mut event,
                &mut err,
                query.as_ptr() as *const u8,
                &$crate::events::EventPropertyType::to_desc($param)?,
                $(
                    &$crate::events::EventPropertyType::to_desc($pars)?
                ),+
            )
        };

        if res == 0 {
            let res = unsafe {
                $crate::sys::AESendMessage(
                    &event,
                    std::ptr::null_mut(),
                    $crate::sys::kAENoReply | $crate::sys::kAENeverInteract,
                    $crate::sys::kAEDefaultTimeout,
                )
            };

            if res == 0 {
                Ok(())
            } else {
                Err(std::io::Error::from_raw_os_error(res))
            }
        } else {
            Err($crate::events::EventBuildError::new(err))
        }
    }}
}

pub trait EventEnum: Sized + Copy + Into<u32> {
    fn from_int(value: u32) -> Self;

    fn from_res_type(res_type: ResType) -> Self {
        EventEnum::from_int(res_type.to_u32())
    }

    fn to_int(self) -> u32 {
        Into::<u32>::into(self)
    }

    fn to_res_type(self) -> ResType {
        ResType::new(EventEnum::to_int(self))
    }

    fn read(reader: EventPropertyReader) -> Result<Option<Self>> {
        let descriptor = reader.property_descriptor()?;

        if descriptor.prop_type() == typeNull {
            Ok(None)
        } else if descriptor.prop_type() == typeEnumerated {
            let mut buffer: u32 = Default::default();
            let tmp = descriptor.size();

            match descriptor.read(typeEnumerated, &mut buffer as *mut u32 as *mut c_void, tmp) {
                Ok(_desc) => Ok(Some(Self::from_int(buffer))),
                Err((err, _)) => Err(err),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Value cannot be read as enum",
            ))
        }
    }

    fn to_desc(&self) -> Result<AEDesc> {
        build_desc!(typeEnumerated, self.to_int())
    }
}

#[derive(Debug)]
pub struct EventBuildError {
    code: u32,
    pos: u32,
}

impl EventBuildError {
    pub fn new(err: AEBuildError) -> Error {
        Error::new(
            ErrorKind::InvalidInput,
            EventBuildError {
                code: err.fError,
                pos: err.fErrorPos,
            },
        )
    }

    pub fn code(&self) -> u32 {
        self.code
    }

    pub fn position(&self) -> u32 {
        self.pos
    }

    pub fn pos(&self) -> u32 {
        self.pos
    }
}

impl std::error::Error for EventBuildError {
    fn description(&self) -> &str {
        "An error occurred while building the event"
    }
}

impl fmt::Display for EventBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {} occurred while building event at position {}",
            self.code(),
            self.pos()
        )
    }
}

fn property_getter_format(target_object: &AEDesc, desc_type: DescType) -> Result<CString> {
    match CString::new(format!(
        "'----':obj {{ form:prop, want:type(prop), seld:type({}), from:{} }}",
        desc_type,
        if target_object.is_null() {
            "'null'()"
        } else {
            "@"
        },
    )) {
        Ok(cstr) => Ok(cstr),
        Err(err) => Err(Error::new(ErrorKind::InvalidInput, err)),
    }
}

fn property_setter_format(target_object: &AEDesc, desc_type: DescType) -> Result<CString> {
    match CString::new(format!(
        "data:@, '----':obj {{ form:prop, want:type(prop), seld:type({}), from:{} }}",
        desc_type,
        if target_object.is_null() {
            "'null'()"
        } else {
            "@"
        },
    )) {
        Ok(cstr) => Ok(cstr),
        Err(err) => Err(Error::new(ErrorKind::InvalidInput, err)),
    }
}

#[derive(Debug)]
pub struct PropertyDescriptor {
    event: AppleEvent,
    prop_type: DescType,
    size: usize,
}

impl PropertyDescriptor {
    fn new(event: AppleEvent) -> Result<PropertyDescriptor> {
        let mut prop_type: DescType = Default::default();
        let mut size: usize = Default::default();

        let res = unsafe { AESizeOfParam(&event, keyDirectObject, &mut prop_type, &mut size) };

        if res == 0 {
            Ok(PropertyDescriptor {
                event,
                prop_type,
                size,
            })
        } else {
            Err(Error::from_raw_os_error(res.into()))
        }
    }

    pub fn prop_type(&self) -> DescType {
        self.prop_type
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn read(
        self,
        buffer_type: DescType,
        buffer: *mut c_void,
        buffer_size: usize,
    ) -> std::result::Result<PropertyDescriptor, (Error, PropertyDescriptor)> {
        let mut prop_type: DescType = Default::default();
        let mut size: usize = Default::default();

        let res = unsafe {
            AEGetParamPtr(
                &self.event,
                keyDirectObject,
                buffer_type,
                &mut prop_type,
                buffer,
                buffer_size,
                &mut size,
            )
        };

        if res == 0 {
            Ok(self)
        } else {
            Err((
                Error::from_raw_os_error(res.into()),
                PropertyDescriptor {
                    event: self.event,
                    prop_type,
                    size,
                },
            ))
        }
    }
}

#[derive(Debug)]
pub struct EventPropertyReader<'a> {
    signature: ResType,
    bundle_id: *const c_char,
    target_object: &'a AEDesc,
    query: *const u8,
}

fn compose_and_read_getter_event<'a>(reader: &'a EventPropertyReader) -> Result<AppleEvent> {
    let mut event: AppleEvent = Default::default();
    let mut err: AEBuildError = Default::default();

    let res = unsafe {
        if reader.target_object.is_null() {
            AEBuildAppleEvent(
                kAECoreSuite,
                kAEGetData,
                typeApplicationBundleID,
                reader.bundle_id as *const c_void,
                strlen(reader.bundle_id),
                kAutoGenerateReturnID,
                kAnyTransactionID,
                &mut event,
                &mut err,
                reader.query,
            )
        } else {
            AEBuildAppleEvent(
                kAECoreSuite,
                kAEGetData,
                typeApplicationBundleID,
                reader.bundle_id as *const c_void,
                strlen(reader.bundle_id),
                kAutoGenerateReturnID,
                kAnyTransactionID,
                &mut event,
                &mut err,
                reader.query,
                reader.target_object,
            )
        }
    };

    if res == 0 {
        let mut reply: AppleEvent = Default::default();
        let res = unsafe {
            AESendMessage(
                &event,
                &mut reply,
                kAEWaitReply | kAENeverInteract,
                kAEDefaultTimeout,
            )
        };

        if res == 0 {
            Ok(reply)
        } else {
            Err(Error::from_raw_os_error(res.into()))
        }
    } else {
        Err(EventBuildError::new(err))
    }
}

impl<'a> EventPropertyReader<'a> {
    fn new(
        signature: ResType,
        bundle_id: *const c_char,
        target_object: &'a AEDesc,
        property: DescType,
    ) -> Result<EventPropertyReader<'a>> {
        Ok(EventPropertyReader {
            signature,
            bundle_id,
            target_object,
            query: property_getter_format(target_object, property)?.into_raw() as *const u8,
        })
    }

    pub fn property_descriptor(&self) -> Result<PropertyDescriptor> {
        PropertyDescriptor::new(compose_and_read_getter_event(self)?)
    }

    pub fn descriptor(&self) -> Result<AEDesc> {
        let event = compose_and_read_getter_event(self)?;
        let mut object: AEDesc = Default::default();

        let res = unsafe { AEGetParamDesc(&event, keyDirectObject, typeWildCard, &mut object) };

        if res == 0 {
            Ok(object)
        } else {
            Err(Error::from_raw_os_error(res.into()))
        }
    }

    pub fn signature(&self) -> ResType {
        self.signature
    }

    pub fn bundle_id(&self) -> *const c_char {
        self.bundle_id
    }
}

#[derive(Debug)]
pub struct EventPropertyWriter<'a> {
    signature: ResType,
    bundle_id: *const c_char,
    target_object: &'a AEDesc,
    query: *const u8,
}

impl<'a> EventPropertyWriter<'a> {
    fn new(
        signature: ResType,
        bundle_id: *const c_char,
        target_object: &'a AEDesc,
        property: DescType,
    ) -> Result<EventPropertyWriter<'a>> {
        Ok(EventPropertyWriter {
            signature,
            bundle_id,
            target_object,
            query: property_setter_format(target_object, property)?.into_raw() as *const u8,
        })
    }

    fn write(&self, property: AEDesc) -> Result<()> {
        let mut event: AppleEvent = Default::default();
        let mut err: AEBuildError = Default::default();

        let res = unsafe {
            if self.target_object.is_null() {
                AEBuildAppleEvent(
                    kAECoreSuite,
                    kAESetData,
                    typeApplicationBundleID,
                    self.bundle_id as *const c_void,
                    strlen(self.bundle_id),
                    kAutoGenerateReturnID,
                    kAnyTransactionID,
                    &mut event,
                    &mut err,
                    self.query,
                    &property,
                )
            } else {
                AEBuildAppleEvent(
                    kAECoreSuite,
                    kAESetData,
                    typeApplicationBundleID,
                    self.bundle_id as *const c_void,
                    strlen(self.bundle_id),
                    kAutoGenerateReturnID,
                    kAnyTransactionID,
                    &mut event,
                    &mut err,
                    self.query,
                    &property,
                    self.target_object,
                )
            }
        };

        if res == 0 {
            let res = unsafe {
                AESendMessage(&event, std::ptr::null_mut(), kAENoReply, kAEDefaultTimeout)
            };

            if res == 0 {
                Ok(())
            } else {
                Err(Error::from_raw_os_error(res.into()))
            }
        } else {
            Err(EventBuildError::new(err))
        }
    }
}

pub trait EventPropertyType: Sized {
    fn read(reader: EventPropertyReader) -> Result<Option<Self>>;

    fn to_desc(&self) -> Result<AEDesc> {
        unimplemented!();
    }

    fn write(&self, writer: EventPropertyWriter) -> Result<()> {
        writer.write(self.to_desc()?)
    }
}

impl EventPropertyType for String {
    fn read(reader: EventPropertyReader) -> Result<Option<String>> {
        let descriptor = reader.property_descriptor()?;

        if descriptor.prop_type() == typeUnicodeText || descriptor.prop_type() == typeVersion {
            let mut buffer: Vec<u8> = Vec::with_capacity(descriptor.size());
            let tmp = descriptor.size();
            match descriptor.read(typeUnicodeText, buffer.as_mut_ptr() as *mut c_void, tmp) {
                Ok(desc) => {
                    unsafe { buffer.set_len(desc.size()) };
                    match UTF_16LE.decode(&buffer[..], DecoderTrap::Strict) {
                        Ok(str) => Ok(Some(str)),
                        Err(err) => Err(Error::new(ErrorKind::InvalidInput, err)),
                    }
                }
                Err((err, _)) => Err(err),
            }
        } else if descriptor.prop_type() == typeChar {
            let mut buffer: Vec<u8> = Vec::with_capacity(descriptor.size());
            let tmp = descriptor.size();
            match descriptor.read(typeChar, buffer.as_mut_ptr() as *mut c_void, tmp) {
                Ok(desc) => {
                    unsafe { buffer.set_len(desc.size()) };
                    match ASCII.decode(&buffer[..], DecoderTrap::Strict) {
                        Ok(str) => Ok(Some(str)),
                        Err(err) => Err(Error::new(ErrorKind::InvalidInput, err)),
                    }
                }
                Err((err, _)) => Err(err),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Value cannot be read as String",
            ))
        }
    }

    fn to_desc(&self) -> Result<AEDesc> {
        match UTF_16LE.encode(self, EncoderTrap::Strict) {
            Ok(vec) => build_desc!(typeUnicodeText, vec.len(), vec.as_ptr()),
            Err(err) => Err(Error::new(ErrorKind::InvalidInput, err)),
        }
    }
}

impl EventPropertyType for bool {
    fn read(reader: EventPropertyReader) -> Result<Option<bool>> {
        let descriptor = reader.property_descriptor()?;

        if descriptor.prop_type() == typeNull {
            Ok(None)
        } else if descriptor.prop_type() == typeBoolean {
            let mut buffer: bool = Default::default();
            let tmp = descriptor.size();

            match descriptor.read(typeBoolean, &mut buffer as *mut bool as *mut c_void, tmp) {
                Ok(_desc) => Ok(Some(buffer)),
                Err((err, _)) => Err(err),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Value cannot be read as bool",
            ))
        }
    }

    fn to_desc(&self) -> Result<AEDesc> {
        build_desc!(typeBoolean, *self as c_uint)
    }
}

impl EventPropertyType for f64 {
    fn read(reader: EventPropertyReader) -> Result<Option<f64>> {
        let descriptor = reader.property_descriptor()?;

        if descriptor.prop_type() == typeNull {
            Ok(None)
        } else if descriptor.prop_type() == typeFloat {
            let mut buffer: f64 = Default::default();
            let tmp = descriptor.size();

            match descriptor.read(typeFloat, &mut buffer as *mut f64 as *mut c_void, tmp) {
                Ok(_desc) => Ok(Some(buffer)),
                Err((err, _)) => Err(err),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Value cannot be read as f64",
            ))
        }
    }

    fn to_desc(&self) -> Result<AEDesc> {
        build_desc!(typeFloat, *self)
    }
}

impl EventPropertyType for i16 {
    fn read(reader: EventPropertyReader) -> Result<Option<i16>> {
        let descriptor = reader.property_descriptor()?;

        if descriptor.prop_type() == typeNull {
            Ok(None)
        } else if descriptor.prop_type() == typeSInt16 {
            let mut buffer: i16 = Default::default();
            let tmp = descriptor.size();

            match descriptor.read(typeSInt16, &mut buffer as *mut i16 as *mut c_void, tmp) {
                Ok(_desc) => Ok(Some(buffer)),
                Err((err, _)) => Err(err),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Value cannot be read as i16",
            ))
        }
    }

    fn to_desc(&self) -> Result<AEDesc> {
        build_desc!(typeSInt16, *self as c_uint)
    }
}

impl EventPropertyType for i32 {
    fn read(reader: EventPropertyReader) -> Result<Option<i32>> {
        let descriptor = reader.property_descriptor()?;

        if descriptor.prop_type() == typeNull {
            Ok(None)
        } else if descriptor.prop_type() == typeSInt32 {
            let mut buffer: i32 = Default::default();
            let tmp = descriptor.size();

            match descriptor.read(typeSInt32, &mut buffer as *mut i32 as *mut c_void, tmp) {
                Ok(_desc) => Ok(Some(buffer)),
                Err((err, _)) => Err(err),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Value cannot be read as i32",
            ))
        }
    }

    fn to_desc(&self) -> Result<AEDesc> {
        build_desc!(typeSInt32, *self)
    }
}

impl EventPropertyType for i64 {
    fn read(reader: EventPropertyReader) -> Result<Option<i64>> {
        let descriptor = reader.property_descriptor()?;

        if descriptor.prop_type() == typeNull {
            Ok(None)
        } else if descriptor.prop_type() == typeSInt64 {
            let mut buffer: i64 = Default::default();
            let tmp = descriptor.size();

            match descriptor.read(typeSInt64, &mut buffer as *mut i64 as *mut c_void, tmp) {
                Ok(_desc) => Ok(Some(buffer)),
                Err((err, _)) => Err(err),
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Value cannot be read as i64",
            ))
        }
    }

    fn to_desc(&self) -> Result<AEDesc> {
        build_desc!(typeSInt64, *self)
    }
}

// impl EventPropertyType for Vec<u8> {
//     fn read(reader: EventPropertyReader) -> Result<Option<Vec<u8>>> {
//         let descriptor = reader.property_descriptor()?;
//
//         if descriptor.prop_type() == typeNull {
//             Ok(None)
//         } else if descriptor.prop_type() == typeType {
//             let mut buffer: Vec<u8> = Vec::with_capacity(descriptor.size());
//             let tmp = descriptor.size();
//
//             match descriptor.read(typeType, buffer.as_mut_ptr() as *mut c_void, tmp) {
//                 Ok(desc) => {
//                     unsafe { buffer.set_len(tmp) };
//                     Ok(Some(buffer))
//                 },
//                 Err((err, _)) => Err(err),
//             }
//         } else {
//             Err(Error::new(
//                 ErrorKind::InvalidInput,
//                 "Value cannot be read as Vec<u8>",
//             ))
//         }
//     }
// }

impl<T: AutoPropertyType> EventPropertyType for T {
    fn read(reader: EventPropertyReader) -> Result<Option<T>> {
        <T as AutoPropertyType>::read(reader)
    }
}

pub trait EventedObject {
    fn signature(&self) -> ResType;

    fn bundle_id(&self) -> *const c_char;

    fn target_object(&self) -> &AEDesc;

    fn set_property<T: EventPropertyType>(&self, property: DescType, value: &T) -> Result<()> {
        EventPropertyType::write(
            value,
            EventPropertyWriter::new(
                EventedObject::signature(self),
                EventedObject::bundle_id(self),
                self.target_object(),
                property,
            )?,
        )
    }

    fn get_property<T: EventPropertyType>(&self, property: DescType) -> Result<Option<T>> {
        EventPropertyType::read(EventPropertyReader::new(
            EventedObject::signature(self),
            EventedObject::bundle_id(self),
            self.target_object(),
            property,
        )?)
    }
}

pub trait EventedRootObject: EventedObject {}

pub trait EventedSubObject: EventedObject + AutoPropertyType {
    fn instantiate(signature: ResType, bundle_id: *const c_char, target_object: AEDesc) -> Self;

    fn read(reader: EventPropertyReader) -> Result<Option<Self>> {
        let descriptor = reader.descriptor()?;

        Ok(Some(Self::instantiate(
            reader.signature(),
            reader.bundle_id(),
            descriptor,
        )))
    }
}
