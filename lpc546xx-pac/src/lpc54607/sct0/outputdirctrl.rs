#[doc = "Register `OUTPUTDIRCTRL` reader"]
pub struct R(crate::R<OUTPUTDIRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUTDIRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPUTDIRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPUTDIRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPUTDIRCTRL` writer"]
pub struct W(crate::W<OUTPUTDIRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUTDIRCTRL_SPEC>;
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
impl From<crate::W<OUTPUTDIRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPUTDIRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR0_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR0_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR0` reader - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR0_R(crate::FieldReader<u8, SETCLR0_A>);
impl SETCLR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR0_A> {
        match self.bits {
            0 => Some(SETCLR0_A::INDEPENDENT),
            1 => Some(SETCLR0_A::L_REVERSED),
            2 => Some(SETCLR0_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR0_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR0_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR0_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR0_R {
    type Target = crate::FieldReader<u8, SETCLR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR0` writer - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR0_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR0_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR0_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR1_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR1_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR1` reader - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR1_R(crate::FieldReader<u8, SETCLR1_A>);
impl SETCLR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR1_A> {
        match self.bits {
            0 => Some(SETCLR1_A::INDEPENDENT),
            1 => Some(SETCLR1_A::L_REVERSED),
            2 => Some(SETCLR1_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR1_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR1_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR1_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR1_R {
    type Target = crate::FieldReader<u8, SETCLR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR1` writer - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR1_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR1_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR1_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR2_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR2_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR2` reader - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR2_R(crate::FieldReader<u8, SETCLR2_A>);
impl SETCLR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR2_A> {
        match self.bits {
            0 => Some(SETCLR2_A::INDEPENDENT),
            1 => Some(SETCLR2_A::L_REVERSED),
            2 => Some(SETCLR2_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR2_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR2_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR2_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR2_R {
    type Target = crate::FieldReader<u8, SETCLR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR2` writer - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR2_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR2_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR2_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR3_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR3_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR3` reader - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR3_R(crate::FieldReader<u8, SETCLR3_A>);
impl SETCLR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR3_A> {
        match self.bits {
            0 => Some(SETCLR3_A::INDEPENDENT),
            1 => Some(SETCLR3_A::L_REVERSED),
            2 => Some(SETCLR3_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR3_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR3_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR3_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR3_R {
    type Target = crate::FieldReader<u8, SETCLR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR3` writer - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR3_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR3_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR3_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR4_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR4_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR4` reader - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR4_R(crate::FieldReader<u8, SETCLR4_A>);
impl SETCLR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR4_A> {
        match self.bits {
            0 => Some(SETCLR4_A::INDEPENDENT),
            1 => Some(SETCLR4_A::L_REVERSED),
            2 => Some(SETCLR4_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR4_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR4_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR4_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR4_R {
    type Target = crate::FieldReader<u8, SETCLR4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR4` writer - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR4_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR4_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR4_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR5_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR5_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR5` reader - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR5_R(crate::FieldReader<u8, SETCLR5_A>);
impl SETCLR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR5_A> {
        match self.bits {
            0 => Some(SETCLR5_A::INDEPENDENT),
            1 => Some(SETCLR5_A::L_REVERSED),
            2 => Some(SETCLR5_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR5_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR5_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR5_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR5_R {
    type Target = crate::FieldReader<u8, SETCLR5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR5` writer - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR5_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR5_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR5_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR6_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR6_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR6` reader - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR6_R(crate::FieldReader<u8, SETCLR6_A>);
impl SETCLR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR6_A> {
        match self.bits {
            0 => Some(SETCLR6_A::INDEPENDENT),
            1 => Some(SETCLR6_A::L_REVERSED),
            2 => Some(SETCLR6_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR6_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR6_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR6_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR6_R {
    type Target = crate::FieldReader<u8, SETCLR6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR6` writer - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR6_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR6_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR6_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR7_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR7_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR7` reader - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR7_R(crate::FieldReader<u8, SETCLR7_A>);
impl SETCLR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR7_A> {
        match self.bits {
            0 => Some(SETCLR7_A::INDEPENDENT),
            1 => Some(SETCLR7_A::L_REVERSED),
            2 => Some(SETCLR7_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR7_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR7_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR7_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR7_R {
    type Target = crate::FieldReader<u8, SETCLR7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR7` writer - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR7_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR7_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR7_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR8_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR8_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR8` reader - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR8_R(crate::FieldReader<u8, SETCLR8_A>);
impl SETCLR8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR8_A> {
        match self.bits {
            0 => Some(SETCLR8_A::INDEPENDENT),
            1 => Some(SETCLR8_A::L_REVERSED),
            2 => Some(SETCLR8_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR8_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR8_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR8_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR8_R {
    type Target = crate::FieldReader<u8, SETCLR8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR8` writer - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR8_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR8_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR8_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR9_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR9_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR9` reader - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR9_R(crate::FieldReader<u8, SETCLR9_A>);
impl SETCLR9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR9_A> {
        match self.bits {
            0 => Some(SETCLR9_A::INDEPENDENT),
            1 => Some(SETCLR9_A::L_REVERSED),
            2 => Some(SETCLR9_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR9_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR9_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR9_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR9_R {
    type Target = crate::FieldReader<u8, SETCLR9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR9` writer - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR9_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR9_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR9_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR10_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR10_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR10` reader - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR10_R(crate::FieldReader<u8, SETCLR10_A>);
impl SETCLR10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR10_A> {
        match self.bits {
            0 => Some(SETCLR10_A::INDEPENDENT),
            1 => Some(SETCLR10_A::L_REVERSED),
            2 => Some(SETCLR10_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR10_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR10_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR10_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR10_R {
    type Target = crate::FieldReader<u8, SETCLR10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR10` writer - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR10_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR10_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR10_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR11_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR11_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR11` reader - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR11_R(crate::FieldReader<u8, SETCLR11_A>);
impl SETCLR11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR11_A> {
        match self.bits {
            0 => Some(SETCLR11_A::INDEPENDENT),
            1 => Some(SETCLR11_A::L_REVERSED),
            2 => Some(SETCLR11_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR11_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR11_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR11_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR11_R {
    type Target = crate::FieldReader<u8, SETCLR11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR11` writer - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR11_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR11_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR11_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR12_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR12_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR12` reader - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR12_R(crate::FieldReader<u8, SETCLR12_A>);
impl SETCLR12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR12_A> {
        match self.bits {
            0 => Some(SETCLR12_A::INDEPENDENT),
            1 => Some(SETCLR12_A::L_REVERSED),
            2 => Some(SETCLR12_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR12_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR12_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR12_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR12_R {
    type Target = crate::FieldReader<u8, SETCLR12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR12` writer - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR12_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR12_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR12_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR13_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR13_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR13` reader - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR13_R(crate::FieldReader<u8, SETCLR13_A>);
impl SETCLR13_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR13_A> {
        match self.bits {
            0 => Some(SETCLR13_A::INDEPENDENT),
            1 => Some(SETCLR13_A::L_REVERSED),
            2 => Some(SETCLR13_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR13_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR13_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR13_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR13_R {
    type Target = crate::FieldReader<u8, SETCLR13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR13` writer - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR13_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR13_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR13_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR14_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR14_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR14` reader - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR14_R(crate::FieldReader<u8, SETCLR14_A>);
impl SETCLR14_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR14_A> {
        match self.bits {
            0 => Some(SETCLR14_A::INDEPENDENT),
            1 => Some(SETCLR14_A::L_REVERSED),
            2 => Some(SETCLR14_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR14_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR14_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR14_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR14_R {
    type Target = crate::FieldReader<u8, SETCLR14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR14` writer - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR14_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR14_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR14_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETCLR15_A {
    #[doc = "0: Set and clear do not depend on the direction of any counter."]
    INDEPENDENT = 0,
    #[doc = "1: Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED = 1,
    #[doc = "2: Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED = 2,
}
impl From<SETCLR15_A> for u8 {
    #[inline(always)]
    fn from(variant: SETCLR15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SETCLR15` reader - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR15_R(crate::FieldReader<u8, SETCLR15_A>);
impl SETCLR15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETCLR15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETCLR15_A> {
        match self.bits {
            0 => Some(SETCLR15_A::INDEPENDENT),
            1 => Some(SETCLR15_A::L_REVERSED),
            2 => Some(SETCLR15_A::H_REVERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == SETCLR15_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        **self == SETCLR15_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        **self == SETCLR15_A::H_REVERSED
    }
}
impl core::ops::Deref for SETCLR15_R {
    type Target = crate::FieldReader<u8, SETCLR15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETCLR15` writer - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
pub struct SETCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> SETCLR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR15_A::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR15_A::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR15_A::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&self) -> SETCLR0_R {
        SETCLR0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&self) -> SETCLR1_R {
        SETCLR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&self) -> SETCLR2_R {
        SETCLR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&self) -> SETCLR3_R {
        SETCLR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr4(&self) -> SETCLR4_R {
        SETCLR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr5(&self) -> SETCLR5_R {
        SETCLR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr6(&self) -> SETCLR6_R {
        SETCLR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr7(&self) -> SETCLR7_R {
        SETCLR7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr8(&self) -> SETCLR8_R {
        SETCLR8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr9(&self) -> SETCLR9_R {
        SETCLR9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr10(&self) -> SETCLR10_R {
        SETCLR10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr11(&self) -> SETCLR11_R {
        SETCLR11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr12(&self) -> SETCLR12_R {
        SETCLR12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr13(&self) -> SETCLR13_R {
        SETCLR13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr14(&self) -> SETCLR14_R {
        SETCLR14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr15(&self) -> SETCLR15_R {
        SETCLR15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&mut self) -> SETCLR0_W {
        SETCLR0_W { w: self }
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&mut self) -> SETCLR1_W {
        SETCLR1_W { w: self }
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&mut self) -> SETCLR2_W {
        SETCLR2_W { w: self }
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&mut self) -> SETCLR3_W {
        SETCLR3_W { w: self }
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr4(&mut self) -> SETCLR4_W {
        SETCLR4_W { w: self }
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr5(&mut self) -> SETCLR5_W {
        SETCLR5_W { w: self }
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr6(&mut self) -> SETCLR6_W {
        SETCLR6_W { w: self }
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr7(&mut self) -> SETCLR7_W {
        SETCLR7_W { w: self }
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr8(&mut self) -> SETCLR8_W {
        SETCLR8_W { w: self }
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr9(&mut self) -> SETCLR9_W {
        SETCLR9_W { w: self }
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr10(&mut self) -> SETCLR10_W {
        SETCLR10_W { w: self }
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr11(&mut self) -> SETCLR11_W {
        SETCLR11_W { w: self }
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr12(&mut self) -> SETCLR12_W {
        SETCLR12_W { w: self }
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr13(&mut self) -> SETCLR13_W {
        SETCLR13_W { w: self }
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr14(&mut self) -> SETCLR14_W {
        SETCLR14_W { w: self }
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr15(&mut self) -> SETCLR15_W {
        SETCLR15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT output counter direction control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outputdirctrl](index.html) module"]
pub struct OUTPUTDIRCTRL_SPEC;
impl crate::RegisterSpec for OUTPUTDIRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outputdirctrl::R](R) reader structure"]
impl crate::Readable for OUTPUTDIRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outputdirctrl::W](W) writer structure"]
impl crate::Writable for OUTPUTDIRCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTPUTDIRCTRL to value 0"]
impl crate::Resettable for OUTPUTDIRCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
