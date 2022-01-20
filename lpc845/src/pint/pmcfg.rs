#[doc = "Register `PMCFG` reader"]
pub struct R(crate::R<PMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCFG` writer"]
pub struct W(crate::W<PMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCFG_SPEC>;
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
impl From<crate::W<PMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Determines whether slice 0 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS0_A {
    #[doc = "0: No effect. Slice 0 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS0_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS0` reader - Determines whether slice 0 is an endpoint."]
pub struct PROD_ENDPTS0_R(crate::FieldReader<bool, PROD_ENDPTS0_A>);
impl PROD_ENDPTS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROD_ENDPTS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS0_A {
        match self.bits {
            false => PROD_ENDPTS0_A::NO_EFFECT,
            true => PROD_ENDPTS0_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == PROD_ENDPTS0_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        **self == PROD_ENDPTS0_A::ENDPOINT
    }
}
impl core::ops::Deref for PROD_ENDPTS0_R {
    type Target = crate::FieldReader<bool, PROD_ENDPTS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROD_ENDPTS0` writer - Determines whether slice 0 is an endpoint."]
pub struct PROD_ENDPTS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PROD_ENDPTS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect. Slice 0 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS0_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS0_A::ENDPOINT)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Determines whether slice 1 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS1_A {
    #[doc = "0: No effect. Slice 1 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS1_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS1` reader - Determines whether slice 1 is an endpoint."]
pub struct PROD_ENDPTS1_R(crate::FieldReader<bool, PROD_ENDPTS1_A>);
impl PROD_ENDPTS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROD_ENDPTS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS1_A {
        match self.bits {
            false => PROD_ENDPTS1_A::NO_EFFECT,
            true => PROD_ENDPTS1_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == PROD_ENDPTS1_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        **self == PROD_ENDPTS1_A::ENDPOINT
    }
}
impl core::ops::Deref for PROD_ENDPTS1_R {
    type Target = crate::FieldReader<bool, PROD_ENDPTS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROD_ENDPTS1` writer - Determines whether slice 1 is an endpoint."]
pub struct PROD_ENDPTS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROD_ENDPTS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect. Slice 1 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS1_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS1_A::ENDPOINT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Determines whether slice 2 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS2_A {
    #[doc = "0: No effect. Slice 2 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS2_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS2` reader - Determines whether slice 2 is an endpoint."]
pub struct PROD_ENDPTS2_R(crate::FieldReader<bool, PROD_ENDPTS2_A>);
impl PROD_ENDPTS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROD_ENDPTS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS2_A {
        match self.bits {
            false => PROD_ENDPTS2_A::NO_EFFECT,
            true => PROD_ENDPTS2_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == PROD_ENDPTS2_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        **self == PROD_ENDPTS2_A::ENDPOINT
    }
}
impl core::ops::Deref for PROD_ENDPTS2_R {
    type Target = crate::FieldReader<bool, PROD_ENDPTS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROD_ENDPTS2` writer - Determines whether slice 2 is an endpoint."]
pub struct PROD_ENDPTS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PROD_ENDPTS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect. Slice 2 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS2_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS2_A::ENDPOINT)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Determines whether slice 3 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS3_A {
    #[doc = "0: No effect. Slice 3 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS3_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS3` reader - Determines whether slice 3 is an endpoint."]
pub struct PROD_ENDPTS3_R(crate::FieldReader<bool, PROD_ENDPTS3_A>);
impl PROD_ENDPTS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROD_ENDPTS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS3_A {
        match self.bits {
            false => PROD_ENDPTS3_A::NO_EFFECT,
            true => PROD_ENDPTS3_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == PROD_ENDPTS3_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        **self == PROD_ENDPTS3_A::ENDPOINT
    }
}
impl core::ops::Deref for PROD_ENDPTS3_R {
    type Target = crate::FieldReader<bool, PROD_ENDPTS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROD_ENDPTS3` writer - Determines whether slice 3 is an endpoint."]
pub struct PROD_ENDPTS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PROD_ENDPTS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect. Slice 3 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS3_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS3_A::ENDPOINT)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Determines whether slice 4 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS4_A {
    #[doc = "0: No effect. Slice 4 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS4_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS4` reader - Determines whether slice 4 is an endpoint."]
pub struct PROD_ENDPTS4_R(crate::FieldReader<bool, PROD_ENDPTS4_A>);
impl PROD_ENDPTS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROD_ENDPTS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS4_A {
        match self.bits {
            false => PROD_ENDPTS4_A::NO_EFFECT,
            true => PROD_ENDPTS4_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == PROD_ENDPTS4_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        **self == PROD_ENDPTS4_A::ENDPOINT
    }
}
impl core::ops::Deref for PROD_ENDPTS4_R {
    type Target = crate::FieldReader<bool, PROD_ENDPTS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROD_ENDPTS4` writer - Determines whether slice 4 is an endpoint."]
pub struct PROD_ENDPTS4_W<'a> {
    w: &'a mut W,
}
impl<'a> PROD_ENDPTS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect. Slice 4 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS4_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS4_A::ENDPOINT)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Determines whether slice 5 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS5_A {
    #[doc = "0: No effect. Slice 5 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS5_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS5` reader - Determines whether slice 5 is an endpoint."]
pub struct PROD_ENDPTS5_R(crate::FieldReader<bool, PROD_ENDPTS5_A>);
impl PROD_ENDPTS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROD_ENDPTS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS5_A {
        match self.bits {
            false => PROD_ENDPTS5_A::NO_EFFECT,
            true => PROD_ENDPTS5_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == PROD_ENDPTS5_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        **self == PROD_ENDPTS5_A::ENDPOINT
    }
}
impl core::ops::Deref for PROD_ENDPTS5_R {
    type Target = crate::FieldReader<bool, PROD_ENDPTS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROD_ENDPTS5` writer - Determines whether slice 5 is an endpoint."]
pub struct PROD_ENDPTS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PROD_ENDPTS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect. Slice 5 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS5_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS5_A::ENDPOINT)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Determines whether slice 6 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS6_A {
    #[doc = "0: No effect. Slice 6 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS6_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROD_ENDPTS6` reader - Determines whether slice 6 is an endpoint."]
pub struct PROD_ENDPTS6_R(crate::FieldReader<bool, PROD_ENDPTS6_A>);
impl PROD_ENDPTS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROD_ENDPTS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS6_A {
        match self.bits {
            false => PROD_ENDPTS6_A::NO_EFFECT,
            true => PROD_ENDPTS6_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == PROD_ENDPTS6_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        **self == PROD_ENDPTS6_A::ENDPOINT
    }
}
impl core::ops::Deref for PROD_ENDPTS6_R {
    type Target = crate::FieldReader<bool, PROD_ENDPTS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROD_ENDPTS6` writer - Determines whether slice 6 is an endpoint."]
pub struct PROD_ENDPTS6_W<'a> {
    w: &'a mut W,
}
impl<'a> PROD_ENDPTS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect. Slice 6 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS6_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS6_A::ENDPOINT)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Specifies the match contribution condition for bit slice 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG0_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG0_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG0` reader - Specifies the match contribution condition for bit slice 0."]
pub struct CFG0_R(crate::FieldReader<u8, CFG0_A>);
impl CFG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG0_A {
        match self.bits {
            0 => CFG0_A::CONSTANT_HIGH,
            1 => CFG0_A::STICKY_RISING_EDGE,
            2 => CFG0_A::STICKY_FALLING_EDGE,
            3 => CFG0_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG0_A::HIGH_LEVEL,
            5 => CFG0_A::LOW_LEVEL,
            6 => CFG0_A::CONSTANT_ZERO,
            7 => CFG0_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        **self == CFG0_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        **self == CFG0_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        **self == CFG0_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        **self == CFG0_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        **self == CFG0_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        **self == CFG0_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        **self == CFG0_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CFG0_A::EVENT
    }
}
impl core::ops::Deref for CFG0_R {
    type Target = crate::FieldReader<u8, CFG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG0` writer - Specifies the match contribution condition for bit slice 0."]
pub struct CFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG0_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG0_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG0_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG0_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG0_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG0_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG0_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG0_A::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Specifies the match contribution condition for bit slice 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG1_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG1_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG1` reader - Specifies the match contribution condition for bit slice 1."]
pub struct CFG1_R(crate::FieldReader<u8, CFG1_A>);
impl CFG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG1_A {
        match self.bits {
            0 => CFG1_A::CONSTANT_HIGH,
            1 => CFG1_A::STICKY_RISING_EDGE,
            2 => CFG1_A::STICKY_FALLING_EDGE,
            3 => CFG1_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG1_A::HIGH_LEVEL,
            5 => CFG1_A::LOW_LEVEL,
            6 => CFG1_A::CONSTANT_ZERO,
            7 => CFG1_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        **self == CFG1_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        **self == CFG1_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        **self == CFG1_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        **self == CFG1_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        **self == CFG1_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        **self == CFG1_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        **self == CFG1_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CFG1_A::EVENT
    }
}
impl core::ops::Deref for CFG1_R {
    type Target = crate::FieldReader<u8, CFG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG1` writer - Specifies the match contribution condition for bit slice 1."]
pub struct CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG1_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG1_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG1_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG1_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG1_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG1_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG1_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG1_A::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Specifies the match contribution condition for bit slice 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG2_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG2_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG2` reader - Specifies the match contribution condition for bit slice 2."]
pub struct CFG2_R(crate::FieldReader<u8, CFG2_A>);
impl CFG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG2_A {
        match self.bits {
            0 => CFG2_A::CONSTANT_HIGH,
            1 => CFG2_A::STICKY_RISING_EDGE,
            2 => CFG2_A::STICKY_FALLING_EDGE,
            3 => CFG2_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG2_A::HIGH_LEVEL,
            5 => CFG2_A::LOW_LEVEL,
            6 => CFG2_A::CONSTANT_ZERO,
            7 => CFG2_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        **self == CFG2_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        **self == CFG2_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        **self == CFG2_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        **self == CFG2_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        **self == CFG2_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        **self == CFG2_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        **self == CFG2_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CFG2_A::EVENT
    }
}
impl core::ops::Deref for CFG2_R {
    type Target = crate::FieldReader<u8, CFG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG2` writer - Specifies the match contribution condition for bit slice 2."]
pub struct CFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG2_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG2_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG2_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG2_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG2_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG2_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG2_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG2_A::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
#[doc = "Specifies the match contribution condition for bit slice 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG3_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG3_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG3` reader - Specifies the match contribution condition for bit slice 3."]
pub struct CFG3_R(crate::FieldReader<u8, CFG3_A>);
impl CFG3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG3_A {
        match self.bits {
            0 => CFG3_A::CONSTANT_HIGH,
            1 => CFG3_A::STICKY_RISING_EDGE,
            2 => CFG3_A::STICKY_FALLING_EDGE,
            3 => CFG3_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG3_A::HIGH_LEVEL,
            5 => CFG3_A::LOW_LEVEL,
            6 => CFG3_A::CONSTANT_ZERO,
            7 => CFG3_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        **self == CFG3_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        **self == CFG3_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        **self == CFG3_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        **self == CFG3_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        **self == CFG3_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        **self == CFG3_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        **self == CFG3_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CFG3_A::EVENT
    }
}
impl core::ops::Deref for CFG3_R {
    type Target = crate::FieldReader<u8, CFG3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG3` writer - Specifies the match contribution condition for bit slice 3."]
pub struct CFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG3_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG3_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG3_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG3_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG3_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG3_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG3_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG3_A::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Specifies the match contribution condition for bit slice 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG4_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG4_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG4` reader - Specifies the match contribution condition for bit slice 4."]
pub struct CFG4_R(crate::FieldReader<u8, CFG4_A>);
impl CFG4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG4_A {
        match self.bits {
            0 => CFG4_A::CONSTANT_HIGH,
            1 => CFG4_A::STICKY_RISING_EDGE,
            2 => CFG4_A::STICKY_FALLING_EDGE,
            3 => CFG4_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG4_A::HIGH_LEVEL,
            5 => CFG4_A::LOW_LEVEL,
            6 => CFG4_A::CONSTANT_ZERO,
            7 => CFG4_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        **self == CFG4_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        **self == CFG4_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        **self == CFG4_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        **self == CFG4_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        **self == CFG4_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        **self == CFG4_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        **self == CFG4_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CFG4_A::EVENT
    }
}
impl core::ops::Deref for CFG4_R {
    type Target = crate::FieldReader<u8, CFG4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG4` writer - Specifies the match contribution condition for bit slice 4."]
pub struct CFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG4_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG4_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG4_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG4_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG4_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG4_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG4_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG4_A::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Specifies the match contribution condition for bit slice 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG5_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG5_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG5` reader - Specifies the match contribution condition for bit slice 5."]
pub struct CFG5_R(crate::FieldReader<u8, CFG5_A>);
impl CFG5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG5_A {
        match self.bits {
            0 => CFG5_A::CONSTANT_HIGH,
            1 => CFG5_A::STICKY_RISING_EDGE,
            2 => CFG5_A::STICKY_FALLING_EDGE,
            3 => CFG5_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG5_A::HIGH_LEVEL,
            5 => CFG5_A::LOW_LEVEL,
            6 => CFG5_A::CONSTANT_ZERO,
            7 => CFG5_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        **self == CFG5_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        **self == CFG5_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        **self == CFG5_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        **self == CFG5_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        **self == CFG5_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        **self == CFG5_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        **self == CFG5_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CFG5_A::EVENT
    }
}
impl core::ops::Deref for CFG5_R {
    type Target = crate::FieldReader<u8, CFG5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG5` writer - Specifies the match contribution condition for bit slice 5."]
pub struct CFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG5_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG5_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG5_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG5_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG5_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG5_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG5_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG5_A::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
#[doc = "Specifies the match contribution condition for bit slice 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG6_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG6_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG6` reader - Specifies the match contribution condition for bit slice 6."]
pub struct CFG6_R(crate::FieldReader<u8, CFG6_A>);
impl CFG6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG6_A {
        match self.bits {
            0 => CFG6_A::CONSTANT_HIGH,
            1 => CFG6_A::STICKY_RISING_EDGE,
            2 => CFG6_A::STICKY_FALLING_EDGE,
            3 => CFG6_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG6_A::HIGH_LEVEL,
            5 => CFG6_A::LOW_LEVEL,
            6 => CFG6_A::CONSTANT_ZERO,
            7 => CFG6_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        **self == CFG6_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        **self == CFG6_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        **self == CFG6_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        **self == CFG6_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        **self == CFG6_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        **self == CFG6_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        **self == CFG6_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CFG6_A::EVENT
    }
}
impl core::ops::Deref for CFG6_R {
    type Target = crate::FieldReader<u8, CFG6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG6` writer - Specifies the match contribution condition for bit slice 6."]
pub struct CFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG6_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG6_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG6_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG6_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG6_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG6_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG6_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG6_A::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | ((value as u32 & 0x07) << 26);
        self.w
    }
}
#[doc = "Specifies the match contribution condition for bit slice 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG7_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG7_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG7` reader - Specifies the match contribution condition for bit slice 7."]
pub struct CFG7_R(crate::FieldReader<u8, CFG7_A>);
impl CFG7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG7_A {
        match self.bits {
            0 => CFG7_A::CONSTANT_HIGH,
            1 => CFG7_A::STICKY_RISING_EDGE,
            2 => CFG7_A::STICKY_FALLING_EDGE,
            3 => CFG7_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG7_A::HIGH_LEVEL,
            5 => CFG7_A::LOW_LEVEL,
            6 => CFG7_A::CONSTANT_ZERO,
            7 => CFG7_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        **self == CFG7_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        **self == CFG7_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        **self == CFG7_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        **self == CFG7_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        **self == CFG7_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        **self == CFG7_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        **self == CFG7_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == CFG7_A::EVENT
    }
}
impl core::ops::Deref for CFG7_R {
    type Target = crate::FieldReader<u8, CFG7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG7` writer - Specifies the match contribution condition for bit slice 7."]
pub struct CFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG7_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG7_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG7_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG7_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG7_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG7_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG7_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG7_A::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts0(&self) -> PROD_ENDPTS0_R {
        PROD_ENDPTS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts1(&self) -> PROD_ENDPTS1_R {
        PROD_ENDPTS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts2(&self) -> PROD_ENDPTS2_R {
        PROD_ENDPTS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts3(&self) -> PROD_ENDPTS3_R {
        PROD_ENDPTS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts4(&self) -> PROD_ENDPTS4_R {
        PROD_ENDPTS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts5(&self) -> PROD_ENDPTS5_R {
        PROD_ENDPTS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts6(&self) -> PROD_ENDPTS6_R {
        PROD_ENDPTS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub fn cfg0(&self) -> CFG0_R {
        CFG0_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts0(&mut self) -> PROD_ENDPTS0_W {
        PROD_ENDPTS0_W { w: self }
    }
    #[doc = "Bit 1 - Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts1(&mut self) -> PROD_ENDPTS1_W {
        PROD_ENDPTS1_W { w: self }
    }
    #[doc = "Bit 2 - Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts2(&mut self) -> PROD_ENDPTS2_W {
        PROD_ENDPTS2_W { w: self }
    }
    #[doc = "Bit 3 - Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts3(&mut self) -> PROD_ENDPTS3_W {
        PROD_ENDPTS3_W { w: self }
    }
    #[doc = "Bit 4 - Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts4(&mut self) -> PROD_ENDPTS4_W {
        PROD_ENDPTS4_W { w: self }
    }
    #[doc = "Bit 5 - Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts5(&mut self) -> PROD_ENDPTS5_W {
        PROD_ENDPTS5_W { w: self }
    }
    #[doc = "Bit 6 - Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts6(&mut self) -> PROD_ENDPTS6_W {
        PROD_ENDPTS6_W { w: self }
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub fn cfg0(&mut self) -> CFG0_W {
        CFG0_W { w: self }
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub fn cfg1(&mut self) -> CFG1_W {
        CFG1_W { w: self }
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub fn cfg2(&mut self) -> CFG2_W {
        CFG2_W { w: self }
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub fn cfg3(&mut self) -> CFG3_W {
        CFG3_W { w: self }
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub fn cfg4(&mut self) -> CFG4_W {
        CFG4_W { w: self }
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub fn cfg5(&mut self) -> CFG5_W {
        CFG5_W { w: self }
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub fn cfg6(&mut self) -> CFG6_W {
        CFG6_W { w: self }
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub fn cfg7(&mut self) -> CFG7_W {
        CFG7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pattern match interrupt bit slice configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmcfg](index.html) module"]
pub struct PMCFG_SPEC;
impl crate::RegisterSpec for PMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmcfg::R](R) reader structure"]
impl crate::Readable for PMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmcfg::W](W) writer structure"]
impl crate::Writable for PMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMCFG to value 0"]
impl crate::Resettable for PMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
