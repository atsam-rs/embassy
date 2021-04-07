#![no_std]
#![feature(generic_associated_types)]
#![feature(asm)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

#[cfg(feature = "atsam4e16e")]
pub use {atsam4_hal as hal, atsam4_hal::atsam4e16e as pac};

#[cfg(feature = "atsam4sd32c")]
pub use {atsam4_hal as hal, atsam4_hal::atsam4sd32c as pac};
