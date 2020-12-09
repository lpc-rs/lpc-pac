#[doc = "Reader of register PAL[%s]"]
pub type R = crate::R<u32, super::PAL>;
#[doc = "Writer for register PAL[%s]"]
pub type W = crate::W<u32, super::PAL>;
#[doc = "Register PAL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::PAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `R04_0`"]
pub type R04_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `R04_0`"]
pub struct R04_0_W<'a> {
    w: &'a mut W,
}
impl<'a> R04_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `G04_0`"]
pub type G04_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `G04_0`"]
pub struct G04_0_W<'a> {
    w: &'a mut W,
}
impl<'a> G04_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `B04_0`"]
pub type B04_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B04_0`"]
pub struct B04_0_W<'a> {
    w: &'a mut W,
}
impl<'a> B04_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `I0`"]
pub type I0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I0`"]
pub struct I0_W<'a> {
    w: &'a mut W,
}
impl<'a> I0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `R14_0`"]
pub type R14_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `R14_0`"]
pub struct R14_0_W<'a> {
    w: &'a mut W,
}
impl<'a> R14_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `G14_0`"]
pub type G14_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `G14_0`"]
pub struct G14_0_W<'a> {
    w: &'a mut W,
}
impl<'a> G14_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
#[doc = "Reader of field `B14_0`"]
pub type B14_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B14_0`"]
pub struct B14_0_W<'a> {
    w: &'a mut W,
}
impl<'a> B14_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "Reader of field `I1`"]
pub type I1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I1`"]
pub struct I1_W<'a> {
    w: &'a mut W,
}
impl<'a> I1_W<'a> {
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
    #[doc = "Bits 0:4 - Red palette data."]
    #[inline(always)]
    pub fn r04_0(&self) -> R04_0_R {
        R04_0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Green palette data."]
    #[inline(always)]
    pub fn g04_0(&self) -> G04_0_R {
        G04_0_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Blue palette data."]
    #[inline(always)]
    pub fn b04_0(&self) -> B04_0_R {
        B04_0_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i0(&self) -> I0_R {
        I0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Red palette data."]
    #[inline(always)]
    pub fn r14_0(&self) -> R14_0_R {
        R14_0_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Green palette data."]
    #[inline(always)]
    pub fn g14_0(&self) -> G14_0_R {
        G14_0_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - Blue palette data."]
    #[inline(always)]
    pub fn b14_0(&self) -> B14_0_R {
        B14_0_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i1(&self) -> I1_R {
        I1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Red palette data."]
    #[inline(always)]
    pub fn r04_0(&mut self) -> R04_0_W {
        R04_0_W { w: self }
    }
    #[doc = "Bits 5:9 - Green palette data."]
    #[inline(always)]
    pub fn g04_0(&mut self) -> G04_0_W {
        G04_0_W { w: self }
    }
    #[doc = "Bits 10:14 - Blue palette data."]
    #[inline(always)]
    pub fn b04_0(&mut self) -> B04_0_W {
        B04_0_W { w: self }
    }
    #[doc = "Bit 15 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i0(&mut self) -> I0_W {
        I0_W { w: self }
    }
    #[doc = "Bits 16:20 - Red palette data."]
    #[inline(always)]
    pub fn r14_0(&mut self) -> R14_0_W {
        R14_0_W { w: self }
    }
    #[doc = "Bits 21:25 - Green palette data."]
    #[inline(always)]
    pub fn g14_0(&mut self) -> G14_0_W {
        G14_0_W { w: self }
    }
    #[doc = "Bits 26:30 - Blue palette data."]
    #[inline(always)]
    pub fn b14_0(&mut self) -> B14_0_W {
        B14_0_W { w: self }
    }
    #[doc = "Bit 31 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i1(&mut self) -> I1_W {
        I1_W { w: self }
    }
}
