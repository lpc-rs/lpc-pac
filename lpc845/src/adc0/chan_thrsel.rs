#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHAN_THRSEL {
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
#[doc = "Possible values of the field `CH0_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_THRSELR {
    #[doc = "Threshold 0. Results for this channel will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers."]
    THRESHOLD0,
    #[doc = "Threshold 1. Results for this channel will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers."]
    THRESHOLD1,
}
impl CH0_THRSELR {
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
            CH0_THRSELR::THRESHOLD0 => false,
            CH0_THRSELR::THRESHOLD1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0_THRSELR {
        match value {
            false => CH0_THRSELR::THRESHOLD0,
            true => CH0_THRSELR::THRESHOLD1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD0`"]
    #[inline]
    pub fn is_threshold0(&self) -> bool {
        *self == CH0_THRSELR::THRESHOLD0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD1`"]
    #[inline]
    pub fn is_threshold1(&self) -> bool {
        *self == CH0_THRSELR::THRESHOLD1
    }
}
#[doc = r" Value of the field"]
pub struct CH1_THRSELR {
    bits: bool,
}
impl CH1_THRSELR {
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
pub struct CH2_THRSELR {
    bits: bool,
}
impl CH2_THRSELR {
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
pub struct CH3_THRSELR {
    bits: bool,
}
impl CH3_THRSELR {
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
pub struct CH4_THRSELR {
    bits: bool,
}
impl CH4_THRSELR {
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
pub struct CH5_THRSELR {
    bits: bool,
}
impl CH5_THRSELR {
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
pub struct CH6_THRSELR {
    bits: bool,
}
impl CH6_THRSELR {
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
pub struct CH7_THRSELR {
    bits: bool,
}
impl CH7_THRSELR {
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
pub struct CH8_THRSELR {
    bits: bool,
}
impl CH8_THRSELR {
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
pub struct CH9_THRSELR {
    bits: bool,
}
impl CH9_THRSELR {
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
pub struct CH10_THRSELR {
    bits: bool,
}
impl CH10_THRSELR {
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
pub struct CH11_THRSELR {
    bits: bool,
}
impl CH11_THRSELR {
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
#[doc = "Values that can be written to the field `CH0_THRSEL`"]
pub enum CH0_THRSELW {
    #[doc = "Threshold 0. Results for this channel will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers."]
    THRESHOLD0,
    #[doc = "Threshold 1. Results for this channel will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers."]
    THRESHOLD1,
}
impl CH0_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0_THRSELW::THRESHOLD0 => false,
            CH0_THRSELW::THRESHOLD1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Results for this channel will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers."]
    #[inline]
    pub fn threshold0(self) -> &'a mut W {
        self.variant(CH0_THRSELW::THRESHOLD0)
    }
    #[doc = "Threshold 1. Results for this channel will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers."]
    #[inline]
    pub fn threshold1(self) -> &'a mut W {
        self.variant(CH0_THRSELW::THRESHOLD1)
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
#[doc = r" Proxy"]
pub struct _CH1_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_THRSELW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_THRSELW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH3_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_THRSELW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH4_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_THRSELW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH5_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_THRSELW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH6_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_THRSELW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH7_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7_THRSELW<'a> {
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
pub struct _CH8_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8_THRSELW<'a> {
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
pub struct _CH9_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9_THRSELW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH10_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10_THRSELW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH11_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11_THRSELW<'a> {
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Threshold select for channel 0."]
    #[inline]
    pub fn ch0_thrsel(&self) -> CH0_THRSELR {
        CH0_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Threshold select for channel 1. See description for channel 0."]
    #[inline]
    pub fn ch1_thrsel(&self) -> CH1_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH1_THRSELR { bits }
    }
    #[doc = "Bit 2 - Threshold select for channel 2. See description for channel 0."]
    #[inline]
    pub fn ch2_thrsel(&self) -> CH2_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH2_THRSELR { bits }
    }
    #[doc = "Bit 3 - Threshold select for channel 3. See description for channel 0."]
    #[inline]
    pub fn ch3_thrsel(&self) -> CH3_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH3_THRSELR { bits }
    }
    #[doc = "Bit 4 - Threshold select for channel 4. See description for channel 0."]
    #[inline]
    pub fn ch4_thrsel(&self) -> CH4_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH4_THRSELR { bits }
    }
    #[doc = "Bit 5 - Threshold select for channel 5. See description for channel 0."]
    #[inline]
    pub fn ch5_thrsel(&self) -> CH5_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH5_THRSELR { bits }
    }
    #[doc = "Bit 6 - Threshold select for channel 6. See description for channel 0."]
    #[inline]
    pub fn ch6_thrsel(&self) -> CH6_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH6_THRSELR { bits }
    }
    #[doc = "Bit 7 - Threshold select for channel 7. See description for channel 0."]
    #[inline]
    pub fn ch7_thrsel(&self) -> CH7_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH7_THRSELR { bits }
    }
    #[doc = "Bit 8 - Threshold select for channel 8. See description for channel 0."]
    #[inline]
    pub fn ch8_thrsel(&self) -> CH8_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH8_THRSELR { bits }
    }
    #[doc = "Bit 9 - Threshold select for channel 9. See description for channel 0."]
    #[inline]
    pub fn ch9_thrsel(&self) -> CH9_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH9_THRSELR { bits }
    }
    #[doc = "Bit 10 - Threshold select for channel 10. See description for channel 0."]
    #[inline]
    pub fn ch10_thrsel(&self) -> CH10_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH10_THRSELR { bits }
    }
    #[doc = "Bit 11 - Threshold select for channel 11. See description for channel 0."]
    #[inline]
    pub fn ch11_thrsel(&self) -> CH11_THRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH11_THRSELR { bits }
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
    #[doc = "Bit 0 - Threshold select for channel 0."]
    #[inline]
    pub fn ch0_thrsel(&mut self) -> _CH0_THRSELW {
        _CH0_THRSELW { w: self }
    }
    #[doc = "Bit 1 - Threshold select for channel 1. See description for channel 0."]
    #[inline]
    pub fn ch1_thrsel(&mut self) -> _CH1_THRSELW {
        _CH1_THRSELW { w: self }
    }
    #[doc = "Bit 2 - Threshold select for channel 2. See description for channel 0."]
    #[inline]
    pub fn ch2_thrsel(&mut self) -> _CH2_THRSELW {
        _CH2_THRSELW { w: self }
    }
    #[doc = "Bit 3 - Threshold select for channel 3. See description for channel 0."]
    #[inline]
    pub fn ch3_thrsel(&mut self) -> _CH3_THRSELW {
        _CH3_THRSELW { w: self }
    }
    #[doc = "Bit 4 - Threshold select for channel 4. See description for channel 0."]
    #[inline]
    pub fn ch4_thrsel(&mut self) -> _CH4_THRSELW {
        _CH4_THRSELW { w: self }
    }
    #[doc = "Bit 5 - Threshold select for channel 5. See description for channel 0."]
    #[inline]
    pub fn ch5_thrsel(&mut self) -> _CH5_THRSELW {
        _CH5_THRSELW { w: self }
    }
    #[doc = "Bit 6 - Threshold select for channel 6. See description for channel 0."]
    #[inline]
    pub fn ch6_thrsel(&mut self) -> _CH6_THRSELW {
        _CH6_THRSELW { w: self }
    }
    #[doc = "Bit 7 - Threshold select for channel 7. See description for channel 0."]
    #[inline]
    pub fn ch7_thrsel(&mut self) -> _CH7_THRSELW {
        _CH7_THRSELW { w: self }
    }
    #[doc = "Bit 8 - Threshold select for channel 8. See description for channel 0."]
    #[inline]
    pub fn ch8_thrsel(&mut self) -> _CH8_THRSELW {
        _CH8_THRSELW { w: self }
    }
    #[doc = "Bit 9 - Threshold select for channel 9. See description for channel 0."]
    #[inline]
    pub fn ch9_thrsel(&mut self) -> _CH9_THRSELW {
        _CH9_THRSELW { w: self }
    }
    #[doc = "Bit 10 - Threshold select for channel 10. See description for channel 0."]
    #[inline]
    pub fn ch10_thrsel(&mut self) -> _CH10_THRSELW {
        _CH10_THRSELW { w: self }
    }
    #[doc = "Bit 11 - Threshold select for channel 11. See description for channel 0."]
    #[inline]
    pub fn ch11_thrsel(&mut self) -> _CH11_THRSELW {
        _CH11_THRSELW { w: self }
    }
}
