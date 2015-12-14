//! # Day 11: Corporate Policy
//!
//! Santa's previous password expired, and he needs help choosing a new one.
//!
//! To help him remember his new password after the old one expires, Santa has devised a method of
//! coming up with a password based on the previous one. Corporate policy dictates that passwords
//! must be exactly eight lowercase letters (for security reasons), so he finds his new password by
//! incrementing his old password string repeatedly until it is valid.
//!
//! Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the
//! rightmost letter one step; if it was `z`, it wraps around to `a`, and repeat with the next
//! letter to the left until one doesn't wrap around.
//!
//! Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some
//! additional password requirements:
//!
//! - Passwords must include one increasing straight of at least three letters, like `abc`, `bcd`,
//!   `cde`, and so on, up to `xyz`. They cannot skip letters; `abd` doesn't count.
//! - Passwords may not contain the letters `i`, `o`, or `l`, as these letters can be mistaken for
//!   other characters and are therefore confusing.
//! - Passwords must contain at least two different, non-overlapping pairs of letters, like `aa`,
//!   `bb`, or `zz`.
