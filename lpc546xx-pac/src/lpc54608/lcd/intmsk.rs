///Register `INTMSK` reader
pub struct R(crate::R<INTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTMSK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `INTMSK` writer
pub struct W(crate::W<INTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTMSK_SPEC>;
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
impl From<crate::W<INTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTMSK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FUFIM` reader - FIFO underflow interrupt enable.
pub struct FUFIM_R(crate::FieldReader<bool, bool>);
impl FUFIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FUFIM` writer - FIFO underflow interrupt enable.
pub struct FUFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FUFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Field `LNBUIM` reader - LCD next base address update interrupt enable.
pub struct LNBUIM_R(crate::FieldReader<bool, bool>);
impl LNBUIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LNBUIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNBUIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LNBUIM` writer - LCD next base address update interrupt enable.
pub struct LNBUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LNBUIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `VCOMPIM` reader - Vertical compare interrupt enable.
pub struct VCOMPIM_R(crate::FieldReader<bool, bool>);
impl VCOMPIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCOMPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCOMPIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VCOMPIM` writer - Vertical compare interrupt enable.
pub struct VCOMPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOMPIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `BERIM` reader - AHB master error interrupt enable.
pub struct BERIM_R(crate::FieldReader<bool, bool>);
impl BERIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BERIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BERIM` writer - AHB master error interrupt enable.
pub struct BERIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BERIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    ///Bit 1 - FIFO underflow interrupt enable.
    #[inline(always)]
    pub fn fufim(&self) -> FUFIM_R {
        FUFIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - LCD next base address update interrupt enable.
    #[inline(always)]
    pub fn lnbuim(&self) -> LNBUIM_R {
        LNBUIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Vertical compare interrupt enable.
    #[inline(always)]
    pub fn vcompim(&self) -> VCOMPIM_R {
        VCOMPIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - AHB master error interrupt enable.
    #[inline(always)]
    pub fn berim(&self) -> BERIM_R {
        BERIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - FIFO underflow interrupt enable.
    #[inline(always)]
    pub fn fufim(&mut self) -> FUFIM_W {
        FUFIM_W { w: self }
    }
    ///Bit 2 - LCD next base address update interrupt enable.
    #[inline(always)]
    pub fn lnbuim(&mut self) -> LNBUIM_W {
        LNBUIM_W { w: self }
    }
    ///Bit 3 - Vertical compare interrupt enable.
    #[inline(always)]
    pub fn vcompim(&mut self) -> VCOMPIM_W {
        VCOMPIM_W { w: self }
    }
    ///Bit 4 - AHB master error interrupt enable.
    #[inline(always)]
    pub fn berim(&mut self) -> BERIM_W {
        BERIM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt Mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [intmsk](index.html) module
pub struct INTMSK_SPEC;
impl crate::RegisterSpec for INTMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [intmsk::R](R) reader structure
impl crate::Readable for INTMSK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [intmsk::W](W) writer structure
impl crate::Writable for INTMSK_SPEC {
    type Writer = W;
}
///`reset()` method sets INTMSK to value 0
impl crate::Resettable for INTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
