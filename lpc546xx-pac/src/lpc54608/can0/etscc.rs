///Register `ETSCC` reader
pub struct R(crate::R<ETSCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETSCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETSCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETSCC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETSCC` writer
pub struct W(crate::W<ETSCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETSCC_SPEC>;
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
impl From<crate::W<ETSCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETSCC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ETCP` reader - External timestamp prescaler value.
pub struct ETCP_R(crate::FieldReader<u16, u16>);
impl ETCP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ETCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETCP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ETCP` writer - External timestamp prescaler value.
pub struct ETCP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETCP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
///Field `ETCE` reader - External timestamp counter enable.
pub struct ETCE_R(crate::FieldReader<bool, bool>);
impl ETCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ETCE` writer - External timestamp counter enable.
pub struct ETCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:10 - External timestamp prescaler value.
    #[inline(always)]
    pub fn etcp(&self) -> ETCP_R {
        ETCP_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 31 - External timestamp counter enable.
    #[inline(always)]
    pub fn etce(&self) -> ETCE_R {
        ETCE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:10 - External timestamp prescaler value.
    #[inline(always)]
    pub fn etcp(&mut self) -> ETCP_W {
        ETCP_W { w: self }
    }
    ///Bit 31 - External timestamp counter enable.
    #[inline(always)]
    pub fn etce(&mut self) -> ETCE_W {
        ETCE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///External Timestamp Counter Configuration
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [etscc](index.html) module
pub struct ETSCC_SPEC;
impl crate::RegisterSpec for ETSCC_SPEC {
    type Ux = u32;
}
///`read()` method returns [etscc::R](R) reader structure
impl crate::Readable for ETSCC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [etscc::W](W) writer structure
impl crate::Writable for ETSCC_SPEC {
    type Writer = W;
}
///`reset()` method sets ETSCC to value 0
impl crate::Resettable for ETSCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
