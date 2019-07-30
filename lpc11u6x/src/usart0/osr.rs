#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSR {
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
        0xf0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type OSFRAC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _OSFRACW<'a> {
    w: &'a mut W,
}
impl<'a> _OSFRACW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type OSINT_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _OSINTW<'a> {
    w: &'a mut W,
}
impl<'a> _OSINTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FDINT_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FDINTW<'a> {
    w: &'a mut W,
}
impl<'a> _FDINTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
    #[inline(always)]
    pub fn osfrac(&self) -> OSFRAC_R {
        OSFRAC_R::new(((self.bits() >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
    #[inline(always)]
    pub fn osint(&self) -> OSINT_R {
        OSINT_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - In Smart Card mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In Smart Card mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
    #[inline(always)]
    pub fn fdint(&self) -> FDINT_R {
        FDINT_R::new(((self.bits() >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
    #[inline(always)]
    pub fn osfrac(&mut self) -> _OSFRACW {
        _OSFRACW { w: self }
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
    #[inline(always)]
    pub fn osint(&mut self) -> _OSINTW {
        _OSINTW { w: self }
    }
    #[doc = "Bits 8:14 - In Smart Card mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In Smart Card mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
    #[inline(always)]
    pub fn fdint(&mut self) -> _FDINTW {
        _FDINTW { w: self }
    }
}
