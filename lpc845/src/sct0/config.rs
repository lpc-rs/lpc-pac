#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `UNIFY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFYR {
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DUAL_COUNTER,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    UNIFIED_COUNTER,
}
impl UNIFYR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UNIFYR::DUAL_COUNTER => false,
            UNIFYR::UNIFIED_COUNTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNIFYR {
        match value {
            false => UNIFYR::DUAL_COUNTER,
            true => UNIFYR::UNIFIED_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `DUAL_COUNTER`"]
    #[inline]
    pub fn is_dual_counter(&self) -> bool {
        *self == UNIFYR::DUAL_COUNTER
    }
    #[doc = "Checks if the value of the field is `UNIFIED_COUNTER`"]
    #[inline]
    pub fn is_unified_counter(&self) -> bool {
        *self == UNIFYR::UNIFIED_COUNTER
    }
}
#[doc = "Possible values of the field `CLKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKMODER {
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SYSTEM_CLOCK_MODE,
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SAMPLED_SYSTEM_CLOCK_MODE,
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SCT_INPUT_CLOCK_MODE,
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    ASYNCHRONOUS_MODE,
}
impl CLKMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKMODER::SYSTEM_CLOCK_MODE => 0,
            CLKMODER::SAMPLED_SYSTEM_CLOCK_MODE => 1,
            CLKMODER::SCT_INPUT_CLOCK_MODE => 2,
            CLKMODER::ASYNCHRONOUS_MODE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKMODER {
        match value {
            0 => CLKMODER::SYSTEM_CLOCK_MODE,
            1 => CLKMODER::SAMPLED_SYSTEM_CLOCK_MODE,
            2 => CLKMODER::SCT_INPUT_CLOCK_MODE,
            3 => CLKMODER::ASYNCHRONOUS_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSTEM_CLOCK_MODE`"]
    #[inline]
    pub fn is_system_clock_mode(&self) -> bool {
        *self == CLKMODER::SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SAMPLED_SYSTEM_CLOCK_MODE`"]
    #[inline]
    pub fn is_sampled_system_clock_mode(&self) -> bool {
        *self == CLKMODER::SAMPLED_SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SCT_INPUT_CLOCK_MODE`"]
    #[inline]
    pub fn is_sct_input_clock_mode(&self) -> bool {
        *self == CLKMODER::SCT_INPUT_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == CLKMODER::ASYNCHRONOUS_MODE
    }
}
#[doc = "Possible values of the field `CKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSELR {
    #[doc = "Rising edges on input 0."]
    INPUT_0_RISING_EDGES,
    #[doc = "Falling edges on input 0."]
    INPUT_0_FALLING_EDGE,
    #[doc = "Rising edges on input 1."]
    INPUT_1_RISING_EDGES,
    #[doc = "Falling edges on input 1."]
    INPUT_1_FALLING_EDGE,
    #[doc = "Rising edges on input 2."]
    INPUT_2_RISING_EDGES,
    #[doc = "Falling edges on input 2."]
    INPUT_2_FALLING_EDGE,
    #[doc = "Rising edges on input 3."]
    INPUT_3_RISING_EDGES,
    #[doc = "Falling edges on input 3."]
    INPUT_3_FALLING_EDGE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKSELR::INPUT_0_RISING_EDGES => 0,
            CKSELR::INPUT_0_FALLING_EDGE => 1,
            CKSELR::INPUT_1_RISING_EDGES => 2,
            CKSELR::INPUT_1_FALLING_EDGE => 3,
            CKSELR::INPUT_2_RISING_EDGES => 4,
            CKSELR::INPUT_2_FALLING_EDGE => 5,
            CKSELR::INPUT_3_RISING_EDGES => 6,
            CKSELR::INPUT_3_FALLING_EDGE => 7,
            CKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKSELR {
        match value {
            0 => CKSELR::INPUT_0_RISING_EDGES,
            1 => CKSELR::INPUT_0_FALLING_EDGE,
            2 => CKSELR::INPUT_1_RISING_EDGES,
            3 => CKSELR::INPUT_1_FALLING_EDGE,
            4 => CKSELR::INPUT_2_RISING_EDGES,
            5 => CKSELR::INPUT_2_FALLING_EDGE,
            6 => CKSELR::INPUT_3_RISING_EDGES,
            7 => CKSELR::INPUT_3_FALLING_EDGE,
            i => CKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_0_RISING_EDGES`"]
    #[inline]
    pub fn is_input_0_rising_edges(&self) -> bool {
        *self == CKSELR::INPUT_0_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_0_FALLING_EDGE`"]
    #[inline]
    pub fn is_input_0_falling_edge(&self) -> bool {
        *self == CKSELR::INPUT_0_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_1_RISING_EDGES`"]
    #[inline]
    pub fn is_input_1_rising_edges(&self) -> bool {
        *self == CKSELR::INPUT_1_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_1_FALLING_EDGE`"]
    #[inline]
    pub fn is_input_1_falling_edge(&self) -> bool {
        *self == CKSELR::INPUT_1_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_2_RISING_EDGES`"]
    #[inline]
    pub fn is_input_2_rising_edges(&self) -> bool {
        *self == CKSELR::INPUT_2_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_2_FALLING_EDGE`"]
    #[inline]
    pub fn is_input_2_falling_edge(&self) -> bool {
        *self == CKSELR::INPUT_2_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_3_RISING_EDGES`"]
    #[inline]
    pub fn is_input_3_rising_edges(&self) -> bool {
        *self == CKSELR::INPUT_3_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_3_FALLING_EDGE`"]
    #[inline]
    pub fn is_input_3_falling_edge(&self) -> bool {
        *self == CKSELR::INPUT_3_FALLING_EDGE
    }
}
#[doc = r" Value of the field"]
pub struct NORELAOD_LR {
    bits: bool,
}
impl NORELAOD_LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct NORELOAD_HR {
    bits: bool,
}
impl NORELOAD_HR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct INSYNCR {
    bits: u8,
}
impl INSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AUTOLIMIT_LR {
    bits: bool,
}
impl AUTOLIMIT_LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct AUTOLIMIT_HR {
    bits: bool,
}
impl AUTOLIMIT_HR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `UNIFY`"]
pub enum UNIFYW {
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DUAL_COUNTER,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    UNIFIED_COUNTER,
}
impl UNIFYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNIFYW::DUAL_COUNTER => false,
            UNIFYW::UNIFIED_COUNTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNIFYW<'a> {
    w: &'a mut W,
}
impl<'a> _UNIFYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNIFYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    #[inline]
    pub fn dual_counter(self) -> &'a mut W {
        self.variant(UNIFYW::DUAL_COUNTER)
    }
    #[doc = "The SCT operates as a unified 32-bit counter."]
    #[inline]
    pub fn unified_counter(self) -> &'a mut W {
        self.variant(UNIFYW::UNIFIED_COUNTER)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKMODE`"]
pub enum CLKMODEW {
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SYSTEM_CLOCK_MODE,
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SAMPLED_SYSTEM_CLOCK_MODE,
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SCT_INPUT_CLOCK_MODE,
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    ASYNCHRONOUS_MODE,
}
impl CLKMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKMODEW::SYSTEM_CLOCK_MODE => 0,
            CLKMODEW::SAMPLED_SYSTEM_CLOCK_MODE => 1,
            CLKMODEW::SCT_INPUT_CLOCK_MODE => 2,
            CLKMODEW::ASYNCHRONOUS_MODE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    #[inline]
    pub fn system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODEW::SYSTEM_CLOCK_MODE)
    }
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    #[inline]
    pub fn sampled_system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODEW::SAMPLED_SYSTEM_CLOCK_MODE)
    }
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    #[inline]
    pub fn sct_input_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODEW::SCT_INPUT_CLOCK_MODE)
    }
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    #[inline]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(CLKMODEW::ASYNCHRONOUS_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CKSEL`"]
pub enum CKSELW {
    #[doc = "Rising edges on input 0."]
    INPUT_0_RISING_EDGES,
    #[doc = "Falling edges on input 0."]
    INPUT_0_FALLING_EDGE,
    #[doc = "Rising edges on input 1."]
    INPUT_1_RISING_EDGES,
    #[doc = "Falling edges on input 1."]
    INPUT_1_FALLING_EDGE,
    #[doc = "Rising edges on input 2."]
    INPUT_2_RISING_EDGES,
    #[doc = "Falling edges on input 2."]
    INPUT_2_FALLING_EDGE,
    #[doc = "Rising edges on input 3."]
    INPUT_3_RISING_EDGES,
    #[doc = "Falling edges on input 3."]
    INPUT_3_FALLING_EDGE,
}
impl CKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKSELW::INPUT_0_RISING_EDGES => 0,
            CKSELW::INPUT_0_FALLING_EDGE => 1,
            CKSELW::INPUT_1_RISING_EDGES => 2,
            CKSELW::INPUT_1_FALLING_EDGE => 3,
            CKSELW::INPUT_2_RISING_EDGES => 4,
            CKSELW::INPUT_2_FALLING_EDGE => 5,
            CKSELW::INPUT_3_RISING_EDGES => 6,
            CKSELW::INPUT_3_FALLING_EDGE => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Rising edges on input 0."]
    #[inline]
    pub fn input_0_rising_edges(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_0_RISING_EDGES)
    }
    #[doc = "Falling edges on input 0."]
    #[inline]
    pub fn input_0_falling_edge(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_0_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 1."]
    #[inline]
    pub fn input_1_rising_edges(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_1_RISING_EDGES)
    }
    #[doc = "Falling edges on input 1."]
    #[inline]
    pub fn input_1_falling_edge(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_1_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 2."]
    #[inline]
    pub fn input_2_rising_edges(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_2_RISING_EDGES)
    }
    #[doc = "Falling edges on input 2."]
    #[inline]
    pub fn input_2_falling_edge(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_2_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 3."]
    #[inline]
    pub fn input_3_rising_edges(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_3_RISING_EDGES)
    }
    #[doc = "Falling edges on input 3."]
    #[inline]
    pub fn input_3_falling_edge(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_3_FALLING_EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NORELAOD_LW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELAOD_LW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NORELOAD_HW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELOAD_HW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _INSYNCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOLIMIT_LW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_LW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOLIMIT_HW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_HW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SCT operation"]
    #[inline]
    pub fn unify(&self) -> UNIFYR {
        UNIFYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline]
    pub fn clkmode(&self) -> CLKMODER {
        CLKMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline]
    pub fn cksel(&self) -> CKSELR {
        CKSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn norelaod_l(&self) -> NORELAOD_LR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NORELAOD_LR { bits }
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn noreload_h(&self) -> NORELOAD_HR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NORELOAD_HR { bits }
    }
    #[doc = "Bits 9:12 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline]
    pub fn insync(&self) -> INSYNCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INSYNCR { bits }
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_l(&self) -> AUTOLIMIT_LR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOLIMIT_LR { bits }
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_h(&self) -> AUTOLIMIT_HR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOLIMIT_HR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 7680 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SCT operation"]
    #[inline]
    pub fn unify(&mut self) -> _UNIFYW {
        _UNIFYW { w: self }
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline]
    pub fn clkmode(&mut self) -> _CLKMODEW {
        _CLKMODEW { w: self }
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline]
    pub fn cksel(&mut self) -> _CKSELW {
        _CKSELW { w: self }
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn norelaod_l(&mut self) -> _NORELAOD_LW {
        _NORELAOD_LW { w: self }
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn noreload_h(&mut self) -> _NORELOAD_HW {
        _NORELOAD_HW { w: self }
    }
    #[doc = "Bits 9:12 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline]
    pub fn insync(&mut self) -> _INSYNCW {
        _INSYNCW { w: self }
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_l(&mut self) -> _AUTOLIMIT_LW {
        _AUTOLIMIT_LW { w: self }
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline]
    pub fn autolimit_h(&mut self) -> _AUTOLIMIT_HW {
        _AUTOLIMIT_HW { w: self }
    }
}
