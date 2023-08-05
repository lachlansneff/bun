pub use self::prefixes::*;

unit_system!(Si {
    (s, Second),
    (m, Meter),
    (kg, Kilogram),
    (A, Ampere),
    (K, Kelvin),
    (mol, Mole),
    (cd, Candela),
});

pub mod prefixes {
    use crate::{ratio, Mul};

    pub type Deka<U> = Mul<U, ratio!(10)>;
    pub type Hecto<U> = Mul<U, ratio!(100)>;
    pub type Kilo<U> = Mul<U, ratio!(1_000)>;
    pub type Mega<U> = Mul<U, ratio!(1_000_000)>;
    pub type Giga<U> = Mul<U, ratio!(1_000_000_000)>;
    pub type Tera<U> = Mul<U, ratio!(1_000_000_000_000)>;
    pub type Peta<U> = Mul<U, ratio!(1_000_000_000_000_000)>;
    pub type Exa<U> = Mul<U, ratio!(1_000_000_000_000_000_000)>;
    pub type Zetta<U> = Mul<U, ratio!(1_000_000_000_000_000_000_000)>;
    pub type Yotta<U> = Mul<U, ratio!(1_000_000_000_000_000_000_000_000)>;
    pub type Ronna<U> = Mul<U, ratio!(1_000_000_000_000_000_000_000_000_000)>;
    pub type Quetta<U> = Mul<U, ratio!(1_000_000_000_000_000_000_000_000_000_000)>;

    pub type Deci<U> = Mul<U, ratio!(1 / 10)>;
    pub type Centi<U> = Mul<U, ratio!(1 / 100)>;
    pub type Milli<U> = Mul<U, ratio!(1 / 1_000)>;
    pub type Micro<U> = Mul<U, ratio!(1 / 1_000_000)>;
    pub type Nano<U> = Mul<U, ratio!(1 / 1_000_000_000)>;
    pub type Pico<U> = Mul<U, ratio!(1 / 1_000_000_000_000)>;
    pub type Femto<U> = Mul<U, ratio!(1 / 1_000_000_000_000_000)>;
    pub type Atto<U> = Mul<U, ratio!(1 / 1_000_000_000_000_000_000)>;
    pub type Zepto<U> = Mul<U, ratio!(1 / 1_000_000_000_000_000_000_000)>;
    pub type Yocto<U> = Mul<U, ratio!(1 / 1_000_000_000_000_000_000_000_000)>;
    pub type Ronto<U> = Mul<U, ratio!(1 / 1_000_000_000_000_000_000_000_000_000)>;
    pub type Quecto<U> = Mul<U, ratio!(1 / 1_000_000_000_000_000_000_000_000_000_000)>;
}
