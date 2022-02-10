///Register `EMCDLYCAL` reader
pub struct R(crate::R<EMCDLYCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMCDLYCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMCDLYCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMCDLYCAL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EMCDLYCAL` writer
pub struct W(crate::W<EMCDLYCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMCDLYCAL_SPEC>;
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
impl From<crate::W<EMCDLYCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMCDLYCAL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CALVALUE` reader - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz.
pub struct CALVALUE_R(crate::FieldReader<u8, u8>);
impl CALVALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CALVALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALVALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CALVALUE` writer - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz.
pub struct CALVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALVALUE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `START` reader - Start control bit for the EMC calibration counter.
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `START` writer - Start control bit for the EMC calibration counter.
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Field `DONE` reader - Measurement completion flag.
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DONE` writer - Measurement completion flag.
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz.
    #[inline(always)]
    pub fn calvalue(&self) -> CALVALUE_R {
        CALVALUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 14 - Start control bit for the EMC calibration counter.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Measurement completion flag.
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:7 - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz.
    #[inline(always)]
    pub fn calvalue(&mut self) -> CALVALUE_W {
        CALVALUE_W { w: self }
    }
    ///Bit 14 - Start control bit for the EMC calibration counter.
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    ///Bit 15 - Measurement completion flag.
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EMC delay chain calibration control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [emcdlycal](index.html) module
pub struct EMCDLYCAL_SPEC;
impl crate::RegisterSpec for EMCDLYCAL_SPEC {
    type Ux = u32;
}
///`read()` method returns [emcdlycal::R](R) reader structure
impl crate::Readable for EMCDLYCAL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [emcdlycal::W](W) writer structure
impl crate::Writable for EMCDLYCAL_SPEC {
    type Writer = W;
}
///`reset()` method sets EMCDLYCAL to value 0
impl crate::Resettable for EMCDLYCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
