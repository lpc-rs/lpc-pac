#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRM {
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
        0x0f00
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `VRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRANGER {
    #[doc = "High voltage. VDDA = 2.7 V to 3.6 V."]
    HIGH_VOLTAGE,
    #[doc = "Low voltage. VDDA = 1.8 V to 2.7 V."]
    LOW_VOLTAGE,
}
impl crate::ToBits<bool> for VRANGER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            VRANGER::HIGH_VOLTAGE => false,
            VRANGER::LOW_VOLTAGE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type VRANGE_R = crate::FR<bool, VRANGER>;
impl VRANGE_R {
    #[doc = "Checks if the value of the field is `HIGH_VOLTAGE`"]
    #[inline(always)]
    pub fn is_high_voltage(&self) -> bool {
        *self == VRANGER::HIGH_VOLTAGE
    }
    #[doc = "Checks if the value of the field is `LOW_VOLTAGE`"]
    #[inline(always)]
    pub fn is_low_voltage(&self) -> bool {
        *self == VRANGER::LOW_VOLTAGE
    }
}
#[doc = "Values that can be written to the field `VRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRANGEW {
    #[doc = "High voltage. VDDA = 2.7 V to 3.6 V."]
    HIGH_VOLTAGE,
    #[doc = "Low voltage. VDDA = 1.8 V to 2.7 V."]
    LOW_VOLTAGE,
}
impl VRANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            VRANGEW::HIGH_VOLTAGE => false,
            VRANGEW::LOW_VOLTAGE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _VRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _VRANGEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VRANGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High voltage. VDDA = 2.7 V to 3.6 V."]
    #[inline(always)]
    pub fn high_voltage(self) -> &'a mut W {
        self.variant(VRANGEW::HIGH_VOLTAGE)
    }
    #[doc = "Low voltage. VDDA = 1.8 V to 2.7 V."]
    #[inline(always)]
    pub fn low_voltage(self) -> &'a mut W {
        self.variant(VRANGEW::LOW_VOLTAGE)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 5 - Reserved."]
    #[inline(always)]
    pub fn vrange(&self) -> VRANGE_R {
        VRANGE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 5 - Reserved."]
    #[inline(always)]
    pub fn vrange(&mut self) -> _VRANGEW {
        _VRANGEW { w: self }
    }
}
