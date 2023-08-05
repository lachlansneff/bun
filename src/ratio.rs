use core::marker::PhantomData;
use core::ops;

use typ::typ;
use typenum::{
    type_operators::{Abs, Gcd},
    Bit, Integer, NInt, NonZero, PInt, Unsigned, U1,
};

#[derive(Default)]
pub struct Ratio<Num: Integer, Den: Unsigned + NonZero = U1>(PhantomData<(Num, Den)>);

impl<Num: Integer, Den: Unsigned + NonZero> Ratio<Num, Den> {
    pub const NUM: i32 = <Num as Integer>::I32;
    pub const DEN: u32 = <Den as Unsigned>::U32;
}

macro_rules! impl_rational_op {
    ($op:ident) => {
        paste::paste! {
            impl<Num1: Integer, Den1: Unsigned + NonZero, Num2: Integer, Den2: Unsigned + NonZero>
                ops::[<$op:camel>]<Ratio<Num2, Den2>> for Ratio<Num1, Den1>
            where
                (): [<Ratio $op:camel>]<Ratio<Num1, Den1>, Ratio<Num2, Den2>>,
            {
                type Output = [<Ratio $op:camel Op>]<Ratio<Num1, Den1>, Ratio<Num2, Den2>>;

                fn [<$op:lower>](self, _rhs: Ratio<Num2, Den2>) -> Self::Output {
                    Default::default()
                }
            }

            // impl<Num: Integer, Den: Unsigned + NonZero, I: Integer> ops::Add<I> for Ratio<Num, Den>
            // where
            //     (): RatioAdd<Ratio<Num, Den>, Ratio<I, U1>>,
            // {
            //     type Output = RatioAddOp<Ratio<Num, Den>, Ratio<I, U1>>;

            //     fn add(self, _rhs: I) -> Self::Output {
            //         Default::default()
            //     }
            // }
        }
    };
}

impl_rational_op!(add);
impl_rational_op!(sub);
impl_rational_op!(mul);
impl_rational_op!(div);

typ! {
    fn ExtractUnsigned<uint: Unsigned + NonZero>(PInt::<uint>: _) -> Unsigned + NonZero {
        uint
    }

    fn Reduce<num, den>(num: Integer, den: Unsigned + NonZero) -> Default {
        let gcd: Unsigned + NonZero = ExtractUnsigned(num.Abs()).Gcd(den);
        let new_num: Integer = num / PInt::<gcd>;
        let new_den: Unsigned + NonZero = den / gcd;
        Ratio::<new_num, new_den>
    }

    fn RatioAdd<
        num1: Integer,
        den1: Unsigned + NonZero,
        num2: Integer,
        den2: Unsigned + NonZero
    >(Ratio::<num1, den1>: _, Ratio::<num2, den2>: _) -> Default {
        let den: Unsigned + NonZero = den1 * den2;
        let num: Integer = num1 * PInt::<den2> + num2 * PInt::<den1>;

        Reduce(num, den)
    }

    fn RatioSub<
        num1: Integer,
        den1: Unsigned + NonZero,
        num2: Integer,
        den2: Unsigned + NonZero
    >(Ratio::<num1, den1>: _, Ratio::<num2, den2>: _) -> Default {
        let den: Unsigned + NonZero = den1 * den2;
        let num: Integer = num1 * PInt::<den2> - num2 * PInt::<den1>;

        Reduce(num, den)
    }

    fn RatioMul<
        num1: Integer,
        den1: Unsigned + NonZero,
        num2: Integer,
        den2: Unsigned + NonZero
    >(Ratio::<num1, den1>: _, Ratio::<num2, den2>: _) -> Default {
        let den: Unsigned + NonZero = den1 * den2;
        let num: Integer = num1 * num2;

        Reduce(num, den)
    }

    fn RatioDiv<
        num1: Integer,
        den1: Unsigned + NonZero,
        num2: Integer,
        den2: Unsigned + NonZero
    >(Ratio::<num1, den1>: _, Ratio::<num2, den2>: _) -> Default {
        let negative: Bit = (num1 < 0) || (num2 < 0);
        let den: Unsigned + NonZero = den1 * ExtractUnsigned(num2.Abs());
        let unsigned_num: Unsigned + NonZero = ExtractUnsigned(num1.Abs()) * den2;
        let num: Integer = if negative {
            NInt::<unsigned_num>
        } else {
            PInt::<unsigned_num>
        };

        Reduce(num, den)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use typenum::*;

    #[test]
    fn test_ratio_add_op() {
        pub type R1 = Ratio<N4, U9>;
        pub type R2 = Ratio<P6, U3>;
        pub type R3 = Sum<R1, R2>;

        assert_eq!(R3::NUM, 14);
        assert_eq!(R3::DEN, 9);
    }

    #[test]
    fn test_ratio_sub_op() {
        pub type R1 = Ratio<N4, U9>;
        pub type R2 = Ratio<P6, U3>;
        pub type R3 = Diff<R1, R2>;

        assert_eq!(R3::NUM, -22);
        assert_eq!(R3::DEN, 9);
    }

    #[test]
    fn test_ratio_mul_op() {
        pub type R1 = Ratio<N4, U9>;
        pub type R2 = Ratio<P6, U3>;
        pub type R3 = Prod<R1, R2>;

        assert_eq!(R3::NUM, -8);
        assert_eq!(R3::DEN, 9);
    }

    #[test]
    fn test_ratio_div_op() {
        pub type R1 = Ratio<N4, U9>;
        pub type R2 = Ratio<P6, U3>;
        pub type R3 = Quot<R1, R2>;

        assert_eq!(R3::NUM, -2);
        assert_eq!(R3::DEN, 9)
    }
}
