#[doc = "Reader of register LCR"]
pub type R = crate::R<u32, super::LCR>;
#[doc = "Writer for register LCR"]
pub type W = crate::W<u32, super::LCR>;
#[doc = "Register LCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Word Length Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLS_A {
    #[doc = "0: 5-bit character length"]
    _5_BIT_CHARACTER_LENG = 0,
    #[doc = "1: 6-bit character length"]
    _6_BIT_CHARACTER_LENG = 1,
    #[doc = "2: 7-bit character length"]
    _7_BIT_CHARACTER_LENG = 2,
    #[doc = "3: 8-bit character length"]
    _8_BIT_CHARACTER_LENG = 3,
}
impl From<WLS_A> for u8 {
    #[inline(always)]
    fn from(variant: WLS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLS`"]
pub type WLS_R = crate::R<u8, WLS_A>;
impl WLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLS_A {
        match self.bits {
            0 => WLS_A::_5_BIT_CHARACTER_LENG,
            1 => WLS_A::_6_BIT_CHARACTER_LENG,
            2 => WLS_A::_7_BIT_CHARACTER_LENG,
            3 => WLS_A::_8_BIT_CHARACTER_LENG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_BIT_CHARACTER_LENG`"]
    #[inline(always)]
    pub fn is_5_bit_character_leng(&self) -> bool {
        *self == WLS_A::_5_BIT_CHARACTER_LENG
    }
    #[doc = "Checks if the value of the field is `_6_BIT_CHARACTER_LENG`"]
    #[inline(always)]
    pub fn is_6_bit_character_leng(&self) -> bool {
        *self == WLS_A::_6_BIT_CHARACTER_LENG
    }
    #[doc = "Checks if the value of the field is `_7_BIT_CHARACTER_LENG`"]
    #[inline(always)]
    pub fn is_7_bit_character_leng(&self) -> bool {
        *self == WLS_A::_7_BIT_CHARACTER_LENG
    }
    #[doc = "Checks if the value of the field is `_8_BIT_CHARACTER_LENG`"]
    #[inline(always)]
    pub fn is_8_bit_character_leng(&self) -> bool {
        *self == WLS_A::_8_BIT_CHARACTER_LENG
    }
}
#[doc = "Write proxy for field `WLS`"]
pub struct WLS_W<'a> {
    w: &'a mut W,
}
impl<'a> WLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "5-bit character length"]
    #[inline(always)]
    pub fn _5_bit_character_leng(self) -> &'a mut W {
        self.variant(WLS_A::_5_BIT_CHARACTER_LENG)
    }
    #[doc = "6-bit character length"]
    #[inline(always)]
    pub fn _6_bit_character_leng(self) -> &'a mut W {
        self.variant(WLS_A::_6_BIT_CHARACTER_LENG)
    }
    #[doc = "7-bit character length"]
    #[inline(always)]
    pub fn _7_bit_character_leng(self) -> &'a mut W {
        self.variant(WLS_A::_7_BIT_CHARACTER_LENG)
    }
    #[doc = "8-bit character length"]
    #[inline(always)]
    pub fn _8_bit_character_leng(self) -> &'a mut W {
        self.variant(WLS_A::_8_BIT_CHARACTER_LENG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Stop Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBS_A {
    #[doc = "0: 1 stop bit."]
    _1_STOP_BIT_ = 0,
    #[doc = "1: 2 stop bits (1.5 if UnLCR\\[1:0\\]=00)."]
    _2_STOP_BITS_1_5_IF_ = 1,
}
impl From<SBS_A> for bool {
    #[inline(always)]
    fn from(variant: SBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBS`"]
pub type SBS_R = crate::R<bool, SBS_A>;
impl SBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBS_A {
        match self.bits {
            false => SBS_A::_1_STOP_BIT_,
            true => SBS_A::_2_STOP_BITS_1_5_IF_,
        }
    }
    #[doc = "Checks if the value of the field is `_1_STOP_BIT_`"]
    #[inline(always)]
    pub fn is_1_stop_bit_(&self) -> bool {
        *self == SBS_A::_1_STOP_BIT_
    }
    #[doc = "Checks if the value of the field is `_2_STOP_BITS_1_5_IF_`"]
    #[inline(always)]
    pub fn is_2_stop_bits_1_5_if_(&self) -> bool {
        *self == SBS_A::_2_STOP_BITS_1_5_IF_
    }
}
#[doc = "Write proxy for field `SBS`"]
pub struct SBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn _1_stop_bit_(self) -> &'a mut W {
        self.variant(SBS_A::_1_STOP_BIT_)
    }
    #[doc = "2 stop bits (1.5 if UnLCR\\[1:0\\]=00)."]
    #[inline(always)]
    pub fn _2_stop_bits_1_5_if_(self) -> &'a mut W {
        self.variant(SBS_A::_2_STOP_BITS_1_5_IF_)
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
#[doc = "Parity Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: Disable parity generation and checking."]
    DISABLE_PARITY_GENER = 0,
    #[doc = "1: Enable parity generation and checking."]
    ENABLE_PARITY_GENERA = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::DISABLE_PARITY_GENER,
            true => PE_A::ENABLE_PARITY_GENERA,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_PARITY_GENER`"]
    #[inline(always)]
    pub fn is_disable_parity_gener(&self) -> bool {
        *self == PE_A::DISABLE_PARITY_GENER
    }
    #[doc = "Checks if the value of the field is `ENABLE_PARITY_GENERA`"]
    #[inline(always)]
    pub fn is_enable_parity_genera(&self) -> bool {
        *self == PE_A::ENABLE_PARITY_GENERA
    }
}
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable parity generation and checking."]
    #[inline(always)]
    pub fn disable_parity_gener(self) -> &'a mut W {
        self.variant(PE_A::DISABLE_PARITY_GENER)
    }
    #[doc = "Enable parity generation and checking."]
    #[inline(always)]
    pub fn enable_parity_genera(self) -> &'a mut W {
        self.variant(PE_A::ENABLE_PARITY_GENERA)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    ODD_PARITY_NUMBER_O = 0,
    #[doc = "1: Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EVEN_PARITY_NUMBER_ = 1,
    #[doc = "2: Forced 1 stick parity."]
    FORCED_1_STICK_PARIT = 2,
    #[doc = "3: Forced 0 stick parity."]
    FORCED_0_STICK_PARIT = 3,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::ODD_PARITY_NUMBER_O,
            1 => PS_A::EVEN_PARITY_NUMBER_,
            2 => PS_A::FORCED_1_STICK_PARIT,
            3 => PS_A::FORCED_0_STICK_PARIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ODD_PARITY_NUMBER_O`"]
    #[inline(always)]
    pub fn is_odd_parity_number_o(&self) -> bool {
        *self == PS_A::ODD_PARITY_NUMBER_O
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY_NUMBER_`"]
    #[inline(always)]
    pub fn is_even_parity_number_(&self) -> bool {
        *self == PS_A::EVEN_PARITY_NUMBER_
    }
    #[doc = "Checks if the value of the field is `FORCED_1_STICK_PARIT`"]
    #[inline(always)]
    pub fn is_forced_1_stick_parit(&self) -> bool {
        *self == PS_A::FORCED_1_STICK_PARIT
    }
    #[doc = "Checks if the value of the field is `FORCED_0_STICK_PARIT`"]
    #[inline(always)]
    pub fn is_forced_0_stick_parit(&self) -> bool {
        *self == PS_A::FORCED_0_STICK_PARIT
    }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline(always)]
    pub fn odd_parity_number_o(self) -> &'a mut W {
        self.variant(PS_A::ODD_PARITY_NUMBER_O)
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline(always)]
    pub fn even_parity_number_(self) -> &'a mut W {
        self.variant(PS_A::EVEN_PARITY_NUMBER_)
    }
    #[doc = "Forced 1 stick parity."]
    #[inline(always)]
    pub fn forced_1_stick_parit(self) -> &'a mut W {
        self.variant(PS_A::FORCED_1_STICK_PARIT)
    }
    #[doc = "Forced 0 stick parity."]
    #[inline(always)]
    pub fn forced_0_stick_parit(self) -> &'a mut W {
        self.variant(PS_A::FORCED_0_STICK_PARIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Break Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BC_A {
    #[doc = "0: Disable break transmission."]
    DISABLE_BREAK_TRANSM = 0,
    #[doc = "1: Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR\\[6\\]
is active high."]
    ENABLE_BREAK_TRANSMI = 1,
}
impl From<BC_A> for bool {
    #[inline(always)]
    fn from(variant: BC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BC`"]
pub type BC_R = crate::R<bool, BC_A>;
impl BC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BC_A {
        match self.bits {
            false => BC_A::DISABLE_BREAK_TRANSM,
            true => BC_A::ENABLE_BREAK_TRANSMI,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_BREAK_TRANSM`"]
    #[inline(always)]
    pub fn is_disable_break_transm(&self) -> bool {
        *self == BC_A::DISABLE_BREAK_TRANSM
    }
    #[doc = "Checks if the value of the field is `ENABLE_BREAK_TRANSMI`"]
    #[inline(always)]
    pub fn is_enable_break_transmi(&self) -> bool {
        *self == BC_A::ENABLE_BREAK_TRANSMI
    }
}
#[doc = "Write proxy for field `BC`"]
pub struct BC_W<'a> {
    w: &'a mut W,
}
impl<'a> BC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable break transmission."]
    #[inline(always)]
    pub fn disable_break_transm(self) -> &'a mut W {
        self.variant(BC_A::DISABLE_BREAK_TRANSM)
    }
    #[doc = "Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR\\[6\\]
is active high."]
    #[inline(always)]
    pub fn enable_break_transmi(self) -> &'a mut W {
        self.variant(BC_A::ENABLE_BREAK_TRANSMI)
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
#[doc = "Divisor Latch Access Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLAB_A {
    #[doc = "0: Disable access to Divisor Latches."]
    DISABLE_ACCESS_TO_DI = 0,
    #[doc = "1: Enable access to Divisor Latches."]
    ENABLE_ACCESS_TO_DIV = 1,
}
impl From<DLAB_A> for bool {
    #[inline(always)]
    fn from(variant: DLAB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLAB`"]
pub type DLAB_R = crate::R<bool, DLAB_A>;
impl DLAB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLAB_A {
        match self.bits {
            false => DLAB_A::DISABLE_ACCESS_TO_DI,
            true => DLAB_A::ENABLE_ACCESS_TO_DIV,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_ACCESS_TO_DI`"]
    #[inline(always)]
    pub fn is_disable_access_to_di(&self) -> bool {
        *self == DLAB_A::DISABLE_ACCESS_TO_DI
    }
    #[doc = "Checks if the value of the field is `ENABLE_ACCESS_TO_DIV`"]
    #[inline(always)]
    pub fn is_enable_access_to_div(&self) -> bool {
        *self == DLAB_A::ENABLE_ACCESS_TO_DIV
    }
}
#[doc = "Write proxy for field `DLAB`"]
pub struct DLAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLAB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable access to Divisor Latches."]
    #[inline(always)]
    pub fn disable_access_to_di(self) -> &'a mut W {
        self.variant(DLAB_A::DISABLE_ACCESS_TO_DI)
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline(always)]
    pub fn enable_access_to_div(self) -> &'a mut W {
        self.variant(DLAB_A::ENABLE_ACCESS_TO_DIV)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&mut self) -> WLS_W {
        WLS_W { w: self }
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline(always)]
    pub fn sbs(&mut self) -> SBS_W {
        SBS_W { w: self }
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W {
        BC_W { w: self }
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&mut self) -> DLAB_W {
        DLAB_W { w: self }
    }
}
