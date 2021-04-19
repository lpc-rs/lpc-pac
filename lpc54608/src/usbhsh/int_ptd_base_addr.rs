#[doc = "Register `INT_PTD_BASE_ADDR` reader"]
pub struct R(crate::R<INT_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_PTD_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INT_PTD_BASE_ADDR_SPEC>> for R {
    fn from(reader: crate::R<INT_PTD_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_PTD_BASE_ADDR` writer"]
pub struct W(crate::W<INT_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_PTD_BASE_ADDR_SPEC>;
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
impl core::convert::From<crate::W<INT_PTD_BASE_ADDR_SPEC>> for W {
    fn from(writer: crate::W<INT_PTD_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_FIRST` reader - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
pub struct INT_FIRST_R(crate::FieldReader<u8, u8>);
impl INT_FIRST_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_FIRST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_FIRST` writer - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
pub struct INT_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_FIRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `INT_BASE` reader - Base address to be used by the hardware to find the start of the INT list."]
pub struct INT_BASE_R(crate::FieldReader<u32, u32>);
impl INT_BASE_R {
    pub(crate) fn new(bits: u32) -> Self {
        INT_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_BASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_BASE` writer - Base address to be used by the hardware to find the start of the INT list."]
pub struct INT_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[inline(always)]
    pub fn int_first(&self) -> INT_FIRST_R {
        INT_FIRST_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the INT list."]
    #[inline(always)]
    pub fn int_base(&self) -> INT_BASE_R {
        INT_BASE_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the INT list."]
    #[inline(always)]
    pub fn int_first(&mut self) -> INT_FIRST_W {
        INT_FIRST_W { w: self }
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the INT list."]
    #[inline(always)]
    pub fn int_base(&mut self) -> INT_BASE_W {
        INT_BASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory base address where INT PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ptd_base_addr](index.html) module"]
pub struct INT_PTD_BASE_ADDR_SPEC;
impl crate::RegisterSpec for INT_PTD_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ptd_base_addr::R](R) reader structure"]
impl crate::Readable for INT_PTD_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ptd_base_addr::W](W) writer structure"]
impl crate::Writable for INT_PTD_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_PTD_BASE_ADDR to value 0"]
impl crate::Resettable for INT_PTD_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
