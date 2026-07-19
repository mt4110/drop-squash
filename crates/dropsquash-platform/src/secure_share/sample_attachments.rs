use std::ffi::c_void;

use objc2::runtime::AnyObject;
use objc2_core_foundation::{CFDictionary, CFString, CFType};
use objc2_core_media::CMSampleBuffer;
use objc2_foundation::{NSNumber, NSString};

pub fn has(sample_buffer: &CMSampleBuffer, key: &NSString) -> bool {
    value(sample_buffer, key).is_some()
}

pub fn value<'a>(sample_buffer: &'a CMSampleBuffer, key: &NSString) -> Option<&'a AnyObject> {
    let dict = first_sample_attachment_dictionary(sample_buffer)?;
    let key = cf_string_key(key);
    let value = unsafe { dict.value(key) };
    if value.is_null() {
        return None;
    }
    Some(unsafe { &*value.cast::<AnyObject>() })
}

fn first_sample_attachment_dictionary(sample_buffer: &CMSampleBuffer) -> Option<&CFDictionary> {
    let attachments = unsafe { sample_buffer.sample_attachments_array(false) }?;
    if attachments.count() <= 0 {
        return None;
    }
    let value = unsafe { attachments.value_at_index(0) };
    if value.is_null() {
        return None;
    }
    Some(unsafe { &*value.cast::<CFDictionary>() })
}

fn cf_string_key(key: &NSString) -> *const c_void {
    let key = unsafe { &*(key as *const NSString as *const CFString) };
    key as *const CFString as *const c_void
}

pub fn describe(value: &AnyObject) -> String {
    let value = unsafe { &*(value as *const AnyObject as *const CFType) };
    format!("{value:?}")
}

pub fn dictionary_number(value: &AnyObject, key: &str) -> Option<f64> {
    let dict = cf_type(value).downcast_ref::<CFDictionary>()?;
    let key = NSString::from_str(key);
    let value = unsafe { dict.value(cf_string_key(&key)) };
    if value.is_null() {
        return None;
    }
    unsafe { &*value.cast::<AnyObject>() }
        .downcast_ref::<NSNumber>()
        .map(|number| f64::from(number.as_f32()))
}

fn cf_type(value: &AnyObject) -> &CFType {
    unsafe { &*(value as *const AnyObject as *const CFType) }
}
