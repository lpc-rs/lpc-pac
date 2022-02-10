///Register `UTMIPLUS_ULPI_DEBUG` reader
pub struct R(crate::R<UTMIPLUS_ULPI_DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UTMIPLUS_ULPI_DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UTMIPLUS_ULPI_DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UTMIPLUS_ULPI_DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `UTMIPLUS_ULPI_DEBUG` writer
pub struct W(crate::W<UTMIPLUS_ULPI_DEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UTMIPLUS_ULPI_DEBUG_SPEC>;
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
impl From<crate::W<UTMIPLUS_ULPI_DEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UTMIPLUS_ULPI_DEBUG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PHY_ADDR` reader - UTMI+ mode: Bits 3:0 are used to control VControl signal on Vendor Interface of UTMI+ ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface.
pub struct PHY_ADDR_R(crate::FieldReader<u8, u8>);
impl PHY_ADDR_R {
    #[inline(always)]
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
///Field `PHY_ADDR` writer - UTMI+ mode: Bits 3:0 are used to control VControl signal on Vendor Interface of UTMI+ ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface.
pub struct PHY_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_ADDR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `PHY_WDATA` reader - UTMI+ mode: Reserved.
pub struct PHY_WDATA_R(crate::FieldReader<u8, u8>);
impl PHY_WDATA_R {
    #[inline(always)]
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
///Field `PHY_WDATA` writer - UTMI+ mode: Reserved.
pub struct PHY_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_WDATA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
///Field `PHY_RDATA` reader - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+ ULPI mode: Bits 7:0 are used for the read data when reading a value to a ULPI PHY register.
pub struct PHY_RDATA_R(crate::FieldReader<u8, u8>);
impl PHY_RDATA_R {
    #[inline(always)]
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
///Field `PHY_RDATA` writer - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+ ULPI mode: Bits 7:0 are used for the read data when reading a value to a ULPI PHY register.
pub struct PHY_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_RDATA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
///Field `PHY_RW` reader - UTMI+ mode: Reserved.
pub struct PHY_RW_R(crate::FieldReader<bool, bool>);
impl PHY_RW_R {
    #[inline(always)]
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
///Field `PHY_RW` writer - UTMI+ mode: Reserved.
pub struct PHY_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_RW_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Field `PHY_ACCESS` reader - Software writes this bit to one to start a read or write operation.
pub struct PHY_ACCESS_R(crate::FieldReader<bool, bool>);
impl PHY_ACCESS_R {
    #[inline(always)]
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
///Field `PHY_ACCESS` writer - Software writes this bit to one to start a read or write operation.
pub struct PHY_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_ACCESS_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///Field `PHY_MODE` reader - This bit indicates if the interface between the controller is UTMI+ or ULPI 0b: UTMI+ 1b: ULPI If the hardware supports both modes, this bit is RW by SW.
pub struct PHY_MODE_R(crate::FieldReader<bool, bool>);
impl PHY_MODE_R {
    #[inline(always)]
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
///Field `PHY_MODE` writer - This bit indicates if the interface between the controller is UTMI+ or ULPI 0b: UTMI+ 1b: ULPI If the hardware supports both modes, this bit is RW by SW.
pub struct PHY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_MODE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:7 - UTMI+ mode: Bits 3:0 are used to control VControl signal on Vendor Interface of UTMI+ ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface.
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - UTMI+ mode: Reserved.
    #[inline(always)]
    pub fn phy_wdata(&self) -> PHY_WDATA_R {
        PHY_WDATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+ ULPI mode: Bits 7:0 are used for the read data when reading a value to a ULPI PHY register.
    #[inline(always)]
    pub fn phy_rdata(&self) -> PHY_RDATA_R {
        PHY_RDATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - UTMI+ mode: Reserved.
    #[inline(always)]
    pub fn phy_rw(&self) -> PHY_RW_R {
        PHY_RW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Software writes this bit to one to start a read or write operation.
    #[inline(always)]
    pub fn phy_access(&self) -> PHY_ACCESS_R {
        PHY_ACCESS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 31 - This bit indicates if the interface between the controller is UTMI+ or ULPI 0b: UTMI+ 1b: ULPI If the hardware supports both modes, this bit is RW by SW.
    #[inline(always)]
    pub fn phy_mode(&self) -> PHY_MODE_R {
        PHY_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:7 - UTMI+ mode: Bits 3:0 are used to control VControl signal on Vendor Interface of UTMI+ ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface.
    #[inline(always)]
    pub fn phy_addr(&mut self) -> PHY_ADDR_W {
        PHY_ADDR_W { w: self }
    }
    ///Bits 8:15 - UTMI+ mode: Reserved.
    #[inline(always)]
    pub fn phy_wdata(&mut self) -> PHY_WDATA_W {
        PHY_WDATA_W { w: self }
    }
    ///Bits 16:23 - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+ ULPI mode: Bits 7:0 are used for the read data when reading a value to a ULPI PHY register.
    #[inline(always)]
    pub fn phy_rdata(&mut self) -> PHY_RDATA_W {
        PHY_RDATA_W { w: self }
    }
    ///Bit 24 - UTMI+ mode: Reserved.
    #[inline(always)]
    pub fn phy_rw(&mut self) -> PHY_RW_W {
        PHY_RW_W { w: self }
    }
    ///Bit 25 - Software writes this bit to one to start a read or write operation.
    #[inline(always)]
    pub fn phy_access(&mut self) -> PHY_ACCESS_W {
        PHY_ACCESS_W { w: self }
    }
    ///Bit 31 - This bit indicates if the interface between the controller is UTMI+ or ULPI 0b: UTMI+ 1b: ULPI If the hardware supports both modes, this bit is RW by SW.
    #[inline(always)]
    pub fn phy_mode(&mut self) -> PHY_MODE_W {
        PHY_MODE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Register to read/write registers in the attached USB PHY
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [utmiplus_ulpi_debug](index.html) module
pub struct UTMIPLUS_ULPI_DEBUG_SPEC;
impl crate::RegisterSpec for UTMIPLUS_ULPI_DEBUG_SPEC {
    type Ux = u32;
}
///`read()` method returns [utmiplus_ulpi_debug::R](R) reader structure
impl crate::Readable for UTMIPLUS_ULPI_DEBUG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [utmiplus_ulpi_debug::W](W) writer structure
impl crate::Writable for UTMIPLUS_ULPI_DEBUG_SPEC {
    type Writer = W;
}
///`reset()` method sets UTMIPLUS_ULPI_DEBUG to value 0
impl crate::Resettable for UTMIPLUS_ULPI_DEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
