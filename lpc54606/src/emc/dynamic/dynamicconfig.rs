#[doc = "Reader of register DYNAMICCONFIG"]
pub type R = crate::R<u32, super::DYNAMICCONFIG>;
#[doc = "Writer for register DYNAMICCONFIG"]
pub type W = crate::W<u32, super::DYNAMICCONFIG>;
#[doc = "Register DYNAMICCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::DYNAMICCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MD`"]
pub type MD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD`"]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `AM0`"]
pub type AM0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AM0`"]
pub struct AM0_W<'a> {
    w: &'a mut W,
}
impl<'a> AM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | (((value as u32) & 0x3f) << 7);
        self.w
    }
}
#[doc = "Reader of field `AM1`"]
pub type AM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AM1`"]
pub struct AM1_W<'a> {
    w: &'a mut W,
}
impl<'a> AM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `B`"]
pub type B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B`"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `P`"]
pub type P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P`"]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 7:12 - See Table 933."]
    #[inline(always)]
    pub fn am0(&self) -> AM0_R {
        AM0_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - See Table 933."]
    #[inline(always)]
    pub fn am1(&self) -> AM1_R {
        AM1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
    }
    #[doc = "Bits 7:12 - See Table 933."]
    #[inline(always)]
    pub fn am0(&mut self) -> AM0_W {
        AM0_W { w: self }
    }
    #[doc = "Bit 14 - See Table 933."]
    #[inline(always)]
    pub fn am1(&mut self) -> AM1_W {
        AM1_W { w: self }
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
}
