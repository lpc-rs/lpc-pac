#[doc = "Reader of register DMAINTEN"]
pub type R = crate::R<u32, super::DMAINTEN>;
#[doc = "Writer for register DMAINTEN"]
pub type W = crate::W<u32, super::DMAINTEN>;
#[doc = "Register DMAINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "End of Transfer Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOT_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Enabled."]
    ENABLED_ = 1,
}
impl From<EOT_A> for bool {
    #[inline(always)]
    fn from(variant: EOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOT`"]
pub type EOT_R = crate::R<bool, EOT_A>;
impl EOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOT_A {
        match self.bits {
            false => EOT_A::DISABLED_,
            true => EOT_A::ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == EOT_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == EOT_A::ENABLED_
    }
}
#[doc = "Write proxy for field `EOT`"]
pub struct EOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(EOT_A::DISABLED_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(EOT_A::ENABLED_)
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
#[doc = "New DD Request Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDDR_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Enabled."]
    ENABLED_ = 1,
}
impl From<NDDR_A> for bool {
    #[inline(always)]
    fn from(variant: NDDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NDDR`"]
pub type NDDR_R = crate::R<bool, NDDR_A>;
impl NDDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDDR_A {
        match self.bits {
            false => NDDR_A::DISABLED_,
            true => NDDR_A::ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == NDDR_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == NDDR_A::ENABLED_
    }
}
#[doc = "Write proxy for field `NDDR`"]
pub struct NDDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NDDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(NDDR_A::DISABLED_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(NDDR_A::ENABLED_)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "System Error Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Enabled."]
    ENABLED_ = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, ERR_A>;
impl ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::DISABLED_,
            true => ERR_A::ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == ERR_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == ERR_A::ENABLED_
    }
}
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(ERR_A::DISABLED_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(ERR_A::ENABLED_)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - End of Transfer Interrupt enable bit."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New DD Request Interrupt enable bit."]
    #[inline(always)]
    pub fn nddr(&self) -> NDDR_R {
        NDDR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System Error Interrupt enable bit."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of Transfer Interrupt enable bit."]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W {
        EOT_W { w: self }
    }
    #[doc = "Bit 1 - New DD Request Interrupt enable bit."]
    #[inline(always)]
    pub fn nddr(&mut self) -> NDDR_W {
        NDDR_W { w: self }
    }
    #[doc = "Bit 2 - System Error Interrupt enable bit."]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
}
