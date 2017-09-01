//   Copyright 2017 James Duley
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

#![feature(proc_macro)]
#![feature(asm)]

extern crate gcc_asm;

use gcc_asm::gcc_asm;


#[test]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
fn test_nop() {
    // unsafe {asm!("nop")};
    unsafe {gcc_asm!("nop")};
}

#[test]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
fn test_add() {
    let a = 1;
    let b = 2;
    let c;
    // unsafe {asm!("add $0, $1, $2" : "=r"(c), : "r"(a), "r"(b),)};
    unsafe {gcc_asm!("add %0, %1, %2" : "=r"(c) : "r"(a), "r"(b))};
    assert_eq!(3, c);
}

#[test]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
fn test_add_multi_string() {
    let a = 1;
    let b = 2;
    let c;
    // unsafe {asm!("add $0, $1, $2" : "=r"(c), : "r"(a), "r"(b),)};
    unsafe {gcc_asm!("add %0, ""%1, %2" : "=r"(c) : "r"(a), "r"(b))};
    assert_eq!(3, c);
}

#[test]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
fn test_add_symbolic() {
    let a = 1;
    let b = 2;
    let c;
    // unsafe {asm!("add $0, $1, $2" : "=r"(c), : "r"(a), "r"(b),)};
    unsafe {gcc_asm!("add %[c], %[a], %[b]" : [c]"=r"(c) : [a]"r"(a), [b]"r"(b))};
    assert_eq!(3, c);
}

#[test]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
fn test_uid() {
    // unsafe {asm!("b unique_label${:uid}\n\tnop\nunqiue_label${uid}")};
    unsafe {gcc_asm!("b unique_label%=\nnop\nunique_label%=:")};
}
