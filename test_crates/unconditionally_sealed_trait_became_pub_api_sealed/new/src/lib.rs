#![no_std]

// Traits transitioning from Unconditionally Sealed -> Public API Sealed (Lint should detect these)
pub mod unconditionally_sealed_to_public_api_sealed {
    #[doc(hidden)]
    pub mod hidden {
        pub trait Sealed {}
        pub struct Token;
    }

    pub trait ExtendsTraitInHiddenModule: hidden::Sealed {}

    pub trait ReturnsTraitInHiddenModule {
        fn method(&self) -> hidden::Token;
    }

    pub trait AcceptsTraitInHiddenModule {
        fn method(&self, token: hidden::Token);
    }

    pub trait SealedWithWhereSelfBound
    where
        Self: hidden::Sealed,
    {
    }
}
