#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKOUTUEN {
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
#[doc = "Possible values of the field `ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENAR {
    #[doc = "No change"]
    NO_CHANGE,
    #[doc = "Update clock source"]
    UPDATE_CLOCK_SOURCE,
}
impl crate::ToBits<bool> for ENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ENAR::NO_CHANGE => false,
            ENAR::UPDATE_CLOCK_SOURCE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENA_R = crate::FR<bool, ENAR>;
impl ENA_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == ENAR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `UPDATE_CLOCK_SOURCE`"]
    #[inline(always)]
    pub fn is_update_clock_source(&self) -> bool {
        *self == ENAR::UPDATE_CLOCK_SOURCE
    }
}
#[doc = "Values that can be written to the field `ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENAW {
    #[doc = "No change"]
    NO_CHANGE,
    #[doc = "Update clock source"]
    UPDATE_CLOCK_SOURCE,
}
impl ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ENAW::NO_CHANGE => false,
            ENAW::UPDATE_CLOCK_SOURCE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(ENAW::NO_CHANGE)
    }
    #[doc = "Update clock source"]
    #[inline(always)]
    pub fn update_clock_source(self) -> &'a mut W {
        self.variant(ENAW::UPDATE_CLOCK_SOURCE)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable CLKOUT clock source update"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits() & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable CLKOUT clock source update"]
    #[inline(always)]
    pub fn ena(&mut self) -> _ENAW {
        _ENAW { w: self }
    }
}
