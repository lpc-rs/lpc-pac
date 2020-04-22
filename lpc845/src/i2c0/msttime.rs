#[doc = "Reader of register MSTTIME"]
pub type R = crate::R<u32, super::MSTTIME>;
#[doc = "Writer for register MSTTIME"]
pub type W = crate::W<u32, super::MSTTIME>;
#[doc = "Register MSTTIME `reset()`'s with value 0x77"]
impl crate::ResetValue for super::MSTTIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x77
    }
}
#[doc = "Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSTSCLLOW_A {
    #[doc = "0: 2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    CLOCKS_2 = 0,
    #[doc = "1: 3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    CLOCKS_3 = 1,
    #[doc = "2: 4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    CLOCKS_4 = 2,
    #[doc = "3: 5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    CLOCKS_5 = 3,
    #[doc = "4: 6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    CLOCKS_6 = 4,
    #[doc = "5: 7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    CLOCKS_7 = 5,
    #[doc = "6: 8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    CLOCKS_8 = 6,
    #[doc = "7: 9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    CLOCKS_9 = 7,
}
impl From<MSTSCLLOW_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTSCLLOW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSTSCLLOW`"]
pub type MSTSCLLOW_R = crate::R<u8, MSTSCLLOW_A>;
impl MSTSCLLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSCLLOW_A {
        match self.bits {
            0 => MSTSCLLOW_A::CLOCKS_2,
            1 => MSTSCLLOW_A::CLOCKS_3,
            2 => MSTSCLLOW_A::CLOCKS_4,
            3 => MSTSCLLOW_A::CLOCKS_5,
            4 => MSTSCLLOW_A::CLOCKS_6,
            5 => MSTSCLLOW_A::CLOCKS_7,
            6 => MSTSCLLOW_A::CLOCKS_8,
            7 => MSTSCLLOW_A::CLOCKS_9,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLOCKS_2`"]
    #[inline(always)]
    pub fn is_clocks_2(&self) -> bool {
        *self == MSTSCLLOW_A::CLOCKS_2
    }
    #[doc = "Checks if the value of the field is `CLOCKS_3`"]
    #[inline(always)]
    pub fn is_clocks_3(&self) -> bool {
        *self == MSTSCLLOW_A::CLOCKS_3
    }
    #[doc = "Checks if the value of the field is `CLOCKS_4`"]
    #[inline(always)]
    pub fn is_clocks_4(&self) -> bool {
        *self == MSTSCLLOW_A::CLOCKS_4
    }
    #[doc = "Checks if the value of the field is `CLOCKS_5`"]
    #[inline(always)]
    pub fn is_clocks_5(&self) -> bool {
        *self == MSTSCLLOW_A::CLOCKS_5
    }
    #[doc = "Checks if the value of the field is `CLOCKS_6`"]
    #[inline(always)]
    pub fn is_clocks_6(&self) -> bool {
        *self == MSTSCLLOW_A::CLOCKS_6
    }
    #[doc = "Checks if the value of the field is `CLOCKS_7`"]
    #[inline(always)]
    pub fn is_clocks_7(&self) -> bool {
        *self == MSTSCLLOW_A::CLOCKS_7
    }
    #[doc = "Checks if the value of the field is `CLOCKS_8`"]
    #[inline(always)]
    pub fn is_clocks_8(&self) -> bool {
        *self == MSTSCLLOW_A::CLOCKS_8
    }
    #[doc = "Checks if the value of the field is `CLOCKS_9`"]
    #[inline(always)]
    pub fn is_clocks_9(&self) -> bool {
        *self == MSTSCLLOW_A::CLOCKS_9
    }
}
#[doc = "Write proxy for field `MSTSCLLOW`"]
pub struct MSTSCLLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSCLLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSCLLOW_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_2(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_2)
    }
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_3(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_3)
    }
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_4(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_4)
    }
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_5(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_5)
    }
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_6(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_6)
    }
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_7(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_7)
    }
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_8(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_8)
    }
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_9(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSTSCLHIGH_A {
    #[doc = "0: 2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    CLOCKS_2 = 0,
    #[doc = "1: 3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    CLOCKS_3 = 1,
    #[doc = "2: 4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    CLOCKS_4 = 2,
    #[doc = "3: 5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    CLOCKS_5 = 3,
    #[doc = "4: 6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    CLOCKS_6 = 4,
    #[doc = "5: 7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    CLOCKS_7 = 5,
    #[doc = "6: 8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    CLOCKS_8 = 6,
    #[doc = "7: 9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    CLOCKS_9 = 7,
}
impl From<MSTSCLHIGH_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTSCLHIGH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSTSCLHIGH`"]
pub type MSTSCLHIGH_R = crate::R<u8, MSTSCLHIGH_A>;
impl MSTSCLHIGH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSCLHIGH_A {
        match self.bits {
            0 => MSTSCLHIGH_A::CLOCKS_2,
            1 => MSTSCLHIGH_A::CLOCKS_3,
            2 => MSTSCLHIGH_A::CLOCKS_4,
            3 => MSTSCLHIGH_A::CLOCKS_5,
            4 => MSTSCLHIGH_A::CLOCKS_6,
            5 => MSTSCLHIGH_A::CLOCKS_7,
            6 => MSTSCLHIGH_A::CLOCKS_8,
            7 => MSTSCLHIGH_A::CLOCKS_9,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLOCKS_2`"]
    #[inline(always)]
    pub fn is_clocks_2(&self) -> bool {
        *self == MSTSCLHIGH_A::CLOCKS_2
    }
    #[doc = "Checks if the value of the field is `CLOCKS_3`"]
    #[inline(always)]
    pub fn is_clocks_3(&self) -> bool {
        *self == MSTSCLHIGH_A::CLOCKS_3
    }
    #[doc = "Checks if the value of the field is `CLOCKS_4`"]
    #[inline(always)]
    pub fn is_clocks_4(&self) -> bool {
        *self == MSTSCLHIGH_A::CLOCKS_4
    }
    #[doc = "Checks if the value of the field is `CLOCKS_5`"]
    #[inline(always)]
    pub fn is_clocks_5(&self) -> bool {
        *self == MSTSCLHIGH_A::CLOCKS_5
    }
    #[doc = "Checks if the value of the field is `CLOCKS_6`"]
    #[inline(always)]
    pub fn is_clocks_6(&self) -> bool {
        *self == MSTSCLHIGH_A::CLOCKS_6
    }
    #[doc = "Checks if the value of the field is `CLOCKS_7`"]
    #[inline(always)]
    pub fn is_clocks_7(&self) -> bool {
        *self == MSTSCLHIGH_A::CLOCKS_7
    }
    #[doc = "Checks if the value of the field is `CLOCKS_8`"]
    #[inline(always)]
    pub fn is_clocks_8(&self) -> bool {
        *self == MSTSCLHIGH_A::CLOCKS_8
    }
    #[doc = "Checks if the value of the field is `CLOCKS_9`"]
    #[inline(always)]
    pub fn is_clocks_9(&self) -> bool {
        *self == MSTSCLHIGH_A::CLOCKS_9
    }
}
#[doc = "Write proxy for field `MSTSCLHIGH`"]
pub struct MSTSCLHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSCLHIGH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSCLHIGH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_2(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_2)
    }
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    #[inline(always)]
    pub fn clocks_3(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_3)
    }
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_4(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_4)
    }
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_5(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_5)
    }
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_6(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_6)
    }
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_7(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_7)
    }
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_8(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_8)
    }
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_9(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[inline(always)]
    pub fn mstscllow(&self) -> MSTSCLLOW_R {
        MSTSCLLOW_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[inline(always)]
    pub fn mstsclhigh(&self) -> MSTSCLHIGH_R {
        MSTSCLHIGH_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[inline(always)]
    pub fn mstscllow(&mut self) -> MSTSCLLOW_W {
        MSTSCLLOW_W { w: self }
    }
    #[doc = "Bits 4:6 - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[inline(always)]
    pub fn mstsclhigh(&mut self) -> MSTSCLHIGH_W {
        MSTSCLHIGH_W { w: self }
    }
}
