#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATE {
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
pub type STATE_L_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _STATE_LW<'a> {
    w: &'a mut W,
}
impl<'a> _STATE_LW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type STATE_H_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _STATE_HW<'a> {
    w: &'a mut W,
}
impl<'a> _STATE_HW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - State variable."]
    #[inline(always)]
    pub fn state_l(&self) -> STATE_L_R {
        STATE_L_R::new((self.bits() & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - State variable."]
    #[inline(always)]
    pub fn state_h(&self) -> STATE_H_R {
        STATE_H_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - State variable."]
    #[inline(always)]
    pub fn state_l(&mut self) -> _STATE_LW {
        _STATE_LW { w: self }
    }
    #[doc = "Bits 16:20 - State variable."]
    #[inline(always)]
    pub fn state_h(&mut self) -> _STATE_HW {
        _STATE_HW { w: self }
    }
}
