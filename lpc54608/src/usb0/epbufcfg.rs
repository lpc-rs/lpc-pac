#[doc = "Register `EPBUFCFG` reader"]
pub struct R(crate::R<EPBUFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPBUFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPBUFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPBUFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPBUFCFG` writer"]
pub struct W(crate::W<EPBUFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPBUFCFG_SPEC>;
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
impl From<crate::W<EPBUFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPBUFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF_SB` reader - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
pub struct BUF_SB_R(crate::FieldReader<u8, u8>);
impl BUF_SB_R {
    pub(crate) fn new(bits: u8) -> Self {
        BUF_SB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_SB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF_SB` writer - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
pub struct BUF_SB_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_SB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | ((value as u32 & 0xff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:9 - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
    #[inline(always)]
    pub fn buf_sb(&self) -> BUF_SB_R {
        BUF_SB_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:9 - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
    #[inline(always)]
    pub fn buf_sb(&mut self) -> BUF_SB_W {
        BUF_SB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Buffer Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epbufcfg](index.html) module"]
pub struct EPBUFCFG_SPEC;
impl crate::RegisterSpec for EPBUFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epbufcfg::R](R) reader structure"]
impl crate::Readable for EPBUFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epbufcfg::W](W) writer structure"]
impl crate::Writable for EPBUFCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPBUFCFG to value 0"]
impl crate::Resettable for EPBUFCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
