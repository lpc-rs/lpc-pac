///Register `ISO_PTD_BASE_ADDR` reader
pub struct R(crate::R<ISO_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISO_PTD_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISO_PTD_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISO_PTD_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ISO_PTD_BASE_ADDR` writer
pub struct W(crate::W<ISO_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISO_PTD_BASE_ADDR_SPEC>;
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
impl From<crate::W<ISO_PTD_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISO_PTD_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ISO_FIRST` reader - This indicates the first PTD that is used by the hardware when it is processing the ISO list.
pub struct ISO_FIRST_R(crate::FieldReader<u8, u8>);
impl ISO_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ISO_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISO_FIRST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ISO_FIRST` writer - This indicates the first PTD that is used by the hardware when it is processing the ISO list.
pub struct ISO_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_FIRST_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
///Field `ISO_BASE` reader - Base address to be used by the hardware to find the start of the ISO list.
pub struct ISO_BASE_R(crate::FieldReader<u32, u32>);
impl ISO_BASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ISO_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISO_BASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ISO_BASE` writer - Base address to be used by the hardware to find the start of the ISO list.
pub struct ISO_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_BASE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    ///Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list.
    #[inline(always)]
    pub fn iso_first(&self) -> ISO_FIRST_R {
        ISO_FIRST_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list.
    #[inline(always)]
    pub fn iso_base(&self) -> ISO_BASE_R {
        ISO_BASE_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    ///Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list.
    #[inline(always)]
    pub fn iso_first(&mut self) -> ISO_FIRST_W {
        ISO_FIRST_W { w: self }
    }
    ///Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list.
    #[inline(always)]
    pub fn iso_base(&mut self) -> ISO_BASE_W {
        ISO_BASE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Memory base address where ISO PTD0 is stored
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iso_ptd_base_addr](index.html) module
pub struct ISO_PTD_BASE_ADDR_SPEC;
impl crate::RegisterSpec for ISO_PTD_BASE_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iso_ptd_base_addr::R](R) reader structure
impl crate::Readable for ISO_PTD_BASE_ADDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [iso_ptd_base_addr::W](W) writer structure
impl crate::Writable for ISO_PTD_BASE_ADDR_SPEC {
    type Writer = W;
}
///`reset()` method sets ISO_PTD_BASE_ADDR to value 0
impl crate::Resettable for ISO_PTD_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
