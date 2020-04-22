#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTR`"]
pub type INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTR`"]
pub struct INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_W<'a> {
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
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVE`"]
pub struct ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interrupt flag. 0 = No interrupt is pending. 1 = An interrupt is pending. A write of any value to this register clears this flag."]
    #[inline(always)]
    pub fn intr(&self) -> INTR_R {
        INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active flag. 0 = The Micro-Tick Timer is stopped. 1 = The Micro-Tick Timer is currently active."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag. 0 = No interrupt is pending. 1 = An interrupt is pending. A write of any value to this register clears this flag."]
    #[inline(always)]
    pub fn intr(&mut self) -> INTR_W {
        INTR_W { w: self }
    }
    #[doc = "Bit 1 - Active flag. 0 = The Micro-Tick Timer is stopped. 1 = The Micro-Tick Timer is currently active."]
    #[inline(always)]
    pub fn active(&mut self) -> ACTIVE_W {
        ACTIVE_W { w: self }
    }
}
