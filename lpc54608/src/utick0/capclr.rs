#[doc = "Writer for register CAPCLR"]
pub type W = crate::W<u32, super::CAPCLR>;
#[doc = "Register CAPCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CAPCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CAPCLR0`"]
pub struct CAPCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCLR0_W<'a> {
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
#[doc = "Write proxy for field `CAPCLR1`"]
pub struct CAPCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCLR1_W<'a> {
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
#[doc = "Write proxy for field `CAPCLR2`"]
pub struct CAPCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCLR2_W<'a> {
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
#[doc = "Write proxy for field `CAPCLR3`"]
pub struct CAPCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCLR3_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[inline(always)]
    pub fn capclr0(&mut self) -> CAPCLR0_W {
        CAPCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[inline(always)]
    pub fn capclr1(&mut self) -> CAPCLR1_W {
        CAPCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[inline(always)]
    pub fn capclr2(&mut self) -> CAPCLR2_W {
        CAPCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[inline(always)]
    pub fn capclr3(&mut self) -> CAPCLR3_W {
        CAPCLR3_W { w: self }
    }
}
