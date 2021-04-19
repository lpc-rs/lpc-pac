#[doc = "Register `NOT%s` writer"]
pub struct W(crate::W<NOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<NOT_SPEC>> for W {
    fn from(writer: crate::W<NOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOTP0` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP0_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `NOTP1` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP1_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `NOTP2` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP2_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `NOTP3` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP3_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `NOTP4` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP4_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `NOTP5` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP5_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `NOTP6` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP6_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `NOTP7` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP7_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `NOTP8` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP8_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `NOTP9` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP9_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `NOTP10` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP10_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `NOTP11` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP11_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `NOTP12` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP12_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `NOTP13` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP13_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `NOTP14` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP14_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `NOTP15` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP15_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `NOTP16` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP16_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `NOTP17` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP17_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `NOTP18` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP18_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `NOTP19` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP19_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `NOTP20` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP20_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `NOTP21` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP21_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `NOTP22` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP22_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `NOTP23` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP23_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `NOTP24` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP24_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `NOTP25` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP25_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `NOTP26` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP26_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `NOTP27` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP27_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `NOTP28` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP28_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `NOTP29` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP29_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `NOTP30` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP30_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `NOTP31` writer - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
pub struct NOTP31_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp0(&mut self) -> NOTP0_W {
        NOTP0_W { w: self }
    }
    #[doc = "Bit 1 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp1(&mut self) -> NOTP1_W {
        NOTP1_W { w: self }
    }
    #[doc = "Bit 2 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp2(&mut self) -> NOTP2_W {
        NOTP2_W { w: self }
    }
    #[doc = "Bit 3 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp3(&mut self) -> NOTP3_W {
        NOTP3_W { w: self }
    }
    #[doc = "Bit 4 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp4(&mut self) -> NOTP4_W {
        NOTP4_W { w: self }
    }
    #[doc = "Bit 5 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp5(&mut self) -> NOTP5_W {
        NOTP5_W { w: self }
    }
    #[doc = "Bit 6 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp6(&mut self) -> NOTP6_W {
        NOTP6_W { w: self }
    }
    #[doc = "Bit 7 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp7(&mut self) -> NOTP7_W {
        NOTP7_W { w: self }
    }
    #[doc = "Bit 8 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp8(&mut self) -> NOTP8_W {
        NOTP8_W { w: self }
    }
    #[doc = "Bit 9 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp9(&mut self) -> NOTP9_W {
        NOTP9_W { w: self }
    }
    #[doc = "Bit 10 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp10(&mut self) -> NOTP10_W {
        NOTP10_W { w: self }
    }
    #[doc = "Bit 11 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp11(&mut self) -> NOTP11_W {
        NOTP11_W { w: self }
    }
    #[doc = "Bit 12 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp12(&mut self) -> NOTP12_W {
        NOTP12_W { w: self }
    }
    #[doc = "Bit 13 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp13(&mut self) -> NOTP13_W {
        NOTP13_W { w: self }
    }
    #[doc = "Bit 14 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp14(&mut self) -> NOTP14_W {
        NOTP14_W { w: self }
    }
    #[doc = "Bit 15 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp15(&mut self) -> NOTP15_W {
        NOTP15_W { w: self }
    }
    #[doc = "Bit 16 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp16(&mut self) -> NOTP16_W {
        NOTP16_W { w: self }
    }
    #[doc = "Bit 17 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp17(&mut self) -> NOTP17_W {
        NOTP17_W { w: self }
    }
    #[doc = "Bit 18 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp18(&mut self) -> NOTP18_W {
        NOTP18_W { w: self }
    }
    #[doc = "Bit 19 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp19(&mut self) -> NOTP19_W {
        NOTP19_W { w: self }
    }
    #[doc = "Bit 20 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp20(&mut self) -> NOTP20_W {
        NOTP20_W { w: self }
    }
    #[doc = "Bit 21 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp21(&mut self) -> NOTP21_W {
        NOTP21_W { w: self }
    }
    #[doc = "Bit 22 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp22(&mut self) -> NOTP22_W {
        NOTP22_W { w: self }
    }
    #[doc = "Bit 23 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp23(&mut self) -> NOTP23_W {
        NOTP23_W { w: self }
    }
    #[doc = "Bit 24 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp24(&mut self) -> NOTP24_W {
        NOTP24_W { w: self }
    }
    #[doc = "Bit 25 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp25(&mut self) -> NOTP25_W {
        NOTP25_W { w: self }
    }
    #[doc = "Bit 26 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp26(&mut self) -> NOTP26_W {
        NOTP26_W { w: self }
    }
    #[doc = "Bit 27 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp27(&mut self) -> NOTP27_W {
        NOTP27_W { w: self }
    }
    #[doc = "Bit 28 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp28(&mut self) -> NOTP28_W {
        NOTP28_W { w: self }
    }
    #[doc = "Bit 29 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp29(&mut self) -> NOTP29_W {
        NOTP29_W { w: self }
    }
    #[doc = "Bit 30 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp30(&mut self) -> NOTP30_W {
        NOTP30_W { w: self }
    }
    #[doc = "Bit 31 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp31(&mut self) -> NOTP31_W {
        NOTP31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Toggle port 0/1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [not](index.html) module"]
pub struct NOT_SPEC;
impl crate::RegisterSpec for NOT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [not::W](W) writer structure"]
impl crate::Writable for NOT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NOT%s to value 0"]
impl crate::Resettable for NOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
