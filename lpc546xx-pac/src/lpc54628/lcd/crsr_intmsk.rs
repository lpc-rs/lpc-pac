///Register `CRSR_INTMSK` reader
pub struct R(crate::R<CRSR_INTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_INTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_INTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_INTMSK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRSR_INTMSK` writer
pub struct W(crate::W<CRSR_INTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_INTMSK_SPEC>;
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
impl From<crate::W<CRSR_INTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_INTMSK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CRSRIM` reader - Cursor interrupt mask.
pub struct CRSRIM_R(crate::FieldReader<bool, bool>);
impl CRSRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRSRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSRIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CRSRIM` writer - Cursor interrupt mask.
pub struct CRSRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRIM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    ///Bit 0 - Cursor interrupt mask.
    #[inline(always)]
    pub fn crsrim(&self) -> CRSRIM_R {
        CRSRIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Cursor interrupt mask.
    #[inline(always)]
    pub fn crsrim(&mut self) -> CRSRIM_W {
        CRSRIM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Cursor Interrupt Mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crsr_intmsk](index.html) module
pub struct CRSR_INTMSK_SPEC;
impl crate::RegisterSpec for CRSR_INTMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [crsr_intmsk::R](R) reader structure
impl crate::Readable for CRSR_INTMSK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crsr_intmsk::W](W) writer structure
impl crate::Writable for CRSR_INTMSK_SPEC {
    type Writer = W;
}
///`reset()` method sets CRSR_INTMSK to value 0
impl crate::Resettable for CRSR_INTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
