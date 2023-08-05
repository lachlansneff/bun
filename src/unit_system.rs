#[macro_export]
macro_rules! unit_system {
    ($system:ident { $(( $short:ident, $long:ident ) $(,)?)* }) => {
        paste::paste! {
            pub struct $system<Scale, Offset, $([<$short:camel>]),*>(core::marker::PhantomData<(Scale, Offset, $([<$short:camel>]),*)>);

            ::bun::generate_unit_types! {
                $system ($( $long ),*)
            }

            impl<Scale, Offset, $([<$short:camel>]),*> crate::Unit for Si<Scale, Offset, $([<$short:camel>]),*> {}

            impl<
                Scale1Num: typenum::Integer, Scale1Den: typenum::Unsigned + typenum::NonZero, Scale2Num: typenum::Integer, Scale2Den: typenum::Unsigned + typenum::NonZero,
                Offset1Num: typenum::Integer, Offset1Den: typenum::Unsigned + typenum::NonZero, Offset2Num: typenum::Integer, Offset2Den: typenum::Unsigned + typenum::NonZero,
                $(
                    [<$short:camel 1 Num>]: typenum::Integer, [<$short:camel 1 Den>]: typenum::Unsigned + typenum::NonZero,
                    [<$short:camel 2 Num>]: typenum::Integer, [<$short:camel 2 Den>]: typenum::Unsigned + typenum::NonZero,
                )*
            > core::ops::Mul<$system<
                crate::ratio::Ratio<Scale2Num, Scale2Den>,
                crate::ratio::Ratio<Offset2Num, Offset2Den>,
                $( crate::ratio::Ratio<[<$short:camel 2 Num>], [<$short:camel 2 Den>]> ),*
            >> for $system<
                crate::ratio::Ratio<Scale1Num, Scale1Den>,
                crate::ratio::Ratio<Offset1Num, Offset1Den>,
                $( crate::ratio::Ratio<[<$short:camel 1 Num>], [<$short:camel 1 Den>]> ),*
            >
            where
                crate::ratio::Ratio<Scale1Num, Scale1Den>: core::ops::Mul<crate::ratio::Ratio<Scale2Num, Scale2Den>>,
                crate::ratio::Ratio<Offset1Num, Offset1Den>: core::ops::Add<crate::ratio::Ratio<Offset2Num, Offset2Den>>,
                $( crate::ratio::Ratio<[<$short:camel 1 Num>], [<$short:camel 1 Den>]>: core::ops::Add<crate::ratio::Ratio<[<$short:camel 2 Num>], [<$short:camel 2 Den>]>> ),*
            {
                type Output = $system<
                    <crate::ratio::Ratio<Scale1Num, Scale1Den> as core::ops::Mul<crate::ratio::Ratio<Scale2Num, Scale2Den>>>::Output,
                    <crate::ratio::Ratio<Offset1Num, Offset1Den> as core::ops::Add<crate::ratio::Ratio<Offset2Num, Offset2Den>>>::Output,
                    $( <crate::ratio::Ratio<[<$short:camel 1 Num>], [<$short:camel 1 Den>]> as core::ops::Add<crate::ratio::Ratio<[<$short:camel 2 Num>], [<$short:camel 2 Den>]>>>::Output ),*
                >;

                fn mul(
                    self,
                    _rhs: $system<
                        crate::ratio::Ratio<Scale2Num, Scale2Den>,
                        crate::ratio::Ratio<Offset2Num, Offset2Den>,
                        $( crate::ratio::Ratio<[<$short:camel 2 Num>], [<$short:camel 2 Den>]> ),*
                    >,
                ) -> Self::Output {
                    $system(core::marker::PhantomData)
                }
            }

            impl<
                Scale1Num: typenum::Integer, Scale1Den: typenum::Unsigned + typenum::NonZero, Scale2Num: typenum::Integer, Scale2Den: typenum::Unsigned + typenum::NonZero,
                OffsetNum: typenum::Integer, OffsetDen: typenum::Unsigned + typenum::NonZero,
                $(
                    [<$short:camel Num>]: typenum::Integer, [<$short:camel Den>]: typenum::Unsigned + typenum::NonZero,
                )*
            > core::ops::Mul<crate::ratio::Ratio<Scale2Num, Scale2Den>> for $system<
                crate::ratio::Ratio<Scale1Num, Scale1Den>,
                crate::ratio::Ratio<OffsetNum, OffsetDen>,
                $( crate::ratio::Ratio<[<$short:camel Num>], [<$short:camel Den>]> ),*
            >
            where
                crate::ratio::Ratio<Scale1Num, Scale1Den>: core::ops::Mul<crate::ratio::Ratio<Scale2Num, Scale2Den>>,
            {
                type Output = $system<
                    <crate::ratio::Ratio<Scale1Num, Scale1Den> as core::ops::Mul<crate::ratio::Ratio<Scale2Num, Scale2Den>>>::Output,
                    crate::ratio::Ratio<OffsetNum, OffsetDen>,
                    $( crate::ratio::Ratio<[<$short:camel Num>], [<$short:camel Den>]> ),*
                >;

                fn mul(
                    self,
                    _rhs: crate::ratio::Ratio<Scale2Num, Scale2Den>,
                ) -> Self::Output {
                    $system(core::marker::PhantomData)
                }
            }

            impl<
                Scale1Num: typenum::Integer, Scale1Den: typenum::Unsigned + typenum::NonZero, Scale2Num: typenum::Integer, Scale2Den: typenum::Unsigned + typenum::NonZero,
                Offset1Num: typenum::Integer, Offset1Den: typenum::Unsigned + typenum::NonZero, Offset2Num: typenum::Integer, Offset2Den: typenum::Unsigned + typenum::NonZero,
                $(
                    [<$short:camel 1 Num>]: typenum::Integer, [<$short:camel 1 Den>]: typenum::Unsigned + typenum::NonZero,
                    [<$short:camel 2 Num>]: typenum::Integer, [<$short:camel 2 Den>]: typenum::Unsigned + typenum::NonZero,
                )*
            > core::ops::Div<$system<
                crate::ratio::Ratio<Scale2Num, Scale2Den>,
                crate::ratio::Ratio<Offset2Num, Offset2Den>,
                $( crate::ratio::Ratio<[<$short:camel 2 Num>], [<$short:camel 2 Den>]> ),*
            >> for $system<
                crate::ratio::Ratio<Scale1Num, Scale1Den>,
                crate::ratio::Ratio<Offset1Num, Offset1Den>,
                $( crate::ratio::Ratio<[<$short:camel 1 Num>], [<$short:camel 1 Den>]> ),*
            >
            where
                crate::ratio::Ratio<Scale1Num, Scale1Den>: core::ops::Div<crate::ratio::Ratio<Scale2Num, Scale2Den>>,
                crate::ratio::Ratio<Offset1Num, Offset1Den>: core::ops::Sub<crate::ratio::Ratio<Offset2Num, Offset2Den>>,
                $( crate::ratio::Ratio<[<$short:camel 1 Num>], [<$short:camel 1 Den>]>: core::ops::Sub<crate::ratio::Ratio<[<$short:camel 2 Num>], [<$short:camel 2 Den>]>> ),*
            {
                type Output = $system<
                    <crate::ratio::Ratio<Scale1Num, Scale1Den> as core::ops::Div<crate::ratio::Ratio<Scale2Num, Scale2Den>>>::Output,
                    <crate::ratio::Ratio<Offset1Num, Offset1Den> as core::ops::Sub<crate::ratio::Ratio<Offset2Num, Offset2Den>>>::Output,
                    $( <crate::ratio::Ratio<[<$short:camel 1 Num>], [<$short:camel 1 Den>]> as core::ops::Sub<crate::ratio::Ratio<[<$short:camel 2 Num>], [<$short:camel 2 Den>]>>>::Output ),*
                >;

                fn div(
                    self,
                    _rhs: $system<
                        crate::ratio::Ratio<Scale2Num, Scale2Den>,
                        crate::ratio::Ratio<Offset2Num, Offset2Den>,
                        $( crate::ratio::Ratio<[<$short:camel 2 Num>], [<$short:camel 2 Den>]> ),*
                    >,
                ) -> Self::Output {
                    $system(core::marker::PhantomData)
                }
            }
        }
    };
}
