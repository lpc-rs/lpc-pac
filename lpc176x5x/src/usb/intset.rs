#[doc = "Writer for register INTSET"]
pub type W = crate::W<u32, super::INTSET>;
#[doc = "Register INTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TMR_SET`"]
pub struct TMR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_SET_W<'a> {
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
#[doc = "Write proxy for field `REMOVE_PU_SET`"]
pub struct REMOVE_PU_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> REMOVE_PU_SET_W<'a> {
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
#[doc = "Write proxy for field `HNP_FAILURE_SET`"]
pub struct HNP_FAILURE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_FAILURE_SET_W<'a> {
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
#[doc = "Write proxy for field `HNP_SUCCES_SET`"]
pub struct HNP_SUCCES_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_SUCCES_SET_W<'a> {
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
    #[doc = "Bit 0 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_set(&mut self) -> TMR_SET_W {
        TMR_SET_W { w: self }
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_set(&mut self) -> REMOVE_PU_SET_W {
        REMOVE_PU_SET_W { w: self }
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_set(&mut self) -> HNP_FAILURE_SET_W {
        HNP_FAILURE_SET_W { w: self }
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_set(&mut self) -> HNP_SUCCES_SET_W {
        HNP_SUCCES_SET_W { w: self }
    }
}
