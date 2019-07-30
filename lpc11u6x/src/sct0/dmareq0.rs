#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMAREQ0 {
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
pub type DEV_0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DEV_0W<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DRL0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRL0W<'a> {
    w: &'a mut W,
}
impl<'a> _DRL0W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DRQ0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _DRQ0W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..., event 5 = bit 5)."]
    #[inline(always)]
    pub fn dev_0(&self) -> DEV_0_R {
        DEV_0_R::new((self.bits() & 0x3f) as u8)
    }
    #[doc = "Bit 30 - A 1 in this bit makes the SCT set DMA request 0 when it loads the Match_L/Unified registers from the Reload_L/Unified registers."]
    #[inline(always)]
    pub fn drl0(&self) -> DRL0_R {
        DRL0_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0"]
    #[inline(always)]
    pub fn drq0(&self) -> DRQ0_R {
        DRQ0_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..., event 5 = bit 5)."]
    #[inline(always)]
    pub fn dev_0(&mut self) -> _DEV_0W {
        _DEV_0W { w: self }
    }
    #[doc = "Bit 30 - A 1 in this bit makes the SCT set DMA request 0 when it loads the Match_L/Unified registers from the Reload_L/Unified registers."]
    #[inline(always)]
    pub fn drl0(&mut self) -> _DRL0W {
        _DRL0W { w: self }
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0"]
    #[inline(always)]
    pub fn drq0(&mut self) -> _DRQ0W {
        _DRQ0W { w: self }
    }
}
