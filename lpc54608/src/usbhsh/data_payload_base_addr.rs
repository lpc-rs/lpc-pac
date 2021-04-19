#[doc = "Register `DATA_PAYLOAD_BASE_ADDR` reader"]
pub struct R(crate::R<DATA_PAYLOAD_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_PAYLOAD_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DATA_PAYLOAD_BASE_ADDR_SPEC>> for R {
    fn from(reader: crate::R<DATA_PAYLOAD_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_PAYLOAD_BASE_ADDR` writer"]
pub struct W(crate::W<DATA_PAYLOAD_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_PAYLOAD_BASE_ADDR_SPEC>;
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
impl core::convert::From<crate::W<DATA_PAYLOAD_BASE_ADDR_SPEC>> for W {
    fn from(writer: crate::W<DATA_PAYLOAD_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT_BASE` reader - Base address to be used by the hardware to find the start of the data payload section."]
pub struct DAT_BASE_R(crate::FieldReader<u16, u16>);
impl DAT_BASE_R {
    pub(crate) fn new(bits: u16) -> Self {
        DAT_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAT_BASE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_BASE` writer - Base address to be used by the hardware to find the start of the data payload section."]
pub struct DAT_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub fn dat_base(&self) -> DAT_BASE_R {
        DAT_BASE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub fn dat_base(&mut self) -> DAT_BASE_W {
        DAT_BASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory base address that indicates the start of the data payload buffers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_payload_base_addr](index.html) module"]
pub struct DATA_PAYLOAD_BASE_ADDR_SPEC;
impl crate::RegisterSpec for DATA_PAYLOAD_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_payload_base_addr::R](R) reader structure"]
impl crate::Readable for DATA_PAYLOAD_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_payload_base_addr::W](W) writer structure"]
impl crate::Writable for DATA_PAYLOAD_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_PAYLOAD_BASE_ADDR to value 0"]
impl crate::Resettable for DATA_PAYLOAD_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
