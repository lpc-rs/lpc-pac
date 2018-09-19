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
    _2_CLOCKS,
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    _3_CLOCKS,
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    _4_CLOCKS,
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    _5_CLOCKS,
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    _6_CLOCKS,
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    _7_CLOCKS,
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    _8_CLOCKS,
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    _9_CLOCKS,
}
impl MSTSCLLOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSTSCLLOWR::_2_CLOCKS => 0,
            MSTSCLLOWR::_3_CLOCKS => 1,
            MSTSCLLOWR::_4_CLOCKS => 2,
            MSTSCLLOWR::_5_CLOCKS => 3,
            MSTSCLLOWR::_6_CLOCKS => 4,
            MSTSCLLOWR::_7_CLOCKS => 5,
            MSTSCLLOWR::_8_CLOCKS => 6,
            MSTSCLLOWR::_9_CLOCKS => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSTSCLLOWR {
        match value {
            0 => MSTSCLLOWR::_2_CLOCKS,
            1 => MSTSCLLOWR::_3_CLOCKS,
            2 => MSTSCLLOWR::_4_CLOCKS,
            3 => MSTSCLLOWR::_5_CLOCKS,
            4 => MSTSCLLOWR::_6_CLOCKS,
            5 => MSTSCLLOWR::_7_CLOCKS,
            6 => MSTSCLLOWR::_8_CLOCKS,
            7 => MSTSCLLOWR::_9_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2_CLOCKS`"]
    #[inline]
    pub fn is_2_clocks(&self) -> bool {
        *self == MSTSCLLOWR::_2_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_3_CLOCKS`"]
    #[inline]
    pub fn is_3_clocks(&self) -> bool {
        *self == MSTSCLLOWR::_3_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_4_CLOCKS`"]
    #[inline]
    pub fn is_4_clocks(&self) -> bool {
        *self == MSTSCLLOWR::_4_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_5_CLOCKS`"]
    #[inline]
    pub fn is_5_clocks(&self) -> bool {
        *self == MSTSCLLOWR::_5_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_6_CLOCKS`"]
    #[inline]
    pub fn is_6_clocks(&self) -> bool {
        *self == MSTSCLLOWR::_6_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_7_CLOCKS`"]
    #[inline]
    pub fn is_7_clocks(&self) -> bool {
        *self == MSTSCLLOWR::_7_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_8_CLOCKS`"]
    #[inline]
    pub fn is_8_clocks(&self) -> bool {
        *self == MSTSCLLOWR::_8_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_9_CLOCKS`"]
    #[inline]
    pub fn is_9_clocks(&self) -> bool {
        *self == MSTSCLLOWR::_9_CLOCKS
    }
}
#[doc = "Possible values of the field `MSTSCLHIGH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSCLHIGHR {
    #[doc = "2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    _2_CLOCKS,
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    _3_CLOCKS,
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    _4_CLOCKS,
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    _5_CLOCKS,
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    _6_CLOCKS,
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    _7_CLOCKS,
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    _8_CLOCKS,
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    _9_CLOCKS,
}
impl MSTSCLHIGHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSTSCLHIGHR::_2_CLOCKS => 0,
            MSTSCLHIGHR::_3_CLOCKS => 1,
            MSTSCLHIGHR::_4_CLOCKS => 2,
            MSTSCLHIGHR::_5_CLOCKS => 3,
            MSTSCLHIGHR::_6_CLOCKS => 4,
            MSTSCLHIGHR::_7_CLOCKS => 5,
            MSTSCLHIGHR::_8_CLOCKS => 6,
            MSTSCLHIGHR::_9_CLOCKS => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSTSCLHIGHR {
        match value {
            0 => MSTSCLHIGHR::_2_CLOCKS,
            1 => MSTSCLHIGHR::_3_CLOCKS,
            2 => MSTSCLHIGHR::_4_CLOCKS,
            3 => MSTSCLHIGHR::_5_CLOCKS,
            4 => MSTSCLHIGHR::_6_CLOCKS,
            5 => MSTSCLHIGHR::_7_CLOCKS,
            6 => MSTSCLHIGHR::_8_CLOCKS,
            7 => MSTSCLHIGHR::_9_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2_CLOCKS`"]
    #[inline]
    pub fn is_2_clocks(&self) -> bool {
        *self == MSTSCLHIGHR::_2_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_3_CLOCKS`"]
    #[inline]
    pub fn is_3_clocks(&self) -> bool {
        *self == MSTSCLHIGHR::_3_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_4_CLOCKS`"]
    #[inline]
    pub fn is_4_clocks(&self) -> bool {
        *self == MSTSCLHIGHR::_4_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_5_CLOCKS`"]
    #[inline]
    pub fn is_5_clocks(&self) -> bool {
        *self == MSTSCLHIGHR::_5_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_6_CLOCKS`"]
    #[inline]
    pub fn is_6_clocks(&self) -> bool {
        *self == MSTSCLHIGHR::_6_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_7_CLOCKS`"]
    #[inline]
    pub fn is_7_clocks(&self) -> bool {
        *self == MSTSCLHIGHR::_7_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_8_CLOCKS`"]
    #[inline]
    pub fn is_8_clocks(&self) -> bool {
        *self == MSTSCLHIGHR::_8_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_9_CLOCKS`"]
    #[inline]
    pub fn is_9_clocks(&self) -> bool {
        *self == MSTSCLHIGHR::_9_CLOCKS
    }
}
#[doc = "Values that can be written to the field `MSTSCLLOW`"]
pub enum MSTSCLLOWW {
    #[doc = "2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    _2_CLOCKS,
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    _3_CLOCKS,
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    _4_CLOCKS,
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    _5_CLOCKS,
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    _6_CLOCKS,
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    _7_CLOCKS,
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    _8_CLOCKS,
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    _9_CLOCKS,
}
impl MSTSCLLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSTSCLLOWW::_2_CLOCKS => 0,
            MSTSCLLOWW::_3_CLOCKS => 1,
            MSTSCLLOWW::_4_CLOCKS => 2,
            MSTSCLLOWW::_5_CLOCKS => 3,
            MSTSCLLOWW::_6_CLOCKS => 4,
            MSTSCLLOWW::_7_CLOCKS => 5,
            MSTSCLLOWW::_8_CLOCKS => 6,
            MSTSCLLOWW::_9_CLOCKS => 7,
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
    pub fn _2_clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::_2_CLOCKS)
    }
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn _3_clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::_3_CLOCKS)
    }
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn _4_clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::_4_CLOCKS)
    }
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn _5_clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::_5_CLOCKS)
    }
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn _6_clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::_6_CLOCKS)
    }
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn _7_clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::_7_CLOCKS)
    }
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn _8_clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::_8_CLOCKS)
    }
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn _9_clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOWW::_9_CLOCKS)
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
    _2_CLOCKS,
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    _3_CLOCKS,
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    _4_CLOCKS,
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    _5_CLOCKS,
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    _6_CLOCKS,
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    _7_CLOCKS,
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    _8_CLOCKS,
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    _9_CLOCKS,
}
impl MSTSCLHIGHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSTSCLHIGHW::_2_CLOCKS => 0,
            MSTSCLHIGHW::_3_CLOCKS => 1,
            MSTSCLHIGHW::_4_CLOCKS => 2,
            MSTSCLHIGHW::_5_CLOCKS => 3,
            MSTSCLHIGHW::_6_CLOCKS => 4,
            MSTSCLHIGHW::_7_CLOCKS => 5,
            MSTSCLHIGHW::_8_CLOCKS => 6,
            MSTSCLHIGHW::_9_CLOCKS => 7,
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
    pub fn _2_clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::_2_CLOCKS)
    }
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    #[inline]
    pub fn _3_clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::_3_CLOCKS)
    }
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn _4_clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::_4_CLOCKS)
    }
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn _5_clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::_5_CLOCKS)
    }
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn _6_clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::_6_CLOCKS)
    }
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn _7_clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::_7_CLOCKS)
    }
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    #[inline]
    pub fn _8_clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::_8_CLOCKS)
    }
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    #[inline]
    pub fn _9_clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGHW::_9_CLOCKS)
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
    #[doc = "Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter tLOW in the I2C bus specification. I2C bus specification parameters tBUF and t SU;STA have the same values and are also controlled by MSTSCLLOW."]
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
    #[doc = "Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter tLOW in the I2C bus specification. I2C bus specification parameters tBUF and t SU;STA have the same values and are also controlled by MSTSCLLOW."]
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
