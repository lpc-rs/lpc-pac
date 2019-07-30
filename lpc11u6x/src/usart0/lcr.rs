#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LCR {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `WLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLSR {
    #[doc = "5-bit character length."]
    _5_BIT_CHARACTER_LENG,
    #[doc = "6-bit character length."]
    _6_BIT_CHARACTER_LENG,
    #[doc = "7-bit character length."]
    _7_BIT_CHARACTER_LENG,
    #[doc = "8-bit character length."]
    _8_BIT_CHARACTER_LENG,
}
impl crate::ToBits<u8> for WLSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WLSR::_5_BIT_CHARACTER_LENG => 0,
            WLSR::_6_BIT_CHARACTER_LENG => 1,
            WLSR::_7_BIT_CHARACTER_LENG => 2,
            WLSR::_8_BIT_CHARACTER_LENG => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WLS_R = crate::FR<u8, WLSR>;
impl WLS_R {
    #[doc = "Checks if the value of the field is `_5_BIT_CHARACTER_LENG`"]
    #[inline(always)]
    pub fn is_5_bit_character_leng(&self) -> bool {
        *self == WLSR::_5_BIT_CHARACTER_LENG
    }
    #[doc = "Checks if the value of the field is `_6_BIT_CHARACTER_LENG`"]
    #[inline(always)]
    pub fn is_6_bit_character_leng(&self) -> bool {
        *self == WLSR::_6_BIT_CHARACTER_LENG
    }
    #[doc = "Checks if the value of the field is `_7_BIT_CHARACTER_LENG`"]
    #[inline(always)]
    pub fn is_7_bit_character_leng(&self) -> bool {
        *self == WLSR::_7_BIT_CHARACTER_LENG
    }
    #[doc = "Checks if the value of the field is `_8_BIT_CHARACTER_LENG`"]
    #[inline(always)]
    pub fn is_8_bit_character_leng(&self) -> bool {
        *self == WLSR::_8_BIT_CHARACTER_LENG
    }
}
#[doc = "Values that can be written to the field `WLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLSW {
    #[doc = "5-bit character length."]
    _5_BIT_CHARACTER_LENG,
    #[doc = "6-bit character length."]
    _6_BIT_CHARACTER_LENG,
    #[doc = "7-bit character length."]
    _7_BIT_CHARACTER_LENG,
    #[doc = "8-bit character length."]
    _8_BIT_CHARACTER_LENG,
}
impl WLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            WLSW::_5_BIT_CHARACTER_LENG => 0,
            WLSW::_6_BIT_CHARACTER_LENG => 1,
            WLSW::_7_BIT_CHARACTER_LENG => 2,
            WLSW::_8_BIT_CHARACTER_LENG => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WLSW<'a> {
    w: &'a mut W,
}
impl<'a> _WLSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "5-bit character length."]
    #[inline(always)]
    pub fn _5_bit_character_leng(self) -> &'a mut W {
        self.variant(WLSW::_5_BIT_CHARACTER_LENG)
    }
    #[doc = "6-bit character length."]
    #[inline(always)]
    pub fn _6_bit_character_leng(self) -> &'a mut W {
        self.variant(WLSW::_6_BIT_CHARACTER_LENG)
    }
    #[doc = "7-bit character length."]
    #[inline(always)]
    pub fn _7_bit_character_leng(self) -> &'a mut W {
        self.variant(WLSW::_7_BIT_CHARACTER_LENG)
    }
    #[doc = "8-bit character length."]
    #[inline(always)]
    pub fn _8_bit_character_leng(self) -> &'a mut W {
        self.variant(WLSW::_8_BIT_CHARACTER_LENG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `SBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBSR {
    #[doc = "1 stop bit."]
    _1_STOP_BIT_,
    #[doc = "2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    _2_STOP_BITS_1_5_IF_,
}
impl crate::ToBits<bool> for SBSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SBSR::_1_STOP_BIT_ => false,
            SBSR::_2_STOP_BITS_1_5_IF_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SBS_R = crate::FR<bool, SBSR>;
impl SBS_R {
    #[doc = "Checks if the value of the field is `_1_STOP_BIT_`"]
    #[inline(always)]
    pub fn is_1_stop_bit_(&self) -> bool {
        *self == SBSR::_1_STOP_BIT_
    }
    #[doc = "Checks if the value of the field is `_2_STOP_BITS_1_5_IF_`"]
    #[inline(always)]
    pub fn is_2_stop_bits_1_5_if_(&self) -> bool {
        *self == SBSR::_2_STOP_BITS_1_5_IF_
    }
}
#[doc = "Values that can be written to the field `SBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBSW {
    #[doc = "1 stop bit."]
    _1_STOP_BIT_,
    #[doc = "2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    _2_STOP_BITS_1_5_IF_,
}
impl SBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SBSW::_1_STOP_BIT_ => false,
            SBSW::_2_STOP_BITS_1_5_IF_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SBSW<'a> {
    w: &'a mut W,
}
impl<'a> _SBSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn _1_stop_bit_(self) -> &'a mut W {
        self.variant(SBSW::_1_STOP_BIT_)
    }
    #[doc = "2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    #[inline(always)]
    pub fn _2_stop_bits_1_5_if_(self) -> &'a mut W {
        self.variant(SBSW::_2_STOP_BITS_1_5_IF_)
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
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Disable parity generation and checking."]
    DISABLE_PARITY_GENER,
    #[doc = "Enable parity generation and checking."]
    ENABLE_PARITY_GENERA,
}
impl crate::ToBits<bool> for PER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PER::DISABLE_PARITY_GENER => false,
            PER::ENABLE_PARITY_GENERA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PE_R = crate::FR<bool, PER>;
impl PE_R {
    #[doc = "Checks if the value of the field is `DISABLE_PARITY_GENER`"]
    #[inline(always)]
    pub fn is_disable_parity_gener(&self) -> bool {
        *self == PER::DISABLE_PARITY_GENER
    }
    #[doc = "Checks if the value of the field is `ENABLE_PARITY_GENERA`"]
    #[inline(always)]
    pub fn is_enable_parity_genera(&self) -> bool {
        *self == PER::ENABLE_PARITY_GENERA
    }
}
#[doc = "Values that can be written to the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEW {
    #[doc = "Disable parity generation and checking."]
    DISABLE_PARITY_GENER,
    #[doc = "Enable parity generation and checking."]
    ENABLE_PARITY_GENERA,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::DISABLE_PARITY_GENER => false,
            PEW::ENABLE_PARITY_GENERA => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable parity generation and checking."]
    #[inline(always)]
    pub fn disable_parity_gener(self) -> &'a mut W {
        self.variant(PEW::DISABLE_PARITY_GENER)
    }
    #[doc = "Enable parity generation and checking."]
    #[inline(always)]
    pub fn enable_parity_genera(self) -> &'a mut W {
        self.variant(PEW::ENABLE_PARITY_GENERA)
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
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    ODD_PARITY_NUMBER_O,
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EVEN_PARITY_NUMBER_,
    #[doc = "Forced 1 stick parity."]
    FORCED_1_STICK_PARIT,
    #[doc = "Forced 0 stick parity."]
    FORCED_0_STICK_PARIT,
}
impl crate::ToBits<u8> for PSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PSR::ODD_PARITY_NUMBER_O => 0,
            PSR::EVEN_PARITY_NUMBER_ => 1,
            PSR::FORCED_1_STICK_PARIT => 2,
            PSR::FORCED_0_STICK_PARIT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PS_R = crate::FR<u8, PSR>;
impl PS_R {
    #[doc = "Checks if the value of the field is `ODD_PARITY_NUMBER_O`"]
    #[inline(always)]
    pub fn is_odd_parity_number_o(&self) -> bool {
        *self == PSR::ODD_PARITY_NUMBER_O
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY_NUMBER_`"]
    #[inline(always)]
    pub fn is_even_parity_number_(&self) -> bool {
        *self == PSR::EVEN_PARITY_NUMBER_
    }
    #[doc = "Checks if the value of the field is `FORCED_1_STICK_PARIT`"]
    #[inline(always)]
    pub fn is_forced_1_stick_parit(&self) -> bool {
        *self == PSR::FORCED_1_STICK_PARIT
    }
    #[doc = "Checks if the value of the field is `FORCED_0_STICK_PARIT`"]
    #[inline(always)]
    pub fn is_forced_0_stick_parit(&self) -> bool {
        *self == PSR::FORCED_0_STICK_PARIT
    }
}
#[doc = "Values that can be written to the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSW {
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    ODD_PARITY_NUMBER_O,
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EVEN_PARITY_NUMBER_,
    #[doc = "Forced 1 stick parity."]
    FORCED_1_STICK_PARIT,
    #[doc = "Forced 0 stick parity."]
    FORCED_0_STICK_PARIT,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::ODD_PARITY_NUMBER_O => 0,
            PSW::EVEN_PARITY_NUMBER_ => 1,
            PSW::FORCED_1_STICK_PARIT => 2,
            PSW::FORCED_0_STICK_PARIT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline(always)]
    pub fn odd_parity_number_o(self) -> &'a mut W {
        self.variant(PSW::ODD_PARITY_NUMBER_O)
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline(always)]
    pub fn even_parity_number_(self) -> &'a mut W {
        self.variant(PSW::EVEN_PARITY_NUMBER_)
    }
    #[doc = "Forced 1 stick parity."]
    #[inline(always)]
    pub fn forced_1_stick_parit(self) -> &'a mut W {
        self.variant(PSW::FORCED_1_STICK_PARIT)
    }
    #[doc = "Forced 0 stick parity."]
    #[inline(always)]
    pub fn forced_0_stick_parit(self) -> &'a mut W {
        self.variant(PSW::FORCED_0_STICK_PARIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `BC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCR {
    #[doc = "Disable break transmission."]
    DISABLE_BREAK_TRANSM,
    #[doc = "Enable break transmission. Output pin USART TXD is forced to logic 0 when LCR\\[6\\] is active high."]
    ENABLE_BREAK_TRANSMI,
}
impl crate::ToBits<bool> for BCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BCR::DISABLE_BREAK_TRANSM => false,
            BCR::ENABLE_BREAK_TRANSMI => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BC_R = crate::FR<bool, BCR>;
impl BC_R {
    #[doc = "Checks if the value of the field is `DISABLE_BREAK_TRANSM`"]
    #[inline(always)]
    pub fn is_disable_break_transm(&self) -> bool {
        *self == BCR::DISABLE_BREAK_TRANSM
    }
    #[doc = "Checks if the value of the field is `ENABLE_BREAK_TRANSMI`"]
    #[inline(always)]
    pub fn is_enable_break_transmi(&self) -> bool {
        *self == BCR::ENABLE_BREAK_TRANSMI
    }
}
#[doc = "Values that can be written to the field `BC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCW {
    #[doc = "Disable break transmission."]
    DISABLE_BREAK_TRANSM,
    #[doc = "Enable break transmission. Output pin USART TXD is forced to logic 0 when LCR\\[6\\] is active high."]
    ENABLE_BREAK_TRANSMI,
}
impl BCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BCW::DISABLE_BREAK_TRANSM => false,
            BCW::ENABLE_BREAK_TRANSMI => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BCW<'a> {
    w: &'a mut W,
}
impl<'a> _BCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable break transmission."]
    #[inline(always)]
    pub fn disable_break_transm(self) -> &'a mut W {
        self.variant(BCW::DISABLE_BREAK_TRANSM)
    }
    #[doc = "Enable break transmission. Output pin USART TXD is forced to logic 0 when LCR\\[6\\] is active high."]
    #[inline(always)]
    pub fn enable_break_transmi(self) -> &'a mut W {
        self.variant(BCW::ENABLE_BREAK_TRANSMI)
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
#[doc = "Possible values of the field `DLAB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLABR {
    #[doc = "Disable access to Divisor Latches."]
    DISABLE_ACCESS_TO_DI,
    #[doc = "Enable access to Divisor Latches."]
    ENABLE_ACCESS_TO_DIV,
}
impl crate::ToBits<bool> for DLABR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DLABR::DISABLE_ACCESS_TO_DI => false,
            DLABR::ENABLE_ACCESS_TO_DIV => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DLAB_R = crate::FR<bool, DLABR>;
impl DLAB_R {
    #[doc = "Checks if the value of the field is `DISABLE_ACCESS_TO_DI`"]
    #[inline(always)]
    pub fn is_disable_access_to_di(&self) -> bool {
        *self == DLABR::DISABLE_ACCESS_TO_DI
    }
    #[doc = "Checks if the value of the field is `ENABLE_ACCESS_TO_DIV`"]
    #[inline(always)]
    pub fn is_enable_access_to_div(&self) -> bool {
        *self == DLABR::ENABLE_ACCESS_TO_DIV
    }
}
#[doc = "Values that can be written to the field `DLAB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLABW {
    #[doc = "Disable access to Divisor Latches."]
    DISABLE_ACCESS_TO_DI,
    #[doc = "Enable access to Divisor Latches."]
    ENABLE_ACCESS_TO_DIV,
}
impl DLABW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DLABW::DISABLE_ACCESS_TO_DI => false,
            DLABW::ENABLE_ACCESS_TO_DIV => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DLABW<'a> {
    w: &'a mut W,
}
impl<'a> _DLABW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLABW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable access to Divisor Latches."]
    #[inline(always)]
    pub fn disable_access_to_di(self) -> &'a mut W {
        self.variant(DLABW::DISABLE_ACCESS_TO_DI)
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline(always)]
    pub fn enable_access_to_div(self) -> &'a mut W {
        self.variant(DLABW::ENABLE_ACCESS_TO_DIV)
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
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Word Length Select"]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Word Length Select"]
    #[inline(always)]
    pub fn wls(&mut self) -> _WLSW {
        _WLSW { w: self }
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline(always)]
    pub fn sbs(&mut self) -> _SBSW {
        _SBSW { w: self }
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    pub fn bc(&mut self) -> _BCW {
        _BCW { w: self }
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&mut self) -> _DLABW {
        _DLABW { w: self }
    }
}
