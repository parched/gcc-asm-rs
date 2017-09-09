# gcc-asm
A Rust procedural macro for GCC style inline assembly. A work in progress, not fully compatible yet. Currently requires nightly.

# Example usage
```rust

#![feature(proc_macro)]
#![feature(asm)]

extern crate gcc_asm;
use gcc_asm::gcc_asm;

fn add() {
    let a = 1;
    let b = 2;
    let c;
    unsafe {gcc_asm!("add %0, %1, %2" : "=r"(c) : "r"(a), "r"(b))};
    assert_eq!(3, c);
}
```

# Supported and planned features
- [x] `r` constraint
- [x] `i` constraint
- [x] symbolic labels
- [x] `%=`
- [ ] `m` constraint
- [ ] `volatile`
- [x] `+` constraint modifier
