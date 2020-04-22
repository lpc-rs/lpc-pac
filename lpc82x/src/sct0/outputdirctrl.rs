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
}
