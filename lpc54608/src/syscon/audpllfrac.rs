#[doc = "Reader of register AUDPLLFRAC"]
pub type R = crate::R<u32, super::AUDPLLFRAC>;
#[doc = "Writer for register AUDPLLFRAC"]
pub type W = crate::W<u32, super::AUDPLLFRAC>;
#[doc = "Register AUDPLLFRAC `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDPLLFRAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTRL`"]
pub type CTRL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CTRL`"]
pub struct CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
#[doc = "Reader of field `REQ`"]
pub type REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REQ`"]
pub struct REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SEL_EXT`"]
pub type SEL_EXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEL_EXT`"]
pub struct SEL_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_EXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - PLL fractional divider control word"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 22 - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Select fractional divider."]
    #[inline(always)]
    pub fn sel_ext(&self) -> SEL_EXT_R {
        SEL_EXT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - PLL fractional divider control word"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W {
        CTRL_W { w: self }
    }
    #[doc = "Bit 22 - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
    #[doc = "Bit 23 - Select fractional divider."]
    #[inline(always)]
    pub fn sel_ext(&mut self) -> SEL_EXT_W {
        SEL_EXT_W { w: self }
    }
}
