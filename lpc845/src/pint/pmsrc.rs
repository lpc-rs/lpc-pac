#[doc = "Reader of register PMSRC"]
pub type R = crate::R<u32, super::PMSRC>;
#[doc = "Writer for register PMSRC"]
pub type W = crate::W<u32, super::PMSRC>;
#[doc = "Register PMSRC `reset()`'s with value 0"]
impl crate::ResetValue for super::PMSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the input source for bit slice 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC0_A {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 0."]
    INPUT0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 0."]
    INPUT1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 0."]
    INPUT2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 0."]
    INPUT3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 0."]
    INPUT4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 0."]
    INPUT5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 0."]
    INPUT6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 0."]
    INPUT7 = 7,
}
impl From<SRC0_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC0`"]
pub type SRC0_R = crate::R<u8, SRC0_A>;
impl SRC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC0_A {
        match self.bits {
            0 => SRC0_A::INPUT0,
            1 => SRC0_A::INPUT1,
            2 => SRC0_A::INPUT2,
            3 => SRC0_A::INPUT3,
            4 => SRC0_A::INPUT4,
            5 => SRC0_A::INPUT5,
            6 => SRC0_A::INPUT6,
            7 => SRC0_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == SRC0_A::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == SRC0_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == SRC0_A::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == SRC0_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == SRC0_A::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == SRC0_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == SRC0_A::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == SRC0_A::INPUT7
    }
}
#[doc = "Write proxy for field `SRC0`"]
pub struct SRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC0_A::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC0_A::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC0_A::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC0_A::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC0_A::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC0_A::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC0_A::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 0."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC0_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Selects the input source for bit slice 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC1_A {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 1."]
    INPUT0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 1."]
    INPUT1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 1."]
    INPUT2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 1."]
    INPUT3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 1."]
    INPUT4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 1."]
    INPUT5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 1."]
    INPUT6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 1."]
    INPUT7 = 7,
}
impl From<SRC1_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC1`"]
pub type SRC1_R = crate::R<u8, SRC1_A>;
impl SRC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC1_A {
        match self.bits {
            0 => SRC1_A::INPUT0,
            1 => SRC1_A::INPUT1,
            2 => SRC1_A::INPUT2,
            3 => SRC1_A::INPUT3,
            4 => SRC1_A::INPUT4,
            5 => SRC1_A::INPUT5,
            6 => SRC1_A::INPUT6,
            7 => SRC1_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == SRC1_A::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == SRC1_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == SRC1_A::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == SRC1_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == SRC1_A::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == SRC1_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == SRC1_A::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == SRC1_A::INPUT7
    }
}
#[doc = "Write proxy for field `SRC1`"]
pub struct SRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC1_A::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC1_A::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC1_A::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC1_A::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC1_A::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC1_A::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC1_A::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 1."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC1_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Selects the input source for bit slice 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC2_A {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 2."]
    INPUT0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 2."]
    INPUT1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 2."]
    INPUT2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 2."]
    INPUT3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 2."]
    INPUT4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 2."]
    INPUT5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 2."]
    INPUT6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 2."]
    INPUT7 = 7,
}
impl From<SRC2_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC2`"]
pub type SRC2_R = crate::R<u8, SRC2_A>;
impl SRC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC2_A {
        match self.bits {
            0 => SRC2_A::INPUT0,
            1 => SRC2_A::INPUT1,
            2 => SRC2_A::INPUT2,
            3 => SRC2_A::INPUT3,
            4 => SRC2_A::INPUT4,
            5 => SRC2_A::INPUT5,
            6 => SRC2_A::INPUT6,
            7 => SRC2_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == SRC2_A::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == SRC2_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == SRC2_A::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == SRC2_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == SRC2_A::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == SRC2_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == SRC2_A::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == SRC2_A::INPUT7
    }
}
#[doc = "Write proxy for field `SRC2`"]
pub struct SRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC2_A::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC2_A::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC2_A::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC2_A::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC2_A::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC2_A::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC2_A::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 2."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC2_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Selects the input source for bit slice 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC3_A {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 3."]
    INPUT0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 3."]
    INPUT1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 3."]
    INPUT2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 3."]
    INPUT3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 3."]
    INPUT4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 3."]
    INPUT5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 3."]
    INPUT6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 3."]
    INPUT7 = 7,
}
impl From<SRC3_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC3`"]
pub type SRC3_R = crate::R<u8, SRC3_A>;
impl SRC3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC3_A {
        match self.bits {
            0 => SRC3_A::INPUT0,
            1 => SRC3_A::INPUT1,
            2 => SRC3_A::INPUT2,
            3 => SRC3_A::INPUT3,
            4 => SRC3_A::INPUT4,
            5 => SRC3_A::INPUT5,
            6 => SRC3_A::INPUT6,
            7 => SRC3_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == SRC3_A::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == SRC3_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == SRC3_A::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == SRC3_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == SRC3_A::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == SRC3_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == SRC3_A::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == SRC3_A::INPUT7
    }
}
#[doc = "Write proxy for field `SRC3`"]
pub struct SRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC3_A::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC3_A::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC3_A::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC3_A::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC3_A::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC3_A::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC3_A::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 3."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC3_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Selects the input source for bit slice 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC4_A {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 4."]
    INPUT0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 4."]
    INPUT1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 4."]
    INPUT2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 4."]
    INPUT3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 4."]
    INPUT4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 4."]
    INPUT5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 4."]
    INPUT6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 4."]
    INPUT7 = 7,
}
impl From<SRC4_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC4`"]
pub type SRC4_R = crate::R<u8, SRC4_A>;
impl SRC4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC4_A {
        match self.bits {
            0 => SRC4_A::INPUT0,
            1 => SRC4_A::INPUT1,
            2 => SRC4_A::INPUT2,
            3 => SRC4_A::INPUT3,
            4 => SRC4_A::INPUT4,
            5 => SRC4_A::INPUT5,
            6 => SRC4_A::INPUT6,
            7 => SRC4_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == SRC4_A::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == SRC4_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == SRC4_A::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == SRC4_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == SRC4_A::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == SRC4_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == SRC4_A::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == SRC4_A::INPUT7
    }
}
#[doc = "Write proxy for field `SRC4`"]
pub struct SRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC4_A::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC4_A::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC4_A::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC4_A::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC4_A::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC4_A::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC4_A::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 4."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC4_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Selects the input source for bit slice 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC5_A {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 5."]
    INPUT0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 5."]
    INPUT1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 5."]
    INPUT2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 5."]
    INPUT3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 5."]
    INPUT4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 5."]
    INPUT5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 5."]
    INPUT6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 5."]
    INPUT7 = 7,
}
impl From<SRC5_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC5`"]
pub type SRC5_R = crate::R<u8, SRC5_A>;
impl SRC5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC5_A {
        match self.bits {
            0 => SRC5_A::INPUT0,
            1 => SRC5_A::INPUT1,
            2 => SRC5_A::INPUT2,
            3 => SRC5_A::INPUT3,
            4 => SRC5_A::INPUT4,
            5 => SRC5_A::INPUT5,
            6 => SRC5_A::INPUT6,
            7 => SRC5_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == SRC5_A::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == SRC5_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == SRC5_A::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == SRC5_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == SRC5_A::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == SRC5_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == SRC5_A::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == SRC5_A::INPUT7
    }
}
#[doc = "Write proxy for field `SRC5`"]
pub struct SRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC5_A::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC5_A::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC5_A::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC5_A::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC5_A::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC5_A::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC5_A::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 5."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC5_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Selects the input source for bit slice 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC6_A {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 6."]
    INPUT0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 6."]
    INPUT1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 6."]
    INPUT2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 6."]
    INPUT3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 6."]
    INPUT4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 6."]
    INPUT5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 6."]
    INPUT6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 6."]
    INPUT7 = 7,
}
impl From<SRC6_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC6`"]
pub type SRC6_R = crate::R<u8, SRC6_A>;
impl SRC6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC6_A {
        match self.bits {
            0 => SRC6_A::INPUT0,
            1 => SRC6_A::INPUT1,
            2 => SRC6_A::INPUT2,
            3 => SRC6_A::INPUT3,
            4 => SRC6_A::INPUT4,
            5 => SRC6_A::INPUT5,
            6 => SRC6_A::INPUT6,
            7 => SRC6_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == SRC6_A::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == SRC6_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == SRC6_A::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == SRC6_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == SRC6_A::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == SRC6_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == SRC6_A::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == SRC6_A::INPUT7
    }
}
#[doc = "Write proxy for field `SRC6`"]
pub struct SRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC6_A::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC6_A::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC6_A::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC6_A::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC6_A::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC6_A::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC6_A::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 6."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC6_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Selects the input source for bit slice 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC7_A {
    #[doc = "0: Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 7."]
    INPUT0 = 0,
    #[doc = "1: Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 7."]
    INPUT1 = 1,
    #[doc = "2: Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 7."]
    INPUT2 = 2,
    #[doc = "3: Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 7."]
    INPUT3 = 3,
    #[doc = "4: Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 7."]
    INPUT4 = 4,
    #[doc = "5: Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 7."]
    INPUT5 = 5,
    #[doc = "6: Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 7."]
    INPUT6 = 6,
    #[doc = "7: Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 7."]
    INPUT7 = 7,
}
impl From<SRC7_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC7`"]
pub type SRC7_R = crate::R<u8, SRC7_A>;
impl SRC7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC7_A {
        match self.bits {
            0 => SRC7_A::INPUT0,
            1 => SRC7_A::INPUT1,
            2 => SRC7_A::INPUT2,
            3 => SRC7_A::INPUT3,
            4 => SRC7_A::INPUT4,
            5 => SRC7_A::INPUT5,
            6 => SRC7_A::INPUT6,
            7 => SRC7_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT0`"]
    #[inline(always)]
    pub fn is_input0(&self) -> bool {
        *self == SRC7_A::INPUT0
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == SRC7_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT2`"]
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == SRC7_A::INPUT2
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == SRC7_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT4`"]
    #[inline(always)]
    pub fn is_input4(&self) -> bool {
        *self == SRC7_A::INPUT4
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == SRC7_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT6`"]
    #[inline(always)]
    pub fn is_input6(&self) -> bool {
        *self == SRC7_A::INPUT6
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == SRC7_A::INPUT7
    }
}
#[doc = "Write proxy for field `SRC7`"]
pub struct SRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input0(self) -> &'a mut W {
        self.variant(SRC7_A::INPUT0)
    }
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(SRC7_A::INPUT1)
    }
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input2(self) -> &'a mut W {
        self.variant(SRC7_A::INPUT2)
    }
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(SRC7_A::INPUT3)
    }
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input4(self) -> &'a mut W {
        self.variant(SRC7_A::INPUT4)
    }
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(SRC7_A::INPUT5)
    }
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input6(self) -> &'a mut W {
        self.variant(SRC7_A::INPUT6)
    }
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 7."]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(SRC7_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Selects the input source for bit slice 0"]
    #[inline(always)]
    pub fn src0(&self) -> SRC0_R {
        SRC0_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Selects the input source for bit slice 1"]
    #[inline(always)]
    pub fn src1(&self) -> SRC1_R {
        SRC1_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - Selects the input source for bit slice 2"]
    #[inline(always)]
    pub fn src2(&self) -> SRC2_R {
        SRC2_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - Selects the input source for bit slice 3"]
    #[inline(always)]
    pub fn src3(&self) -> SRC3_R {
        SRC3_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Selects the input source for bit slice 4"]
    #[inline(always)]
    pub fn src4(&self) -> SRC4_R {
        SRC4_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - Selects the input source for bit slice 5"]
    #[inline(always)]
    pub fn src5(&self) -> SRC5_R {
        SRC5_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - Selects the input source for bit slice 6"]
    #[inline(always)]
    pub fn src6(&self) -> SRC6_R {
        SRC6_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Selects the input source for bit slice 7"]
    #[inline(always)]
    pub fn src7(&self) -> SRC7_R {
        SRC7_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Selects the input source for bit slice 0"]
    #[inline(always)]
    pub fn src0(&mut self) -> SRC0_W {
        SRC0_W { w: self }
    }
    #[doc = "Bits 11:13 - Selects the input source for bit slice 1"]
    #[inline(always)]
    pub fn src1(&mut self) -> SRC1_W {
        SRC1_W { w: self }
    }
    #[doc = "Bits 14:16 - Selects the input source for bit slice 2"]
    #[inline(always)]
    pub fn src2(&mut self) -> SRC2_W {
        SRC2_W { w: self }
    }
    #[doc = "Bits 17:19 - Selects the input source for bit slice 3"]
    #[inline(always)]
    pub fn src3(&mut self) -> SRC3_W {
        SRC3_W { w: self }
    }
    #[doc = "Bits 20:22 - Selects the input source for bit slice 4"]
    #[inline(always)]
    pub fn src4(&mut self) -> SRC4_W {
        SRC4_W { w: self }
    }
    #[doc = "Bits 23:25 - Selects the input source for bit slice 5"]
    #[inline(always)]
    pub fn src5(&mut self) -> SRC5_W {
        SRC5_W { w: self }
    }
    #[doc = "Bits 26:28 - Selects the input source for bit slice 6"]
    #[inline(always)]
    pub fn src6(&mut self) -> SRC6_W {
        SRC6_W { w: self }
    }
    #[doc = "Bits 29:31 - Selects the input source for bit slice 7"]
    #[inline(always)]
    pub fn src7(&mut self) -> SRC7_W {
        SRC7_W { w: self }
    }
}
