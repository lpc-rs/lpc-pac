#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTCOSCCTRL {
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
        0x01
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `RTCOSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCOSCENR {
    #[doc = "Disabled. 32 kHz output disabled."]
    DISABLED,
    #[doc = "Enabled. 32 kHz output enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for RTCOSCENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RTCOSCENR::DISABLED => false,
            RTCOSCENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RTCOSCEN_R = crate::FR<bool, RTCOSCENR>;
impl RTCOSCEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCOSCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCOSCENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RTCOSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCOSCENW {
    #[doc = "Disabled. 32 kHz output disabled."]
    DISABLED,
    #[doc = "Enabled. 32 kHz output enabled."]
    ENABLED,
}
impl RTCOSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCOSCENW::DISABLED => false,
            RTCOSCENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RTCOSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCOSCENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCOSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. 32 kHz output disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCOSCENW::DISABLED)
    }
    #[doc = "Enabled. 32 kHz output enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCOSCENW::ENABLED)
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
    #[doc = "Bit 0 - Enable the RTC 32 kHz output."]
    #[inline(always)]
    pub fn rtcoscen(&self) -> RTCOSCEN_R {
        RTCOSCEN_R::new((self.bits() & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable the RTC 32 kHz output."]
    #[inline(always)]
    pub fn rtcoscen(&mut self) -> _RTCOSCENW {
        _RTCOSCENW { w: self }
    }
}
