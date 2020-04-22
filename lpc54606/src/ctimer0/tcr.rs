#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    #[doc = "0: Disabled.The counters are disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The Timer Counter and Prescale Counter are enabled."]
    ENABLED = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEN`"]
pub type CEN_R = crate::R<bool, CEN_A>;
impl CEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::DISABLED,
            true => CEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CEN`"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled.The counters are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEN_A::DISABLED)
    }
    #[doc = "Enabled. The Timer Counter and Prescale Counter are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEN_A::ENABLED)
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
#[doc = "Counter reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRST_A {
    #[doc = "0: Disabled. Do nothing."]
    DISABLED = 0,
    #[doc = "1: Enabled. The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of the APB bus clock. The counters remain reset until TCR\\[1\\]
is returned to zero."]
    ENABLED = 1,
}
impl From<CRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRST`"]
pub type CRST_R = crate::R<bool, CRST_A>;
impl CRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRST_A {
        match self.bits {
            false => CRST_A::DISABLED,
            true => CRST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRST_A::ENABLED
    }
}
#[doc = "Write proxy for field `CRST`"]
pub struct CRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. Do nothing."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRST_A::DISABLED)
    }
    #[doc = "Enabled. The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of the APB bus clock. The counters remain reset until TCR\\[1\\]
is returned to zero."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRST_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Counter enable."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline(always)]
    pub fn crst(&self) -> CRST_R {
        CRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable."]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline(always)]
    pub fn crst(&mut self) -> CRST_W {
        CRST_W { w: self }
    }
}
