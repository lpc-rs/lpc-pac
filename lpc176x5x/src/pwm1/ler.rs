#[doc = "Reader of register LER"]
pub type R = crate::R<u32, super::LER>;
#[doc = "Writer for register LER"]
pub type W = crate::W<u32, super::LER>;
#[doc = "Register LER `reset()`'s with value 0"]
impl crate::ResetValue for super::LER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAT0LATCHEN`"]
pub type MAT0LATCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAT0LATCHEN`"]
pub struct MAT0LATCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAT0LATCHEN_W<'a> {
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
#[doc = "Reader of field `MAT1LATCHEN`"]
pub type MAT1LATCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAT1LATCHEN`"]
pub struct MAT1LATCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAT1LATCHEN_W<'a> {
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
#[doc = "Reader of field `MAT2LATCHEN`"]
pub type MAT2LATCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAT2LATCHEN`"]
pub struct MAT2LATCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAT2LATCHEN_W<'a> {
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
#[doc = "Reader of field `MAT3LATCHEN`"]
pub type MAT3LATCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAT3LATCHEN`"]
pub struct MAT3LATCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAT3LATCHEN_W<'a> {
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
#[doc = "Reader of field `MAT4LATCHEN`"]
pub type MAT4LATCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAT4LATCHEN`"]
pub struct MAT4LATCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAT4LATCHEN_W<'a> {
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
#[doc = "Reader of field `MAT5LATCHEN`"]
pub type MAT5LATCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAT5LATCHEN`"]
pub struct MAT5LATCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAT5LATCHEN_W<'a> {
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
#[doc = "Reader of field `MAT6LATCHEN`"]
pub type MAT6LATCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAT6LATCHEN`"]
pub struct MAT6LATCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAT6LATCHEN_W<'a> {
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
    #[doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
    #[inline(always)]
    pub fn mat0latchen(&self) -> MAT0LATCHEN_R {
        MAT0LATCHEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat1latchen(&self) -> MAT1LATCHEN_R {
        MAT1LATCHEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat2latchen(&self) -> MAT2LATCHEN_R {
        MAT2LATCHEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat3latchen(&self) -> MAT3LATCHEN_R {
        MAT3LATCHEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat4latchen(&self) -> MAT4LATCHEN_R {
        MAT4LATCHEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat5latchen(&self) -> MAT5LATCHEN_R {
        MAT5LATCHEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat6latchen(&self) -> MAT6LATCHEN_R {
        MAT6LATCHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
    #[inline(always)]
    pub fn mat0latchen(&mut self) -> MAT0LATCHEN_W {
        MAT0LATCHEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat1latchen(&mut self) -> MAT1LATCHEN_W {
        MAT1LATCHEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat2latchen(&mut self) -> MAT2LATCHEN_W {
        MAT2LATCHEN_W { w: self }
    }
    #[doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat3latchen(&mut self) -> MAT3LATCHEN_W {
        MAT3LATCHEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat4latchen(&mut self) -> MAT4LATCHEN_W {
        MAT4LATCHEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat5latchen(&mut self) -> MAT5LATCHEN_W {
        MAT5LATCHEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat6latchen(&mut self) -> MAT6LATCHEN_W {
        MAT6LATCHEN_W { w: self }
    }
}
