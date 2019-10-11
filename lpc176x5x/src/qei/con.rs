#[doc = "Writer for register CON"]
pub type W = crate::W<u32, super::CON>;
#[doc = "Register CON `reset()`'s with value 0"]
impl crate::ResetValue for super::CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RESP`"]
pub struct RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_W<'a> {
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
#[doc = "Write proxy for field `RESPI`"]
pub struct RESPI_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPI_W<'a> {
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
#[doc = "Write proxy for field `RESV`"]
pub struct RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> RESV_W<'a> {
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
#[doc = "Write proxy for field `RESI`"]
pub struct RESI_W<'a> {
    w: &'a mut W,
}
impl<'a> RESI_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Reset position counter. When set = 1, resets the position counter to all zeros. Autoclears when the position counter is cleared."]
    #[inline(always)]
    pub fn resp(&mut self) -> RESP_W {
        RESP_W { w: self }
    }
    #[doc = "Bit 1 - Reset position counter on index. When set = 1, resets the position counter to all zeros once only the first time an index pulse occurs. Autoclears when the position counter is cleared."]
    #[inline(always)]
    pub fn respi(&mut self) -> RESPI_W {
        RESPI_W { w: self }
    }
    #[doc = "Bit 2 - Reset velocity. When set = 1, resets the velocity counter to all zeros, reloads the velocity timer, and presets the velocity compare register. Autoclears when the velocity counter is cleared."]
    #[inline(always)]
    pub fn resv(&mut self) -> RESV_W {
        RESV_W { w: self }
    }
    #[doc = "Bit 3 - Reset index counter. When set = 1, resets the index counter to all zeros. Autoclears when the index counter is cleared."]
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W {
        RESI_W { w: self }
    }
}
