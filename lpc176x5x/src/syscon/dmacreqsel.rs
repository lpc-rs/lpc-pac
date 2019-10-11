#[doc = "Reader of register DMACREQSEL"]
pub type R = crate::R<u32, super::DMACREQSEL>;
#[doc = "Writer for register DMACREQSEL"]
pub type W = crate::W<u32, super::DMACREQSEL>;
#[doc = "Register DMACREQSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACREQSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMASEL08`"]
pub type DMASEL08_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL08`"]
pub struct DMASEL08_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL08_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DMASEL09`"]
pub type DMASEL09_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL09`"]
pub struct DMASEL09_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL09_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DMASEL10`"]
pub type DMASEL10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL10`"]
pub struct DMASEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMASEL11`"]
pub type DMASEL11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL11`"]
pub struct DMASEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DMASEL12`"]
pub type DMASEL12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL12`"]
pub struct DMASEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DMASEL13`"]
pub type DMASEL13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL13`"]
pub struct DMASEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DMASEL14`"]
pub type DMASEL14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL14`"]
pub struct DMASEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMASEL15`"]
pub type DMASEL15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL15`"]
pub struct DMASEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel08(&self) -> DMASEL08_R {
        DMASEL08_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel09(&self) -> DMASEL09_R {
        DMASEL09_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel10(&self) -> DMASEL10_R {
        DMASEL10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel11(&self) -> DMASEL11_R {
        DMASEL11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel12(&self) -> DMASEL12_R {
        DMASEL12_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel13(&self) -> DMASEL13_R {
        DMASEL13_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&self) -> DMASEL14_R {
        DMASEL14_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&self) -> DMASEL15_R {
        DMASEL15_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel08(&mut self) -> DMASEL08_W {
        DMASEL08_W { w: self }
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel09(&mut self) -> DMASEL09_W {
        DMASEL09_W { w: self }
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel10(&mut self) -> DMASEL10_W {
        DMASEL10_W { w: self }
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel11(&mut self) -> DMASEL11_W {
        DMASEL11_W { w: self }
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel12(&mut self) -> DMASEL12_W {
        DMASEL12_W { w: self }
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel13(&mut self) -> DMASEL13_W {
        DMASEL13_W { w: self }
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&mut self) -> DMASEL14_W {
        DMASEL14_W { w: self }
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&mut self) -> DMASEL15_W {
        DMASEL15_W { w: self }
    }
}
