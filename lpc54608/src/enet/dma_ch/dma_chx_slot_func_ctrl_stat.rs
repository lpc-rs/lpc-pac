#[doc = "Reader of register DMA_CHx_SLOT_FUNC_CTRL_STAT"]
pub type R = crate::R<u32, super::DMA_CHX_SLOT_FUNC_CTRL_STAT>;
#[doc = "Writer for register DMA_CHx_SLOT_FUNC_CTRL_STAT"]
pub type W = crate::W<u32, super::DMA_CHX_SLOT_FUNC_CTRL_STAT>;
#[doc = "Register DMA_CHx_SLOT_FUNC_CTRL_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CHX_SLOT_FUNC_CTRL_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ESC`"]
pub type ESC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESC`"]
pub struct ESC_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_W<'a> {
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
#[doc = "Reader of field `ASC`"]
pub type ASC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC`"]
pub struct ASC_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC_W<'a> {
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
#[doc = "Reader of field `RSN`"]
pub type RSN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Reference Slot Number This field gives the current value of the reference slot number in the DMA."]
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    #[inline(always)]
    pub fn esc(&mut self) -> ESC_W {
        ESC_W { w: self }
    }
    #[doc = "Bit 1 - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    #[inline(always)]
    pub fn asc(&mut self) -> ASC_W {
        ASC_W { w: self }
    }
}
