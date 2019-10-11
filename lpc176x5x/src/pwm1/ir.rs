#[doc = "Reader of register IR"]
pub type R = crate::R<u32, super::IR>;
#[doc = "Writer for register IR"]
pub type W = crate::W<u32, super::IR>;
#[doc = "Register IR `reset()`'s with value 0"]
impl crate::ResetValue for super::IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWMMR0INT`"]
pub type PWMMR0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMMR0INT`"]
pub struct PWMMR0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR0INT_W<'a> {
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
#[doc = "Reader of field `PWMMR1INT`"]
pub type PWMMR1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMMR1INT`"]
pub struct PWMMR1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR1INT_W<'a> {
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
#[doc = "Reader of field `PWMMR2INT`"]
pub type PWMMR2INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMMR2INT`"]
pub struct PWMMR2INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR2INT_W<'a> {
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
#[doc = "Reader of field `PWMMR3INT`"]
pub type PWMMR3INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMMR3INT`"]
pub struct PWMMR3INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR3INT_W<'a> {
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
#[doc = "Reader of field `PWMCAP0INT`"]
pub type PWMCAP0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMCAP0INT`"]
pub struct PWMCAP0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCAP0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PWMCAP1INT`"]
pub type PWMCAP1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMCAP1INT`"]
pub struct PWMCAP1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCAP1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PWMMR4INT`"]
pub type PWMMR4INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMMR4INT`"]
pub struct PWMMR4INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR4INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PWMMR5INT`"]
pub type PWMMR5INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMMR5INT`"]
pub struct PWMMR5INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR5INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PWMMR6INT`"]
pub type PWMMR6INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMMR6INT`"]
pub struct PWMMR6INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR6INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    pub fn pwmmr0int(&self) -> PWMMR0INT_R {
        PWMMR0INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    pub fn pwmmr1int(&self) -> PWMMR1INT_R {
        PWMMR1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    pub fn pwmmr2int(&self) -> PWMMR2INT_R {
        PWMMR2INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    pub fn pwmmr3int(&self) -> PWMMR3INT_R {
        PWMMR3INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    pub fn pwmcap0int(&self) -> PWMCAP0INT_R {
        PWMCAP0INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    pub fn pwmcap1int(&self) -> PWMCAP1INT_R {
        PWMCAP1INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    pub fn pwmmr4int(&self) -> PWMMR4INT_R {
        PWMMR4INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    pub fn pwmmr5int(&self) -> PWMMR5INT_R {
        PWMMR5INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    pub fn pwmmr6int(&self) -> PWMMR6INT_R {
        PWMMR6INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    pub fn pwmmr0int(&mut self) -> PWMMR0INT_W {
        PWMMR0INT_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    pub fn pwmmr1int(&mut self) -> PWMMR1INT_W {
        PWMMR1INT_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    pub fn pwmmr2int(&mut self) -> PWMMR2INT_W {
        PWMMR2INT_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    pub fn pwmmr3int(&mut self) -> PWMMR3INT_W {
        PWMMR3INT_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    pub fn pwmcap0int(&mut self) -> PWMCAP0INT_W {
        PWMCAP0INT_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    pub fn pwmcap1int(&mut self) -> PWMCAP1INT_W {
        PWMCAP1INT_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    pub fn pwmmr4int(&mut self) -> PWMMR4INT_W {
        PWMMR4INT_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    pub fn pwmmr5int(&mut self) -> PWMMR5INT_W {
        PWMMR5INT_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    pub fn pwmmr6int(&mut self) -> PWMMR6INT_W {
        PWMMR6INT_W { w: self }
    }
}
