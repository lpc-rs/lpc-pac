#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMSRC {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SRC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC0R {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 0."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 0."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 0."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 0."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 0."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 0."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 0."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 0."]
    INPUT7,
}
impl SRC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC0R::INPUT0 => 0,
            SRC0R::INPUT1 => 1,
            SRC0R::INPUT2 => 2,
            SRC0R::INPUT3 => 3,
            SRC0R::INPUT4 => 4,
            SRC0R::INPUT5 => 5,
            SRC0R::INPUT6 => 6,
            SRC0R::INPUT7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC0R {
        match value {
            0 => SRC0R::INPUT0,
            1 => SRC0R::INPUT1,
            2 => SRC0R::INPUT2,
            3 => SRC0R::INPUT3,
            4 => SRC0R::INPUT4,
            5 => SRC0R::INPUT5,
            6 => SRC0R::INPUT6,
            7 => SRC0R::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline]
    pub fn is_input0(&self) -> bool {
        *self == SRC0R::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == SRC0R::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline]
    pub fn is_input2(&self) -> bool {
        *self == SRC0R::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == SRC0R::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline]
    pub fn is_input4(&self) -> bool {
        *self == SRC0R::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == SRC0R::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline]
    pub fn is_input6(&self) -> bool {
        *self == SRC0R::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == SRC0R::INPUT7
    }
}
#[doc = "Possible values of the field `SRC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC1R {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 1."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 1."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 1."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 1."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 1."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 1."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 1."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 1."]
    INPUT7,
}
impl SRC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC1R::INPUT0 => 0,
            SRC1R::INPUT1 => 1,
            SRC1R::INPUT2 => 2,
            SRC1R::INPUT3 => 3,
            SRC1R::INPUT4 => 4,
            SRC1R::INPUT5 => 5,
            SRC1R::INPUT6 => 6,
            SRC1R::INPUT7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC1R {
        match value {
            0 => SRC1R::INPUT0,
            1 => SRC1R::INPUT1,
            2 => SRC1R::INPUT2,
            3 => SRC1R::INPUT3,
            4 => SRC1R::INPUT4,
            5 => SRC1R::INPUT5,
            6 => SRC1R::INPUT6,
            7 => SRC1R::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline]
    pub fn is_input0(&self) -> bool {
        *self == SRC1R::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == SRC1R::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline]
    pub fn is_input2(&self) -> bool {
        *self == SRC1R::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == SRC1R::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline]
    pub fn is_input4(&self) -> bool {
        *self == SRC1R::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == SRC1R::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline]
    pub fn is_input6(&self) -> bool {
        *self == SRC1R::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == SRC1R::INPUT7
    }
}
#[doc = "Possible values of the field `SRC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC2R {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 2."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 2."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 2."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 2."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 2."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 2."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 2."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 2."]
    INPUT7,
}
impl SRC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC2R::INPUT0 => 0,
            SRC2R::INPUT1 => 1,
            SRC2R::INPUT2 => 2,
            SRC2R::INPUT3 => 3,
            SRC2R::INPUT4 => 4,
            SRC2R::INPUT5 => 5,
            SRC2R::INPUT6 => 6,
            SRC2R::INPUT7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC2R {
        match value {
            0 => SRC2R::INPUT0,
            1 => SRC2R::INPUT1,
            2 => SRC2R::INPUT2,
            3 => SRC2R::INPUT3,
            4 => SRC2R::INPUT4,
            5 => SRC2R::INPUT5,
            6 => SRC2R::INPUT6,
            7 => SRC2R::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline]
    pub fn is_input0(&self) -> bool {
        *self == SRC2R::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == SRC2R::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline]
    pub fn is_input2(&self) -> bool {
        *self == SRC2R::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == SRC2R::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline]
    pub fn is_input4(&self) -> bool {
        *self == SRC2R::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == SRC2R::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline]
    pub fn is_input6(&self) -> bool {
        *self == SRC2R::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == SRC2R::INPUT7
    }
}
#[doc = "Possible values of the field `SRC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC3R {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 3."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 3."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 3."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 3."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 3."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 3."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 3."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 3."]
    INPUT7,
}
impl SRC3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC3R::INPUT0 => 0,
            SRC3R::INPUT1 => 1,
            SRC3R::INPUT2 => 2,
            SRC3R::INPUT3 => 3,
            SRC3R::INPUT4 => 4,
            SRC3R::INPUT5 => 5,
            SRC3R::INPUT6 => 6,
            SRC3R::INPUT7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC3R {
        match value {
            0 => SRC3R::INPUT0,
            1 => SRC3R::INPUT1,
            2 => SRC3R::INPUT2,
            3 => SRC3R::INPUT3,
            4 => SRC3R::INPUT4,
            5 => SRC3R::INPUT5,
            6 => SRC3R::INPUT6,
            7 => SRC3R::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline]
    pub fn is_input0(&self) -> bool {
        *self == SRC3R::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == SRC3R::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline]
    pub fn is_input2(&self) -> bool {
        *self == SRC3R::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == SRC3R::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline]
    pub fn is_input4(&self) -> bool {
        *self == SRC3R::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == SRC3R::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline]
    pub fn is_input6(&self) -> bool {
        *self == SRC3R::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == SRC3R::INPUT7
    }
}
#[doc = "Possible values of the field `SRC4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC4R {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 4."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 4."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 4."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 4."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 4."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 4."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 4."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 4."]
    INPUT7,
}
impl SRC4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC4R::INPUT0 => 0,
            SRC4R::INPUT1 => 1,
            SRC4R::INPUT2 => 2,
            SRC4R::INPUT3 => 3,
            SRC4R::INPUT4 => 4,
            SRC4R::INPUT5 => 5,
            SRC4R::INPUT6 => 6,
            SRC4R::INPUT7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC4R {
        match value {
            0 => SRC4R::INPUT0,
            1 => SRC4R::INPUT1,
            2 => SRC4R::INPUT2,
            3 => SRC4R::INPUT3,
            4 => SRC4R::INPUT4,
            5 => SRC4R::INPUT5,
            6 => SRC4R::INPUT6,
            7 => SRC4R::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline]
    pub fn is_input0(&self) -> bool {
        *self == SRC4R::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == SRC4R::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline]
    pub fn is_input2(&self) -> bool {
        *self == SRC4R::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == SRC4R::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline]
    pub fn is_input4(&self) -> bool {
        *self == SRC4R::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == SRC4R::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline]
    pub fn is_input6(&self) -> bool {
        *self == SRC4R::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == SRC4R::INPUT7
    }
}
#[doc = "Possible values of the field `SRC5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC5R {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 5."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 5."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 5."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 5."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 5."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 5."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 5."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 5."]
    INPUT7,
}
impl SRC5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC5R::INPUT0 => 0,
            SRC5R::INPUT1 => 1,
            SRC5R::INPUT2 => 2,
            SRC5R::INPUT3 => 3,
            SRC5R::INPUT4 => 4,
            SRC5R::INPUT5 => 5,
            SRC5R::INPUT6 => 6,
            SRC5R::INPUT7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC5R {
        match value {
            0 => SRC5R::INPUT0,
            1 => SRC5R::INPUT1,
            2 => SRC5R::INPUT2,
            3 => SRC5R::INPUT3,
            4 => SRC5R::INPUT4,
            5 => SRC5R::INPUT5,
            6 => SRC5R::INPUT6,
            7 => SRC5R::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline]
    pub fn is_input0(&self) -> bool {
        *self == SRC5R::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == SRC5R::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline]
    pub fn is_input2(&self) -> bool {
        *self == SRC5R::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == SRC5R::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline]
    pub fn is_input4(&self) -> bool {
        *self == SRC5R::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == SRC5R::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline]
    pub fn is_input6(&self) -> bool {
        *self == SRC5R::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == SRC5R::INPUT7
    }
}
#[doc = "Possible values of the field `SRC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC6R {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 6."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 6."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 6."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 6."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 6."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 6."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 6."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 6."]
    INPUT7,
}
impl SRC6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC6R::INPUT0 => 0,
            SRC6R::INPUT1 => 1,
            SRC6R::INPUT2 => 2,
            SRC6R::INPUT3 => 3,
            SRC6R::INPUT4 => 4,
            SRC6R::INPUT5 => 5,
            SRC6R::INPUT6 => 6,
            SRC6R::INPUT7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC6R {
        match value {
            0 => SRC6R::INPUT0,
            1 => SRC6R::INPUT1,
            2 => SRC6R::INPUT2,
            3 => SRC6R::INPUT3,
            4 => SRC6R::INPUT4,
            5 => SRC6R::INPUT5,
            6 => SRC6R::INPUT6,
            7 => SRC6R::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline]
    pub fn is_input0(&self) -> bool {
        *self == SRC6R::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == SRC6R::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline]
    pub fn is_input2(&self) -> bool {
        *self == SRC6R::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == SRC6R::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline]
    pub fn is_input4(&self) -> bool {
        *self == SRC6R::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == SRC6R::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline]
    pub fn is_input6(&self) -> bool {
        *self == SRC6R::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == SRC6R::INPUT7
    }
}
#[doc = "Possible values of the field `SRC7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC7R {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 7."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 7."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 7."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 7."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 7."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 7."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 7."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 7."]
    INPUT7,
}
impl SRC7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC7R::INPUT0 => 0,
            SRC7R::INPUT1 => 1,
            SRC7R::INPUT2 => 2,
            SRC7R::INPUT3 => 3,
            SRC7R::INPUT4 => 4,
            SRC7R::INPUT5 => 5,
            SRC7R::INPUT6 => 6,
            SRC7R::INPUT7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC7R {
        match value {
            0 => SRC7R::INPUT0,
            1 => SRC7R::INPUT1,
            2 => SRC7R::INPUT2,
            3 => SRC7R::INPUT3,
            4 => SRC7R::INPUT4,
            5 => SRC7R::INPUT5,
            6 => SRC7R::INPUT6,
            7 => SRC7R::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline]
    pub fn is_input0(&self) -> bool {
        *self == SRC7R::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline]
    pub fn is_input1(&self) -> bool {
        *self == SRC7R::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline]
    pub fn is_input2(&self) -> bool {
        *self == SRC7R::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline]
    pub fn is_input3(&self) -> bool {
        *self == SRC7R::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline]
    pub fn is_input4(&self) -> bool {
        *self == SRC7R::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline]
    pub fn is_input5(&self) -> bool {
        *self == SRC7R::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline]
    pub fn is_input6(&self) -> bool {
        *self == SRC7R::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline]
    pub fn is_input7(&self) -> bool {
        *self == SRC7R::INPUT7
    }
}
#[doc = "Values that can be written to the field `SRC0`"]
pub enum SRC0W {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 0."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 0."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 0."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 0."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 0."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 0."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 0."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 0."]
    INPUT7,
}
impl SRC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC0W::INPUT0 => 0,
            SRC0W::INPUT1 => 1,
            SRC0W::INPUT2 => 2,
            SRC0W::INPUT3 => 3,
            SRC0W::INPUT4 => 4,
            SRC0W::INPUT5 => 5,
            SRC0W::INPUT6 => 6,
            SRC0W::INPUT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC0W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 0."]
    #[inline]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC0W::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 0."]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC0W::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 0."]
    #[inline]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC0W::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 0."]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC0W::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 0."]
    #[inline]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC0W::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 0."]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC0W::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 0."]
    #[inline]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC0W::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 0."]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC0W::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC1`"]
pub enum SRC1W {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 1."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 1."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 1."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 1."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 1."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 1."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 1."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 1."]
    INPUT7,
}
impl SRC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC1W::INPUT0 => 0,
            SRC1W::INPUT1 => 1,
            SRC1W::INPUT2 => 2,
            SRC1W::INPUT3 => 3,
            SRC1W::INPUT4 => 4,
            SRC1W::INPUT5 => 5,
            SRC1W::INPUT6 => 6,
            SRC1W::INPUT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC1W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 1."]
    #[inline]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC1W::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 1."]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC1W::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 1."]
    #[inline]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC1W::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 1."]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC1W::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 1."]
    #[inline]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC1W::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 1."]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC1W::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 1."]
    #[inline]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC1W::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 1."]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC1W::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC2`"]
pub enum SRC2W {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 2."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 2."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 2."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 2."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 2."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 2."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 2."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 2."]
    INPUT7,
}
impl SRC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC2W::INPUT0 => 0,
            SRC2W::INPUT1 => 1,
            SRC2W::INPUT2 => 2,
            SRC2W::INPUT3 => 3,
            SRC2W::INPUT4 => 4,
            SRC2W::INPUT5 => 5,
            SRC2W::INPUT6 => 6,
            SRC2W::INPUT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC2W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 2."]
    #[inline]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC2W::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 2."]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC2W::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 2."]
    #[inline]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC2W::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 2."]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC2W::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 2."]
    #[inline]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC2W::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 2."]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC2W::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 2."]
    #[inline]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC2W::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 2."]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC2W::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC3`"]
pub enum SRC3W {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 3."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 3."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 3."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 3."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 3."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 3."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 3."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 3."]
    INPUT7,
}
impl SRC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC3W::INPUT0 => 0,
            SRC3W::INPUT1 => 1,
            SRC3W::INPUT2 => 2,
            SRC3W::INPUT3 => 3,
            SRC3W::INPUT4 => 4,
            SRC3W::INPUT5 => 5,
            SRC3W::INPUT6 => 6,
            SRC3W::INPUT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC3W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 3."]
    #[inline]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC3W::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 3."]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC3W::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 3."]
    #[inline]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC3W::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 3."]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC3W::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 3."]
    #[inline]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC3W::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 3."]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC3W::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 3."]
    #[inline]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC3W::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 3."]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC3W::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC4`"]
pub enum SRC4W {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 4."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 4."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 4."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 4."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 4."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 4."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 4."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 4."]
    INPUT7,
}
impl SRC4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC4W::INPUT0 => 0,
            SRC4W::INPUT1 => 1,
            SRC4W::INPUT2 => 2,
            SRC4W::INPUT3 => 3,
            SRC4W::INPUT4 => 4,
            SRC4W::INPUT5 => 5,
            SRC4W::INPUT6 => 6,
            SRC4W::INPUT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC4W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 4."]
    #[inline]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC4W::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 4."]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC4W::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 4."]
    #[inline]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC4W::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 4."]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC4W::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 4."]
    #[inline]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC4W::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 4."]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC4W::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 4."]
    #[inline]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC4W::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 4."]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC4W::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC5`"]
pub enum SRC5W {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 5."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 5."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 5."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 5."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 5."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 5."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 5."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 5."]
    INPUT7,
}
impl SRC5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC5W::INPUT0 => 0,
            SRC5W::INPUT1 => 1,
            SRC5W::INPUT2 => 2,
            SRC5W::INPUT3 => 3,
            SRC5W::INPUT4 => 4,
            SRC5W::INPUT5 => 5,
            SRC5W::INPUT6 => 6,
            SRC5W::INPUT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC5W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 5."]
    #[inline]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC5W::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 5."]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC5W::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 5."]
    #[inline]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC5W::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 5."]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC5W::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 5."]
    #[inline]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC5W::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 5."]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC5W::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 5."]
    #[inline]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC5W::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 5."]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC5W::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC6`"]
pub enum SRC6W {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 6."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 6."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 6."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 6."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 6."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 6."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 6."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 6."]
    INPUT7,
}
impl SRC6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC6W::INPUT0 => 0,
            SRC6W::INPUT1 => 1,
            SRC6W::INPUT2 => 2,
            SRC6W::INPUT3 => 3,
            SRC6W::INPUT4 => 4,
            SRC6W::INPUT5 => 5,
            SRC6W::INPUT6 => 6,
            SRC6W::INPUT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC6W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 6."]
    #[inline]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC6W::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 6."]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC6W::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 6."]
    #[inline]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC6W::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 6."]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC6W::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 6."]
    #[inline]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC6W::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 6."]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC6W::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 6."]
    #[inline]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC6W::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 6."]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC6W::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC7`"]
pub enum SRC7W {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 7."]
    INPUT0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 7."]
    INPUT1,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 7."]
    INPUT2,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 7."]
    INPUT3,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 7."]
    INPUT4,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 7."]
    INPUT5,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 7."]
    INPUT6,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 7."]
    INPUT7,
}
impl SRC7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC7W::INPUT0 => 0,
            SRC7W::INPUT1 => 1,
            SRC7W::INPUT2 => 2,
            SRC7W::INPUT3 => 3,
            SRC7W::INPUT4 => 4,
            SRC7W::INPUT5 => 5,
            SRC7W::INPUT6 => 6,
            SRC7W::INPUT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC7W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 7."]
    #[inline]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC7W::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 7."]
    #[inline]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC7W::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 7."]
    #[inline]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC7W::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 7."]
    #[inline]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC7W::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 7."]
    #[inline]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC7W::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 7."]
    #[inline]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC7W::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 7."]
    #[inline]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC7W::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 7."]
    #[inline]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC7W::INPUT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:10 - Selects the input source for bit slice 0"]
    #[inline]
    pub fn src0(&self) -> SRC0R {
        SRC0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Selects the input source for bit slice 1"]
    #[inline]
    pub fn src1(&self) -> SRC1R {
        SRC1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:16 - Selects the input source for bit slice 2"]
    #[inline]
    pub fn src2(&self) -> SRC2R {
        SRC2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:19 - Selects the input source for bit slice 3"]
    #[inline]
    pub fn src3(&self) -> SRC3R {
        SRC3R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Selects the input source for bit slice 4"]
    #[inline]
    pub fn src4(&self) -> SRC4R {
        SRC4R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 23:25 - Selects the input source for bit slice 5"]
    #[inline]
    pub fn src5(&self) -> SRC5R {
        SRC5R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:28 - Selects the input source for bit slice 6"]
    #[inline]
    pub fn src6(&self) -> SRC6R {
        SRC6R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 29:31 - Selects the input source for bit slice 7"]
    #[inline]
    pub fn src7(&self) -> SRC7R {
        SRC7R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:10 - Selects the input source for bit slice 0"]
    #[inline]
    pub fn src0(&mut self) -> _SRC0W {
        _SRC0W { w: self }
    }
    #[doc = "Bits 11:13 - Selects the input source for bit slice 1"]
    #[inline]
    pub fn src1(&mut self) -> _SRC1W {
        _SRC1W { w: self }
    }
    #[doc = "Bits 14:16 - Selects the input source for bit slice 2"]
    #[inline]
    pub fn src2(&mut self) -> _SRC2W {
        _SRC2W { w: self }
    }
    #[doc = "Bits 17:19 - Selects the input source for bit slice 3"]
    #[inline]
    pub fn src3(&mut self) -> _SRC3W {
        _SRC3W { w: self }
    }
    #[doc = "Bits 20:22 - Selects the input source for bit slice 4"]
    #[inline]
    pub fn src4(&mut self) -> _SRC4W {
        _SRC4W { w: self }
    }
    #[doc = "Bits 23:25 - Selects the input source for bit slice 5"]
    #[inline]
    pub fn src5(&mut self) -> _SRC5W {
        _SRC5W { w: self }
    }
    #[doc = "Bits 26:28 - Selects the input source for bit slice 6"]
    #[inline]
    pub fn src6(&mut self) -> _SRC6W {
        _SRC6W { w: self }
    }
    #[doc = "Bits 29:31 - Selects the input source for bit slice 7"]
    #[inline]
    pub fn src7(&mut self) -> _SRC7W {
        _SRC7W { w: self }
    }
}
