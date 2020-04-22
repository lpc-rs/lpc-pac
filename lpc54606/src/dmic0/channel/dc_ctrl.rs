#[doc = "Reader of register DC_CTRL"]
pub type R = crate::R<u32, super::DC_CTRL>;
#[doc = "Writer for register DC_CTRL"]
pub type W = crate::W<u32, super::DC_CTRL>;
#[doc = "Register DC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DC block filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCPOLE_A {
    #[doc = "0: Flat response, no filter."]
    FLAT_RESPONSE = 0,
    #[doc = "1: 155 Hz."]
    HZ_155 = 1,
    #[doc = "2: 78 Hz."]
    HZ_78 = 2,
    #[doc = "3: 39 Hz"]
    HZ_39 = 3,
}
impl From<DCPOLE_A> for u8 {
    #[inline(always)]
    fn from(variant: DCPOLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCPOLE`"]
pub type DCPOLE_R = crate::R<u8, DCPOLE_A>;
impl DCPOLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCPOLE_A {
        match self.bits {
            0 => DCPOLE_A::FLAT_RESPONSE,
            1 => DCPOLE_A::HZ_155,
            2 => DCPOLE_A::HZ_78,
            3 => DCPOLE_A::HZ_39,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLAT_RESPONSE`"]
    #[inline(always)]
    pub fn is_flat_response(&self) -> bool {
        *self == DCPOLE_A::FLAT_RESPONSE
    }
    #[doc = "Checks if the value of the field is `HZ_155`"]
    #[inline(always)]
    pub fn is_hz_155(&self) -> bool {
        *self == DCPOLE_A::HZ_155
    }
    #[doc = "Checks if the value of the field is `HZ_78`"]
    #[inline(always)]
    pub fn is_hz_78(&self) -> bool {
        *self == DCPOLE_A::HZ_78
    }
    #[doc = "Checks if the value of the field is `HZ_39`"]
    #[inline(always)]
    pub fn is_hz_39(&self) -> bool {
        *self == DCPOLE_A::HZ_39
    }
}
#[doc = "Write proxy for field `DCPOLE`"]
pub struct DCPOLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCPOLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCPOLE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Flat response, no filter."]
    #[inline(always)]
    pub fn flat_response(self) -> &'a mut W {
        self.variant(DCPOLE_A::FLAT_RESPONSE)
    }
    #[doc = "155 Hz."]
    #[inline(always)]
    pub fn hz_155(self) -> &'a mut W {
        self.variant(DCPOLE_A::HZ_155)
    }
    #[doc = "78 Hz."]
    #[inline(always)]
    pub fn hz_78(self) -> &'a mut W {
        self.variant(DCPOLE_A::HZ_78)
    }
    #[doc = "39 Hz"]
    #[inline(always)]
    pub fn hz_39(self) -> &'a mut W {
        self.variant(DCPOLE_A::HZ_39)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DCGAIN`"]
pub type DCGAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCGAIN`"]
pub struct DCGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Selects 16-bit saturation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SATURATEAT16BIT_A {
    #[doc = "0: Results roll over if out range and do not saturate."]
    DO_NOT_SATURATE = 0,
    #[doc = "1: If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    SATURATE = 1,
}
impl From<SATURATEAT16BIT_A> for bool {
    #[inline(always)]
    fn from(variant: SATURATEAT16BIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SATURATEAT16BIT`"]
pub type SATURATEAT16BIT_R = crate::R<bool, SATURATEAT16BIT_A>;
impl SATURATEAT16BIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SATURATEAT16BIT_A {
        match self.bits {
            false => SATURATEAT16BIT_A::DO_NOT_SATURATE,
            true => SATURATEAT16BIT_A::SATURATE,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOT_SATURATE`"]
    #[inline(always)]
    pub fn is_do_not_saturate(&self) -> bool {
        *self == SATURATEAT16BIT_A::DO_NOT_SATURATE
    }
    #[doc = "Checks if the value of the field is `SATURATE`"]
    #[inline(always)]
    pub fn is_saturate(&self) -> bool {
        *self == SATURATEAT16BIT_A::SATURATE
    }
}
#[doc = "Write proxy for field `SATURATEAT16BIT`"]
pub struct SATURATEAT16BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SATURATEAT16BIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SATURATEAT16BIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Results roll over if out range and do not saturate."]
    #[inline(always)]
    pub fn do_not_saturate(self) -> &'a mut W {
        self.variant(SATURATEAT16BIT_A::DO_NOT_SATURATE)
    }
    #[doc = "If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    #[inline(always)]
    pub fn saturate(self) -> &'a mut W {
        self.variant(SATURATEAT16BIT_A::SATURATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DC block filter"]
    #[inline(always)]
    pub fn dcpole(&self) -> DCPOLE_R {
        DCPOLE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Fine gain adjustment in the form of a number of bits to downshift."]
    #[inline(always)]
    pub fn dcgain(&self) -> DCGAIN_R {
        DCGAIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Selects 16-bit saturation."]
    #[inline(always)]
    pub fn saturateat16bit(&self) -> SATURATEAT16BIT_R {
        SATURATEAT16BIT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DC block filter"]
    #[inline(always)]
    pub fn dcpole(&mut self) -> DCPOLE_W {
        DCPOLE_W { w: self }
    }
    #[doc = "Bits 4:7 - Fine gain adjustment in the form of a number of bits to downshift."]
    #[inline(always)]
    pub fn dcgain(&mut self) -> DCGAIN_W {
        DCGAIN_W { w: self }
    }
    #[doc = "Bit 8 - Selects 16-bit saturation."]
    #[inline(always)]
    pub fn saturateat16bit(&mut self) -> SATURATEAT16BIT_W {
        SATURATEAT16BIT_W { w: self }
    }
}
