#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSTTIME {
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
#[doc = "Possible values of the field `MSTSCLLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSCLLOWR {
    #[doc = "2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    CLOCKS_2,
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    CLOCKS_3,
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    CLOCKS_4,
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    CLOCKS_5,
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    CLOCKS_6,
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    CLOCKS_7,
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    CLOCKS_8,
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    CLOCKS_9,
}
impl MSTSCLLOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSTSCLLOWR::CLOCKS_2 => 0,
            MSTSCLLOWR::CLOCKS_3 => 1,
            MSTSCLLOWR::CLOCKS_4 => 2,
            MSTSCLLOWR::CLOCKS_5 => 3,
            MSTSCLLOWR::CLOCKS_6 => 4,
            MSTSCLLOWR::CLOCKS_7 => 5,
            MSTSCLLOWR::CLOCKS_8 => 6,
            MSTSCLLOWR::CLOCKS_9 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSTSCLLOWR {
        match value {
            0 => MSTSCLLOWR::CLOCKS_2,
            1 => MSTSCLLOWR::CLOCKS_3,
            2 => MSTSCLLOWR::CLOCKS_4,
            3 => MSTSCLLOWR::CLOCKS_5,
            4 => MSTSCLLOWR::CLOCKS_6,
            5 => MSTSCLLOWR::CLOCKS_7,
            6 => MSTSCLLOWR::CLOCKS_8,
            7 => MSTSCLLOWR::CLOCKS_9,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLOCKS_2`"]
    #[inline]
    pub fn is_clocks_2(&self) -> bool {
        *self == MSTSCLLOWR::CLOCKS_2
    }
    #[doc = "Checks if the value of the field is `CLOCKS_3`"]
    #[inline]
    pub fn is_clocks_3(&self) -> bool {
        *self == MSTSCLLOWR::CLOCKS_3
    }
    #[doc = "Checks if the value of the field is `CLOCKS_4`"]
    #[inline]
    pub fn is_clocks_4(&self) -> bool {
        *self == MSTSCLLOWR::CLOCKS_4
    }
    #[doc = "Checks if the value of the field is `CLOCKS_5`"]
    #[inline]
    pub fn is_clocks_5(&self) -> bool {
        *self == MSTSCLLOWR::CLOCKS_5
    }
    #[doc = "Checks if the value of the field is `CLOCKS_6`"]
    #[inline]
    pub fn is_clocks_6(&self) -> bool {
        *self == MSTSCLLOWR::CLOCKS_6
    }
    #[doc = "Checks if the value of the field is `CLOCKS_7`"]
    #[inline]
    pub fn is_clocks_7(&self) -> bool {
        *self == MSTSCLLOWR::CLOCKS_7
    }
    #[doc = "Checks if the value of the field is `CLOCKS_8`"]
    #[inline]
    pub fn is_clocks_8(&self) -> bool {
        *self == MSTSCLLOWR::CLOCKS_8
    }
    #[doc = "Checks if the value of the field is `CLOCKS_9`"]
    #[inline]
    pub fn is_clocks_9(&self) -> bool {
        *self == MSTSCLLOWR::CLOCKS_9
    }
}
#[doc = "Possible values of the field `MSTSCLHIGH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSCLHIGHR {
    #[doc = "2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    CLOCKS_2,
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    CLOCKS_3,
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    CLOCKS_4,
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    CLOCKS_5,
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    CLOCKS_6,
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    CLOCKS_7,
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    CLOCKS_8,
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    CLOCKS_9,
}
impl MSTSCLHIGHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSTSCLHIGHR::CLOCKS_2 => 0,
            MSTSCLHIGHR::CLOCKS_3 => 1,
            MSTSCLHIGHR::CLOCKS_4 => 2,
            MSTSCLHIGHR::CLOCKS_5 => 3,
            MSTSCLHIGHR::CLOCKS_6 => 4,
            MSTSCLHIGHR::CLOCKS_7 => 5,
            MSTSCLHIGHR::CLOCKS_8 => 6,
            MSTSCLHIGHR::CLOCKS_9 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSTSCLHIGHR {
        match value {
            0 => MSTSCLHIGHR::CLOCKS_2,
            1 => MSTSCLHIGHR::CLOCKS_3,
            2 => MSTSCLHIGHR::CLOCKS_4,
            3 => MSTSCLHIGHR::CLOCKS_5,
            4 => MSTSCLHIGHR::CLOCKS_6,
            5 => MSTSCLHIGHR::CLOCKS_7,
            6 => MSTSCLHIGHR::CLOCKS_8,
            7 => MSTSCLHIGHR::CLOCKS_9,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLOCKS_2`"]
    #[inline]
    pub fn is_clocks_2(&self) -> bool {
        *self == MSTSCLHIGHR::CLOCKS_2
    }
    #[doc = "Checks if the value of the field is `CLOCKS_3`"]
    #[inline]
    pub fn is_clocks_3(&self) -> bool {
        *self == MSTSCLHIGHR::CLOCKS_3
    }
    #[doc = "Checks if the value of the field is `CLOCKS_4`"]
    #[inline]
    pub fn is_clocks_4(&self) -> bool {
        *self == MSTSCLHIGHR::CLOCKS_4
    }
    #[doc = "Checks if the value of the field is `CLOCKS_5`"]
    #[inline]
    pub fn is_clocks_5(&self) -> bool {
        *self == MSTSCLHIGHR::CLOCKS_5
    }
    #[doc = "Checks if the value of the field is `CLOCKS_6`"]
    #[inline]
    pub fn is_clocks_6(&self) -> bool {
        *self == MSTSCLHIGHR::CLOCKS_6
    }
    #[doc = "Checks if the value of the field is `CLOCKS_7`"]
    #[inline]
    pub fn is_clocks_7(&self) -> bool {
        *self == MSTSCLHIGHR::CLOCKS_7
    }
    #[doc = "Checks if the value of the field is `CLOCKS_8`"]
    #[inline]
    pub fn is_clocks_8(&self) -> bool {
        *self == MSTSCLHIGHR::CLOCKS_8
    }
    #[doc = "Checks if the value of the field is `CLOCKS_9`"]
    #[inline]
    pub fn is_clocks_9(&self) -> bool {
        *self == MSTSCLHIGHR::CLOCKS_9
    }
}
#[doc = "Values that can be written to the field `MSTSCLLOW`"]
pub enum MSTSCLLOWW {
    #[doc = "2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    CLOCKS_2,
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    CLOCKS_3,
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    CLOCKS_4,
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    CLOCKS_5,
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    CLOCKS_6,
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    CLOCKS_7,
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    CLOCKS_8,
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    CLOCKS_9,
}
impl MSTSCLLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSTSCLLOWW::CLOCKS_2 => 0,
            MSTSCLLOWW::CLOCKS_3 => 1,
            MSTSCLLOWW::CLOCKS_4 => 2,
            MSTSCLLOWW::CLOCKS_5 => 3,
            MSTSCLLOWW::CLOCKS_6 => 4,
            MSTSCLLOWW::CLOCKS_7 => 5,
            MSTSCLLOWW::CLOCKS_8 => 6,
            MSTSCLLOWW::CLOCKS_9 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSCLLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSCLLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSCLLOWW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_2(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::CLOCKS_2)
    }
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_3(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::CLOCKS_3)
    }
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_4(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::CLOCKS_4)
    }
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_5(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::CLOCKS_5)
    }
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_6(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::CLOCKS_6)
    }
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_7(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::CLOCKS_7)
    }
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_8(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::CLOCKS_8)
    }
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_9(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::CLOCKS_9)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTSCLHIGH`"]
pub enum MSTSCLHIGHW {
    #[doc = "2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    CLOCKS_2,
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    CLOCKS_3,
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    CLOCKS_4,
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    CLOCKS_5,
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    CLOCKS_6,
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    CLOCKS_7,
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    CLOCKS_8,
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    CLOCKS_9,
}
impl MSTSCLHIGHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSTSCLHIGHW::CLOCKS_2 => 0,
            MSTSCLHIGHW::CLOCKS_3 => 1,
            MSTSCLHIGHW::CLOCKS_4 => 2,
            MSTSCLHIGHW::CLOCKS_5 => 3,
            MSTSCLHIGHW::CLOCKS_6 => 4,
            MSTSCLHIGHW::CLOCKS_7 => 5,
            MSTSCLHIGHW::CLOCKS_8 => 6,
            MSTSCLHIGHW::CLOCKS_9 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSCLHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSCLHIGHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSCLHIGHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_2(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::CLOCKS_2)
    }
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    #[inline]
    pub fn clocks_3(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::CLOCKS_3)
    }
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_4(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::CLOCKS_4)
    }
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_5(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::CLOCKS_5)
    }
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_6(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::CLOCKS_6)
    }
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_7(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::CLOCKS_7)
    }
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_8(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::CLOCKS_8)
    }
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn clocks_9(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::CLOCKS_9)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[inline]
    pub fn mstscllow(&self) -> MSTSCLLOWR {
        MSTSCLLOWR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[inline]
    pub fn mstsclhigh(&self) -> MSTSCLHIGHR {
        MSTSCLHIGHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 119 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[inline]
    pub fn mstscllow(&mut self) -> _MSTSCLLOWW {
        _MSTSCLLOWW { w: self }
    }
    #[doc = "Bits 4:6 - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[inline]
    pub fn mstsclhigh(&mut self) -> _MSTSCLHIGHW {
        _MSTSCLHIGHW { w: self }
    }
}
