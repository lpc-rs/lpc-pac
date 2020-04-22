#[doc = "Reader of register PIO0_22"]
pub type R = crate::R<u32, super::PIO0_22>;
#[doc = "Writer for register PIO0_22"]
pub type W = crate::W<u32, super::PIO0_22>;
#[doc = "Register PIO0_22 `reset()`'s with value 0xb0"]
impl crate::ResetValue for super::PIO0_22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb0
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater. Repeater mode."]
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
            0 => MODE_A::INACTIVE,
            1 => MODE_A::PULL_DOWN,
            2 => MODE_A::PULL_UP,
            3 => MODE_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == MODE_A::INACTIVE
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
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODE_A::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
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
#[doc = "Hysteresis.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYS_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
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
            false => HYS_A::DISABLE,
            true => HYS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HYS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HYS_A::ENABLE
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYS_A::ENABLE)
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
    #[doc = "0: Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
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
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
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
    DISABLE = 0,
    #[doc = "1: Open-drain mode enabled. Remark: This is not a true open-drain mode."]
    ENABLED = 1,
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
            false => OD_A::DISABLE,
            true => OD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OD_A::ENABLED
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
    pub fn disable(self) -> &'a mut W {
        self.variant(OD_A::DISABLE)
    }
    #[doc = "Open-drain mode enabled. Remark: This is not a true open-drain mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OD_A::ENABLED)
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
#[doc = "Digital filter sample mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum S_MODE_A {
    #[doc = "0: Bypass input filter."]
    S_MODE_0 = 0,
    #[doc = "1: 1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    S_MODE_1 = 1,
    #[doc = "2: 2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    S_MODE_2 = 2,
    #[doc = "3: 3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    S_MODE_3 = 3,
}
impl From<S_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: S_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `S_MODE`"]
pub type S_MODE_R = crate::R<u8, S_MODE_A>;
impl S_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_MODE_A {
        match self.bits {
            0 => S_MODE_A::S_MODE_0,
            1 => S_MODE_A::S_MODE_1,
            2 => S_MODE_A::S_MODE_2,
            3 => S_MODE_A::S_MODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S_MODE_0`"]
    #[inline(always)]
    pub fn is_s_mode_0(&self) -> bool {
        *self == S_MODE_A::S_MODE_0
    }
    #[doc = "Checks if the value of the field is `S_MODE_1`"]
    #[inline(always)]
    pub fn is_s_mode_1(&self) -> bool {
        *self == S_MODE_A::S_MODE_1
    }
    #[doc = "Checks if the value of the field is `S_MODE_2`"]
    #[inline(always)]
    pub fn is_s_mode_2(&self) -> bool {
        *self == S_MODE_A::S_MODE_2
    }
    #[doc = "Checks if the value of the field is `S_MODE_3`"]
    #[inline(always)]
    pub fn is_s_mode_3(&self) -> bool {
        *self == S_MODE_A::S_MODE_3
    }
}
#[doc = "Write proxy for field `S_MODE`"]
pub struct S_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bypass input filter."]
    #[inline(always)]
    pub fn s_mode_0(self) -> &'a mut W {
        self.variant(S_MODE_A::S_MODE_0)
    }
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    #[inline(always)]
    pub fn s_mode_1(self) -> &'a mut W {
        self.variant(S_MODE_A::S_MODE_1)
    }
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    #[inline(always)]
    pub fn s_mode_2(self) -> &'a mut W {
        self.variant(S_MODE_A::S_MODE_2)
    }
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    #[inline(always)]
    pub fn s_mode_3(self) -> &'a mut W {
        self.variant(S_MODE_A::S_MODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_DIV_A {
    #[doc = "0: IOCONCLKDIV0"]
    CLK_DIV_0 = 0,
    #[doc = "1: IOCONCLKDIV1"]
    CLK_DIV_1 = 1,
    #[doc = "2: IOCONCLKDIV2"]
    CLK_DIV_2 = 2,
    #[doc = "3: IOCONCLKDIV3"]
    CLK_DIV_3 = 3,
    #[doc = "4: IOCONCLKDIV4"]
    CLK_DIV_4 = 4,
    #[doc = "5: IOCONCLKDIV5"]
    CLK_DIV_5 = 5,
    #[doc = "6: IOCONCLKDIV6"]
    CLK_DIV_6 = 6,
}
impl From<CLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_DIV`"]
pub type CLK_DIV_R = crate::R<u8, CLK_DIV_A>;
impl CLK_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_DIV_A::CLK_DIV_0),
            1 => Val(CLK_DIV_A::CLK_DIV_1),
            2 => Val(CLK_DIV_A::CLK_DIV_2),
            3 => Val(CLK_DIV_A::CLK_DIV_3),
            4 => Val(CLK_DIV_A::CLK_DIV_4),
            5 => Val(CLK_DIV_A::CLK_DIV_5),
            6 => Val(CLK_DIV_A::CLK_DIV_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_0`"]
    #[inline(always)]
    pub fn is_clk_div_0(&self) -> bool {
        *self == CLK_DIV_A::CLK_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_1`"]
    #[inline(always)]
    pub fn is_clk_div_1(&self) -> bool {
        *self == CLK_DIV_A::CLK_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_2`"]
    #[inline(always)]
    pub fn is_clk_div_2(&self) -> bool {
        *self == CLK_DIV_A::CLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_3`"]
    #[inline(always)]
    pub fn is_clk_div_3(&self) -> bool {
        *self == CLK_DIV_A::CLK_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_4`"]
    #[inline(always)]
    pub fn is_clk_div_4(&self) -> bool {
        *self == CLK_DIV_A::CLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_5`"]
    #[inline(always)]
    pub fn is_clk_div_5(&self) -> bool {
        *self == CLK_DIV_A::CLK_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_6`"]
    #[inline(always)]
    pub fn is_clk_div_6(&self) -> bool {
        *self == CLK_DIV_A::CLK_DIV_6
    }
}
#[doc = "Write proxy for field `CLK_DIV`"]
pub struct CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IOCONCLKDIV0"]
    #[inline(always)]
    pub fn clk_div_0(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_0)
    }
    #[doc = "IOCONCLKDIV1"]
    #[inline(always)]
    pub fn clk_div_1(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_1)
    }
    #[doc = "IOCONCLKDIV2"]
    #[inline(always)]
    pub fn clk_div_2(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_2)
    }
    #[doc = "IOCONCLKDIV3"]
    #[inline(always)]
    pub fn clk_div_3(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_3)
    }
    #[doc = "IOCONCLKDIV4"]
    #[inline(always)]
    pub fn clk_div_4(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_4)
    }
    #[doc = "IOCONCLKDIV5"]
    #[inline(always)]
    pub fn clk_div_5(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_5)
    }
    #[doc = "IOCONCLKDIV6"]
    #[inline(always)]
    pub fn clk_div_6(self) -> &'a mut W {
        self.variant(CLK_DIV_A::CLK_DIV_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
impl R {
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
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline(always)]
    pub fn s_mode(&self) -> S_MODE_R {
        S_MODE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
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
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline(always)]
    pub fn s_mode(&mut self) -> S_MODE_W {
        S_MODE_W { w: self }
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline(always)]
    pub fn clk_div(&mut self) -> CLK_DIV_W {
        CLK_DIV_W { w: self }
    }
}
