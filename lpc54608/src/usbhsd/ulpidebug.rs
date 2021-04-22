#[doc = "Register `ULPIDEBUG` reader"]
pub struct R(crate::R<ULPIDEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULPIDEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ULPIDEBUG_SPEC>> for R {
    fn from(reader: crate::R<ULPIDEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULPIDEBUG` writer"]
pub struct W(crate::W<ULPIDEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULPIDEBUG_SPEC>;
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
impl core::convert::From<crate::W<ULPIDEBUG_SPEC>> for W {
    fn from(writer: crate::W<ULPIDEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_ADDR` reader - ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface."]
pub struct PHY_ADDR_R(crate::FieldReader<u8, u8>);
impl PHY_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHY_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_ADDR` writer - ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface."]
pub struct PHY_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PHY_WDATA` reader - UTMI+ mode: Reserved."]
pub struct PHY_WDATA_R(crate::FieldReader<u8, u8>);
impl PHY_WDATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHY_WDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_WDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_WDATA` writer - UTMI+ mode: Reserved."]
pub struct PHY_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PHY_RDATA` reader - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+."]
pub struct PHY_RDATA_R(crate::FieldReader<u8, u8>);
impl PHY_RDATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHY_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_RDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_RDATA` writer - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+."]
pub struct PHY_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PHY_RW` reader - UTMI+ mode: Reserved."]
pub struct PHY_RW_R(crate::FieldReader<bool, bool>);
impl PHY_RW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHY_RW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_RW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_RW` writer - UTMI+ mode: Reserved."]
pub struct PHY_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_RW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PHY_ACCESS` reader - Software writes this bit to one to start a read or write operation."]
pub struct PHY_ACCESS_R(crate::FieldReader<bool, bool>);
impl PHY_ACCESS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHY_ACCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_ACCESS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_ACCESS` writer - Software writes this bit to one to start a read or write operation."]
pub struct PHY_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_ACCESS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PHY_MODE` reader - This bit indicates if the interface between the controller is UTMI+ or ULPI."]
pub struct PHY_MODE_R(crate::FieldReader<bool, bool>);
impl PHY_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_MODE` writer - This bit indicates if the interface between the controller is UTMI+ or ULPI."]
pub struct PHY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface."]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - UTMI+ mode: Reserved."]
    #[inline(always)]
    pub fn phy_wdata(&self) -> PHY_WDATA_R {
        PHY_WDATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+."]
    #[inline(always)]
    pub fn phy_rdata(&self) -> PHY_RDATA_R {
        PHY_RDATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - UTMI+ mode: Reserved."]
    #[inline(always)]
    pub fn phy_rw(&self) -> PHY_RW_R {
        PHY_RW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Software writes this bit to one to start a read or write operation."]
    #[inline(always)]
    pub fn phy_access(&self) -> PHY_ACCESS_R {
        PHY_ACCESS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit indicates if the interface between the controller is UTMI+ or ULPI."]
    #[inline(always)]
    pub fn phy_mode(&self) -> PHY_MODE_R {
        PHY_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface."]
    #[inline(always)]
    pub fn phy_addr(&mut self) -> PHY_ADDR_W {
        PHY_ADDR_W { w: self }
    }
    #[doc = "Bits 8:15 - UTMI+ mode: Reserved."]
    #[inline(always)]
    pub fn phy_wdata(&mut self) -> PHY_WDATA_W {
        PHY_WDATA_W { w: self }
    }
    #[doc = "Bits 16:23 - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+."]
    #[inline(always)]
    pub fn phy_rdata(&mut self) -> PHY_RDATA_W {
        PHY_RDATA_W { w: self }
    }
    #[doc = "Bit 24 - UTMI+ mode: Reserved."]
    #[inline(always)]
    pub fn phy_rw(&mut self) -> PHY_RW_W {
        PHY_RW_W { w: self }
    }
    #[doc = "Bit 25 - Software writes this bit to one to start a read or write operation."]
    #[inline(always)]
    pub fn phy_access(&mut self) -> PHY_ACCESS_W {
        PHY_ACCESS_W { w: self }
    }
    #[doc = "Bit 31 - This bit indicates if the interface between the controller is UTMI+ or ULPI."]
    #[inline(always)]
    pub fn phy_mode(&mut self) -> PHY_MODE_W {
        PHY_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UTMI/ULPI debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulpidebug](index.html) module"]
pub struct ULPIDEBUG_SPEC;
impl crate::RegisterSpec for ULPIDEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulpidebug::R](R) reader structure"]
impl crate::Readable for ULPIDEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulpidebug::W](W) writer structure"]
impl crate::Writable for ULPIDEBUG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULPIDEBUG to value 0"]
impl crate::Resettable for ULPIDEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
