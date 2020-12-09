#[doc = "Reader of register OUTPUTDIRCTRL"]
pub type R = crate::R<u32, super::OUTPUTDIRCTRL>;
#[doc = "Writer for register OUTPUTDIRCTRL"]
pub type W = crate::W<u32, super::OUTPUTDIRCTRL>;
#[doc = "Register OUTPUTDIRCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTPUTDIRCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `SETCLR0`"]
pub type SETCLR0_R = crate::R<u8, SETCLR0_A>;
impl SETCLR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR0_A::INDEPENDENT),
            1 => Val(SETCLR0_A::L_REVERSED),
            2 => Val(SETCLR0_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR0_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR0_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR0_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR0`"]
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
#[doc = "Reader of field `SETCLR1`"]
pub type SETCLR1_R = crate::R<u8, SETCLR1_A>;
impl SETCLR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR1_A::INDEPENDENT),
            1 => Val(SETCLR1_A::L_REVERSED),
            2 => Val(SETCLR1_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR1_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR1_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR1_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR1`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
#[doc = "Reader of field `SETCLR2`"]
pub type SETCLR2_R = crate::R<u8, SETCLR2_A>;
impl SETCLR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR2_A::INDEPENDENT),
            1 => Val(SETCLR2_A::L_REVERSED),
            2 => Val(SETCLR2_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR2_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR2_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR2_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR2`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
#[doc = "Reader of field `SETCLR3`"]
pub type SETCLR3_R = crate::R<u8, SETCLR3_A>;
impl SETCLR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR3_A::INDEPENDENT),
            1 => Val(SETCLR3_A::L_REVERSED),
            2 => Val(SETCLR3_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR3_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR3_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR3_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR3`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
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
#[doc = "Reader of field `SETCLR4`"]
pub type SETCLR4_R = crate::R<u8, SETCLR4_A>;
impl SETCLR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR4_A::INDEPENDENT),
            1 => Val(SETCLR4_A::L_REVERSED),
            2 => Val(SETCLR4_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR4_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR4_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR4_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR4`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
#[doc = "Reader of field `SETCLR5`"]
pub type SETCLR5_R = crate::R<u8, SETCLR5_A>;
impl SETCLR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR5_A::INDEPENDENT),
            1 => Val(SETCLR5_A::L_REVERSED),
            2 => Val(SETCLR5_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR5_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR5_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR5_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR5`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
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
#[doc = "Reader of field `SETCLR6`"]
pub type SETCLR6_R = crate::R<u8, SETCLR6_A>;
impl SETCLR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR6_A::INDEPENDENT),
            1 => Val(SETCLR6_A::L_REVERSED),
            2 => Val(SETCLR6_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR6_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR6_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR6_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR6`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
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
#[doc = "Reader of field `SETCLR7`"]
pub type SETCLR7_R = crate::R<u8, SETCLR7_A>;
impl SETCLR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR7_A::INDEPENDENT),
            1 => Val(SETCLR7_A::L_REVERSED),
            2 => Val(SETCLR7_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR7_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR7_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR7_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR7`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
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
#[doc = "Reader of field `SETCLR8`"]
pub type SETCLR8_R = crate::R<u8, SETCLR8_A>;
impl SETCLR8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR8_A::INDEPENDENT),
            1 => Val(SETCLR8_A::L_REVERSED),
            2 => Val(SETCLR8_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR8_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR8_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR8_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR8`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
#[doc = "Reader of field `SETCLR9`"]
pub type SETCLR9_R = crate::R<u8, SETCLR9_A>;
impl SETCLR9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR9_A::INDEPENDENT),
            1 => Val(SETCLR9_A::L_REVERSED),
            2 => Val(SETCLR9_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR9_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR9_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR9_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR9`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
#[doc = "Reader of field `SETCLR10`"]
pub type SETCLR10_R = crate::R<u8, SETCLR10_A>;
impl SETCLR10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR10_A::INDEPENDENT),
            1 => Val(SETCLR10_A::L_REVERSED),
            2 => Val(SETCLR10_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR10_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR10_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR10_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR10`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
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
#[doc = "Reader of field `SETCLR11`"]
pub type SETCLR11_R = crate::R<u8, SETCLR11_A>;
impl SETCLR11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR11_A::INDEPENDENT),
            1 => Val(SETCLR11_A::L_REVERSED),
            2 => Val(SETCLR11_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR11_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR11_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR11_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR11`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
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
#[doc = "Reader of field `SETCLR12`"]
pub type SETCLR12_R = crate::R<u8, SETCLR12_A>;
impl SETCLR12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR12_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR12_A::INDEPENDENT),
            1 => Val(SETCLR12_A::L_REVERSED),
            2 => Val(SETCLR12_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR12_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR12_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR12_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR12`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
#[doc = "Reader of field `SETCLR13`"]
pub type SETCLR13_R = crate::R<u8, SETCLR13_A>;
impl SETCLR13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR13_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR13_A::INDEPENDENT),
            1 => Val(SETCLR13_A::L_REVERSED),
            2 => Val(SETCLR13_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR13_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR13_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR13_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR13`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
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
#[doc = "Reader of field `SETCLR14`"]
pub type SETCLR14_R = crate::R<u8, SETCLR14_A>;
impl SETCLR14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR14_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR14_A::INDEPENDENT),
            1 => Val(SETCLR14_A::L_REVERSED),
            2 => Val(SETCLR14_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR14_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR14_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR14_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR14`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
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
#[doc = "Reader of field `SETCLR15`"]
pub type SETCLR15_R = crate::R<u8, SETCLR15_A>;
impl SETCLR15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETCLR15_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETCLR15_A::INDEPENDENT),
            1 => Val(SETCLR15_A::L_REVERSED),
            2 => Val(SETCLR15_A::H_REVERSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR15_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR15_A::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR15_A::H_REVERSED
    }
}
#[doc = "Write proxy for field `SETCLR15`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
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
}
