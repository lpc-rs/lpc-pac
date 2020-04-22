#[doc = "Reader of register ULPIDEBUG"]
pub type R = crate::R<u32, super::ULPIDEBUG>;
#[doc = "Writer for register ULPIDEBUG"]
pub type W = crate::W<u32, super::ULPIDEBUG>;
#[doc = "Register ULPIDEBUG `reset()`'s with value 0"]
impl crate::ResetValue for super::ULPIDEBUG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHY_ADDR`"]
pub type PHY_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHY_ADDR`"]
pub struct PHY_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PHY_WDATA`"]
pub type PHY_WDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHY_WDATA`"]
pub struct PHY_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PHY_RDATA`"]
pub type PHY_RDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHY_RDATA`"]
pub struct PHY_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PHY_RW`"]
pub type PHY_RW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHY_RW`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `PHY_ACCESS`"]
pub type PHY_ACCESS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHY_ACCESS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PHY_MODE`"]
pub type PHY_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHY_MODE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
