#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTSETSTAT {
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
pub type EP_SET_INT_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _EP_SET_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_SET_INTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FRAME_SET_INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FRAME_SET_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAME_SET_INTW<'a> {
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
pub type DEV_SET_INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DEV_SET_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_SET_INTW<'a> {
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
    #[doc = "Bits 0:9 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn ep_set_int(&self) -> EP_SET_INT_R {
        EP_SET_INT_R::new((self.bits() & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn frame_set_int(&self) -> FRAME_SET_INT_R {
        FRAME_SET_INT_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn dev_set_int(&self) -> DEV_SET_INT_R {
        DEV_SET_INT_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn ep_set_int(&mut self) -> _EP_SET_INTW {
        _EP_SET_INTW { w: self }
    }
    #[doc = "Bit 30 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn frame_set_int(&mut self) -> _FRAME_SET_INTW {
        _FRAME_SET_INTW { w: self }
    }
    #[doc = "Bit 31 - If software writes a one to one of these bits, the corresponding USB interrupt status bit is set. When this register is read, the same value as the USB interrupt status register is returned."]
    #[inline(always)]
    pub fn dev_set_int(&mut self) -> _DEV_SET_INTW {
        _DEV_SET_INTW { w: self }
    }
}
