#[doc = "Writer for register INTTCCLEAR"]
pub type W = crate::W<u32, super::INTTCCLEAR>;
#[doc = "Register INTTCCLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTTCCLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INTTCCLEAR0`"]
pub struct INTTCCLEAR0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR0_W<'a> {
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
#[doc = "Write proxy for field `INTTCCLEAR1`"]
pub struct INTTCCLEAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR1_W<'a> {
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
#[doc = "Write proxy for field `INTTCCLEAR2`"]
pub struct INTTCCLEAR2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR2_W<'a> {
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
#[doc = "Write proxy for field `INTTCCLEAR3`"]
pub struct INTTCCLEAR3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR3_W<'a> {
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
#[doc = "Write proxy for field `INTTCCLEAR4`"]
pub struct INTTCCLEAR4_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR4_W<'a> {
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
#[doc = "Write proxy for field `INTTCCLEAR5`"]
pub struct INTTCCLEAR5_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR5_W<'a> {
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
#[doc = "Write proxy for field `INTTCCLEAR6`"]
pub struct INTTCCLEAR6_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR6_W<'a> {
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
#[doc = "Write proxy for field `INTTCCLEAR7`"]
pub struct INTTCCLEAR7_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear0(&mut self) -> INTTCCLEAR0_W {
        INTTCCLEAR0_W { w: self }
    }
    #[doc = "Bit 1 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear1(&mut self) -> INTTCCLEAR1_W {
        INTTCCLEAR1_W { w: self }
    }
    #[doc = "Bit 2 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear2(&mut self) -> INTTCCLEAR2_W {
        INTTCCLEAR2_W { w: self }
    }
    #[doc = "Bit 3 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear3(&mut self) -> INTTCCLEAR3_W {
        INTTCCLEAR3_W { w: self }
    }
    #[doc = "Bit 4 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear4(&mut self) -> INTTCCLEAR4_W {
        INTTCCLEAR4_W { w: self }
    }
    #[doc = "Bit 5 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear5(&mut self) -> INTTCCLEAR5_W {
        INTTCCLEAR5_W { w: self }
    }
    #[doc = "Bit 6 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear6(&mut self) -> INTTCCLEAR6_W {
        INTTCCLEAR6_W { w: self }
    }
    #[doc = "Bit 7 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear7(&mut self) -> INTTCCLEAR7_W {
        INTTCCLEAR7_W { w: self }
    }
}
