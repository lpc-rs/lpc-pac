#[doc = "Reader of register FMSSTOP"]
pub type R = crate::R<u32, super::FMSSTOP>;
#[doc = "Writer for register FMSSTOP"]
pub type W = crate::W<u32, super::FMSSTOP>;
#[doc = "Register FMSSTOP `reset()`'s with value 0"]
impl crate::ResetValue for super::FMSSTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Start control bit for signature generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIG_START_A {
    #[doc = "0: Signature generation is stopped"]
    SIGNATURE_GENERATION = 0,
    #[doc = "1: Initiate signature generation"]
    INITIATE_SIGNATURE_G = 1,
}
impl From<SIG_START_A> for bool {
    #[inline(always)]
    fn from(variant: SIG_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SIG_START`"]
pub type SIG_START_R = crate::R<bool, SIG_START_A>;
impl SIG_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIG_START_A {
        match self.bits {
            false => SIG_START_A::SIGNATURE_GENERATION,
            true => SIG_START_A::INITIATE_SIGNATURE_G,
        }
    }
    #[doc = "Checks if the value of the field is `SIGNATURE_GENERATION`"]
    #[inline(always)]
    pub fn is_signature_generation(&self) -> bool {
        *self == SIG_START_A::SIGNATURE_GENERATION
    }
    #[doc = "Checks if the value of the field is `INITIATE_SIGNATURE_G`"]
    #[inline(always)]
    pub fn is_initiate_signature_g(&self) -> bool {
        *self == SIG_START_A::INITIATE_SIGNATURE_G
    }
}
#[doc = "Write proxy for field `SIG_START`"]
pub struct SIG_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIG_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Signature generation is stopped"]
    #[inline(always)]
    pub fn signature_generation(self) -> &'a mut W {
        self.variant(SIG_START_A::SIGNATURE_GENERATION)
    }
    #[doc = "Initiate signature generation"]
    #[inline(always)]
    pub fn initiate_signature_g(self) -> &'a mut W {
        self.variant(SIG_START_A::INITIATE_SIGNATURE_G)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline(always)]
    pub fn sig_start(&self) -> SIG_START_R {
        SIG_START_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline(always)]
    pub fn sig_start(&mut self) -> SIG_START_W {
        SIG_START_W { w: self }
    }
}
