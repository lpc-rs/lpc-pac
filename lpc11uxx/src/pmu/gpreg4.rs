#[doc = "Reader of register GPREG4"]
pub type R = crate::R<u32, super::GPREG4>;
#[doc = "Writer for register GPREG4"]
pub type W = crate::W<u32, super::GPREG4>;
#[doc = "Register GPREG4 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPREG4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WAKEUP pin hysteresis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYS_A {
    #[doc = "0: Hysteresis for WAKEUP pin disabled."]
    DISABLED = 0,
    #[doc = "1: Hysteresis for WAKEUP pin enabled."]
    ENABLED = 1,
}
impl From<WAKEUPHYS_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUPHYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEUPHYS`"]
pub type WAKEUPHYS_R = crate::R<bool, WAKEUPHYS_A>;
impl WAKEUPHYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUPHYS_A {
        match self.bits {
            false => WAKEUPHYS_A::DISABLED,
            true => WAKEUPHYS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEUPHYS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEUPHYS_A::ENABLED
    }
}
#[doc = "Write proxy for field `WAKEUPHYS`"]
pub struct WAKEUPHYS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPHYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUPHYS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hysteresis for WAKEUP pin disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::DISABLED)
    }
    #[doc = "Hysteresis for WAKEUP pin enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `GPDATA`"]
pub type GPDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPDATA`"]
pub struct GPDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | (((value as u32) & 0x001f_ffff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&self) -> WAKEUPHYS_R {
        WAKEUPHYS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&self) -> GPDATA_R {
        GPDATA_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&mut self) -> WAKEUPHYS_W {
        WAKEUPHYS_W { w: self }
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&mut self) -> GPDATA_W {
        GPDATA_W { w: self }
    }
}
