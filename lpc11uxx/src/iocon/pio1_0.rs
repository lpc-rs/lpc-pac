#[doc = "Reader of register PIO1_0"]
pub type R = crate::R<u32, super::PIO1_0>;
#[doc = "Writer for register PIO1_0"]
pub type W = crate::W<u32, super::PIO1_0>;
#[doc = "Register PIO1_0 `reset()`'s with value 0x90"]
impl crate::ResetValue for super::PIO1_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x90
    }
}
#[doc = "Selects pin function. Values 0x2 to 0x7 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: PIO1_0."]
    PIO1_0 = 0,
    #[doc = "1: CT32B1_MAT1."]
    CT32B1_MAT1 = 1,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FUNC`"]
pub type FUNC_R = crate::R<u8, FUNC_A>;
impl FUNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FUNC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FUNC_A::PIO1_0),
            1 => Val(FUNC_A::CT32B1_MAT1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO1_0`"]
    #[inline(always)]
    pub fn is_pio1_0(&self) -> bool {
        *self == FUNC_A::PIO1_0
    }
    #[doc = "Checks if the value of the field is `CT32B1_MAT1`"]
    #[inline(always)]
    pub fn is_ct32b1_mat1(&self) -> bool {
        *self == FUNC_A::CT32B1_MAT1
    }
}
#[doc = "Write proxy for field `FUNC`"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PIO1_0."]
    #[inline(always)]
    pub fn pio1_0(self) -> &'a mut W {
        self.variant(FUNC_A::PIO1_0)
    }
    #[doc = "CT32B1_MAT1."]
    #[inline(always)]
    pub fn ct32b1_mat1(self) -> &'a mut W {
        self.variant(FUNC_A::CT32B1_MAT1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inactive (no pull-down/pull-up resistor enabled)."]
    FLOATING = 0,
    #[doc = "1: Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater mode."]
    REPEATER = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::FLOATING,
            1 => MODE_A::PULL_DOWN,
            2 => MODE_A::PULL_UP,
            3 => MODE_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLOATING`"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == MODE_A::FLOATING
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == MODE_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == MODE_A::REPEATER
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(MODE_A::FLOATING)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP)
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODE_A::REPEATER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Hysteresis.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYS_A {
    #[doc = "0: Disable."]
    DISABLED = 0,
    #[doc = "1: Enable."]
    ENABLED = 1,
}
impl From<HYS_A> for bool {
    #[inline(always)]
    fn from(variant: HYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HYS`"]
pub type HYS_R = crate::R<bool, HYS_A>;
impl HYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYS_A {
        match self.bits {
            false => HYS_A::DISABLED,
            true => HYS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HYS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HYS_A::ENABLED
    }
}
#[doc = "Write proxy for field `HYS`"]
pub struct HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HYS_A::DISABLED)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HYS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Invert input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_A {
    #[doc = "0: Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    NOT_INVERTED = 0,
    #[doc = "1: Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INVERTED = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INV`"]
pub type INV_R = crate::R<bool, INV_A>;
impl INV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::NOT_INVERTED,
            true => INV_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == INV_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == INV_A::INVERTED
    }
}
#[doc = "Write proxy for field `INV`"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(INV_A::NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(INV_A::INVERTED)
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
#[doc = "Open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_A {
    #[doc = "0: Disable."]
    DISABLED = 0,
    #[doc = "1: Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    OPEN_DRAIN = 1,
}
impl From<OD_A> for bool {
    #[inline(always)]
    fn from(variant: OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD`"]
pub type OD_R = crate::R<bool, OD_A>;
impl OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_A {
        match self.bits {
            false => OD_A::DISABLED,
            true => OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `OD`"]
pub struct OD_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OD_A::DISABLED)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OD_A::OPEN_DRAIN)
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
impl R {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W {
        HYS_W { w: self }
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> OD_W {
        OD_W { w: self }
    }
}
