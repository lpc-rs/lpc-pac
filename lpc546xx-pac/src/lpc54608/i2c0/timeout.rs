///Register `TIMEOUT` reader
pub struct R(crate::R<TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIMEOUT` writer
pub struct W(crate::W<TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUT_SPEC>;
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
impl From<crate::W<TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TOMIN` reader - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks.
pub struct TOMIN_R(crate::FieldReader<u8, u8>);
impl TOMIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOMIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOMIN` writer - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks.
pub struct TOMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOMIN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
///Field `TO` reader - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock.
pub struct TO_R(crate::FieldReader<u16, u16>);
impl TO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TO` writer - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock.
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks.
    #[inline(always)]
    pub fn tomin(&self) -> TOMIN_R {
        TOMIN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:15 - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock.
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:3 - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks.
    #[inline(always)]
    pub fn tomin(&mut self) -> TOMIN_W {
        TOMIN_W { w: self }
    }
    ///Bits 4:15 - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock.
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Time-out value register.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timeout](index.html) module
pub struct TIMEOUT_SPEC;
impl crate::RegisterSpec for TIMEOUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [timeout::R](R) reader structure
impl crate::Readable for TIMEOUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [timeout::W](W) writer structure
impl crate::Writable for TIMEOUT_SPEC {
    type Writer = W;
}
///`reset()` method sets TIMEOUT to value 0xffff
impl crate::Resettable for TIMEOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
