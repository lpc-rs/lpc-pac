#[doc = "Writer for register INTCLR"]
pub type W = crate::W<u32, super::INTCLR>;
#[doc = "Register INTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TMR_CLR`"]
pub struct TMR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_CLR_W<'a> {
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
#[doc = "Write proxy for field `REMOVE_PU_CLR`"]
pub struct REMOVE_PU_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REMOVE_PU_CLR_W<'a> {
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
#[doc = "Write proxy for field `HNP_FAILURE_CLR`"]
pub struct HNP_FAILURE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_FAILURE_CLR_W<'a> {
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
#[doc = "Write proxy for field `HNP_SUCCES_CLR`"]
pub struct HNP_SUCCES_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_SUCCES_CLR_W<'a> {
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
    #[doc = "Bit 0 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_clr(&mut self) -> TMR_CLR_W {
        TMR_CLR_W { w: self }
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_clr(&mut self) -> REMOVE_PU_CLR_W {
        REMOVE_PU_CLR_W { w: self }
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_clr(&mut self) -> HNP_FAILURE_CLR_W {
        HNP_FAILURE_CLR_W { w: self }
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_clr(&mut self) -> HNP_SUCCES_CLR_W {
        HNP_SUCCES_CLR_W { w: self }
    }
}
