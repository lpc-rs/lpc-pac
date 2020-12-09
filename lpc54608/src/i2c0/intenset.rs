#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Master Pending interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGEN_A {
    #[doc = "0: Disabled. The MstPending interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MstPending interrupt is enabled."]
    ENABLED = 1,
}
impl From<MSTPENDINGEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPENDINGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTPENDINGEN`"]
pub type MSTPENDINGEN_R = crate::R<bool, MSTPENDINGEN_A>;
impl MSTPENDINGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPENDINGEN_A {
        match self.bits {
            false => MSTPENDINGEN_A::DISABLED,
            true => MSTPENDINGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTPENDINGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTPENDINGEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MSTPENDINGEN`"]
pub struct MSTPENDINGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTPENDINGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTPENDINGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTPENDINGEN_A::DISABLED)
    }
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTPENDINGEN_A::ENABLED)
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
#[doc = "Master Arbitration Loss interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSEN_A {
    #[doc = "0: Disabled. The MstArbLoss interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MstArbLoss interrupt is enabled."]
    ENABLED = 1,
}
impl From<MSTARBLOSSEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTARBLOSSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTARBLOSSEN`"]
pub type MSTARBLOSSEN_R = crate::R<bool, MSTARBLOSSEN_A>;
impl MSTARBLOSSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTARBLOSSEN_A {
        match self.bits {
            false => MSTARBLOSSEN_A::DISABLED,
            true => MSTARBLOSSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTARBLOSSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTARBLOSSEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MSTARBLOSSEN`"]
pub struct MSTARBLOSSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTARBLOSSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTARBLOSSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTARBLOSSEN_A::DISABLED)
    }
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTARBLOSSEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Master Start/Stop Error interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERREN_A {
    #[doc = "0: Disabled. The MstStStpErr interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MstStStpErr interrupt is enabled."]
    ENABLED = 1,
}
impl From<MSTSTSTPERREN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTSTPERREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTSTSTPERREN`"]
pub type MSTSTSTPERREN_R = crate::R<bool, MSTSTSTPERREN_A>;
impl MSTSTSTPERREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSTSTPERREN_A {
        match self.bits {
            false => MSTSTSTPERREN_A::DISABLED,
            true => MSTSTSTPERREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTSTSTPERREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTSTSTPERREN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MSTSTSTPERREN`"]
pub struct MSTSTSTPERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSTSTPERREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSTSTPERREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTSTSTPERREN_A::DISABLED)
    }
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTSTSTPERREN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Slave Pending interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGEN_A {
    #[doc = "0: Disabled. The SlvPending interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The SlvPending interrupt is enabled."]
    ENABLED = 1,
}
impl From<SLVPENDINGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVPENDINGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVPENDINGEN`"]
pub type SLVPENDINGEN_R = crate::R<bool, SLVPENDINGEN_A>;
impl SLVPENDINGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVPENDINGEN_A {
        match self.bits {
            false => SLVPENDINGEN_A::DISABLED,
            true => SLVPENDINGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVPENDINGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVPENDINGEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `SLVPENDINGEN`"]
pub struct SLVPENDINGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVPENDINGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVPENDINGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVPENDINGEN_A::DISABLED)
    }
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVPENDINGEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Slave Not Stretching interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTREN_A {
    #[doc = "0: Disabled. The SlvNotStr interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The SlvNotStr interrupt is enabled."]
    ENABLED = 1,
}
impl From<SLVNOTSTREN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVNOTSTREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVNOTSTREN`"]
pub type SLVNOTSTREN_R = crate::R<bool, SLVNOTSTREN_A>;
impl SLVNOTSTREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVNOTSTREN_A {
        match self.bits {
            false => SLVNOTSTREN_A::DISABLED,
            true => SLVNOTSTREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVNOTSTREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVNOTSTREN_A::ENABLED
    }
}
#[doc = "Write proxy for field `SLVNOTSTREN`"]
pub struct SLVNOTSTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVNOTSTREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVNOTSTREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVNOTSTREN_A::DISABLED)
    }
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVNOTSTREN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Slave Deselect interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELEN_A {
    #[doc = "0: Disabled. The SlvDeSel interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The SlvDeSel interrupt is enabled."]
    ENABLED = 1,
}
impl From<SLVDESELEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVDESELEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVDESELEN`"]
pub type SLVDESELEN_R = crate::R<bool, SLVDESELEN_A>;
impl SLVDESELEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVDESELEN_A {
        match self.bits {
            false => SLVDESELEN_A::DISABLED,
            true => SLVDESELEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVDESELEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVDESELEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `SLVDESELEN`"]
pub struct SLVDESELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVDESELEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVDESELEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVDESELEN_A::DISABLED)
    }
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVDESELEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Monitor data Ready interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYEN_A {
    #[doc = "0: Disabled. The MonRdy interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MonRdy interrupt is enabled."]
    ENABLED = 1,
}
impl From<MONRDYEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONRDYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONRDYEN`"]
pub type MONRDYEN_R = crate::R<bool, MONRDYEN_A>;
impl MONRDYEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONRDYEN_A {
        match self.bits {
            false => MONRDYEN_A::DISABLED,
            true => MONRDYEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONRDYEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONRDYEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MONRDYEN`"]
pub struct MONRDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MONRDYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONRDYEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONRDYEN_A::DISABLED)
    }
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONRDYEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Monitor Overrun interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVEN_A {
    #[doc = "0: Disabled. The MonOv interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MonOv interrupt is enabled."]
    ENABLED = 1,
}
impl From<MONOVEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONOVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONOVEN`"]
pub type MONOVEN_R = crate::R<bool, MONOVEN_A>;
impl MONOVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONOVEN_A {
        match self.bits {
            false => MONOVEN_A::DISABLED,
            true => MONOVEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONOVEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONOVEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MONOVEN`"]
pub struct MONOVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MONOVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONOVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONOVEN_A::DISABLED)
    }
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONOVEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Monitor Idle interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLEEN_A {
    #[doc = "0: Disabled. The MonIdle interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MonIdle interrupt is enabled."]
    ENABLED = 1,
}
impl From<MONIDLEEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONIDLEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONIDLEEN`"]
pub type MONIDLEEN_R = crate::R<bool, MONIDLEEN_A>;
impl MONIDLEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONIDLEEN_A {
        match self.bits {
            false => MONIDLEEN_A::DISABLED,
            true => MONIDLEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONIDLEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONIDLEEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MONIDLEEN`"]
pub struct MONIDLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MONIDLEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONIDLEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONIDLEEN_A::DISABLED)
    }
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONIDLEEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Event time-out interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTEN_A {
    #[doc = "0: Disabled. The Event time-out interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The Event time-out interrupt is enabled."]
    ENABLED = 1,
}
impl From<EVENTTIMEOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTTIMEOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTTIMEOUTEN`"]
pub type EVENTTIMEOUTEN_R = crate::R<bool, EVENTTIMEOUTEN_A>;
impl EVENTTIMEOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTTIMEOUTEN_A {
        match self.bits {
            false => EVENTTIMEOUTEN_A::DISABLED,
            true => EVENTTIMEOUTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EVENTTIMEOUTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EVENTTIMEOUTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `EVENTTIMEOUTEN`"]
pub struct EVENTTIMEOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTTIMEOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTTIMEOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTEN_A::DISABLED)
    }
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "SCL time-out interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTEN_A {
    #[doc = "0: Disabled. The SCL time-out interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The SCL time-out interrupt is enabled."]
    ENABLED = 1,
}
impl From<SCLTIMEOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCLTIMEOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLTIMEOUTEN`"]
pub type SCLTIMEOUTEN_R = crate::R<bool, SCLTIMEOUTEN_A>;
impl SCLTIMEOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLTIMEOUTEN_A {
        match self.bits {
            false => SCLTIMEOUTEN_A::DISABLED,
            true => SCLTIMEOUTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCLTIMEOUTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCLTIMEOUTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `SCLTIMEOUTEN`"]
pub struct SCLTIMEOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLTIMEOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLTIMEOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCLTIMEOUTEN_A::DISABLED)
    }
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCLTIMEOUTEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline(always)]
    pub fn mstpendingen(&self) -> MSTPENDINGEN_R {
        MSTPENDINGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    pub fn mstarblossen(&self) -> MSTARBLOSSEN_R {
        MSTARBLOSSEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    pub fn mstststperren(&self) -> MSTSTSTPERREN_R {
        MSTSTSTPERREN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline(always)]
    pub fn slvpendingen(&self) -> SLVPENDINGEN_R {
        SLVPENDINGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    pub fn slvnotstren(&self) -> SLVNOTSTREN_R {
        SLVNOTSTREN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline(always)]
    pub fn slvdeselen(&self) -> SLVDESELEN_R {
        SLVDESELEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline(always)]
    pub fn monrdyen(&self) -> MONRDYEN_R {
        MONRDYEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline(always)]
    pub fn monoven(&self) -> MONOVEN_R {
        MONOVEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline(always)]
    pub fn monidleen(&self) -> MONIDLEEN_R {
        MONIDLEEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline(always)]
    pub fn eventtimeouten(&self) -> EVENTTIMEOUTEN_R {
        EVENTTIMEOUTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline(always)]
    pub fn scltimeouten(&self) -> SCLTIMEOUTEN_R {
        SCLTIMEOUTEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline(always)]
    pub fn mstpendingen(&mut self) -> MSTPENDINGEN_W {
        MSTPENDINGEN_W { w: self }
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    pub fn mstarblossen(&mut self) -> MSTARBLOSSEN_W {
        MSTARBLOSSEN_W { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    pub fn mstststperren(&mut self) -> MSTSTSTPERREN_W {
        MSTSTSTPERREN_W { w: self }
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline(always)]
    pub fn slvpendingen(&mut self) -> SLVPENDINGEN_W {
        SLVPENDINGEN_W { w: self }
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    pub fn slvnotstren(&mut self) -> SLVNOTSTREN_W {
        SLVNOTSTREN_W { w: self }
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline(always)]
    pub fn slvdeselen(&mut self) -> SLVDESELEN_W {
        SLVDESELEN_W { w: self }
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline(always)]
    pub fn monrdyen(&mut self) -> MONRDYEN_W {
        MONRDYEN_W { w: self }
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline(always)]
    pub fn monoven(&mut self) -> MONOVEN_W {
        MONOVEN_W { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline(always)]
    pub fn monidleen(&mut self) -> MONIDLEEN_W {
        MONIDLEEN_W { w: self }
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline(always)]
    pub fn eventtimeouten(&mut self) -> EVENTTIMEOUTEN_W {
        EVENTTIMEOUTEN_W { w: self }
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline(always)]
    pub fn scltimeouten(&mut self) -> SCLTIMEOUTEN_W {
        SCLTIMEOUTEN_W { w: self }
    }
}
