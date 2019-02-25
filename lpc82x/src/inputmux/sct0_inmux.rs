#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCT0_INMUX {
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
#[doc = "Possible values of the field `INP_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INP_NR {
    #[doc = "SCT_IN0  change the name since this is not the function sct_in0 but the first selection for sct input in the mux.in fact it could be functionally sct_in3 (input 3 of the sct). Assign to pin using the switch matrix."]
    SCT_IN0_CHANGE_THE,
    #[doc = "SCT_IN1. Assign to pin using the switch matrix."]
    SCT_IN1,
    #[doc = "SCT_IN2. Assign to pin using the switch matrix."]
    SCT_IN2,
    #[doc = "SCT_IN3. Assign to pin using the switch matrix."]
    SCT_IN3,
    #[doc = "ADC_THCMP_IRQ"]
    ADC_THCMP_IRQ,
    #[doc = "ACMP_O"]
    ACMP_O,
    #[doc = "ARM_TXEV"]
    ARM_TXEV,
    #[doc = "DEBUG_HALTED"]
    DEBUG_HALTED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INP_NR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INP_NR::SCT_IN0_CHANGE_THE => 0,
            INP_NR::SCT_IN1 => 1,
            INP_NR::SCT_IN2 => 2,
            INP_NR::SCT_IN3 => 3,
            INP_NR::ADC_THCMP_IRQ => 4,
            INP_NR::ACMP_O => 5,
            INP_NR::ARM_TXEV => 6,
            INP_NR::DEBUG_HALTED => 7,
            INP_NR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INP_NR {
        match value {
            0 => INP_NR::SCT_IN0_CHANGE_THE,
            1 => INP_NR::SCT_IN1,
            2 => INP_NR::SCT_IN2,
            3 => INP_NR::SCT_IN3,
            4 => INP_NR::ADC_THCMP_IRQ,
            5 => INP_NR::ACMP_O,
            6 => INP_NR::ARM_TXEV,
            7 => INP_NR::DEBUG_HALTED,
            i => INP_NR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCT_IN0_CHANGE_THE`"]
    #[inline]
    pub fn is_sct_in0_change_the(&self) -> bool {
        *self == INP_NR::SCT_IN0_CHANGE_THE
    }
    #[doc = "Checks if the value of the field is `SCT_IN1`"]
    #[inline]
    pub fn is_sct_in1(&self) -> bool {
        *self == INP_NR::SCT_IN1
    }
    #[doc = "Checks if the value of the field is `SCT_IN2`"]
    #[inline]
    pub fn is_sct_in2(&self) -> bool {
        *self == INP_NR::SCT_IN2
    }
    #[doc = "Checks if the value of the field is `SCT_IN3`"]
    #[inline]
    pub fn is_sct_in3(&self) -> bool {
        *self == INP_NR::SCT_IN3
    }
    #[doc = "Checks if the value of the field is `ADC_THCMP_IRQ`"]
    #[inline]
    pub fn is_adc_thcmp_irq(&self) -> bool {
        *self == INP_NR::ADC_THCMP_IRQ
    }
    #[doc = "Checks if the value of the field is `ACMP_O`"]
    #[inline]
    pub fn is_acmp_o(&self) -> bool {
        *self == INP_NR::ACMP_O
    }
    #[doc = "Checks if the value of the field is `ARM_TXEV`"]
    #[inline]
    pub fn is_arm_txev(&self) -> bool {
        *self == INP_NR::ARM_TXEV
    }
    #[doc = "Checks if the value of the field is `DEBUG_HALTED`"]
    #[inline]
    pub fn is_debug_halted(&self) -> bool {
        *self == INP_NR::DEBUG_HALTED
    }
}
#[doc = "Values that can be written to the field `INP_N`"]
pub enum INP_NW {
    #[doc = "SCT_IN0  change the name since this is not the function sct_in0 but the first selection for sct input in the mux.in fact it could be functionally sct_in3 (input 3 of the sct). Assign to pin using the switch matrix."]
    SCT_IN0_CHANGE_THE,
    #[doc = "SCT_IN1. Assign to pin using the switch matrix."]
    SCT_IN1,
    #[doc = "SCT_IN2. Assign to pin using the switch matrix."]
    SCT_IN2,
    #[doc = "SCT_IN3. Assign to pin using the switch matrix."]
    SCT_IN3,
    #[doc = "ADC_THCMP_IRQ"]
    ADC_THCMP_IRQ,
    #[doc = "ACMP_O"]
    ACMP_O,
    #[doc = "ARM_TXEV"]
    ARM_TXEV,
    #[doc = "DEBUG_HALTED"]
    DEBUG_HALTED,
}
impl INP_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INP_NW::SCT_IN0_CHANGE_THE => 0,
            INP_NW::SCT_IN1 => 1,
            INP_NW::SCT_IN2 => 2,
            INP_NW::SCT_IN3 => 3,
            INP_NW::ADC_THCMP_IRQ => 4,
            INP_NW::ACMP_O => 5,
            INP_NW::ARM_TXEV => 6,
            INP_NW::DEBUG_HALTED => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INP_NW<'a> {
    w: &'a mut W,
}
impl<'a> _INP_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INP_NW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SCT_IN0 change the name since this is not the function sct_in0 but the first selection for sct input in the mux.in fact it could be functionally sct_in3 (input 3 of the sct). Assign to pin using the switch matrix."]
    #[inline]
    pub fn sct_in0_change_the(self) -> &'a mut W {
        self.variant(INP_NW::SCT_IN0_CHANGE_THE)
    }
    #[doc = "SCT_IN1. Assign to pin using the switch matrix."]
    #[inline]
    pub fn sct_in1(self) -> &'a mut W {
        self.variant(INP_NW::SCT_IN1)
    }
    #[doc = "SCT_IN2. Assign to pin using the switch matrix."]
    #[inline]
    pub fn sct_in2(self) -> &'a mut W {
        self.variant(INP_NW::SCT_IN2)
    }
    #[doc = "SCT_IN3. Assign to pin using the switch matrix."]
    #[inline]
    pub fn sct_in3(self) -> &'a mut W {
        self.variant(INP_NW::SCT_IN3)
    }
    #[doc = "ADC_THCMP_IRQ"]
    #[inline]
    pub fn adc_thcmp_irq(self) -> &'a mut W {
        self.variant(INP_NW::ADC_THCMP_IRQ)
    }
    #[doc = "ACMP_O"]
    #[inline]
    pub fn acmp_o(self) -> &'a mut W {
        self.variant(INP_NW::ACMP_O)
    }
    #[doc = "ARM_TXEV"]
    #[inline]
    pub fn arm_txev(self) -> &'a mut W {
        self.variant(INP_NW::ARM_TXEV)
    }
    #[doc = "DEBUG_HALTED"]
    #[inline]
    pub fn debug_halted(self) -> &'a mut W {
        self.variant(INP_NW::DEBUG_HALTED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Input number (decimal value) to SCT0 inputs 0 to 3."]
    #[inline]
    pub fn inp_n(&self) -> INP_NR {
        INP_NR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Input number (decimal value) to SCT0 inputs 0 to 3."]
    #[inline]
    pub fn inp_n(&mut self) -> _INP_NW {
        _INP_NW { w: self }
    }
}
