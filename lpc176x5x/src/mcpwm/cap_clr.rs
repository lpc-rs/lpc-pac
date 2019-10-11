#[doc = "Writer for register CAP_CLR"]
pub type W = crate::W<u32, super::CAP_CLR>;
#[doc = "Register CAP_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CAP_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CAP_CLR0`"]
pub struct CAP_CLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_CLR0_W<'a> {
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
#[doc = "Write proxy for field `CAP_CLR1`"]
pub struct CAP_CLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_CLR1_W<'a> {
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
#[doc = "Write proxy for field `CAP_CLR2`"]
pub struct CAP_CLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_CLR2_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bit clears the CAP0 register."]
    #[inline(always)]
    pub fn cap_clr0(&mut self) -> CAP_CLR0_W {
        CAP_CLR0_W { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 to this bit clears the CAP1 register."]
    #[inline(always)]
    pub fn cap_clr1(&mut self) -> CAP_CLR1_W {
        CAP_CLR1_W { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 to this bit clears the CAP2 register."]
    #[inline(always)]
    pub fn cap_clr2(&mut self) -> CAP_CLR2_W {
        CAP_CLR2_W { w: self }
    }
}
