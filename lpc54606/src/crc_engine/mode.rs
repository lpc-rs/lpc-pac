#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MODE_SPEC>> for R {
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl core::convert::From<crate::W<MODE_SPEC>> for W {
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_POLY` reader - CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
pub struct CRC_POLY_R(crate::FieldReader<u8, u8>);
impl CRC_POLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRC_POLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_POLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_POLY` writer - CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
pub struct CRC_POLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_POLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `BIT_RVS_WR` reader - Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
pub struct BIT_RVS_WR_R(crate::FieldReader<bool, bool>);
impl BIT_RVS_WR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT_RVS_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT_RVS_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT_RVS_WR` writer - Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
pub struct BIT_RVS_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_RVS_WR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CMPL_WR` reader - Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
pub struct CMPL_WR_R(crate::FieldReader<bool, bool>);
impl CMPL_WR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPL_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPL_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPL_WR` writer - Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
pub struct CMPL_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPL_WR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `BIT_RVS_SUM` reader - CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
pub struct BIT_RVS_SUM_R(crate::FieldReader<bool, bool>);
impl BIT_RVS_SUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT_RVS_SUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT_RVS_SUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT_RVS_SUM` writer - CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
pub struct BIT_RVS_SUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_RVS_SUM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CMPL_SUM` reader - CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
pub struct CMPL_SUM_R(crate::FieldReader<bool, bool>);
impl CMPL_SUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPL_SUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPL_SUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPL_SUM` writer - CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
pub struct CMPL_SUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPL_SUM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn crc_poly(&self) -> CRC_POLY_R {
        CRC_POLY_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn bit_rvs_wr(&self) -> BIT_RVS_WR_R {
        BIT_RVS_WR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&self) -> CMPL_WR_R {
        CMPL_WR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&self) -> BIT_RVS_SUM_R {
        BIT_RVS_SUM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&self) -> CMPL_SUM_R {
        CMPL_SUM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn crc_poly(&mut self) -> CRC_POLY_W {
        CRC_POLY_W { w: self }
    }
    #[doc = "Bit 2 - Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn bit_rvs_wr(&mut self) -> BIT_RVS_WR_W {
        BIT_RVS_WR_W { w: self }
    }
    #[doc = "Bit 3 - Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&mut self) -> CMPL_WR_W {
        CMPL_WR_W { w: self }
    }
    #[doc = "Bit 4 - CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&mut self) -> BIT_RVS_SUM_W {
        BIT_RVS_SUM_W { w: self }
    }
    #[doc = "Bit 5 - CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&mut self) -> CMPL_SUM_W {
        CMPL_SUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
