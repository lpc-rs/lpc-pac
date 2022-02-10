///Register `MSTTIME` reader
pub struct R(crate::R<MSTTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTTIME_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MSTTIME` writer
pub struct W(crate::W<MSTTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MSTTIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTTIME_SPEC>) -> Self {
        W(writer)
    }
}
///Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW.
///
///Value on reset: 7
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSTSCLLOW_A {
    ///0: 2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider.
    CLOCKS_2 = 0,
    ///1: 3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider.
    CLOCKS_3 = 1,
    ///2: 4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider.
    CLOCKS_4 = 2,
    ///3: 5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider.
    CLOCKS_5 = 3,
    ///4: 6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider.
    CLOCKS_6 = 4,
    ///5: 7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider.
    CLOCKS_7 = 5,
    ///6: 8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider.
    CLOCKS_8 = 6,
    ///7: 9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider.
    CLOCKS_9 = 7,
}
impl From<MSTSCLLOW_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTSCLLOW_A) -> Self {
        variant as _
    }
}
///Field `MSTSCLLOW` reader - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW.
pub struct MSTSCLLOW_R(crate::FieldReader<u8, MSTSCLLOW_A>);
impl MSTSCLLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MSTSCLLOW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
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
    ///Checks if the value of the field is `CLOCKS_2`
    #[inline(always)]
    pub fn is_clocks_2(&self) -> bool {
        **self == MSTSCLLOW_A::CLOCKS_2
    }
    ///Checks if the value of the field is `CLOCKS_3`
    #[inline(always)]
    pub fn is_clocks_3(&self) -> bool {
        **self == MSTSCLLOW_A::CLOCKS_3
    }
    ///Checks if the value of the field is `CLOCKS_4`
    #[inline(always)]
    pub fn is_clocks_4(&self) -> bool {
        **self == MSTSCLLOW_A::CLOCKS_4
    }
    ///Checks if the value of the field is `CLOCKS_5`
    #[inline(always)]
    pub fn is_clocks_5(&self) -> bool {
        **self == MSTSCLLOW_A::CLOCKS_5
    }
    ///Checks if the value of the field is `CLOCKS_6`
    #[inline(always)]
    pub fn is_clocks_6(&self) -> bool {
        **self == MSTSCLLOW_A::CLOCKS_6
    }
    ///Checks if the value of the field is `CLOCKS_7`
    #[inline(always)]
    pub fn is_clocks_7(&self) -> bool {
        **self == MSTSCLLOW_A::CLOCKS_7
    }
    ///Checks if the value of the field is `CLOCKS_8`
    #[inline(always)]
    pub fn is_clocks_8(&self) -> bool {
        **self == MSTSCLLOW_A::CLOCKS_8
    }
    ///Checks if the value of the field is `CLOCKS_9`
    #[inline(always)]
    pub fn is_clocks_9(&self) -> bool {
        **self == MSTSCLLOW_A::CLOCKS_9
    }
}
impl core::ops::Deref for MSTSCLLOW_R {
    type Target = crate::FieldReader<u8, MSTSCLLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MSTSCLLOW` writer - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW.
pub struct MSTSCLLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSCLLOW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSTSCLLOW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_2(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_2)
    }
    ///3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_3(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_3)
    }
    ///4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_4(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_4)
    }
    ///5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_5(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_5)
    }
    ///6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_6(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_6)
    }
    ///7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_7(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_7)
    }
    ///8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_8(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_8)
    }
    ///9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_9(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::CLOCKS_9)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
///Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH.
///
///Value on reset: 7
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSTSCLHIGH_A {
    ///0: 2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider.
    CLOCKS_2 = 0,
    ///1: 3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider .
    CLOCKS_3 = 1,
    ///2: 4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider.
    CLOCKS_4 = 2,
    ///3: 5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider.
    CLOCKS_5 = 3,
    ///4: 6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider.
    CLOCKS_6 = 4,
    ///5: 7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider.
    CLOCKS_7 = 5,
    ///6: 8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider.
    CLOCKS_8 = 6,
    ///7: 9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider.
    CLOCKS_9 = 7,
}
impl From<MSTSCLHIGH_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTSCLHIGH_A) -> Self {
        variant as _
    }
}
///Field `MSTSCLHIGH` reader - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH.
pub struct MSTSCLHIGH_R(crate::FieldReader<u8, MSTSCLHIGH_A>);
impl MSTSCLHIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MSTSCLHIGH_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
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
    ///Checks if the value of the field is `CLOCKS_2`
    #[inline(always)]
    pub fn is_clocks_2(&self) -> bool {
        **self == MSTSCLHIGH_A::CLOCKS_2
    }
    ///Checks if the value of the field is `CLOCKS_3`
    #[inline(always)]
    pub fn is_clocks_3(&self) -> bool {
        **self == MSTSCLHIGH_A::CLOCKS_3
    }
    ///Checks if the value of the field is `CLOCKS_4`
    #[inline(always)]
    pub fn is_clocks_4(&self) -> bool {
        **self == MSTSCLHIGH_A::CLOCKS_4
    }
    ///Checks if the value of the field is `CLOCKS_5`
    #[inline(always)]
    pub fn is_clocks_5(&self) -> bool {
        **self == MSTSCLHIGH_A::CLOCKS_5
    }
    ///Checks if the value of the field is `CLOCKS_6`
    #[inline(always)]
    pub fn is_clocks_6(&self) -> bool {
        **self == MSTSCLHIGH_A::CLOCKS_6
    }
    ///Checks if the value of the field is `CLOCKS_7`
    #[inline(always)]
    pub fn is_clocks_7(&self) -> bool {
        **self == MSTSCLHIGH_A::CLOCKS_7
    }
    ///Checks if the value of the field is `CLOCKS_8`
    #[inline(always)]
    pub fn is_clocks_8(&self) -> bool {
        **self == MSTSCLHIGH_A::CLOCKS_8
    }
    ///Checks if the value of the field is `CLOCKS_9`
    #[inline(always)]
    pub fn is_clocks_9(&self) -> bool {
        **self == MSTSCLHIGH_A::CLOCKS_9
    }
}
impl core::ops::Deref for MSTSCLHIGH_R {
    type Target = crate::FieldReader<u8, MSTSCLHIGH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MSTSCLHIGH` writer - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH.
pub struct MSTSCLHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSCLHIGH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSTSCLHIGH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_2(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_2)
    }
    ///3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider .
    #[inline(always)]
    pub fn clocks_3(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_3)
    }
    ///4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_4(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_4)
    }
    ///5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_5(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_5)
    }
    ///6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_6(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_6)
    }
    ///7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_7(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_7)
    }
    ///8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_8(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_8)
    }
    ///9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider.
    #[inline(always)]
    pub fn clocks_9(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::CLOCKS_9)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW.
    #[inline(always)]
    pub fn mstscllow(&self) -> MSTSCLLOW_R {
        MSTSCLLOW_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 4:6 - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH.
    #[inline(always)]
    pub fn mstsclhigh(&self) -> MSTSCLHIGH_R {
        MSTSCLHIGH_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW.
    #[inline(always)]
    pub fn mstscllow(&mut self) -> MSTSCLLOW_W {
        MSTSCLLOW_W { w: self }
    }
    ///Bits 4:6 - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH.
    #[inline(always)]
    pub fn mstsclhigh(&mut self) -> MSTSCLHIGH_W {
        MSTSCLHIGH_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Master timing configuration.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [msttime](index.html) module
pub struct MSTTIME_SPEC;
impl crate::RegisterSpec for MSTTIME_SPEC {
    type Ux = u32;
}
///`read()` method returns [msttime::R](R) reader structure
impl crate::Readable for MSTTIME_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [msttime::W](W) writer structure
impl crate::Writable for MSTTIME_SPEC {
    type Writer = W;
}
///`reset()` method sets MSTTIME to value 0x77
impl crate::Resettable for MSTTIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x77
    }
}
