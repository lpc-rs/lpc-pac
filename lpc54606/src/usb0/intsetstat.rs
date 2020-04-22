#[doc = "Reader of register INTSETSTAT"]
pub type R = crate::R<u32, super::INTSETSTAT>;
#[doc = "Writer for register INTSETSTAT"]
pub type W = crate::W<u32, super::INTSETSTAT>;
#[doc = "Register INTSETSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSETSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP_SET_INT`"]
pub type EP_SET_INT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EP_SET_INT`"]
pub struct EP_SET_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_SET_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `FRAME_SET_INT`"]
pub type FRAME_SET_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAME_SET_INT`"]
pub struct FRAME_SET_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_SET_INT_W<'a> {
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
#[doc = "Reader of field `DEV_SET_INT`"]
pub type DEV_SET_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_SET_INT`"]
pub struct DEV_SET_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_SET_INT_W<'a> {
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
    #[doc = "Bits 0:9 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn ep_set_int(&self) -> EP_SET_INT_R {
        EP_SET_INT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn frame_set_int(&self) -> FRAME_SET_INT_R {
        FRAME_SET_INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn dev_set_int(&self) -> DEV_SET_INT_R {
        DEV_SET_INT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn ep_set_int(&mut self) -> EP_SET_INT_W {
        EP_SET_INT_W { w: self }
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn frame_set_int(&mut self) -> FRAME_SET_INT_W {
        FRAME_SET_INT_W { w: self }
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn dev_set_int(&mut self) -> DEV_SET_INT_W {
        DEV_SET_INT_W { w: self }
    }
}
