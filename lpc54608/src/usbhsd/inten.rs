#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP_INT_EN`"]
pub type EP_INT_EN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EP_INT_EN`"]
pub struct EP_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `FRAME_INT_EN`"]
pub type FRAME_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAME_INT_EN`"]
pub struct FRAME_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DEV_INT_EN`"]
pub type DEV_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_INT_EN`"]
pub struct DEV_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_INT_EN_W<'a> {
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
    #[doc = "Bits 0:11 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn ep_int_en(&self) -> EP_INT_EN_R {
        EP_INT_EN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn frame_int_en(&self) -> FRAME_INT_EN_R {
        FRAME_INT_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn dev_int_en(&self) -> DEV_INT_EN_R {
        DEV_INT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn ep_int_en(&mut self) -> EP_INT_EN_W {
        EP_INT_EN_W { w: self }
    }
    #[doc = "Bit 30 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn frame_int_en(&mut self) -> FRAME_INT_EN_W {
        FRAME_INT_EN_W { w: self }
    }
    #[doc = "Bit 31 - If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn dev_int_en(&mut self) -> DEV_INT_EN_W {
        DEV_INT_EN_W { w: self }
    }
}
