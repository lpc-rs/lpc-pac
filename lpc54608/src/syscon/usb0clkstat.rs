#[doc = "Reader of register USB0CLKSTAT"]
pub type R = crate::R<u32, super::USB0CLKSTAT>;
#[doc = "Writer for register USB0CLKSTAT"]
pub type W = crate::W<u32, super::USB0CLKSTAT>;
#[doc = "Register USB0CLKSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::USB0CLKSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEV_NEED_CLKST`"]
pub type DEV_NEED_CLKST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_NEED_CLKST`"]
pub struct DEV_NEED_CLKST_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_NEED_CLKST_W<'a> {
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
#[doc = "Reader of field `HOST_NEED_CLKST`"]
pub type HOST_NEED_CLKST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_NEED_CLKST`"]
pub struct HOST_NEED_CLKST_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_NEED_CLKST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal status."]
    #[inline(always)]
    pub fn dev_need_clkst(&self) -> DEV_NEED_CLKST_R {
        DEV_NEED_CLKST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB0 Host USB0_NEEDCLK signal status."]
    #[inline(always)]
    pub fn host_need_clkst(&self) -> HOST_NEED_CLKST_R {
        HOST_NEED_CLKST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal status."]
    #[inline(always)]
    pub fn dev_need_clkst(&mut self) -> DEV_NEED_CLKST_W {
        DEV_NEED_CLKST_W { w: self }
    }
    #[doc = "Bit 1 - USB0 Host USB0_NEEDCLK signal status."]
    #[inline(always)]
    pub fn host_need_clkst(&mut self) -> HOST_NEED_CLKST_W {
        HOST_NEED_CLKST_W { w: self }
    }
}
