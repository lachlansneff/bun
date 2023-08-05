extern crate self as bun;

#[macro_use]
mod unit_system;
pub mod ops;
mod ratio;
pub mod si;

pub use crate::ratio::Ratio;

#[doc(hidden)]
pub use bun_macros::generate_unit_types;
pub use bun_macros::ratio;

pub type Mul<A, B> = <A as core::ops::Mul<B>>::Output;
pub type Div<A, B> = <A as core::ops::Div<B>>::Output;
pub type Root<U, R> = <U as ops::Root<R>>::Output;

pub trait Unit {}

pub struct Quantity<T, U: Unit> {
    val: T,
    unit: U,
}
