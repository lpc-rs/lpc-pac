///Register `ATL_PTD_BASE_ADDR` reader
pub struct R(crate::R<ATL_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATL_PTD_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATL_PTD_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATL_PTD_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ATL_PTD_BASE_ADDR` writer
pub struct W(crate::W<ATL_PTD_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATL_PTD_BASE_ADDR_SPEC>;
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
impl From<crate::W<ATL_PTD_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATL_PTD_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ATL_CUR` reader - This indicates the current PTD that is used by the hardware when it is processing the ATL list.
pub struct ATL_CUR_R(crate::FieldReader<u8, u8>);
impl ATL_CUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ATL_CUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATL_CUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ATL_CUR` writer - This indicates the current PTD that is used by the hardware when it is processing the ATL list.
pub struct ATL_CUR_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_CUR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | ((value as u32 & 0x1f) << 4);
        self.w
    }
}
///Field `ATL_BASE` reader - Base address to be used by the hardware to find the start of the ATL list.
pub struct ATL_BASE_R(crate::FieldReader<u32, u32>);
impl ATL_BASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ATL_BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATL_BASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ATL_BASE` writer - Base address to be used by the hardware to find the start of the ATL list.
pub struct ATL_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_BASE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | ((value as u32 & 0x007f_ffff) << 9);
        self.w
    }
}
impl R {
    ///Bits 4:8 - This indicates the current PTD that is used by the hardware when it is processing the ATL list.
    #[inline(always)]
    pub fn atl_cur(&self) -> ATL_CUR_R {
        ATL_CUR_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    ///Bits 9:31 - Base address to be used by the hardware to find the start of the ATL list.
    #[inline(always)]
    pub fn atl_base(&self) -> ATL_BASE_R {
        ATL_BASE_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    ///Bits 4:8 - This indicates the current PTD that is used by the hardware when it is processing the ATL list.
    #[inline(always)]
    pub fn atl_cur(&mut self) -> ATL_CUR_W {
        ATL_CUR_W { w: self }
    }
    ///Bits 9:31 - Base address to be used by the hardware to find the start of the ATL list.
    #[inline(always)]
    pub fn atl_base(&mut self) -> ATL_BASE_W {
        ATL_BASE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Memory base address where ATL PTD0 is stored
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [atl_ptd_base_addr](index.html) module
pub struct ATL_PTD_BASE_ADDR_SPEC;
impl crate::RegisterSpec for ATL_PTD_BASE_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [atl_ptd_base_addr::R](R) reader structure
impl crate::Readable for ATL_PTD_BASE_ADDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [atl_ptd_base_addr::W](W) writer structure
impl crate::Writable for ATL_PTD_BASE_ADDR_SPEC {
    type Writer = W;
}
///`reset()` method sets ATL_PTD_BASE_ADDR to value 0
impl crate::Resettable for ATL_PTD_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
