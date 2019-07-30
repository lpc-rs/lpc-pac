#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMCFG {
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
pub type PROD_ENDPTS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PROD_ENDPTSW<'a> {
    w: &'a mut W,
}
impl<'a> _PROD_ENDPTSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Possible values of the field `CFG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG0R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl crate::ToBits<u8> for CFG0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG0R::CONSTANT_HIGH_THIS_ => 0,
            CFG0R::STICKY_RISING_EDGEMA => 1,
            CFG0R::STICKY_FALLING_EDGE_ => 2,
            CFG0R::STICKY_RISING_OR_FAL => 3,
            CFG0R::HIGH_LEVEL_MATCH_F => 4,
            CFG0R::LOW_LEVEL_MATCH_OCC => 5,
            CFG0R::CONSTANT_0_THIS_BIT => 6,
            CFG0R::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG0_R = crate::FR<u8, CFG0R>;
impl CFG0_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH_THIS_`"]
    #[inline(always)]
    pub fn is_constant_high_this_(&self) -> bool {
        *self == CFG0R::CONSTANT_HIGH_THIS_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGEMA`"]
    #[inline(always)]
    pub fn is_sticky_rising_edgema(&self) -> bool {
        *self == CFG0R::STICKY_RISING_EDGEMA
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE_`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge_(&self) -> bool {
        *self == CFG0R::STICKY_FALLING_EDGE_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_OR_FAL`"]
    #[inline(always)]
    pub fn is_sticky_rising_or_fal(&self) -> bool {
        *self == CFG0R::STICKY_RISING_OR_FAL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL_MATCH_F`"]
    #[inline(always)]
    pub fn is_high_level_match_f(&self) -> bool {
        *self == CFG0R::HIGH_LEVEL_MATCH_F
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_MATCH_OCC`"]
    #[inline(always)]
    pub fn is_low_level_match_occ(&self) -> bool {
        *self == CFG0R::LOW_LEVEL_MATCH_OCC
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0_THIS_BIT`"]
    #[inline(always)]
    pub fn is_constant_0_this_bit(&self) -> bool {
        *self == CFG0R::CONSTANT_0_THIS_BIT
    }
    #[doc = "Checks if the value of the field is `EVENT_NON_STICKY_RI`"]
    #[inline(always)]
    pub fn is_event_non_sticky_ri(&self) -> bool {
        *self == CFG0R::EVENT_NON_STICKY_RI
    }
}
#[doc = "Values that can be written to the field `CFG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG0W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl CFG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG0W::CONSTANT_HIGH_THIS_ => 0,
            CFG0W::STICKY_RISING_EDGEMA => 1,
            CFG0W::STICKY_FALLING_EDGE_ => 2,
            CFG0W::STICKY_RISING_OR_FAL => 3,
            CFG0W::HIGH_LEVEL_MATCH_F => 4,
            CFG0W::LOW_LEVEL_MATCH_OCC => 5,
            CFG0W::CONSTANT_0_THIS_BIT => 6,
            CFG0W::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high_this_(self) -> &'a mut W {
        self.variant(CFG0W::CONSTANT_HIGH_THIS_)
    }
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edgema(self) -> &'a mut W {
        self.variant(CFG0W::STICKY_RISING_EDGEMA)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge_(self) -> &'a mut W {
        self.variant(CFG0W::STICKY_FALLING_EDGE_)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_or_fal(self) -> &'a mut W {
        self.variant(CFG0W::STICKY_RISING_OR_FAL)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level_match_f(self) -> &'a mut W {
        self.variant(CFG0W::HIGH_LEVEL_MATCH_F)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level_match_occ(self) -> &'a mut W {
        self.variant(CFG0W::LOW_LEVEL_MATCH_OCC)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_0_this_bit(self) -> &'a mut W {
        self.variant(CFG0W::CONSTANT_0_THIS_BIT)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event_non_sticky_ri(self) -> &'a mut W {
        self.variant(CFG0W::EVENT_NON_STICKY_RI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `CFG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG1R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl crate::ToBits<u8> for CFG1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG1R::CONSTANT_HIGH_THIS_ => 0,
            CFG1R::STICKY_RISING_EDGEMA => 1,
            CFG1R::STICKY_FALLING_EDGE_ => 2,
            CFG1R::STICKY_RISING_OR_FAL => 3,
            CFG1R::HIGH_LEVEL_MATCH_F => 4,
            CFG1R::LOW_LEVEL_MATCH_OCC => 5,
            CFG1R::CONSTANT_0_THIS_BIT => 6,
            CFG1R::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG1_R = crate::FR<u8, CFG1R>;
impl CFG1_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH_THIS_`"]
    #[inline(always)]
    pub fn is_constant_high_this_(&self) -> bool {
        *self == CFG1R::CONSTANT_HIGH_THIS_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGEMA`"]
    #[inline(always)]
    pub fn is_sticky_rising_edgema(&self) -> bool {
        *self == CFG1R::STICKY_RISING_EDGEMA
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE_`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge_(&self) -> bool {
        *self == CFG1R::STICKY_FALLING_EDGE_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_OR_FAL`"]
    #[inline(always)]
    pub fn is_sticky_rising_or_fal(&self) -> bool {
        *self == CFG1R::STICKY_RISING_OR_FAL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL_MATCH_F`"]
    #[inline(always)]
    pub fn is_high_level_match_f(&self) -> bool {
        *self == CFG1R::HIGH_LEVEL_MATCH_F
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_MATCH_OCC`"]
    #[inline(always)]
    pub fn is_low_level_match_occ(&self) -> bool {
        *self == CFG1R::LOW_LEVEL_MATCH_OCC
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0_THIS_BIT`"]
    #[inline(always)]
    pub fn is_constant_0_this_bit(&self) -> bool {
        *self == CFG1R::CONSTANT_0_THIS_BIT
    }
    #[doc = "Checks if the value of the field is `EVENT_NON_STICKY_RI`"]
    #[inline(always)]
    pub fn is_event_non_sticky_ri(&self) -> bool {
        *self == CFG1R::EVENT_NON_STICKY_RI
    }
}
#[doc = "Values that can be written to the field `CFG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG1W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl CFG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG1W::CONSTANT_HIGH_THIS_ => 0,
            CFG1W::STICKY_RISING_EDGEMA => 1,
            CFG1W::STICKY_FALLING_EDGE_ => 2,
            CFG1W::STICKY_RISING_OR_FAL => 3,
            CFG1W::HIGH_LEVEL_MATCH_F => 4,
            CFG1W::LOW_LEVEL_MATCH_OCC => 5,
            CFG1W::CONSTANT_0_THIS_BIT => 6,
            CFG1W::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high_this_(self) -> &'a mut W {
        self.variant(CFG1W::CONSTANT_HIGH_THIS_)
    }
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edgema(self) -> &'a mut W {
        self.variant(CFG1W::STICKY_RISING_EDGEMA)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge_(self) -> &'a mut W {
        self.variant(CFG1W::STICKY_FALLING_EDGE_)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_or_fal(self) -> &'a mut W {
        self.variant(CFG1W::STICKY_RISING_OR_FAL)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level_match_f(self) -> &'a mut W {
        self.variant(CFG1W::HIGH_LEVEL_MATCH_F)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level_match_occ(self) -> &'a mut W {
        self.variant(CFG1W::LOW_LEVEL_MATCH_OCC)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_0_this_bit(self) -> &'a mut W {
        self.variant(CFG1W::CONSTANT_0_THIS_BIT)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event_non_sticky_ri(self) -> &'a mut W {
        self.variant(CFG1W::EVENT_NON_STICKY_RI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `CFG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG2R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl crate::ToBits<u8> for CFG2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG2R::CONSTANT_HIGH_THIS_ => 0,
            CFG2R::STICKY_RISING_EDGEMA => 1,
            CFG2R::STICKY_FALLING_EDGE_ => 2,
            CFG2R::STICKY_RISING_OR_FAL => 3,
            CFG2R::HIGH_LEVEL_MATCH_F => 4,
            CFG2R::LOW_LEVEL_MATCH_OCC => 5,
            CFG2R::CONSTANT_0_THIS_BIT => 6,
            CFG2R::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG2_R = crate::FR<u8, CFG2R>;
impl CFG2_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH_THIS_`"]
    #[inline(always)]
    pub fn is_constant_high_this_(&self) -> bool {
        *self == CFG2R::CONSTANT_HIGH_THIS_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGEMA`"]
    #[inline(always)]
    pub fn is_sticky_rising_edgema(&self) -> bool {
        *self == CFG2R::STICKY_RISING_EDGEMA
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE_`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge_(&self) -> bool {
        *self == CFG2R::STICKY_FALLING_EDGE_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_OR_FAL`"]
    #[inline(always)]
    pub fn is_sticky_rising_or_fal(&self) -> bool {
        *self == CFG2R::STICKY_RISING_OR_FAL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL_MATCH_F`"]
    #[inline(always)]
    pub fn is_high_level_match_f(&self) -> bool {
        *self == CFG2R::HIGH_LEVEL_MATCH_F
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_MATCH_OCC`"]
    #[inline(always)]
    pub fn is_low_level_match_occ(&self) -> bool {
        *self == CFG2R::LOW_LEVEL_MATCH_OCC
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0_THIS_BIT`"]
    #[inline(always)]
    pub fn is_constant_0_this_bit(&self) -> bool {
        *self == CFG2R::CONSTANT_0_THIS_BIT
    }
    #[doc = "Checks if the value of the field is `EVENT_NON_STICKY_RI`"]
    #[inline(always)]
    pub fn is_event_non_sticky_ri(&self) -> bool {
        *self == CFG2R::EVENT_NON_STICKY_RI
    }
}
#[doc = "Values that can be written to the field `CFG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG2W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl CFG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG2W::CONSTANT_HIGH_THIS_ => 0,
            CFG2W::STICKY_RISING_EDGEMA => 1,
            CFG2W::STICKY_FALLING_EDGE_ => 2,
            CFG2W::STICKY_RISING_OR_FAL => 3,
            CFG2W::HIGH_LEVEL_MATCH_F => 4,
            CFG2W::LOW_LEVEL_MATCH_OCC => 5,
            CFG2W::CONSTANT_0_THIS_BIT => 6,
            CFG2W::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG2W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high_this_(self) -> &'a mut W {
        self.variant(CFG2W::CONSTANT_HIGH_THIS_)
    }
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edgema(self) -> &'a mut W {
        self.variant(CFG2W::STICKY_RISING_EDGEMA)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge_(self) -> &'a mut W {
        self.variant(CFG2W::STICKY_FALLING_EDGE_)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_or_fal(self) -> &'a mut W {
        self.variant(CFG2W::STICKY_RISING_OR_FAL)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level_match_f(self) -> &'a mut W {
        self.variant(CFG2W::HIGH_LEVEL_MATCH_F)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level_match_occ(self) -> &'a mut W {
        self.variant(CFG2W::LOW_LEVEL_MATCH_OCC)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_0_this_bit(self) -> &'a mut W {
        self.variant(CFG2W::CONSTANT_0_THIS_BIT)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event_non_sticky_ri(self) -> &'a mut W {
        self.variant(CFG2W::EVENT_NON_STICKY_RI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `CFG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG3R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl crate::ToBits<u8> for CFG3R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG3R::CONSTANT_HIGH_THIS_ => 0,
            CFG3R::STICKY_RISING_EDGEMA => 1,
            CFG3R::STICKY_FALLING_EDGE_ => 2,
            CFG3R::STICKY_RISING_OR_FAL => 3,
            CFG3R::HIGH_LEVEL_MATCH_F => 4,
            CFG3R::LOW_LEVEL_MATCH_OCC => 5,
            CFG3R::CONSTANT_0_THIS_BIT => 6,
            CFG3R::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG3_R = crate::FR<u8, CFG3R>;
impl CFG3_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH_THIS_`"]
    #[inline(always)]
    pub fn is_constant_high_this_(&self) -> bool {
        *self == CFG3R::CONSTANT_HIGH_THIS_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGEMA`"]
    #[inline(always)]
    pub fn is_sticky_rising_edgema(&self) -> bool {
        *self == CFG3R::STICKY_RISING_EDGEMA
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE_`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge_(&self) -> bool {
        *self == CFG3R::STICKY_FALLING_EDGE_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_OR_FAL`"]
    #[inline(always)]
    pub fn is_sticky_rising_or_fal(&self) -> bool {
        *self == CFG3R::STICKY_RISING_OR_FAL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL_MATCH_F`"]
    #[inline(always)]
    pub fn is_high_level_match_f(&self) -> bool {
        *self == CFG3R::HIGH_LEVEL_MATCH_F
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_MATCH_OCC`"]
    #[inline(always)]
    pub fn is_low_level_match_occ(&self) -> bool {
        *self == CFG3R::LOW_LEVEL_MATCH_OCC
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0_THIS_BIT`"]
    #[inline(always)]
    pub fn is_constant_0_this_bit(&self) -> bool {
        *self == CFG3R::CONSTANT_0_THIS_BIT
    }
    #[doc = "Checks if the value of the field is `EVENT_NON_STICKY_RI`"]
    #[inline(always)]
    pub fn is_event_non_sticky_ri(&self) -> bool {
        *self == CFG3R::EVENT_NON_STICKY_RI
    }
}
#[doc = "Values that can be written to the field `CFG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG3W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl CFG3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG3W::CONSTANT_HIGH_THIS_ => 0,
            CFG3W::STICKY_RISING_EDGEMA => 1,
            CFG3W::STICKY_FALLING_EDGE_ => 2,
            CFG3W::STICKY_RISING_OR_FAL => 3,
            CFG3W::HIGH_LEVEL_MATCH_F => 4,
            CFG3W::LOW_LEVEL_MATCH_OCC => 5,
            CFG3W::CONSTANT_0_THIS_BIT => 6,
            CFG3W::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG3W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high_this_(self) -> &'a mut W {
        self.variant(CFG3W::CONSTANT_HIGH_THIS_)
    }
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edgema(self) -> &'a mut W {
        self.variant(CFG3W::STICKY_RISING_EDGEMA)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge_(self) -> &'a mut W {
        self.variant(CFG3W::STICKY_FALLING_EDGE_)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_or_fal(self) -> &'a mut W {
        self.variant(CFG3W::STICKY_RISING_OR_FAL)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level_match_f(self) -> &'a mut W {
        self.variant(CFG3W::HIGH_LEVEL_MATCH_F)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level_match_occ(self) -> &'a mut W {
        self.variant(CFG3W::LOW_LEVEL_MATCH_OCC)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_0_this_bit(self) -> &'a mut W {
        self.variant(CFG3W::CONSTANT_0_THIS_BIT)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event_non_sticky_ri(self) -> &'a mut W {
        self.variant(CFG3W::EVENT_NON_STICKY_RI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `CFG4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG4R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl crate::ToBits<u8> for CFG4R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG4R::CONSTANT_HIGH_THIS_ => 0,
            CFG4R::STICKY_RISING_EDGEMA => 1,
            CFG4R::STICKY_FALLING_EDGE_ => 2,
            CFG4R::STICKY_RISING_OR_FAL => 3,
            CFG4R::HIGH_LEVEL_MATCH_F => 4,
            CFG4R::LOW_LEVEL_MATCH_OCC => 5,
            CFG4R::CONSTANT_0_THIS_BIT => 6,
            CFG4R::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG4_R = crate::FR<u8, CFG4R>;
impl CFG4_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH_THIS_`"]
    #[inline(always)]
    pub fn is_constant_high_this_(&self) -> bool {
        *self == CFG4R::CONSTANT_HIGH_THIS_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGEMA`"]
    #[inline(always)]
    pub fn is_sticky_rising_edgema(&self) -> bool {
        *self == CFG4R::STICKY_RISING_EDGEMA
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE_`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge_(&self) -> bool {
        *self == CFG4R::STICKY_FALLING_EDGE_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_OR_FAL`"]
    #[inline(always)]
    pub fn is_sticky_rising_or_fal(&self) -> bool {
        *self == CFG4R::STICKY_RISING_OR_FAL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL_MATCH_F`"]
    #[inline(always)]
    pub fn is_high_level_match_f(&self) -> bool {
        *self == CFG4R::HIGH_LEVEL_MATCH_F
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_MATCH_OCC`"]
    #[inline(always)]
    pub fn is_low_level_match_occ(&self) -> bool {
        *self == CFG4R::LOW_LEVEL_MATCH_OCC
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0_THIS_BIT`"]
    #[inline(always)]
    pub fn is_constant_0_this_bit(&self) -> bool {
        *self == CFG4R::CONSTANT_0_THIS_BIT
    }
    #[doc = "Checks if the value of the field is `EVENT_NON_STICKY_RI`"]
    #[inline(always)]
    pub fn is_event_non_sticky_ri(&self) -> bool {
        *self == CFG4R::EVENT_NON_STICKY_RI
    }
}
#[doc = "Values that can be written to the field `CFG4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG4W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl CFG4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG4W::CONSTANT_HIGH_THIS_ => 0,
            CFG4W::STICKY_RISING_EDGEMA => 1,
            CFG4W::STICKY_FALLING_EDGE_ => 2,
            CFG4W::STICKY_RISING_OR_FAL => 3,
            CFG4W::HIGH_LEVEL_MATCH_F => 4,
            CFG4W::LOW_LEVEL_MATCH_OCC => 5,
            CFG4W::CONSTANT_0_THIS_BIT => 6,
            CFG4W::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG4W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high_this_(self) -> &'a mut W {
        self.variant(CFG4W::CONSTANT_HIGH_THIS_)
    }
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edgema(self) -> &'a mut W {
        self.variant(CFG4W::STICKY_RISING_EDGEMA)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge_(self) -> &'a mut W {
        self.variant(CFG4W::STICKY_FALLING_EDGE_)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_or_fal(self) -> &'a mut W {
        self.variant(CFG4W::STICKY_RISING_OR_FAL)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level_match_f(self) -> &'a mut W {
        self.variant(CFG4W::HIGH_LEVEL_MATCH_F)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level_match_occ(self) -> &'a mut W {
        self.variant(CFG4W::LOW_LEVEL_MATCH_OCC)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_0_this_bit(self) -> &'a mut W {
        self.variant(CFG4W::CONSTANT_0_THIS_BIT)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event_non_sticky_ri(self) -> &'a mut W {
        self.variant(CFG4W::EVENT_NON_STICKY_RI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `CFG5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG5R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl crate::ToBits<u8> for CFG5R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG5R::CONSTANT_HIGH_THIS_ => 0,
            CFG5R::STICKY_RISING_EDGEMA => 1,
            CFG5R::STICKY_FALLING_EDGE_ => 2,
            CFG5R::STICKY_RISING_OR_FAL => 3,
            CFG5R::HIGH_LEVEL_MATCH_F => 4,
            CFG5R::LOW_LEVEL_MATCH_OCC => 5,
            CFG5R::CONSTANT_0_THIS_BIT => 6,
            CFG5R::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG5_R = crate::FR<u8, CFG5R>;
impl CFG5_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH_THIS_`"]
    #[inline(always)]
    pub fn is_constant_high_this_(&self) -> bool {
        *self == CFG5R::CONSTANT_HIGH_THIS_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGEMA`"]
    #[inline(always)]
    pub fn is_sticky_rising_edgema(&self) -> bool {
        *self == CFG5R::STICKY_RISING_EDGEMA
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE_`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge_(&self) -> bool {
        *self == CFG5R::STICKY_FALLING_EDGE_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_OR_FAL`"]
    #[inline(always)]
    pub fn is_sticky_rising_or_fal(&self) -> bool {
        *self == CFG5R::STICKY_RISING_OR_FAL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL_MATCH_F`"]
    #[inline(always)]
    pub fn is_high_level_match_f(&self) -> bool {
        *self == CFG5R::HIGH_LEVEL_MATCH_F
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_MATCH_OCC`"]
    #[inline(always)]
    pub fn is_low_level_match_occ(&self) -> bool {
        *self == CFG5R::LOW_LEVEL_MATCH_OCC
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0_THIS_BIT`"]
    #[inline(always)]
    pub fn is_constant_0_this_bit(&self) -> bool {
        *self == CFG5R::CONSTANT_0_THIS_BIT
    }
    #[doc = "Checks if the value of the field is `EVENT_NON_STICKY_RI`"]
    #[inline(always)]
    pub fn is_event_non_sticky_ri(&self) -> bool {
        *self == CFG5R::EVENT_NON_STICKY_RI
    }
}
#[doc = "Values that can be written to the field `CFG5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG5W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl CFG5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG5W::CONSTANT_HIGH_THIS_ => 0,
            CFG5W::STICKY_RISING_EDGEMA => 1,
            CFG5W::STICKY_FALLING_EDGE_ => 2,
            CFG5W::STICKY_RISING_OR_FAL => 3,
            CFG5W::HIGH_LEVEL_MATCH_F => 4,
            CFG5W::LOW_LEVEL_MATCH_OCC => 5,
            CFG5W::CONSTANT_0_THIS_BIT => 6,
            CFG5W::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG5W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high_this_(self) -> &'a mut W {
        self.variant(CFG5W::CONSTANT_HIGH_THIS_)
    }
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edgema(self) -> &'a mut W {
        self.variant(CFG5W::STICKY_RISING_EDGEMA)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge_(self) -> &'a mut W {
        self.variant(CFG5W::STICKY_FALLING_EDGE_)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_or_fal(self) -> &'a mut W {
        self.variant(CFG5W::STICKY_RISING_OR_FAL)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level_match_f(self) -> &'a mut W {
        self.variant(CFG5W::HIGH_LEVEL_MATCH_F)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level_match_occ(self) -> &'a mut W {
        self.variant(CFG5W::LOW_LEVEL_MATCH_OCC)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_0_this_bit(self) -> &'a mut W {
        self.variant(CFG5W::CONSTANT_0_THIS_BIT)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event_non_sticky_ri(self) -> &'a mut W {
        self.variant(CFG5W::EVENT_NON_STICKY_RI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `CFG6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG6R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl crate::ToBits<u8> for CFG6R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG6R::CONSTANT_HIGH_THIS_ => 0,
            CFG6R::STICKY_RISING_EDGEMA => 1,
            CFG6R::STICKY_FALLING_EDGE_ => 2,
            CFG6R::STICKY_RISING_OR_FAL => 3,
            CFG6R::HIGH_LEVEL_MATCH_F => 4,
            CFG6R::LOW_LEVEL_MATCH_OCC => 5,
            CFG6R::CONSTANT_0_THIS_BIT => 6,
            CFG6R::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG6_R = crate::FR<u8, CFG6R>;
impl CFG6_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH_THIS_`"]
    #[inline(always)]
    pub fn is_constant_high_this_(&self) -> bool {
        *self == CFG6R::CONSTANT_HIGH_THIS_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGEMA`"]
    #[inline(always)]
    pub fn is_sticky_rising_edgema(&self) -> bool {
        *self == CFG6R::STICKY_RISING_EDGEMA
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE_`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge_(&self) -> bool {
        *self == CFG6R::STICKY_FALLING_EDGE_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_OR_FAL`"]
    #[inline(always)]
    pub fn is_sticky_rising_or_fal(&self) -> bool {
        *self == CFG6R::STICKY_RISING_OR_FAL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL_MATCH_F`"]
    #[inline(always)]
    pub fn is_high_level_match_f(&self) -> bool {
        *self == CFG6R::HIGH_LEVEL_MATCH_F
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_MATCH_OCC`"]
    #[inline(always)]
    pub fn is_low_level_match_occ(&self) -> bool {
        *self == CFG6R::LOW_LEVEL_MATCH_OCC
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0_THIS_BIT`"]
    #[inline(always)]
    pub fn is_constant_0_this_bit(&self) -> bool {
        *self == CFG6R::CONSTANT_0_THIS_BIT
    }
    #[doc = "Checks if the value of the field is `EVENT_NON_STICKY_RI`"]
    #[inline(always)]
    pub fn is_event_non_sticky_ri(&self) -> bool {
        *self == CFG6R::EVENT_NON_STICKY_RI
    }
}
#[doc = "Values that can be written to the field `CFG6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG6W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl CFG6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG6W::CONSTANT_HIGH_THIS_ => 0,
            CFG6W::STICKY_RISING_EDGEMA => 1,
            CFG6W::STICKY_FALLING_EDGE_ => 2,
            CFG6W::STICKY_RISING_OR_FAL => 3,
            CFG6W::HIGH_LEVEL_MATCH_F => 4,
            CFG6W::LOW_LEVEL_MATCH_OCC => 5,
            CFG6W::CONSTANT_0_THIS_BIT => 6,
            CFG6W::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG6W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high_this_(self) -> &'a mut W {
        self.variant(CFG6W::CONSTANT_HIGH_THIS_)
    }
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edgema(self) -> &'a mut W {
        self.variant(CFG6W::STICKY_RISING_EDGEMA)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge_(self) -> &'a mut W {
        self.variant(CFG6W::STICKY_FALLING_EDGE_)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_or_fal(self) -> &'a mut W {
        self.variant(CFG6W::STICKY_RISING_OR_FAL)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level_match_f(self) -> &'a mut W {
        self.variant(CFG6W::HIGH_LEVEL_MATCH_F)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level_match_occ(self) -> &'a mut W {
        self.variant(CFG6W::LOW_LEVEL_MATCH_OCC)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_0_this_bit(self) -> &'a mut W {
        self.variant(CFG6W::CONSTANT_0_THIS_BIT)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event_non_sticky_ri(self) -> &'a mut W {
        self.variant(CFG6W::EVENT_NON_STICKY_RI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `CFG7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG7R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl crate::ToBits<u8> for CFG7R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG7R::CONSTANT_HIGH_THIS_ => 0,
            CFG7R::STICKY_RISING_EDGEMA => 1,
            CFG7R::STICKY_FALLING_EDGE_ => 2,
            CFG7R::STICKY_RISING_OR_FAL => 3,
            CFG7R::HIGH_LEVEL_MATCH_F => 4,
            CFG7R::LOW_LEVEL_MATCH_OCC => 5,
            CFG7R::CONSTANT_0_THIS_BIT => 6,
            CFG7R::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG7_R = crate::FR<u8, CFG7R>;
impl CFG7_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH_THIS_`"]
    #[inline(always)]
    pub fn is_constant_high_this_(&self) -> bool {
        *self == CFG7R::CONSTANT_HIGH_THIS_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGEMA`"]
    #[inline(always)]
    pub fn is_sticky_rising_edgema(&self) -> bool {
        *self == CFG7R::STICKY_RISING_EDGEMA
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE_`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge_(&self) -> bool {
        *self == CFG7R::STICKY_FALLING_EDGE_
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_OR_FAL`"]
    #[inline(always)]
    pub fn is_sticky_rising_or_fal(&self) -> bool {
        *self == CFG7R::STICKY_RISING_OR_FAL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL_MATCH_F`"]
    #[inline(always)]
    pub fn is_high_level_match_f(&self) -> bool {
        *self == CFG7R::HIGH_LEVEL_MATCH_F
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL_MATCH_OCC`"]
    #[inline(always)]
    pub fn is_low_level_match_occ(&self) -> bool {
        *self == CFG7R::LOW_LEVEL_MATCH_OCC
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0_THIS_BIT`"]
    #[inline(always)]
    pub fn is_constant_0_this_bit(&self) -> bool {
        *self == CFG7R::CONSTANT_0_THIS_BIT
    }
    #[doc = "Checks if the value of the field is `EVENT_NON_STICKY_RI`"]
    #[inline(always)]
    pub fn is_event_non_sticky_ri(&self) -> bool {
        *self == CFG7R::EVENT_NON_STICKY_RI
    }
}
#[doc = "Values that can be written to the field `CFG7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG7W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH_THIS_,
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGEMA,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE_,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_OR_FAL,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL_MATCH_F,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL_MATCH_OCC,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_0_THIS_BIT,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    EVENT_NON_STICKY_RI,
}
impl CFG7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG7W::CONSTANT_HIGH_THIS_ => 0,
            CFG7W::STICKY_RISING_EDGEMA => 1,
            CFG7W::STICKY_FALLING_EDGE_ => 2,
            CFG7W::STICKY_RISING_OR_FAL => 3,
            CFG7W::HIGH_LEVEL_MATCH_F => 4,
            CFG7W::LOW_LEVEL_MATCH_OCC => 5,
            CFG7W::CONSTANT_0_THIS_BIT => 6,
            CFG7W::EVENT_NON_STICKY_RI => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG7W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high_this_(self) -> &'a mut W {
        self.variant(CFG7W::CONSTANT_HIGH_THIS_)
    }
    #[doc = "Sticky rising edgeMatch occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edgema(self) -> &'a mut W {
        self.variant(CFG7W::STICKY_RISING_EDGEMA)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge_(self) -> &'a mut W {
        self.variant(CFG7W::STICKY_FALLING_EDGE_)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_or_fal(self) -> &'a mut W {
        self.variant(CFG7W::STICKY_RISING_OR_FAL)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level_match_f(self) -> &'a mut W {
        self.variant(CFG7W::HIGH_LEVEL_MATCH_F)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level_match_occ(self) -> &'a mut W {
        self.variant(CFG7W::LOW_LEVEL_MATCH_OCC)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_0_this_bit(self) -> &'a mut W {
        self.variant(CFG7W::CONSTANT_0_THIS_BIT)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of option 3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event_non_sticky_ri(self) -> &'a mut W {
        self.variant(CFG7W::EVENT_NON_STICKY_RI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - A 1 in any bit of this field causes the corresponding bit slice to be the final component of a product term in the boolean expression. This has two effects: 1. The interrupt request associated with this bit-slice will be asserted whenever a match to that product term is detected. 2. The next bit slice will start a new, independent product term in the boolean expression (i.e. an OR will be inserted in the boolean expression following the element controlled by this bit slice)."]
    #[inline(always)]
    pub fn prod_endpts(&self) -> PROD_ENDPTS_R {
        PROD_ENDPTS_R::new((self.bits() & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub fn cfg0(&self) -> CFG0_R {
        CFG0_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new(((self.bits() >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits() >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits() >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits() >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits() >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits() >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits() >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - A 1 in any bit of this field causes the corresponding bit slice to be the final component of a product term in the boolean expression. This has two effects: 1. The interrupt request associated with this bit-slice will be asserted whenever a match to that product term is detected. 2. The next bit slice will start a new, independent product term in the boolean expression (i.e. an OR will be inserted in the boolean expression following the element controlled by this bit slice)."]
    #[inline(always)]
    pub fn prod_endpts(&mut self) -> _PROD_ENDPTSW {
        _PROD_ENDPTSW { w: self }
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub fn cfg0(&mut self) -> _CFG0W {
        _CFG0W { w: self }
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub fn cfg1(&mut self) -> _CFG1W {
        _CFG1W { w: self }
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub fn cfg2(&mut self) -> _CFG2W {
        _CFG2W { w: self }
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub fn cfg3(&mut self) -> _CFG3W {
        _CFG3W { w: self }
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub fn cfg4(&mut self) -> _CFG4W {
        _CFG4W { w: self }
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub fn cfg5(&mut self) -> _CFG5W {
        _CFG5W { w: self }
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub fn cfg6(&mut self) -> _CFG6W {
        _CFG6W { w: self }
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub fn cfg7(&mut self) -> _CFG7W {
        _CFG7W { w: self }
    }
}
