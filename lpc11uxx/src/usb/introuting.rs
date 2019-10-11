#[doc = "Reader of register INTROUTING"]
pub type R = crate::R<u32, super::INTROUTING>;
#[doc = "Writer for register INTROUTING"]
pub type W = crate::W<u32, super::INTROUTING>;
#[doc = "Register INTROUTING `reset()`'s with value 0"]
impl crate::ResetValue for super::INTROUTING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ROUTE_INT9_0`"]
pub type ROUTE_INT9_0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ROUTE_INT9_0`"]
pub struct ROUTE_INT9_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUTE_INT9_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `ROUTE_INT30`"]
pub type ROUTE_INT30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROUTE_INT30`"]
pub struct ROUTE_INT30_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUTE_INT30_W<'a> {
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
#[doc = "Reader of field `ROUTE_INT31`"]
pub type ROUTE_INT31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROUTE_INT31`"]
pub struct ROUTE_INT31_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUTE_INT31_W<'a> {
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
    #[doc = "Bits 0:9 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int9_0(&self) -> ROUTE_INT9_0_R {
        ROUTE_INT9_0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int30(&self) -> ROUTE_INT30_R {
        ROUTE_INT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int31(&self) -> ROUTE_INT31_R {
        ROUTE_INT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int9_0(&mut self) -> ROUTE_INT9_0_W {
        ROUTE_INT9_0_W { w: self }
    }
    #[doc = "Bit 30 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int30(&mut self) -> ROUTE_INT30_W {
        ROUTE_INT30_W { w: self }
    }
    #[doc = "Bit 31 - This bit can control on which hardware interrupt line the interrupt will be generated: 0: IRQ interrupt line is selected for this interrupt bit 1: FIQ interrupt line is selected for this interrupt bit"]
    #[inline(always)]
    pub fn route_int31(&mut self) -> ROUTE_INT31_W {
        ROUTE_INT31_W { w: self }
    }
}
