#[doc = "Reader of register PINMODE_OD4"]
pub type R = crate::R<u32, super::PINMODE_OD4>;
#[doc = "Writer for register PINMODE_OD4"]
pub type W = crate::W<u32, super::PINMODE_OD4>;
#[doc = "Register PINMODE_OD4 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE_OD4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 4 pin 28 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_28OD_A {
    #[doc = "0: Normal. P4.28 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P4.28 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P4_28OD_A> for bool {
    #[inline(always)]
    fn from(variant: P4_28OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P4_28OD`"]
pub type P4_28OD_R = crate::R<bool, P4_28OD_A>;
impl P4_28OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P4_28OD_A {
        match self.bits {
            false => P4_28OD_A::NORMAL,
            true => P4_28OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P4_28OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P4_28OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P4_28OD`"]
pub struct P4_28OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_28OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_28OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P4.28 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P4_28OD_A::NORMAL)
    }
    #[doc = "Open-drain. P4.28 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P4_28OD_A::OPEN_DRAIN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `P4_29OD`"]
pub type P4_29OD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4_29OD`"]
pub struct P4_29OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_29OD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - Port 4 pin 28 open drain mode control."]
    #[inline(always)]
    pub fn p4_28od(&self) -> P4_28OD_R {
        P4_28OD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD"]
    #[inline(always)]
    pub fn p4_29od(&self) -> P4_29OD_R {
        P4_29OD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Port 4 pin 28 open drain mode control."]
    #[inline(always)]
    pub fn p4_28od(&mut self) -> P4_28OD_W {
        P4_28OD_W { w: self }
    }
    #[doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD"]
    #[inline(always)]
    pub fn p4_29od(&mut self) -> P4_29OD_W {
        P4_29OD_W { w: self }
    }
}
