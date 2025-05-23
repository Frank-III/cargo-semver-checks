#![no_std]

#[repr(u8)]
pub enum U8ToU16Enum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum I32ToI8Enum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum I32ToU32Enum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum CU8ToCU16Enum {
    Bar,
    Baz,
}

// The following enums have changes that can be breaking on some systems.
// Since there are no guarantees on what particular system a given crate
// might run on, this is a breaking change.

#[repr(u64)]
pub enum U64ToUsizeEnum {
    Bar,
    Baz,
}

#[repr(usize)]
pub enum UsizeToU64Enum {
    Bar,
    Baz,
}

#[repr(i64)]
pub enum I64ToIsizeEnum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum IsizeToI64Enum {
    Bar,
    Baz,
}

#[repr(usize)]
pub enum UsizeToIsizeEnum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum IsizeToUsizeEnum {
    Bar,
    Baz,
}


// The following enums have *removals* of repr(i*) and repr(u*),
// not changes to another repr(i*) or repr(u*).
// They should not be reported by this rule, because they have their own rule.

#[repr(u8)]
pub enum U8EnumToEnum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum I32EnumToEnum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum IsizeEnumToEnum {
    Bar,
    Baz,
}

#[repr(usize)]
pub enum UsizeEnumToEnum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum CU8EnumToCEnum {
    Bar,
    Baz,
}
