#[doc = "Reader of register PINMODE_OD3"]
pub type R = crate::R<u32, super::PINMODE_OD3>;
#[doc = "Writer for register PINMODE_OD3"]
pub type W = crate::W<u32, super::PINMODE_OD3>;
#[doc = "Register PINMODE_OD3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE_OD3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 3 pin 25 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25OD_A {
    #[doc = "0: Normal. P3.25 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P3.25 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P3_25OD_A> for bool {
    #[inline(always)]
    fn from(variant: P3_25OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P3_25OD`"]
pub type P3_25OD_R = crate::R<bool, P3_25OD_A>;
impl P3_25OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P3_25OD_A {
        match self.bits {
            false => P3_25OD_A::NORMAL,
            true => P3_25OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P3_25OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P3_25OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P3_25OD`"]
pub struct P3_25OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_25OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_25OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P3.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P3_25OD_A::NORMAL)
    }
    #[doc = "Open-drain. P3.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P3_25OD_A::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `P3_26OD`"]
pub type P3_26OD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3_26OD`"]
pub struct P3_26OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_26OD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline(always)]
    pub fn p3_25od(&self) -> P3_25OD_R {
        P3_25OD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline(always)]
    pub fn p3_26od(&self) -> P3_26OD_R {
        P3_26OD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline(always)]
    pub fn p3_25od(&mut self) -> P3_25OD_W {
        P3_25OD_W { w: self }
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline(always)]
    pub fn p3_26od(&mut self) -> P3_26OD_W {
        P3_26OD_W { w: self }
    }
}
