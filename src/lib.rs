extern crate self as bun;

mod ratio;

pub use crate::ratio::Ratio;

pub use bun_macros::ratio;

macro_rules! unit_system {
    ($system:ident { $($dim:ident),* }) => {
        paste::paste! {
            pub struct $system<Scale, Offset, $([<$dim:camel>]),*>(core::marker::PhantomData<(Scale, Offset, $([<$dim:camel>]),*)>);

            impl<Scale, Offset, $([<$dim:camel>]),*> Unit for Si<Scale, Offset, $([<$dim:camel>]),*> {}

            impl<
                Scale1Num: typenum::Integer, Scale1Den: typenum::Unsigned + typenum::NonZero, Scale2Num: typenum::Integer, Scale2Den: typenum::Unsigned + typenum::NonZero,
                Offset1Num: typenum::Integer, Offset1Den: typenum::Unsigned + typenum::NonZero, Offset2Num: typenum::Integer, Offset2Den: typenum::Unsigned + typenum::NonZero,
                $(
                    [<$dim:camel 1 Num>]: typenum::Integer, [<$dim:camel 1 Den>]: typenum::Unsigned + typenum::NonZero,
                    [<$dim:camel 2 Num>]: typenum::Integer, [<$dim:camel 2 Den>]: typenum::Unsigned + typenum::NonZero,
                )*
            > core::ops::Mul<$system<
                crate::ratio::Ratio<Scale2Num, Scale2Den>,
                crate::ratio::Ratio<Offset2Num, Offset2Den>,
                $( crate::ratio::Ratio<[<$dim:camel 2 Num>], [<$dim:camel 2 Den>]> ),*
            >> for $system<
                crate::ratio::Ratio<Scale1Num, Scale1Den>,
                crate::ratio::Ratio<Offset1Num, Offset1Den>,
                $( crate::ratio::Ratio<[<$dim:camel 1 Num>], [<$dim:camel 1 Den>]> ),*
            >
            where
                crate::ratio::Ratio<Scale1Num, Scale1Den>: core::ops::Mul<crate::ratio::Ratio<Scale2Num, Scale2Den>>,
                crate::ratio::Ratio<Offset1Num, Offset1Den>: core::ops::Add<crate::ratio::Ratio<Offset2Num, Offset2Den>>,
                $( crate::ratio::Ratio<[<$dim:camel 1 Num>], [<$dim:camel 1 Den>]>: core::ops::Add<crate::ratio::Ratio<[<$dim:camel 2 Num>], [<$dim:camel 2 Den>]>> ),*
            {
                type Output = $system<
                    <crate::ratio::Ratio<Scale1Num, Scale1Den> as core::ops::Mul<crate::ratio::Ratio<Scale2Num, Scale2Den>>>::Output,
                    <crate::ratio::Ratio<Offset1Num, Offset1Den> as core::ops::Add<crate::ratio::Ratio<Offset2Num, Offset2Den>>>::Output,
                    $( <crate::ratio::Ratio<[<$dim:camel 1 Num>], [<$dim:camel 1 Den>]> as core::ops::Add<crate::ratio::Ratio<[<$dim:camel 2 Num>], [<$dim:camel 2 Den>]>>>::Output ),*
                >;

                fn mul(
                    self,
                    _rhs: $system<
                        crate::ratio::Ratio<Scale2Num, Scale2Den>,
                        crate::ratio::Ratio<Offset2Num, Offset2Den>,
                        $( crate::ratio::Ratio<[<$dim:camel 2 Num>], [<$dim:camel 2 Den>]> ),*
                    >,
                ) -> Self::Output {
                    $system(core::marker::PhantomData)
                }
            }

            impl<
                Scale1Num: typenum::Integer, Scale1Den: typenum::Unsigned + typenum::NonZero, Scale2Num: typenum::Integer, Scale2Den: typenum::Unsigned + typenum::NonZero,
                Offset1Num: typenum::Integer, Offset1Den: typenum::Unsigned + typenum::NonZero, Offset2Num: typenum::Integer, Offset2Den: typenum::Unsigned + typenum::NonZero,
                $(
                    [<$dim:camel 1 Num>]: typenum::Integer, [<$dim:camel 1 Den>]: typenum::Unsigned + typenum::NonZero,
                    [<$dim:camel 2 Num>]: typenum::Integer, [<$dim:camel 2 Den>]: typenum::Unsigned + typenum::NonZero,
                )*
            > core::ops::Div<$system<
                crate::ratio::Ratio<Scale2Num, Scale2Den>,
                crate::ratio::Ratio<Offset2Num, Offset2Den>,
                $( crate::ratio::Ratio<[<$dim:camel 2 Num>], [<$dim:camel 2 Den>]> ),*
            >> for $system<
                crate::ratio::Ratio<Scale1Num, Scale1Den>,
                crate::ratio::Ratio<Offset1Num, Offset1Den>,
                $( crate::ratio::Ratio<[<$dim:camel 1 Num>], [<$dim:camel 1 Den>]> ),*
            >
            where
                crate::ratio::Ratio<Scale1Num, Scale1Den>: core::ops::Div<crate::ratio::Ratio<Scale2Num, Scale2Den>>,
                crate::ratio::Ratio<Offset1Num, Offset1Den>: core::ops::Sub<crate::ratio::Ratio<Offset2Num, Offset2Den>>,
                $( crate::ratio::Ratio<[<$dim:camel 1 Num>], [<$dim:camel 1 Den>]>: core::ops::Sub<crate::ratio::Ratio<[<$dim:camel 2 Num>], [<$dim:camel 2 Den>]>> ),*
            {
                type Output = $system<
                    <crate::ratio::Ratio<Scale1Num, Scale1Den> as core::ops::Div<crate::ratio::Ratio<Scale2Num, Scale2Den>>>::Output,
                    <crate::ratio::Ratio<Offset1Num, Offset1Den> as core::ops::Sub<crate::ratio::Ratio<Offset2Num, Offset2Den>>>::Output,
                    $( <crate::ratio::Ratio<[<$dim:camel 1 Num>], [<$dim:camel 1 Den>]> as core::ops::Sub<crate::ratio::Ratio<[<$dim:camel 2 Num>], [<$dim:camel 2 Den>]>>>::Output ),*
                >;

                fn div(
                    self,
                    _rhs: $system<
                        crate::ratio::Ratio<Scale2Num, Scale2Den>,
                        crate::ratio::Ratio<Offset2Num, Offset2Den>,
                        $( crate::ratio::Ratio<[<$dim:camel 2 Num>], [<$dim:camel 2 Den>]> ),*
                    >,
                ) -> Self::Output {
                    $system(core::marker::PhantomData)
                }
            }
        }
    };
}

unit_system!(Si { m, kg });

pub trait Unit {}

pub struct Quantity<T, U: Unit> {
    val: T,
    unit: U,
}

pub type Meter = Si<ratio!(1), ratio!(0), ratio!(1), ratio!(0)>;
