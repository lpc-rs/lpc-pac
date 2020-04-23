#[doc = "Reader of register HWVADHPFS"]
pub type R = crate::R<u32, super::HWVADHPFS>;
#[doc = "Writer for register HWVADHPFS"]
pub type W = crate::W<u32, super::HWVADHPFS>;
#[doc = "Register HWVADHPFS `reset()`'s with value 0x01"]
impl crate::ResetValue for super::HWVADHPFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "High pass filter\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPFS_A {
    #[doc = "0: First filter by-pass."]
    BYPASS = 0,
    #[doc = "1: High pass filter with -3dB cut-off at 1750Hz."]
    HIGH_PASS_1750HZ = 1,
    #[doc = "2: High pass filter with -3dB cut-off at 215Hz."]
    HIGH_PASS_215HZ = 2,
}
impl From<HPFS_A> for u8 {
    #[inline(always)]
    fn from(variant: HPFS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HPFS`"]
pub type HPFS_R = crate::R<u8, HPFS_A>;
impl HPFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HPFS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HPFS_A::BYPASS),
            1 => Val(HPFS_A::HIGH_PASS_1750HZ),
            2 => Val(HPFS_A::HIGH_PASS_215HZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == HPFS_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `HIGH_PASS_1750HZ`"]
    #[inline(always)]
    pub fn is_high_pass_1750hz(&self) -> bool {
        *self == HPFS_A::HIGH_PASS_1750HZ
    }
    #[doc = "Checks if the value of the field is `HIGH_PASS_215HZ`"]
    #[inline(always)]
    pub fn is_high_pass_215hz(&self) -> bool {
        *self == HPFS_A::HIGH_PASS_215HZ
    }
}
#[doc = "Write proxy for field `HPFS`"]
pub struct HPFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HPFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPFS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "First filter by-pass."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(HPFS_A::BYPASS)
    }
    #[doc = "High pass filter with -3dB cut-off at 1750Hz."]
    #[inline(always)]
    pub fn high_pass_1750hz(self) -> &'a mut W {
        self.variant(HPFS_A::HIGH_PASS_1750HZ)
    }
    #[doc = "High pass filter with -3dB cut-off at 215Hz."]
    #[inline(always)]
    pub fn high_pass_215hz(self) -> &'a mut W {
        self.variant(HPFS_A::HIGH_PASS_215HZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - High pass filter"]
    #[inline(always)]
    pub fn hpfs(&self) -> HPFS_R {
        HPFS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High pass filter"]
    #[inline(always)]
    pub fn hpfs(&mut self) -> HPFS_W {
        HPFS_W { w: self }
    }
}
