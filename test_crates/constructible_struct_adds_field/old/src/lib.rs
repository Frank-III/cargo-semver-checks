#![no_std]

pub struct ExhaustiveEmptyPlainStruct {}

pub struct ExhaustivePlainStruct {
    pub foo: usize,
    pub bar: i64,
}

pub struct ExhaustiveEmptyTupleStruct();

pub struct ExhaustiveTupleStruct(pub usize, pub i64);

// This struct is explicitly #[non_exhaustive] and should not be reported.
#[non_exhaustive]
pub struct NonexhaustivePlainStruct {
    pub foo: usize,
    pub bar: i64,
}

// This struct is explicitly #[non_exhaustive] and should not be reported.
#[non_exhaustive]
pub struct NonexhaustiveTupleStruct(pub usize, pub i64);

// This struct is not externally constructible due to a private field and should not be reported.
pub struct PlainStructWithPrivateField {
    pub foo: usize,
    bar: i64,
}

// This struct is not externally constructible due to a private field and should not be reported.
pub struct TupleStructWithPrivateField(pub usize, i64);

// This struct gains a private field, and thus becomes not externally constructible.
// That's a separate breaking change and should not be reported by this lint.
pub struct PlainStructGainsPrivateField {
    pub foo: usize,
    pub bar: i64,
}

// This struct gains a private field, and thus becomes not externally constructible.
// That's a separate breaking change and should not be reported by this lint.
pub struct TupleStructGainsPrivateField(pub usize, pub i64);

// This struct is externally constructible but changes kind from plain to tuple
// while also adding new public fields.
// The plain -> tuple change takes priority as a separate breaking change,
// so this should not be reported by this lint.
pub struct PlainToTupleStruct {
    pub foo: usize,
    pub bar: i64,
}

// This struct is externally constructible but changes kind from tuple to plain.
// while also adding new public fields.
// The tuple -> plain change takes priority as a separate breaking change,
// so this should not be reported by this lint.
pub struct TupleToPlainStruct(pub usize, pub i64);
