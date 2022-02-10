///Register `TXBCIE` reader
pub struct R(crate::R<TXBCIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCIE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXBCIE` writer
pub struct W(crate::W<TXBCIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBCIE_SPEC>;
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
impl From<crate::W<TXBCIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBCIE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CFIE` reader - Cancellation finished interrupt enable.
pub struct CFIE_R(crate::FieldReader<u32, u32>);
impl CFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFIE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CFIE` writer - Cancellation finished interrupt enable.
pub struct CFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Cancellation finished interrupt enable.
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Cancellation finished interrupt enable.
    #[inline(always)]
    pub fn cfie(&mut self) -> CFIE_W {
        CFIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Tx Buffer Cancellation Finished Interrupt Enable
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbcie](index.html) module
pub struct TXBCIE_SPEC;
impl crate::RegisterSpec for TXBCIE_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbcie::R](R) reader structure
impl crate::Readable for TXBCIE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txbcie::W](W) writer structure
impl crate::Writable for TXBCIE_SPEC {
    type Writer = W;
}
///`reset()` method sets TXBCIE to value 0
impl crate::Resettable for TXBCIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
