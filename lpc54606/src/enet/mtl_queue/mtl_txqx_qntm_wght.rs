#[doc = "Register `MTL_TXQx_QNTM_WGHT` reader"]
pub struct R(crate::R<MTL_TXQX_QNTM_WGHT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_QNTM_WGHT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MTL_TXQX_QNTM_WGHT_SPEC>> for R {
    fn from(reader: crate::R<MTL_TXQX_QNTM_WGHT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_TXQx_QNTM_WGHT` writer"]
pub struct W(crate::W<MTL_TXQX_QNTM_WGHT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_QNTM_WGHT_SPEC>;
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
impl core::convert::From<crate::W<MTL_TXQX_QNTM_WGHT_SPEC>> for W {
    fn from(writer: crate::W<MTL_TXQX_QNTM_WGHT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISCQW` reader - Average Bits per Slot."]
pub struct ISCQW_R(crate::FieldReader<u32, u32>);
impl ISCQW_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISCQW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISCQW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISCQW` writer - Average Bits per Slot."]
pub struct ISCQW_W<'a> {
    w: &'a mut W,
}
impl<'a> ISCQW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | (value as u32 & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - Average Bits per Slot."]
    #[inline(always)]
    pub fn iscqw(&self) -> ISCQW_R {
        ISCQW_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - Average Bits per Slot."]
    #[inline(always)]
    pub fn iscqw(&mut self) -> ISCQW_W {
        ISCQW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_qntm_wght](index.html) module"]
pub struct MTL_TXQX_QNTM_WGHT_SPEC;
impl crate::RegisterSpec for MTL_TXQX_QNTM_WGHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_txqx_qntm_wght::R](R) reader structure"]
impl crate::Readable for MTL_TXQX_QNTM_WGHT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_qntm_wght::W](W) writer structure"]
impl crate::Writable for MTL_TXQX_QNTM_WGHT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_TXQx_QNTM_WGHT to value 0"]
impl crate::Resettable for MTL_TXQX_QNTM_WGHT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
