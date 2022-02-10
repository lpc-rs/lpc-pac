///Register `TXBRP` reader
pub struct R(crate::R<TXBRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBRP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXBRP` writer
pub struct W(crate::W<TXBRP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBRP_SPEC>;
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
impl From<crate::W<TXBRP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBRP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRP` reader - Transmission request pending.
pub struct TRP_R(crate::FieldReader<u32, u32>);
impl TRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRP` writer - Transmission request pending.
pub struct TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Transmission request pending.
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Transmission request pending.
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W {
        TRP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Tx Buffer Request Pending
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbrp](index.html) module
pub struct TXBRP_SPEC;
impl crate::RegisterSpec for TXBRP_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbrp::R](R) reader structure
impl crate::Readable for TXBRP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txbrp::W](W) writer structure
impl crate::Writable for TXBRP_SPEC {
    type Writer = W;
}
///`reset()` method sets TXBRP to value 0
impl crate::Resettable for TXBRP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
