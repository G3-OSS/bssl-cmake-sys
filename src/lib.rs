#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::useless_transmute)]

use core::ffi::c_ulong;

// Wrap the bindgen output in a module and re-export it, so we can override it
// as needed.
mod bindgen {
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
}
pub use bindgen::*;

// bindgen does not handle C constants correctly. See
// https://github.com/rust-lang/rust-bindgen/issues/923. Work around this bug by
// redefining some constants with the correct type. Once the bindgen bug has
// been fixed, remove this.
pub const ASN1_STRFLGS_ESC_2253: c_ulong = bindgen::ASN1_STRFLGS_ESC_2253 as c_ulong;
pub const ASN1_STRFLGS_ESC_CTRL: c_ulong = bindgen::ASN1_STRFLGS_ESC_CTRL as c_ulong;
pub const ASN1_STRFLGS_ESC_MSB: c_ulong = bindgen::ASN1_STRFLGS_ESC_MSB as c_ulong;
pub const ASN1_STRFLGS_ESC_QUOTE: c_ulong = bindgen::ASN1_STRFLGS_ESC_QUOTE as c_ulong;
pub const ASN1_STRFLGS_UTF8_CONVERT: c_ulong = bindgen::ASN1_STRFLGS_UTF8_CONVERT as c_ulong;
pub const ASN1_STRFLGS_IGNORE_TYPE: c_ulong = bindgen::ASN1_STRFLGS_IGNORE_TYPE as c_ulong;
pub const ASN1_STRFLGS_SHOW_TYPE: c_ulong = bindgen::ASN1_STRFLGS_SHOW_TYPE as c_ulong;
pub const ASN1_STRFLGS_DUMP_ALL: c_ulong = bindgen::ASN1_STRFLGS_DUMP_ALL as c_ulong;
pub const ASN1_STRFLGS_DUMP_UNKNOWN: c_ulong = bindgen::ASN1_STRFLGS_DUMP_UNKNOWN as c_ulong;
pub const ASN1_STRFLGS_DUMP_DER: c_ulong = bindgen::ASN1_STRFLGS_DUMP_DER as c_ulong;
pub const ASN1_STRFLGS_RFC2253: c_ulong = bindgen::ASN1_STRFLGS_RFC2253 as c_ulong;
pub const XN_FLAG_COMPAT: c_ulong = bindgen::XN_FLAG_COMPAT as c_ulong;
pub const XN_FLAG_SEP_MASK: c_ulong = bindgen::XN_FLAG_SEP_MASK as c_ulong;
pub const XN_FLAG_SEP_COMMA_PLUS: c_ulong = bindgen::XN_FLAG_SEP_COMMA_PLUS as c_ulong;
pub const XN_FLAG_SEP_CPLUS_SPC: c_ulong = bindgen::XN_FLAG_SEP_CPLUS_SPC as c_ulong;
pub const XN_FLAG_SEP_SPLUS_SPC: c_ulong = bindgen::XN_FLAG_SEP_SPLUS_SPC as c_ulong;
pub const XN_FLAG_SEP_MULTILINE: c_ulong = bindgen::XN_FLAG_SEP_MULTILINE as c_ulong;
pub const XN_FLAG_DN_REV: c_ulong = bindgen::XN_FLAG_DN_REV as c_ulong;
pub const XN_FLAG_FN_MASK: c_ulong = bindgen::XN_FLAG_FN_MASK as c_ulong;
pub const XN_FLAG_FN_SN: c_ulong = bindgen::XN_FLAG_FN_SN as c_ulong;
pub const XN_FLAG_SPC_EQ: c_ulong = bindgen::XN_FLAG_SPC_EQ as c_ulong;
pub const XN_FLAG_DUMP_UNKNOWN_FIELDS: c_ulong = bindgen::XN_FLAG_DUMP_UNKNOWN_FIELDS as c_ulong;
pub const XN_FLAG_RFC2253: c_ulong = bindgen::XN_FLAG_RFC2253 as c_ulong;
pub const XN_FLAG_ONELINE: c_ulong = bindgen::XN_FLAG_ONELINE as c_ulong;

pub fn init() {
    // This function does nothing.
    // TODO(davidben): Remove rust-openssl's dependency on this and remove this.
}
