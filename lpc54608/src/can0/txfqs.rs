#[doc = "Reader of register TXFQS"]
pub type R = crate::R<u32, super::TXFQS>;
#[doc = "Writer for register TXFQS"]
pub type W = crate::W<u32, super::TXFQS>;
#[doc = "Register TXFQS `reset()`'s with value 0"]
impl crate::ResetValue for super::TXFQS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TFGI`"]
pub type TFGI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFGI`"]
pub struct TFGI_W<'a> {
    w: &'a mut W,
}
impl<'a> TFGI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TFQPI`"]
pub type TFQPI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFQPI`"]
pub struct TFQPI_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TFQF`"]
pub type TFQF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFQF`"]
pub struct TFQF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - Tx FIFO get index."]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/queue put index."]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full."]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - Tx FIFO get index."]
    #[inline(always)]
    pub fn tfgi(&mut self) -> TFGI_W {
        TFGI_W { w: self }
    }
    #[doc = "Bits 16:20 - Tx FIFO/queue put index."]
    #[inline(always)]
    pub fn tfqpi(&mut self) -> TFQPI_W {
        TFQPI_W { w: self }
    }
    #[doc = "Bit 21 - Tx FIFO/queue full."]
    #[inline(always)]
    pub fn tfqf(&mut self) -> TFQF_W {
        TFQF_W { w: self }
    }
}
