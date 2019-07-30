#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FMSSTOP {
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
#[doc = r"Reader of the field"]
pub type STOPA_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _STOPAW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type STRTBIST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STRTBISTW<'a> {
    w: &'a mut W,
}
impl<'a> _STRTBISTW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:16 - Stop address for signature generation (the word specified by STOPA is included in the address range). The address is in units of memory words, not bytes. If the option bistprotection=1, bits 2:0 cannot be written and are forced to 111."]
    #[inline(always)]
    pub fn stopa(&self) -> STOPA_R {
        STOPA_R::new((self.bits() & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - When this bit is written to 1, signature generation starts. At the end of signature generation, this bit is automatically cleared."]
    #[inline(always)]
    pub fn strtbist(&self) -> STRTBIST_R {
        STRTBIST_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:16 - Stop address for signature generation (the word specified by STOPA is included in the address range). The address is in units of memory words, not bytes. If the option bistprotection=1, bits 2:0 cannot be written and are forced to 111."]
    #[inline(always)]
    pub fn stopa(&mut self) -> _STOPAW {
        _STOPAW { w: self }
    }
    #[doc = "Bit 17 - When this bit is written to 1, signature generation starts. At the end of signature generation, this bit is automatically cleared."]
    #[inline(always)]
    pub fn strtbist(&mut self) -> _STRTBISTW {
        _STRTBISTW { w: self }
    }
}
