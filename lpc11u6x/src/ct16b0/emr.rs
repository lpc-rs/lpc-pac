#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMR {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type EM0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EM0W<'a> {
    w: &'a mut W,
}
impl<'a> _EM0W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EM1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EM1W<'a> {
    w: &'a mut W,
}
impl<'a> _EM1W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EM2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EM2W<'a> {
    w: &'a mut W,
}
impl<'a> _EM2W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EM3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EM3W<'a> {
    w: &'a mut W,
}
impl<'a> _EM3W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `EMC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC0R {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the\r\ncorresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is\r\nLOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1\r\n(CT16Bn_MAT0 pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding\r\nExternal Match bit/output."]
    TOGGLE,
}
impl crate::ToBits<u8> for EMC0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EMC0R::NOP => 0,
            EMC0R::CLEAR => 1,
            EMC0R::SET => 2,
            EMC0R::TOGGLE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EMC0_R = crate::FR<u8, EMC0R>;
impl EMC0_R {
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == EMC0R::NOP
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC0R::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC0R::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC0R::TOGGLE
    }
}
#[doc = "Values that can be written to the field `EMC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC0W {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the\r\ncorresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is\r\nLOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1\r\n(CT16Bn_MAT0 pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding\r\nExternal Match bit/output."]
    TOGGLE,
}
impl EMC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC0W::NOP => 0,
            EMC0W::CLEAR => 1,
            EMC0W::SET => 2,
            EMC0W::TOGGLE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMC0W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(EMC0W::NOP)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC0W::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (CT16Bn_MAT0 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC0W::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC0W::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `EMC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC1R {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the\r\ncorresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is\r\nLOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1\r\n(CT16Bn_MAT0 pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding\r\nExternal Match bit/output."]
    TOGGLE,
}
impl crate::ToBits<u8> for EMC1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EMC1R::NOP => 0,
            EMC1R::CLEAR => 1,
            EMC1R::SET => 2,
            EMC1R::TOGGLE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EMC1_R = crate::FR<u8, EMC1R>;
impl EMC1_R {
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == EMC1R::NOP
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC1R::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC1R::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC1R::TOGGLE
    }
}
#[doc = "Values that can be written to the field `EMC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC1W {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the\r\ncorresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is\r\nLOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1\r\n(CT16Bn_MAT0 pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding\r\nExternal Match bit/output."]
    TOGGLE,
}
impl EMC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC1W::NOP => 0,
            EMC1W::CLEAR => 1,
            EMC1W::SET => 2,
            EMC1W::TOGGLE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMC1W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(EMC1W::NOP)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC1W::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (CT16Bn_MAT0 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC1W::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC1W::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `EMC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC2R {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the\r\ncorresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is\r\nLOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1\r\n(CT16Bn_MAT0 pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding\r\nExternal Match bit/output."]
    TOGGLE,
}
impl crate::ToBits<u8> for EMC2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EMC2R::NOP => 0,
            EMC2R::CLEAR => 1,
            EMC2R::SET => 2,
            EMC2R::TOGGLE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EMC2_R = crate::FR<u8, EMC2R>;
impl EMC2_R {
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == EMC2R::NOP
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC2R::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC2R::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC2R::TOGGLE
    }
}
#[doc = "Values that can be written to the field `EMC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC2W {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the\r\ncorresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is\r\nLOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1\r\n(CT16Bn_MAT0 pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding\r\nExternal Match bit/output."]
    TOGGLE,
}
impl EMC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC2W::NOP => 0,
            EMC2W::CLEAR => 1,
            EMC2W::SET => 2,
            EMC2W::TOGGLE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMC2W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(EMC2W::NOP)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC2W::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (CT16Bn_MAT0 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC2W::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC2W::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `EMC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC3R {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the\r\ncorresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is\r\nLOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1\r\n(CT16Bn_MAT0 pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding\r\nExternal Match bit/output."]
    TOGGLE,
}
impl crate::ToBits<u8> for EMC3R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EMC3R::NOP => 0,
            EMC3R::CLEAR => 1,
            EMC3R::SET => 2,
            EMC3R::TOGGLE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EMC3_R = crate::FR<u8, EMC3R>;
impl EMC3_R {
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == EMC3R::NOP
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC3R::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC3R::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC3R::TOGGLE
    }
}
#[doc = "Values that can be written to the field `EMC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC3W {
    #[doc = "Do Nothing."]
    NOP,
    #[doc = "Clear. Clear the\r\ncorresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is\r\nLOW if pinned out)."]
    CLEAR,
    #[doc = "Set. Set the corresponding External Match bit/output to 1\r\n(CT16Bn_MAT0 pin is HIGH if pinned out)."]
    SET,
    #[doc = "Toggle. Toggle the corresponding\r\nExternal Match bit/output."]
    TOGGLE,
}
impl EMC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC3W::NOP => 0,
            EMC3W::CLEAR => 1,
            EMC3W::SET => 2,
            EMC3W::TOGGLE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMC3W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(EMC3W::NOP)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (CT16Bn_MAT0 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC3W::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (CT16Bn_MAT0 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC3W::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC3W::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - External Match 0. This bit reflects the state of output CT16B0_MAT0/CT16B1_MAT0, whether or not this output is connected to its pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[5:4\\] control the functionality of this output. This bit is driven to the CT16B0_MAT0/CT16B1_MAT0 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Match 1. This bit reflects the state of output CT16B0_MAT1/CT16B1_MAT1, whether or not this output is connected to its pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[7:6\\] control the functionality of this output. This bit is driven to the CT16B0_MAT0/CT16B1_MAT0 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Match 2. This bit reflects the state of match channel 2. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[9:8\\] control the functionality of this output."]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Match 3. This bit reflects the state of output of match channel 3. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[11:10\\] control the functionality of this output."]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0. Table 267 shows the encoding of these bits."]
    #[inline(always)]
    pub fn emc0(&self) -> EMC0_R {
        EMC0_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&self) -> EMC1_R {
        EMC1_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&self) -> EMC2_R {
        EMC2_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&self) -> EMC3_R {
        EMC3_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - External Match 0. This bit reflects the state of output CT16B0_MAT0/CT16B1_MAT0, whether or not this output is connected to its pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[5:4\\] control the functionality of this output. This bit is driven to the CT16B0_MAT0/CT16B1_MAT0 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    pub fn em0(&mut self) -> _EM0W {
        _EM0W { w: self }
    }
    #[doc = "Bit 1 - External Match 1. This bit reflects the state of output CT16B0_MAT1/CT16B1_MAT1, whether or not this output is connected to its pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[7:6\\] control the functionality of this output. This bit is driven to the CT16B0_MAT0/CT16B1_MAT0 pins if the match function is selected in the IOCON registers (0 = LOW, 1 = HIGH)."]
    #[inline(always)]
    pub fn em1(&mut self) -> _EM1W {
        _EM1W { w: self }
    }
    #[doc = "Bit 2 - External Match 2. This bit reflects the state of match channel 2. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[9:8\\] control the functionality of this output."]
    #[inline(always)]
    pub fn em2(&mut self) -> _EM2W {
        _EM2W { w: self }
    }
    #[doc = "Bit 3 - External Match 3. This bit reflects the state of output of match channel 3. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing. Bits EMR\\[11:10\\] control the functionality of this output."]
    #[inline(always)]
    pub fn em3(&mut self) -> _EM3W {
        _EM3W { w: self }
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0. Table 267 shows the encoding of these bits."]
    #[inline(always)]
    pub fn emc0(&mut self) -> _EMC0W {
        _EMC0W { w: self }
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&mut self) -> _EMC1W {
        _EMC1W { w: self }
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&mut self) -> _EMC2W {
        _EMC2W { w: self }
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&mut self) -> _EMC3W {
        _EMC3W { w: self }
    }
}
