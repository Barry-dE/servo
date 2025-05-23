/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::str::DOMString;

#[allow(non_snake_case)]
pub(crate) fn Product() -> DOMString {
    DOMString::from("Gecko")
}

#[allow(non_snake_case)]
pub(crate) fn ProductSub() -> DOMString {
    DOMString::from("20100101")
}

#[allow(non_snake_case)]
pub(crate) fn Vendor() -> DOMString {
    DOMString::from("")
}

#[allow(non_snake_case)]
pub(crate) fn VendorSub() -> DOMString {
    DOMString::from("")
}

#[allow(non_snake_case)]
pub(crate) fn TaintEnabled() -> bool {
    false
}

#[allow(non_snake_case)]
pub(crate) fn AppName() -> DOMString {
    DOMString::from("Netscape") // Like Gecko/Webkit
}

#[allow(non_snake_case)]
pub(crate) fn AppCodeName() -> DOMString {
    DOMString::from("Mozilla")
}

#[allow(non_snake_case)]
#[cfg(target_os = "windows")]
pub(crate) fn Platform() -> DOMString {
    DOMString::from("Win32")
}

#[allow(non_snake_case)]
#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn Platform() -> DOMString {
    DOMString::from("Linux")
}

#[allow(non_snake_case)]
#[cfg(target_os = "macos")]
pub(crate) fn Platform() -> DOMString {
    DOMString::from("Mac")
}

#[allow(non_snake_case)]
#[cfg(target_os = "ios")]
pub(crate) fn Platform() -> DOMString {
    DOMString::from("iOS")
}

#[allow(non_snake_case)]
pub(crate) fn UserAgent(user_agent: &str) -> DOMString {
    DOMString::from(user_agent)
}

#[allow(non_snake_case)]
pub(crate) fn AppVersion() -> DOMString {
    DOMString::from("4.0")
}

#[allow(non_snake_case)]
pub(crate) fn Language() -> DOMString {
    DOMString::from("en-US")
}
