#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IR {
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
pub type MR0INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0INTW<'a> {
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
#[doc = r"Reader of the field"]
pub type MR1INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1INTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR2INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR2INTW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2INTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR3INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR3INTW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3INTW<'a> {
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
#[doc = r"Reader of the field"]
pub type CR0INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CR0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CR0INTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CR1INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CR1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CR1INTW<'a> {
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
#[doc = r"Reader of the field"]
pub type CR2INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CR2INTW<'a> {
    w: &'a mut W,
}
impl<'a> _CR2INTW<'a> {
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
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&self) -> MR0INT_R {
        MR0INT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&self) -> MR1INT_R {
        MR1INT_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&self) -> MR2INT_R {
        MR2INT_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&self) -> MR3INT_R {
        MR3INT_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&self) -> CR0INT_R {
        CR0INT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&self) -> CR1INT_R {
        CR1INT_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt flag for capture channel 2 event."]
    #[inline(always)]
    pub fn cr2int(&self) -> CR2INT_R {
        CR2INT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&mut self) -> _MR0INTW {
        _MR0INTW { w: self }
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&mut self) -> _MR1INTW {
        _MR1INTW { w: self }
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&mut self) -> _MR2INTW {
        _MR2INTW { w: self }
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&mut self) -> _MR3INTW {
        _MR3INTW { w: self }
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&mut self) -> _CR0INTW {
        _CR0INTW { w: self }
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&mut self) -> _CR1INTW {
        _CR1INTW { w: self }
    }
    #[doc = "Bit 6 - Interrupt flag for capture channel 2 event."]
    #[inline(always)]
    pub fn cr2int(&mut self) -> _CR2INTW {
        _CR2INTW { w: self }
    }
}
