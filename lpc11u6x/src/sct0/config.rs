#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
        0x7e00
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `UNIFY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFYR {
    #[doc = "The SCT operates as two 16-bit counters named L and H."]
    THE_SCT_OPERATES_AS_TWO_16BIT,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    THE_SCT_OPERATES_AS_UNIFIED_32BIT,
}
impl crate::ToBits<bool> for UNIFYR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            UNIFYR::THE_SCT_OPERATES_AS_TWO_16BIT => false,
            UNIFYR::THE_SCT_OPERATES_AS_UNIFIED_32BIT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type UNIFY_R = crate::FR<bool, UNIFYR>;
impl UNIFY_R {
    #[doc = "Checks if the value of the field is `THE_SCT_OPERATES_AS_TWO_16BIT`"]
    #[inline(always)]
    pub fn is_the_sct_operates_as_two_16bit(&self) -> bool {
        *self == UNIFYR::THE_SCT_OPERATES_AS_TWO_16BIT
    }
    #[doc = "Checks if the value of the field is `THE_SCT_OPERATES_AS_UNIFIED_32BIT`"]
    #[inline(always)]
    pub fn is_the_sct_operates_as_unified_32bit(&self) -> bool {
        *self == UNIFYR::THE_SCT_OPERATES_AS_UNIFIED_32BIT
    }
}
#[doc = "Values that can be written to the field `UNIFY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFYW {
    #[doc = "The SCT operates as two 16-bit counters named L and H."]
    THE_SCT_OPERATES_AS_TWO_16BIT,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    THE_SCT_OPERATES_AS_UNIFIED_32BIT,
}
impl UNIFYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            UNIFYW::THE_SCT_OPERATES_AS_TWO_16BIT => false,
            UNIFYW::THE_SCT_OPERATES_AS_UNIFIED_32BIT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UNIFYW<'a> {
    w: &'a mut W,
}
impl<'a> _UNIFYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNIFYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SCT operates as two 16-bit counters named L and H."]
    #[inline(always)]
    pub fn the_sct_operates_as_two_16bit(self) -> &'a mut W {
        self.variant(UNIFYW::THE_SCT_OPERATES_AS_TWO_16BIT)
    }
    #[doc = "The SCT operates as a unified 32-bit counter."]
    #[inline(always)]
    pub fn the_sct_operates_as_unified_32bit(self) -> &'a mut W {
        self.variant(UNIFYW::THE_SCT_OPERATES_AS_UNIFIED_32BIT)
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
#[doc = "Possible values of the field `CLKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKMODER {
    #[doc = "The bus clock clocks the SCT and prescalers."]
    THE_BUS_CLOCK_CLOCKS,
    #[doc = "The SCT clock is the bus clock, but the prescalers are  enabled to count only when sampling of the input selected by  the CKSEL field finds the selected edge. The minimum pulse  width on the clock input is 1 bus clock period. This mode is the high-performance  sampled-clock mode."]
    THE_SCT_CLOCK_IS_THE,
    #[doc = "The input selected by  CKSEL clocks the SCT and prescalers. The input is synchronized to the bus clock and possibly inverted.  The minimum pulse width on the clock input is 1 bus clock  period. This mode is the low-power sampled-clock mode."]
    THE_INPUT_SELECTED_B,
    #[doc = "Prescaled SCT input. The SCT and prescalers are clocked by the input edge  selected by the CKSEL field. In this mode, most of the SCT is clocked by the (selected polarity of the)  input. The outputs are switched synchronously to the input clock. The input clock rate must be at least half the system clock rate and  can the same or faster than the system clock."]
    PRESCALED_SCT_INPUT,
}
impl crate::ToBits<u8> for CLKMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CLKMODER::THE_BUS_CLOCK_CLOCKS => 0,
            CLKMODER::THE_SCT_CLOCK_IS_THE => 1,
            CLKMODER::THE_INPUT_SELECTED_B => 2,
            CLKMODER::PRESCALED_SCT_INPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CLKMODE_R = crate::FR<u8, CLKMODER>;
impl CLKMODE_R {
    #[doc = "Checks if the value of the field is `THE_BUS_CLOCK_CLOCKS`"]
    #[inline(always)]
    pub fn is_the_bus_clock_clocks(&self) -> bool {
        *self == CLKMODER::THE_BUS_CLOCK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `THE_SCT_CLOCK_IS_THE`"]
    #[inline(always)]
    pub fn is_the_sct_clock_is_the(&self) -> bool {
        *self == CLKMODER::THE_SCT_CLOCK_IS_THE
    }
    #[doc = "Checks if the value of the field is `THE_INPUT_SELECTED_B`"]
    #[inline(always)]
    pub fn is_the_input_selected_b(&self) -> bool {
        *self == CLKMODER::THE_INPUT_SELECTED_B
    }
    #[doc = "Checks if the value of the field is `PRESCALED_SCT_INPUT`"]
    #[inline(always)]
    pub fn is_prescaled_sct_input(&self) -> bool {
        *self == CLKMODER::PRESCALED_SCT_INPUT
    }
}
#[doc = "Values that can be written to the field `CLKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKMODEW {
    #[doc = "The bus clock clocks the SCT and prescalers."]
    THE_BUS_CLOCK_CLOCKS,
    #[doc = "The SCT clock is the bus clock, but the prescalers are  enabled to count only when sampling of the input selected by  the CKSEL field finds the selected edge. The minimum pulse  width on the clock input is 1 bus clock period. This mode is the high-performance  sampled-clock mode."]
    THE_SCT_CLOCK_IS_THE,
    #[doc = "The input selected by  CKSEL clocks the SCT and prescalers. The input is synchronized to the bus clock and possibly inverted.  The minimum pulse width on the clock input is 1 bus clock  period. This mode is the low-power sampled-clock mode."]
    THE_INPUT_SELECTED_B,
    #[doc = "Prescaled SCT input. The SCT and prescalers are clocked by the input edge  selected by the CKSEL field. In this mode, most of the SCT is clocked by the (selected polarity of the)  input. The outputs are switched synchronously to the input clock. The input clock rate must be at least half the system clock rate and  can the same or faster than the system clock."]
    PRESCALED_SCT_INPUT,
}
impl CLKMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKMODEW::THE_BUS_CLOCK_CLOCKS => 0,
            CLKMODEW::THE_SCT_CLOCK_IS_THE => 1,
            CLKMODEW::THE_INPUT_SELECTED_B => 2,
            CLKMODEW::PRESCALED_SCT_INPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CLKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The bus clock clocks the SCT and prescalers."]
    #[inline(always)]
    pub fn the_bus_clock_clocks(self) -> &'a mut W {
        self.variant(CLKMODEW::THE_BUS_CLOCK_CLOCKS)
    }
    #[doc = "The SCT clock is the bus clock, but the prescalers are enabled to count only when sampling of the input selected by the CKSEL field finds the selected edge. The minimum pulse width on the clock input is 1 bus clock period. This mode is the high-performance sampled-clock mode."]
    #[inline(always)]
    pub fn the_sct_clock_is_the(self) -> &'a mut W {
        self.variant(CLKMODEW::THE_SCT_CLOCK_IS_THE)
    }
    #[doc = "The input selected by CKSEL clocks the SCT and prescalers. The input is synchronized to the bus clock and possibly inverted. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power sampled-clock mode."]
    #[inline(always)]
    pub fn the_input_selected_b(self) -> &'a mut W {
        self.variant(CLKMODEW::THE_INPUT_SELECTED_B)
    }
    #[doc = "Prescaled SCT input. The SCT and prescalers are clocked by the input edge selected by the CKSEL field. In this mode, most of the SCT is clocked by the (selected polarity of the) input. The outputs are switched synchronously to the input clock. The input clock rate must be at least half the system clock rate and can the same or faster than the system clock."]
    #[inline(always)]
    pub fn prescaled_sct_input(self) -> &'a mut W {
        self.variant(CLKMODEW::PRESCALED_SCT_INPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `CKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSELR {
    #[doc = "Rising edges on input 0."]
    RISING_EDGES_ON_INPUT_0,
    #[doc = "Falling edges on input 0."]
    FALLING_EDGES_ON_INPUT_0,
    #[doc = "Rising edges on input 1."]
    RISING_EDGES_ON_INPUT_1,
    #[doc = "Falling edges on input 1."]
    FALLING_EDGES_ON_INPUT_1,
    #[doc = "Rising edges on input 2."]
    RISING_EDGES_ON_INPUT_2,
    #[doc = "Falling edges on input 2."]
    FALLING_EDGES_ON_INPUT_2,
    #[doc = "Rising edges on input 3."]
    RISING_EDGES_ON_INPUT_3,
    #[doc = "Falling edges on input 3."]
    FALLING_EDGES_ON_INPUT_3,
}
impl crate::ToBits<u8> for CKSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKSELR::RISING_EDGES_ON_INPUT_0 => 0,
            CKSELR::FALLING_EDGES_ON_INPUT_0 => 1,
            CKSELR::RISING_EDGES_ON_INPUT_1 => 2,
            CKSELR::FALLING_EDGES_ON_INPUT_1 => 3,
            CKSELR::RISING_EDGES_ON_INPUT_2 => 4,
            CKSELR::FALLING_EDGES_ON_INPUT_2 => 5,
            CKSELR::RISING_EDGES_ON_INPUT_3 => 6,
            CKSELR::FALLING_EDGES_ON_INPUT_3 => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CKSEL_R = crate::FR<u8, CKSELR>;
impl CKSEL_R {
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPUT_0`"]
    #[inline(always)]
    pub fn is_rising_edges_on_input_0(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPUT_0
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INPUT_0`"]
    #[inline(always)]
    pub fn is_falling_edges_on_input_0(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INPUT_0
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPUT_1`"]
    #[inline(always)]
    pub fn is_rising_edges_on_input_1(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPUT_1
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INPUT_1`"]
    #[inline(always)]
    pub fn is_falling_edges_on_input_1(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INPUT_1
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPUT_2`"]
    #[inline(always)]
    pub fn is_rising_edges_on_input_2(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPUT_2
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INPUT_2`"]
    #[inline(always)]
    pub fn is_falling_edges_on_input_2(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INPUT_2
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES_ON_INPUT_3`"]
    #[inline(always)]
    pub fn is_rising_edges_on_input_3(&self) -> bool {
        *self == CKSELR::RISING_EDGES_ON_INPUT_3
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES_ON_INPUT_3`"]
    #[inline(always)]
    pub fn is_falling_edges_on_input_3(&self) -> bool {
        *self == CKSELR::FALLING_EDGES_ON_INPUT_3
    }
}
#[doc = "Values that can be written to the field `CKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSELW {
    #[doc = "Rising edges on input 0."]
    RISING_EDGES_ON_INPUT_0,
    #[doc = "Falling edges on input 0."]
    FALLING_EDGES_ON_INPUT_0,
    #[doc = "Rising edges on input 1."]
    RISING_EDGES_ON_INPUT_1,
    #[doc = "Falling edges on input 1."]
    FALLING_EDGES_ON_INPUT_1,
    #[doc = "Rising edges on input 2."]
    RISING_EDGES_ON_INPUT_2,
    #[doc = "Falling edges on input 2."]
    FALLING_EDGES_ON_INPUT_2,
    #[doc = "Rising edges on input 3."]
    RISING_EDGES_ON_INPUT_3,
    #[doc = "Falling edges on input 3."]
    FALLING_EDGES_ON_INPUT_3,
}
impl CKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKSELW::RISING_EDGES_ON_INPUT_0 => 0,
            CKSELW::FALLING_EDGES_ON_INPUT_0 => 1,
            CKSELW::RISING_EDGES_ON_INPUT_1 => 2,
            CKSELW::FALLING_EDGES_ON_INPUT_1 => 3,
            CKSELW::RISING_EDGES_ON_INPUT_2 => 4,
            CKSELW::FALLING_EDGES_ON_INPUT_2 => 5,
            CKSELW::RISING_EDGES_ON_INPUT_3 => 6,
            CKSELW::FALLING_EDGES_ON_INPUT_3 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CKSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Rising edges on input 0."]
    #[inline(always)]
    pub fn rising_edges_on_input_0(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPUT_0)
    }
    #[doc = "Falling edges on input 0."]
    #[inline(always)]
    pub fn falling_edges_on_input_0(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INPUT_0)
    }
    #[doc = "Rising edges on input 1."]
    #[inline(always)]
    pub fn rising_edges_on_input_1(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPUT_1)
    }
    #[doc = "Falling edges on input 1."]
    #[inline(always)]
    pub fn falling_edges_on_input_1(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INPUT_1)
    }
    #[doc = "Rising edges on input 2."]
    #[inline(always)]
    pub fn rising_edges_on_input_2(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPUT_2)
    }
    #[doc = "Falling edges on input 2."]
    #[inline(always)]
    pub fn falling_edges_on_input_2(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INPUT_2)
    }
    #[doc = "Rising edges on input 3."]
    #[inline(always)]
    pub fn rising_edges_on_input_3(self) -> &'a mut W {
        self.variant(CKSELW::RISING_EDGES_ON_INPUT_3)
    }
    #[doc = "Falling edges on input 3."]
    #[inline(always)]
    pub fn falling_edges_on_input_3(self) -> &'a mut W {
        self.variant(CKSELW::FALLING_EDGES_ON_INPUT_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NORELAOD_L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NORELAOD_LW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELAOD_LW<'a> {
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
#[doc = r"Reader of the field"]
pub type NORELOAD_H_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NORELOAD_HW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELOAD_HW<'a> {
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
#[doc = r"Reader of the field"]
pub type INSYNC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _INSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _INSYNCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 9)) | (((value as u32) & 0xff) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AUTOLIMIT_L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AUTOLIMIT_LW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_LW<'a> {
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
#[doc = r"Reader of the field"]
pub type AUTOLIMIT_H_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AUTOLIMIT_HW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_HW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&self) -> UNIFY_R {
        UNIFY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&self) -> CLKMODE_R {
        CLKMODE_R::new(((self.bits() >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:6 - SCT clock select"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits() >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn norelaod_l(&self) -> NORELAOD_L_R {
        NORELAOD_L_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&self) -> NORELOAD_H_R {
        NORELOAD_H_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:16 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,..., bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is synchronous to the SCT clock, keep its bit 0 for faster response. When the CKMODE field is 1x, the bit in this field, corresponding to the input selected by the CKSEL field, is not used."]
    #[inline(always)]
    pub fn insync(&self) -> INSYNC_R {
        INSYNC_R::new(((self.bits() >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&self) -> AUTOLIMIT_L_R {
        AUTOLIMIT_L_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&self) -> AUTOLIMIT_H_R {
        AUTOLIMIT_H_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&mut self) -> _UNIFYW {
        _UNIFYW { w: self }
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&mut self) -> _CLKMODEW {
        _CLKMODEW { w: self }
    }
    #[doc = "Bits 3:6 - SCT clock select"]
    #[inline(always)]
    pub fn cksel(&mut self) -> _CKSELW {
        _CKSELW { w: self }
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn norelaod_l(&mut self) -> _NORELAOD_LW {
        _NORELAOD_LW { w: self }
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&mut self) -> _NORELOAD_HW {
        _NORELOAD_HW { w: self }
    }
    #[doc = "Bits 9:16 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,..., bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is synchronous to the SCT clock, keep its bit 0 for faster response. When the CKMODE field is 1x, the bit in this field, corresponding to the input selected by the CKSEL field, is not used."]
    #[inline(always)]
    pub fn insync(&mut self) -> _INSYNCW {
        _INSYNCW { w: self }
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&mut self) -> _AUTOLIMIT_LW {
        _AUTOLIMIT_LW { w: self }
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in uni-directional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&mut self) -> _AUTOLIMIT_HW {
        _AUTOLIMIT_HW { w: self }
    }
}
