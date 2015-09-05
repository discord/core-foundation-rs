// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation byte buffers.

use base::{CFAllocatorRef, CFIndex, CFIndexConvertible, CFRelease};
use base::{CFTypeID, TCFType, kCFAllocatorDefault};

use std::mem;
use std::ops::Deref;
use std::slice;

#[repr(C)]
struct __CFData;

pub type CFDataRef = *const __CFData;

/// A byte buffer.
pub struct CFData(CFDataRef);

impl Drop for CFData {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFData, CFDataRef, CFDataGetTypeID);

impl CFData {
    pub fn from_buffer(buffer: &[u8]) -> CFData {
        unsafe {
            let data_ref = CFDataCreate(kCFAllocatorDefault,
                                        buffer.as_ptr(),
                                        buffer.len().to_CFIndex());
            TCFType::wrap_under_create_rule(data_ref)
        }
    }

    /// Returns a pointer to the underlying bytes in this data. Note that this byte buffer is
    /// read-only.
    #[inline]
    pub fn bytes<'a>(&'a self) -> &'a [u8] {
        unsafe {
            slice::from_raw_parts(CFDataGetBytePtr(self.0), self.len() as usize)
        }
    }

    /// Returns the length of this byte buffer.
    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe {
            CFDataGetLength(self.0)
        }
    }
}

impl Deref for CFData {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.bytes()
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    /*
     * CFData.h
     */

    fn CFDataCreate(allocator: CFAllocatorRef,
                    bytes: *const u8, length: CFIndex) -> CFDataRef;
    //fn CFDataFind
    fn CFDataGetBytePtr(theData: CFDataRef) -> *const u8;
    fn CFDataGetLength(theData: CFDataRef) -> CFIndex;

    fn CFDataGetTypeID() -> CFTypeID;
}
