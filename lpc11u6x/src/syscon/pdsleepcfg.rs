#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDSLEEPCFG {
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
#[doc = "Possible values of the field `BOD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for BOD_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BOD_PDR::POWERED_DOWN => true,
            BOD_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BOD_PD_R = crate::FR<bool, BOD_PDR>;
impl BOD_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `BOD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl BOD_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BOD_PDW::POWERED_DOWN => true,
            BOD_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BOD_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED)
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
#[doc = "Possible values of the field `WDTOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for WDTOSC_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDTOSC_PDR::POWERED_DOWN => true,
            WDTOSC_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDTOSC_PD_R = crate::FR<bool, WDTOSC_PDR>;
impl WDTOSC_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == WDTOSC_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == WDTOSC_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `WDTOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl WDTOSC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTOSC_PDW::POWERED_DOWN => true,
            WDTOSC_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDTOSC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTOSC_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTOSC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::POWERED)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - BOD power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - BOD power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> _BOD_PDW {
        _BOD_PDW { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> _WDTOSC_PDW {
        _WDTOSC_PDW { w: self }
    }
}
