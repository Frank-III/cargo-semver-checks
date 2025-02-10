// Public repr(C) struct with reordered fields - should trigger warning
#[repr(C)]
pub struct PublicStruct {
    pub b: u16, // moved
    pub c: u32, // moved
    pub a: u8,  // moved
}

// Private repr(C) struct with reordered fields - should not trigger
#[repr(C)]
struct PrivateStruct {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Public non-repr(C) struct with reordered fields - should not trigger
pub struct RegularStruct {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Public repr(C) struct with doc(hidden) - should not trigger
#[doc(hidden)]
#[repr(C)]
pub struct DocHiddenStruct {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Public repr(C) struct with mixed visibility fields
#[repr(C)]
pub struct MixedVisibilityStruct {
    b: u16,     // private field moved
    pub c: u32, // moved
    pub a: u8,  // moved
}

// Public repr(C) struct with same order - should not trigger
#[repr(C)]
pub struct UnchangedStruct {
    pub x: u8,  // same position
    pub y: u16, // same position
}

// Test multiple repr attributes
#[repr(C, align(8))]
pub struct MultipleReprStruct {
    pub b: u16, // moved
    pub a: u8,  // moved
}
