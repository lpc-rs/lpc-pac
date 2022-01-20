#[doc = "Register `DC_CTRL` reader"]
pub struct R(crate::R<DC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_CTRL` writer"]
pub struct W(crate::W<DC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_CTRL_SPEC>;
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
impl From<crate::W<DC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_CTRL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `DCPOLE` reader - DC block filter"]
pub struct DCPOLE_R(crate::FieldReader<u8, DCPOLE_A>);
impl DCPOLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCPOLE_R(crate::FieldReader::new(bits))
    }
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
        **self == DCPOLE_A::FLAT_RESPONSE
    }
    #[doc = "Checks if the value of the field is `HZ_155`"]
    #[inline(always)]
    pub fn is_hz_155(&self) -> bool {
        **self == DCPOLE_A::HZ_155
    }
    #[doc = "Checks if the value of the field is `HZ_78`"]
    #[inline(always)]
    pub fn is_hz_78(&self) -> bool {
        **self == DCPOLE_A::HZ_78
    }
    #[doc = "Checks if the value of the field is `HZ_39`"]
    #[inline(always)]
    pub fn is_hz_39(&self) -> bool {
        **self == DCPOLE_A::HZ_39
    }
}
impl core::ops::Deref for DCPOLE_R {
    type Target = crate::FieldReader<u8, DCPOLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCPOLE` writer - DC block filter"]
pub struct DCPOLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCPOLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCPOLE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DCGAIN` reader - Fine gain adjustment in the form of a number of bits to downshift."]
pub struct DCGAIN_R(crate::FieldReader<u8, u8>);
impl DCGAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCGAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCGAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCGAIN` writer - Fine gain adjustment in the form of a number of bits to downshift."]
pub struct DCGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
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
#[doc = "Field `SATURATEAT16BIT` reader - Selects 16-bit saturation."]
pub struct SATURATEAT16BIT_R(crate::FieldReader<bool, SATURATEAT16BIT_A>);
impl SATURATEAT16BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SATURATEAT16BIT_R(crate::FieldReader::new(bits))
    }
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
        **self == SATURATEAT16BIT_A::DO_NOT_SATURATE
    }
    #[doc = "Checks if the value of the field is `SATURATE`"]
    #[inline(always)]
    pub fn is_saturate(&self) -> bool {
        **self == SATURATEAT16BIT_A::SATURATE
    }
}
impl core::ops::Deref for SATURATEAT16BIT_R {
    type Target = crate::FieldReader<bool, SATURATEAT16BIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SATURATEAT16BIT` writer - Selects 16-bit saturation."]
pub struct SATURATEAT16BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SATURATEAT16BIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SATURATEAT16BIT_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DC Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_ctrl](index.html) module"]
pub struct DC_CTRL_SPEC;
impl crate::RegisterSpec for DC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_ctrl::R](R) reader structure"]
impl crate::Readable for DC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_ctrl::W](W) writer structure"]
impl crate::Writable for DC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DC_CTRL to value 0"]
impl crate::Resettable for DC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
