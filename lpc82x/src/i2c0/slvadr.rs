#[doc = "Reader of register SLVADR[%s]"]
pub type R = crate::R<u32, super::SLVADR>;
#[doc = "Writer for register SLVADR[%s]"]
pub type W = crate::W<u32, super::SLVADR>;
#[doc = "Register SLVADR[%s]
`reset()`'s with value 0x01"]
impl crate::ResetValue for super::SLVADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Slave Address n Disable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SADISABLE_A {
    #[doc = "0: Enabled. Slave Address n is enabled."]
    ENABLED = 0,
    #[doc = "1: Ignored Slave Address n is ignored."]
    DISABLED = 1,
}
impl From<SADISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SADISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SADISABLE`"]
pub type SADISABLE_R = crate::R<bool, SADISABLE_A>;
impl SADISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADISABLE_A {
        match self.bits {
            false => SADISABLE_A::ENABLED,
            true => SADISABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SADISABLE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SADISABLE_A::DISABLED
    }
}
#[doc = "Write proxy for field `SADISABLE`"]
pub struct SADISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SADISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SADISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SADISABLE_A::ENABLED)
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SADISABLE_A::DISABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SLVADR`"]
pub type SLVADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLVADR`"]
pub struct SLVADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&self) -> SADISABLE_R {
        SADISABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&self) -> SLVADR_R {
        SLVADR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&mut self) -> SADISABLE_W {
        SADISABLE_W { w: self }
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&mut self) -> SLVADR_W {
        SLVADR_W { w: self }
    }
}
