#[doc = "Register `MAC_1US_TIC_COUNTR` reader"]
pub struct R(crate::R<MAC_1US_TIC_COUNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_1US_TIC_COUNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_1US_TIC_COUNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_1US_TIC_COUNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_1US_TIC_COUNTR` writer"]
pub struct W(crate::W<MAC_1US_TIC_COUNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_1US_TIC_COUNTR_SPEC>;
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
impl From<crate::W<MAC_1US_TIC_COUNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_1US_TIC_COUNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIC_1US_CNTR` reader - 1US TIC Counter The application must program this counter so that the number of clock cycles of CSR clock is 1us."]
pub struct TIC_1US_CNTR_R(crate::FieldReader<u16, u16>);
impl TIC_1US_CNTR_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIC_1US_CNTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIC_1US_CNTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIC_1US_CNTR` writer - 1US TIC Counter The application must program this counter so that the number of clock cycles of CSR clock is 1us."]
pub struct TIC_1US_CNTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIC_1US_CNTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - 1US TIC Counter The application must program this counter so that the number of clock cycles of CSR clock is 1us."]
    #[inline(always)]
    pub fn tic_1us_cntr(&self) -> TIC_1US_CNTR_R {
        TIC_1US_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 1US TIC Counter The application must program this counter so that the number of clock cycles of CSR clock is 1us."]
    #[inline(always)]
    pub fn tic_1us_cntr(&mut self) -> TIC_1US_CNTR_W {
        TIC_1US_CNTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_1us_tic_countr](index.html) module"]
pub struct MAC_1US_TIC_COUNTR_SPEC;
impl crate::RegisterSpec for MAC_1US_TIC_COUNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_1us_tic_countr::R](R) reader structure"]
impl crate::Readable for MAC_1US_TIC_COUNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_1us_tic_countr::W](W) writer structure"]
impl crate::Writable for MAC_1US_TIC_COUNTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_1US_TIC_COUNTR to value 0"]
impl crate::Resettable for MAC_1US_TIC_COUNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
