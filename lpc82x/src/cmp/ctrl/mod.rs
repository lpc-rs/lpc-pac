#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `EDGESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGESELR {
    #[doc = "Falling edges"]
    FALLING_EDGES,
    #[doc = "Rising edges"]
    RISING_EDGES,
    #[doc = "Both edges"]
    BOTH_EDGES_1,
    #[doc = "Both edges"]
    BOTH_EDGES_2,
}
impl EDGESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGESELR::FALLING_EDGES => 0,
            EDGESELR::RISING_EDGES => 1,
            EDGESELR::BOTH_EDGES_1 => 2,
            EDGESELR::BOTH_EDGES_2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGESELR {
        match value {
            0 => EDGESELR::FALLING_EDGES,
            1 => EDGESELR::RISING_EDGES,
            2 => EDGESELR::BOTH_EDGES_1,
            3 => EDGESELR::BOTH_EDGES_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES`"]
    #[inline]
    pub fn is_falling_edges(&self) -> bool {
        *self == EDGESELR::FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES`"]
    #[inline]
    pub fn is_rising_edges(&self) -> bool {
        *self == EDGESELR::RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES_1`"]
    #[inline]
    pub fn is_both_edges_1(&self) -> bool {
        *self == EDGESELR::BOTH_EDGES_1
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES_2`"]
    #[inline]
    pub fn is_both_edges_2(&self) -> bool {
        *self == EDGESELR::BOTH_EDGES_2
    }
}
#[doc = "Possible values of the field `COMPSA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPSAR {
    #[doc = "Comparator output is used directly."]
    DIRECT,
    #[doc = "Comparator output is synchronized to the bus clock for output to other modules."]
    SYNC,
}
impl COMPSAR {
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
            COMPSAR::DIRECT => false,
            COMPSAR::SYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMPSAR {
        match value {
            false => COMPSAR::DIRECT,
            true => COMPSAR::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline]
    pub fn is_direct(&self) -> bool {
        *self == COMPSAR::DIRECT
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline]
    pub fn is_sync(&self) -> bool {
        *self == COMPSAR::SYNC
    }
}
#[doc = "Possible values of the field `COMP_VP_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_VP_SELR {
    #[doc = "Voltage ladder output"]
    VOLTAGE_LADDER_OUTPU,
    #[doc = "ACMP_I1"]
    ACMP_I1,
    #[doc = "ACMP_I2"]
    ACMP_I2,
    #[doc = "ACMP_I3"]
    ACMP_I3,
    #[doc = "ACMP_I4"]
    ACMP_I4,
    #[doc = "Band gap. Internal reference voltage."]
    BAND_GAP,
    #[doc = "ADC_0. ADC channel 0 input."]
    ADC_0,
}
impl COMP_VP_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMP_VP_SELR::VOLTAGE_LADDER_OUTPU => 0,
            COMP_VP_SELR::ACMP_I1 => 1,
            COMP_VP_SELR::ACMP_I2 => 2,
            COMP_VP_SELR::ACMP_I3 => 3,
            COMP_VP_SELR::ACMP_I4 => 4,
            COMP_VP_SELR::BAND_GAP => 5,
            COMP_VP_SELR::ADC_0 => 6,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMP_VP_SELR {
        match value {
            0 => COMP_VP_SELR::VOLTAGE_LADDER_OUTPU,
            1 => COMP_VP_SELR::ACMP_I1,
            2 => COMP_VP_SELR::ACMP_I2,
            3 => COMP_VP_SELR::ACMP_I3,
            4 => COMP_VP_SELR::ACMP_I4,
            5 => COMP_VP_SELR::BAND_GAP,
            6 => COMP_VP_SELR::ADC_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_LADDER_OUTPU`"]
    #[inline]
    pub fn is_voltage_ladder_outpu(&self) -> bool {
        *self == COMP_VP_SELR::VOLTAGE_LADDER_OUTPU
    }
    #[doc = "Checks if the value of the field is `ACMP_I1`"]
    #[inline]
    pub fn is_acmp_i1(&self) -> bool {
        *self == COMP_VP_SELR::ACMP_I1
    }
    #[doc = "Checks if the value of the field is `ACMP_I2`"]
    #[inline]
    pub fn is_acmp_i2(&self) -> bool {
        *self == COMP_VP_SELR::ACMP_I2
    }
    #[doc = "Checks if the value of the field is `ACMP_I3`"]
    #[inline]
    pub fn is_acmp_i3(&self) -> bool {
        *self == COMP_VP_SELR::ACMP_I3
    }
    #[doc = "Checks if the value of the field is `ACMP_I4`"]
    #[inline]
    pub fn is_acmp_i4(&self) -> bool {
        *self == COMP_VP_SELR::ACMP_I4
    }
    #[doc = "Checks if the value of the field is `BAND_GAP`"]
    #[inline]
    pub fn is_band_gap(&self) -> bool {
        *self == COMP_VP_SELR::BAND_GAP
    }
    #[doc = "Checks if the value of the field is `ADC_0`"]
    #[inline]
    pub fn is_adc_0(&self) -> bool {
        *self == COMP_VP_SELR::ADC_0
    }
}
#[doc = "Possible values of the field `COMP_VM_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_VM_SELR {
    #[doc = "Voltage ladder output"]
    VOLTAGE_LADDER_OUTPU,
    #[doc = "ACMP_I1"]
    ACMP_I1,
    #[doc = "ACMP_I2"]
    ACMP_I2,
    #[doc = "ACMP_I3"]
    ACMP_I3,
    #[doc = "ACMP_I4"]
    ACMP_I4,
    #[doc = "Band gap. Internal reference voltage."]
    BAND_GAP,
    #[doc = "ADC_0. ADC channel 0 input."]
    ADC_0,
}
impl COMP_VM_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMP_VM_SELR::VOLTAGE_LADDER_OUTPU => 0,
            COMP_VM_SELR::ACMP_I1 => 1,
            COMP_VM_SELR::ACMP_I2 => 2,
            COMP_VM_SELR::ACMP_I3 => 3,
            COMP_VM_SELR::ACMP_I4 => 4,
            COMP_VM_SELR::BAND_GAP => 5,
            COMP_VM_SELR::ADC_0 => 6,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMP_VM_SELR {
        match value {
            0 => COMP_VM_SELR::VOLTAGE_LADDER_OUTPU,
            1 => COMP_VM_SELR::ACMP_I1,
            2 => COMP_VM_SELR::ACMP_I2,
            3 => COMP_VM_SELR::ACMP_I3,
            4 => COMP_VM_SELR::ACMP_I4,
            5 => COMP_VM_SELR::BAND_GAP,
            6 => COMP_VM_SELR::ADC_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_LADDER_OUTPU`"]
    #[inline]
    pub fn is_voltage_ladder_outpu(&self) -> bool {
        *self == COMP_VM_SELR::VOLTAGE_LADDER_OUTPU
    }
    #[doc = "Checks if the value of the field is `ACMP_I1`"]
    #[inline]
    pub fn is_acmp_i1(&self) -> bool {
        *self == COMP_VM_SELR::ACMP_I1
    }
    #[doc = "Checks if the value of the field is `ACMP_I2`"]
    #[inline]
    pub fn is_acmp_i2(&self) -> bool {
        *self == COMP_VM_SELR::ACMP_I2
    }
    #[doc = "Checks if the value of the field is `ACMP_I3`"]
    #[inline]
    pub fn is_acmp_i3(&self) -> bool {
        *self == COMP_VM_SELR::ACMP_I3
    }
    #[doc = "Checks if the value of the field is `ACMP_I4`"]
    #[inline]
    pub fn is_acmp_i4(&self) -> bool {
        *self == COMP_VM_SELR::ACMP_I4
    }
    #[doc = "Checks if the value of the field is `BAND_GAP`"]
    #[inline]
    pub fn is_band_gap(&self) -> bool {
        *self == COMP_VM_SELR::BAND_GAP
    }
    #[doc = "Checks if the value of the field is `ADC_0`"]
    #[inline]
    pub fn is_adc_0(&self) -> bool {
        *self == COMP_VM_SELR::ADC_0
    }
}
#[doc = r" Value of the field"]
pub struct EDGECLRR {
    bits: bool,
}
impl EDGECLRR {
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
pub struct COMPSTATR {
    bits: bool,
}
impl COMPSTATR {
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
pub struct COMPEDGER {
    bits: bool,
}
impl COMPEDGER {
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
#[doc = "Possible values of the field `HYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSR {
    #[doc = "None (the output will switch as the voltages cross)"]
    NONE_THE_OUTPUT_WIL,
    #[doc = "5 mV"]
    _5_MV,
    #[doc = "10 mV"]
    _10_MV,
    #[doc = "20  mV"]
    _20_MV,
}
impl HYSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HYSR::NONE_THE_OUTPUT_WIL => 0,
            HYSR::_5_MV => 1,
            HYSR::_10_MV => 2,
            HYSR::_20_MV => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HYSR {
        match value {
            0 => HYSR::NONE_THE_OUTPUT_WIL,
            1 => HYSR::_5_MV,
            2 => HYSR::_10_MV,
            3 => HYSR::_20_MV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE_THE_OUTPUT_WIL`"]
    #[inline]
    pub fn is_none_the_output_wil(&self) -> bool {
        *self == HYSR::NONE_THE_OUTPUT_WIL
    }
    #[doc = "Checks if the value of the field is `_5_MV`"]
    #[inline]
    pub fn is_5_mv(&self) -> bool {
        *self == HYSR::_5_MV
    }
    #[doc = "Checks if the value of the field is `_10_MV`"]
    #[inline]
    pub fn is_10_mv(&self) -> bool {
        *self == HYSR::_10_MV
    }
    #[doc = "Checks if the value of the field is `_20_MV`"]
    #[inline]
    pub fn is_20_mv(&self) -> bool {
        *self == HYSR::_20_MV
    }
}
#[doc = "Values that can be written to the field `EDGESEL`"]
pub enum EDGESELW {
    #[doc = "Falling edges"]
    FALLING_EDGES,
    #[doc = "Rising edges"]
    RISING_EDGES,
    #[doc = "Both edges"]
    BOTH_EDGES_1,
    #[doc = "Both edges"]
    BOTH_EDGES_2,
}
impl EDGESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGESELW::FALLING_EDGES => 0,
            EDGESELW::RISING_EDGES => 1,
            EDGESELW::BOTH_EDGES_1 => 2,
            EDGESELW::BOTH_EDGES_2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGESELW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGESELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Falling edges"]
    #[inline]
    pub fn falling_edges(self) -> &'a mut W {
        self.variant(EDGESELW::FALLING_EDGES)
    }
    #[doc = "Rising edges"]
    #[inline]
    pub fn rising_edges(self) -> &'a mut W {
        self.variant(EDGESELW::RISING_EDGES)
    }
    #[doc = "Both edges"]
    #[inline]
    pub fn both_edges_1(self) -> &'a mut W {
        self.variant(EDGESELW::BOTH_EDGES_1)
    }
    #[doc = "Both edges"]
    #[inline]
    pub fn both_edges_2(self) -> &'a mut W {
        self.variant(EDGESELW::BOTH_EDGES_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPSA`"]
pub enum COMPSAW {
    #[doc = "Comparator output is used directly."]
    DIRECT,
    #[doc = "Comparator output is synchronized to the bus clock for output to other modules."]
    SYNC,
}
impl COMPSAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMPSAW::DIRECT => false,
            COMPSAW::SYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPSAW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPSAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPSAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Comparator output is used directly."]
    #[inline]
    pub fn direct(self) -> &'a mut W {
        self.variant(COMPSAW::DIRECT)
    }
    #[doc = "Comparator output is synchronized to the bus clock for output to other modules."]
    #[inline]
    pub fn sync(self) -> &'a mut W {
        self.variant(COMPSAW::SYNC)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMP_VP_SEL`"]
pub enum COMP_VP_SELW {
    #[doc = "Voltage ladder output"]
    VOLTAGE_LADDER_OUTPU,
    #[doc = "ACMP_I1"]
    ACMP_I1,
    #[doc = "ACMP_I2"]
    ACMP_I2,
    #[doc = "ACMP_I3"]
    ACMP_I3,
    #[doc = "ACMP_I4"]
    ACMP_I4,
    #[doc = "Band gap. Internal reference voltage."]
    BAND_GAP,
    #[doc = "ADC_0. ADC channel 0 input."]
    ADC_0,
}
impl COMP_VP_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMP_VP_SELW::VOLTAGE_LADDER_OUTPU => 0,
            COMP_VP_SELW::ACMP_I1 => 1,
            COMP_VP_SELW::ACMP_I2 => 2,
            COMP_VP_SELW::ACMP_I3 => 3,
            COMP_VP_SELW::ACMP_I4 => 4,
            COMP_VP_SELW::BAND_GAP => 5,
            COMP_VP_SELW::ADC_0 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMP_VP_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_VP_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMP_VP_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Voltage ladder output"]
    #[inline]
    pub fn voltage_ladder_outpu(self) -> &'a mut W {
        self.variant(COMP_VP_SELW::VOLTAGE_LADDER_OUTPU)
    }
    #[doc = "ACMP_I1"]
    #[inline]
    pub fn acmp_i1(self) -> &'a mut W {
        self.variant(COMP_VP_SELW::ACMP_I1)
    }
    #[doc = "ACMP_I2"]
    #[inline]
    pub fn acmp_i2(self) -> &'a mut W {
        self.variant(COMP_VP_SELW::ACMP_I2)
    }
    #[doc = "ACMP_I3"]
    #[inline]
    pub fn acmp_i3(self) -> &'a mut W {
        self.variant(COMP_VP_SELW::ACMP_I3)
    }
    #[doc = "ACMP_I4"]
    #[inline]
    pub fn acmp_i4(self) -> &'a mut W {
        self.variant(COMP_VP_SELW::ACMP_I4)
    }
    #[doc = "Band gap. Internal reference voltage."]
    #[inline]
    pub fn band_gap(self) -> &'a mut W {
        self.variant(COMP_VP_SELW::BAND_GAP)
    }
    #[doc = "ADC_0. ADC channel 0 input."]
    #[inline]
    pub fn adc_0(self) -> &'a mut W {
        self.variant(COMP_VP_SELW::ADC_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMP_VM_SEL`"]
pub enum COMP_VM_SELW {
    #[doc = "Voltage ladder output"]
    VOLTAGE_LADDER_OUTPU,
    #[doc = "ACMP_I1"]
    ACMP_I1,
    #[doc = "ACMP_I2"]
    ACMP_I2,
    #[doc = "ACMP_I3"]
    ACMP_I3,
    #[doc = "ACMP_I4"]
    ACMP_I4,
    #[doc = "Band gap. Internal reference voltage."]
    BAND_GAP,
    #[doc = "ADC_0. ADC channel 0 input."]
    ADC_0,
}
impl COMP_VM_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMP_VM_SELW::VOLTAGE_LADDER_OUTPU => 0,
            COMP_VM_SELW::ACMP_I1 => 1,
            COMP_VM_SELW::ACMP_I2 => 2,
            COMP_VM_SELW::ACMP_I3 => 3,
            COMP_VM_SELW::ACMP_I4 => 4,
            COMP_VM_SELW::BAND_GAP => 5,
            COMP_VM_SELW::ADC_0 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMP_VM_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_VM_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMP_VM_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Voltage ladder output"]
    #[inline]
    pub fn voltage_ladder_outpu(self) -> &'a mut W {
        self.variant(COMP_VM_SELW::VOLTAGE_LADDER_OUTPU)
    }
    #[doc = "ACMP_I1"]
    #[inline]
    pub fn acmp_i1(self) -> &'a mut W {
        self.variant(COMP_VM_SELW::ACMP_I1)
    }
    #[doc = "ACMP_I2"]
    #[inline]
    pub fn acmp_i2(self) -> &'a mut W {
        self.variant(COMP_VM_SELW::ACMP_I2)
    }
    #[doc = "ACMP_I3"]
    #[inline]
    pub fn acmp_i3(self) -> &'a mut W {
        self.variant(COMP_VM_SELW::ACMP_I3)
    }
    #[doc = "ACMP_I4"]
    #[inline]
    pub fn acmp_i4(self) -> &'a mut W {
        self.variant(COMP_VM_SELW::ACMP_I4)
    }
    #[doc = "Band gap. Internal reference voltage."]
    #[inline]
    pub fn band_gap(self) -> &'a mut W {
        self.variant(COMP_VM_SELW::BAND_GAP)
    }
    #[doc = "ADC_0. ADC channel 0 input."]
    #[inline]
    pub fn adc_0(self) -> &'a mut W {
        self.variant(COMP_VM_SELW::ADC_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDGECLRW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGECLRW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPSTATW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPEDGEW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HYS`"]
pub enum HYSW {
    #[doc = "None (the output will switch as the voltages cross)"]
    NONE_THE_OUTPUT_WIL,
    #[doc = "5 mV"]
    _5_MV,
    #[doc = "10 mV"]
    _10_MV,
    #[doc = "20  mV"]
    _20_MV,
}
impl HYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HYSW::NONE_THE_OUTPUT_WIL => 0,
            HYSW::_5_MV => 1,
            HYSW::_10_MV => 2,
            HYSW::_20_MV => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "None (the output will switch as the voltages cross)"]
    #[inline]
    pub fn none_the_output_wil(self) -> &'a mut W {
        self.variant(HYSW::NONE_THE_OUTPUT_WIL)
    }
    #[doc = "5 mV"]
    #[inline]
    pub fn _5_mv(self) -> &'a mut W {
        self.variant(HYSW::_5_MV)
    }
    #[doc = "10 mV"]
    #[inline]
    pub fn _10_mv(self) -> &'a mut W {
        self.variant(HYSW::_10_MV)
    }
    #[doc = "20 mV"]
    #[inline]
    pub fn _20_mv(self) -> &'a mut W {
        self.variant(HYSW::_20_MV)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
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
    #[doc = "Bits 3:4 - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
    #[inline]
    pub fn edgesel(&self) -> EDGESELR {
        EDGESELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Comparator output control"]
    #[inline]
    pub fn compsa(&self) -> COMPSAR {
        COMPSAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Selects positive voltage input"]
    #[inline]
    pub fn comp_vp_sel(&self) -> COMP_VP_SELR {
        COMP_VP_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Selects negative voltage input"]
    #[inline]
    pub fn comp_vm_sel(&self) -> COMP_VM_SELR {
        COMP_VM_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
    #[inline]
    pub fn edgeclr(&self) -> EDGECLRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EDGECLRR { bits }
    }
    #[doc = "Bit 21 - Comparator status. This bit reflects the state of the comparator output."]
    #[inline]
    pub fn compstat(&self) -> COMPSTATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMPSTATR { bits }
    }
    #[doc = "Bit 23 - Comparator edge-detect status."]
    #[inline]
    pub fn compedge(&self) -> COMPEDGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMPEDGER { bits }
    }
    #[doc = "Bits 25:26 - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
    #[inline]
    pub fn hys(&self) -> HYSR {
        HYSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:4 - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
    #[inline]
    pub fn edgesel(&mut self) -> _EDGESELW {
        _EDGESELW { w: self }
    }
    #[doc = "Bit 6 - Comparator output control"]
    #[inline]
    pub fn compsa(&mut self) -> _COMPSAW {
        _COMPSAW { w: self }
    }
    #[doc = "Bits 8:10 - Selects positive voltage input"]
    #[inline]
    pub fn comp_vp_sel(&mut self) -> _COMP_VP_SELW {
        _COMP_VP_SELW { w: self }
    }
    #[doc = "Bits 11:13 - Selects negative voltage input"]
    #[inline]
    pub fn comp_vm_sel(&mut self) -> _COMP_VM_SELW {
        _COMP_VM_SELW { w: self }
    }
    #[doc = "Bit 20 - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
    #[inline]
    pub fn edgeclr(&mut self) -> _EDGECLRW {
        _EDGECLRW { w: self }
    }
    #[doc = "Bit 21 - Comparator status. This bit reflects the state of the comparator output."]
    #[inline]
    pub fn compstat(&mut self) -> _COMPSTATW {
        _COMPSTATW { w: self }
    }
    #[doc = "Bit 23 - Comparator edge-detect status."]
    #[inline]
    pub fn compedge(&mut self) -> _COMPEDGEW {
        _COMPEDGEW { w: self }
    }
    #[doc = "Bits 25:26 - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
    #[inline]
    pub fn hys(&mut self) -> _HYSW {
        _HYSW { w: self }
    }
}
