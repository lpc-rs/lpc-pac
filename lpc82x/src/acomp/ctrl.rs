#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CTRL_SPEC>> for R {
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl core::convert::From<crate::W<CTRL_SPEC>> for W {
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGESEL_A {
    #[doc = "0: Falling edges"]
    FALLING_EDGES = 0,
    #[doc = "1: Rising edges"]
    RISING_EDGES = 1,
    #[doc = "2: Both edges"]
    BOTH_EDGES0 = 2,
    #[doc = "3: Both edges"]
    BOTH_EDGES1 = 3,
}
impl From<EDGESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EDGESEL` reader - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
pub struct EDGESEL_R(crate::FieldReader<u8, EDGESEL_A>);
impl EDGESEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EDGESEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGESEL_A {
        match self.bits {
            0 => EDGESEL_A::FALLING_EDGES,
            1 => EDGESEL_A::RISING_EDGES,
            2 => EDGESEL_A::BOTH_EDGES0,
            3 => EDGESEL_A::BOTH_EDGES1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_falling_edges(&self) -> bool {
        **self == EDGESEL_A::FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `RISING_EDGES`"]
    #[inline(always)]
    pub fn is_rising_edges(&self) -> bool {
        **self == EDGESEL_A::RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES0`"]
    #[inline(always)]
    pub fn is_both_edges0(&self) -> bool {
        **self == EDGESEL_A::BOTH_EDGES0
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES1`"]
    #[inline(always)]
    pub fn is_both_edges1(&self) -> bool {
        **self == EDGESEL_A::BOTH_EDGES1
    }
}
impl core::ops::Deref for EDGESEL_R {
    type Target = crate::FieldReader<u8, EDGESEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGESEL` writer - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
pub struct EDGESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGESEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Falling edges"]
    #[inline(always)]
    pub fn falling_edges(self) -> &'a mut W {
        self.variant(EDGESEL_A::FALLING_EDGES)
    }
    #[doc = "Rising edges"]
    #[inline(always)]
    pub fn rising_edges(self) -> &'a mut W {
        self.variant(EDGESEL_A::RISING_EDGES)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both_edges0(self) -> &'a mut W {
        self.variant(EDGESEL_A::BOTH_EDGES0)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both_edges1(self) -> &'a mut W {
        self.variant(EDGESEL_A::BOTH_EDGES1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Comparator output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPSA_A {
    #[doc = "0: Comparator output is used directly."]
    COMPSA_0 = 0,
    #[doc = "1: Comparator output is synchronized to the bus clock for output to other modules."]
    COMPSA_1 = 1,
}
impl From<COMPSA_A> for bool {
    #[inline(always)]
    fn from(variant: COMPSA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPSA` reader - Comparator output control"]
pub struct COMPSA_R(crate::FieldReader<bool, COMPSA_A>);
impl COMPSA_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPSA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPSA_A {
        match self.bits {
            false => COMPSA_A::COMPSA_0,
            true => COMPSA_A::COMPSA_1,
        }
    }
    #[doc = "Checks if the value of the field is `COMPSA_0`"]
    #[inline(always)]
    pub fn is_compsa_0(&self) -> bool {
        **self == COMPSA_A::COMPSA_0
    }
    #[doc = "Checks if the value of the field is `COMPSA_1`"]
    #[inline(always)]
    pub fn is_compsa_1(&self) -> bool {
        **self == COMPSA_A::COMPSA_1
    }
}
impl core::ops::Deref for COMPSA_R {
    type Target = crate::FieldReader<bool, COMPSA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPSA` writer - Comparator output control"]
pub struct COMPSA_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPSA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPSA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator output is used directly."]
    #[inline(always)]
    pub fn compsa_0(self) -> &'a mut W {
        self.variant(COMPSA_A::COMPSA_0)
    }
    #[doc = "Comparator output is synchronized to the bus clock for output to other modules."]
    #[inline(always)]
    pub fn compsa_1(self) -> &'a mut W {
        self.variant(COMPSA_A::COMPSA_1)
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
#[doc = "Selects positive voltage input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_VP_SEL_A {
    #[doc = "0: VOLTAGE_LADDER_OUTPUT"]
    VOLTAGE_LADDER_OUTPUT = 0,
    #[doc = "1: ACMP_I1"]
    ACMP_I1 = 1,
    #[doc = "2: ACMP_I2"]
    ACMP_I2 = 2,
    #[doc = "3: ACMP_I3"]
    ACMP_I3 = 3,
    #[doc = "4: ACMP_I4"]
    ACMP_I4 = 4,
    #[doc = "5: Band gap. Internal reference voltage."]
    BAND_GAP = 5,
    #[doc = "6: ADC channel 0 input"]
    ADC_0 = 6,
}
impl From<COMP_VP_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_VP_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP_VP_SEL` reader - Selects positive voltage input"]
pub struct COMP_VP_SEL_R(crate::FieldReader<u8, COMP_VP_SEL_A>);
impl COMP_VP_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP_VP_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP_VP_SEL_A> {
        match self.bits {
            0 => Some(COMP_VP_SEL_A::VOLTAGE_LADDER_OUTPUT),
            1 => Some(COMP_VP_SEL_A::ACMP_I1),
            2 => Some(COMP_VP_SEL_A::ACMP_I2),
            3 => Some(COMP_VP_SEL_A::ACMP_I3),
            4 => Some(COMP_VP_SEL_A::ACMP_I4),
            5 => Some(COMP_VP_SEL_A::BAND_GAP),
            6 => Some(COMP_VP_SEL_A::ADC_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_LADDER_OUTPUT`"]
    #[inline(always)]
    pub fn is_voltage_ladder_output(&self) -> bool {
        **self == COMP_VP_SEL_A::VOLTAGE_LADDER_OUTPUT
    }
    #[doc = "Checks if the value of the field is `ACMP_I1`"]
    #[inline(always)]
    pub fn is_acmp_i1(&self) -> bool {
        **self == COMP_VP_SEL_A::ACMP_I1
    }
    #[doc = "Checks if the value of the field is `ACMP_I2`"]
    #[inline(always)]
    pub fn is_acmp_i2(&self) -> bool {
        **self == COMP_VP_SEL_A::ACMP_I2
    }
    #[doc = "Checks if the value of the field is `ACMP_I3`"]
    #[inline(always)]
    pub fn is_acmp_i3(&self) -> bool {
        **self == COMP_VP_SEL_A::ACMP_I3
    }
    #[doc = "Checks if the value of the field is `ACMP_I4`"]
    #[inline(always)]
    pub fn is_acmp_i4(&self) -> bool {
        **self == COMP_VP_SEL_A::ACMP_I4
    }
    #[doc = "Checks if the value of the field is `BAND_GAP`"]
    #[inline(always)]
    pub fn is_band_gap(&self) -> bool {
        **self == COMP_VP_SEL_A::BAND_GAP
    }
    #[doc = "Checks if the value of the field is `ADC_0`"]
    #[inline(always)]
    pub fn is_adc_0(&self) -> bool {
        **self == COMP_VP_SEL_A::ADC_0
    }
}
impl core::ops::Deref for COMP_VP_SEL_R {
    type Target = crate::FieldReader<u8, COMP_VP_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP_VP_SEL` writer - Selects positive voltage input"]
pub struct COMP_VP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_VP_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_VP_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VOLTAGE_LADDER_OUTPUT"]
    #[inline(always)]
    pub fn voltage_ladder_output(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::VOLTAGE_LADDER_OUTPUT)
    }
    #[doc = "ACMP_I1"]
    #[inline(always)]
    pub fn acmp_i1(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ACMP_I1)
    }
    #[doc = "ACMP_I2"]
    #[inline(always)]
    pub fn acmp_i2(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ACMP_I2)
    }
    #[doc = "ACMP_I3"]
    #[inline(always)]
    pub fn acmp_i3(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ACMP_I3)
    }
    #[doc = "ACMP_I4"]
    #[inline(always)]
    pub fn acmp_i4(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ACMP_I4)
    }
    #[doc = "Band gap. Internal reference voltage."]
    #[inline(always)]
    pub fn band_gap(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::BAND_GAP)
    }
    #[doc = "ADC channel 0 input"]
    #[inline(always)]
    pub fn adc_0(self) -> &'a mut W {
        self.variant(COMP_VP_SEL_A::ADC_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Selects negative voltage input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_VM_SEL_A {
    #[doc = "0: VOLTAGE_LADDER_OUTPUT"]
    VOLTAGE_LADDER_OUTPUT = 0,
    #[doc = "1: ACMP_I1"]
    ACMP_I1 = 1,
    #[doc = "2: ACMP_I2"]
    ACMP_I2 = 2,
    #[doc = "3: ACMP_I3"]
    ACMP_I3 = 3,
    #[doc = "4: ACMP_I4"]
    ACMP_I4 = 4,
    #[doc = "5: Band gap. Internal reference voltage."]
    BAND_GAP = 5,
    #[doc = "6: ADC channel 0 input"]
    ADC_0 = 6,
}
impl From<COMP_VM_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_VM_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP_VM_SEL` reader - Selects negative voltage input"]
pub struct COMP_VM_SEL_R(crate::FieldReader<u8, COMP_VM_SEL_A>);
impl COMP_VM_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP_VM_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP_VM_SEL_A> {
        match self.bits {
            0 => Some(COMP_VM_SEL_A::VOLTAGE_LADDER_OUTPUT),
            1 => Some(COMP_VM_SEL_A::ACMP_I1),
            2 => Some(COMP_VM_SEL_A::ACMP_I2),
            3 => Some(COMP_VM_SEL_A::ACMP_I3),
            4 => Some(COMP_VM_SEL_A::ACMP_I4),
            5 => Some(COMP_VM_SEL_A::BAND_GAP),
            6 => Some(COMP_VM_SEL_A::ADC_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VOLTAGE_LADDER_OUTPUT`"]
    #[inline(always)]
    pub fn is_voltage_ladder_output(&self) -> bool {
        **self == COMP_VM_SEL_A::VOLTAGE_LADDER_OUTPUT
    }
    #[doc = "Checks if the value of the field is `ACMP_I1`"]
    #[inline(always)]
    pub fn is_acmp_i1(&self) -> bool {
        **self == COMP_VM_SEL_A::ACMP_I1
    }
    #[doc = "Checks if the value of the field is `ACMP_I2`"]
    #[inline(always)]
    pub fn is_acmp_i2(&self) -> bool {
        **self == COMP_VM_SEL_A::ACMP_I2
    }
    #[doc = "Checks if the value of the field is `ACMP_I3`"]
    #[inline(always)]
    pub fn is_acmp_i3(&self) -> bool {
        **self == COMP_VM_SEL_A::ACMP_I3
    }
    #[doc = "Checks if the value of the field is `ACMP_I4`"]
    #[inline(always)]
    pub fn is_acmp_i4(&self) -> bool {
        **self == COMP_VM_SEL_A::ACMP_I4
    }
    #[doc = "Checks if the value of the field is `BAND_GAP`"]
    #[inline(always)]
    pub fn is_band_gap(&self) -> bool {
        **self == COMP_VM_SEL_A::BAND_GAP
    }
    #[doc = "Checks if the value of the field is `ADC_0`"]
    #[inline(always)]
    pub fn is_adc_0(&self) -> bool {
        **self == COMP_VM_SEL_A::ADC_0
    }
}
impl core::ops::Deref for COMP_VM_SEL_R {
    type Target = crate::FieldReader<u8, COMP_VM_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP_VM_SEL` writer - Selects negative voltage input"]
pub struct COMP_VM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_VM_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_VM_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VOLTAGE_LADDER_OUTPUT"]
    #[inline(always)]
    pub fn voltage_ladder_output(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::VOLTAGE_LADDER_OUTPUT)
    }
    #[doc = "ACMP_I1"]
    #[inline(always)]
    pub fn acmp_i1(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ACMP_I1)
    }
    #[doc = "ACMP_I2"]
    #[inline(always)]
    pub fn acmp_i2(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ACMP_I2)
    }
    #[doc = "ACMP_I3"]
    #[inline(always)]
    pub fn acmp_i3(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ACMP_I3)
    }
    #[doc = "ACMP_I4"]
    #[inline(always)]
    pub fn acmp_i4(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ACMP_I4)
    }
    #[doc = "Band gap. Internal reference voltage."]
    #[inline(always)]
    pub fn band_gap(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::BAND_GAP)
    }
    #[doc = "ADC channel 0 input"]
    #[inline(always)]
    pub fn adc_0(self) -> &'a mut W {
        self.variant(COMP_VM_SEL_A::ADC_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `EDGECLR` reader - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
pub struct EDGECLR_R(crate::FieldReader<bool, bool>);
impl EDGECLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGECLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGECLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGECLR` writer - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
pub struct EDGECLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGECLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `COMPSTAT` reader - Comparator status. This bit reflects the state of the comparator output."]
pub struct COMPSTAT_R(crate::FieldReader<bool, bool>);
impl COMPSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPSTAT` writer - Comparator status. This bit reflects the state of the comparator output."]
pub struct COMPSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `COMPEDGE` reader - Comparator edge-detect status."]
pub struct COMPEDGE_R(crate::FieldReader<bool, bool>);
impl COMPEDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPEDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPEDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPEDGE` writer - Comparator edge-detect status."]
pub struct COMPEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYS_A {
    #[doc = "0: None (the output will switch as the voltages cross)"]
    HYS_0 = 0,
    #[doc = "1: 5 mv"]
    HYS_1 = 1,
    #[doc = "2: 10 mv"]
    HYS_2 = 2,
    #[doc = "3: 20 mv"]
    HYS_3 = 3,
}
impl From<HYS_A> for u8 {
    #[inline(always)]
    fn from(variant: HYS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HYS` reader - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
pub struct HYS_R(crate::FieldReader<u8, HYS_A>);
impl HYS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HYS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYS_A {
        match self.bits {
            0 => HYS_A::HYS_0,
            1 => HYS_A::HYS_1,
            2 => HYS_A::HYS_2,
            3 => HYS_A::HYS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYS_0`"]
    #[inline(always)]
    pub fn is_hys_0(&self) -> bool {
        **self == HYS_A::HYS_0
    }
    #[doc = "Checks if the value of the field is `HYS_1`"]
    #[inline(always)]
    pub fn is_hys_1(&self) -> bool {
        **self == HYS_A::HYS_1
    }
    #[doc = "Checks if the value of the field is `HYS_2`"]
    #[inline(always)]
    pub fn is_hys_2(&self) -> bool {
        **self == HYS_A::HYS_2
    }
    #[doc = "Checks if the value of the field is `HYS_3`"]
    #[inline(always)]
    pub fn is_hys_3(&self) -> bool {
        **self == HYS_A::HYS_3
    }
}
impl core::ops::Deref for HYS_R {
    type Target = crate::FieldReader<u8, HYS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYS` writer - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
pub struct HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "None (the output will switch as the voltages cross)"]
    #[inline(always)]
    pub fn hys_0(self) -> &'a mut W {
        self.variant(HYS_A::HYS_0)
    }
    #[doc = "5 mv"]
    #[inline(always)]
    pub fn hys_1(self) -> &'a mut W {
        self.variant(HYS_A::HYS_1)
    }
    #[doc = "10 mv"]
    #[inline(always)]
    pub fn hys_2(self) -> &'a mut W {
        self.variant(HYS_A::HYS_2)
    }
    #[doc = "20 mv"]
    #[inline(always)]
    pub fn hys_3(self) -> &'a mut W {
        self.variant(HYS_A::HYS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
    #[inline(always)]
    pub fn edgesel(&self) -> EDGESEL_R {
        EDGESEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Comparator output control"]
    #[inline(always)]
    pub fn compsa(&self) -> COMPSA_R {
        COMPSA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Selects positive voltage input"]
    #[inline(always)]
    pub fn comp_vp_sel(&self) -> COMP_VP_SEL_R {
        COMP_VP_SEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Selects negative voltage input"]
    #[inline(always)]
    pub fn comp_vm_sel(&self) -> COMP_VM_SEL_R {
        COMP_VM_SEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
    #[inline(always)]
    pub fn edgeclr(&self) -> EDGECLR_R {
        EDGECLR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Comparator status. This bit reflects the state of the comparator output."]
    #[inline(always)]
    pub fn compstat(&self) -> COMPSTAT_R {
        COMPSTAT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Comparator edge-detect status."]
    #[inline(always)]
    pub fn compedge(&self) -> COMPEDGE_R {
        COMPEDGE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4 - This field controls which edges on the comparator output set the COMPEDGE bit (bit 23 below):"]
    #[inline(always)]
    pub fn edgesel(&mut self) -> EDGESEL_W {
        EDGESEL_W { w: self }
    }
    #[doc = "Bit 6 - Comparator output control"]
    #[inline(always)]
    pub fn compsa(&mut self) -> COMPSA_W {
        COMPSA_W { w: self }
    }
    #[doc = "Bits 8:10 - Selects positive voltage input"]
    #[inline(always)]
    pub fn comp_vp_sel(&mut self) -> COMP_VP_SEL_W {
        COMP_VP_SEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Selects negative voltage input"]
    #[inline(always)]
    pub fn comp_vm_sel(&mut self) -> COMP_VM_SEL_W {
        COMP_VM_SEL_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt clear bit. To clear the COMPEDGE bit and thus negate the interrupt request, toggle the EDGECLR bit by first writing a 1 and then a 0."]
    #[inline(always)]
    pub fn edgeclr(&mut self) -> EDGECLR_W {
        EDGECLR_W { w: self }
    }
    #[doc = "Bit 21 - Comparator status. This bit reflects the state of the comparator output."]
    #[inline(always)]
    pub fn compstat(&mut self) -> COMPSTAT_W {
        COMPSTAT_W { w: self }
    }
    #[doc = "Bit 23 - Comparator edge-detect status."]
    #[inline(always)]
    pub fn compedge(&mut self) -> COMPEDGE_W {
        COMPEDGE_W { w: self }
    }
    #[doc = "Bits 25:26 - Controls the hysteresis of the comparator. When the comparator is outputting a certain state, this is the difference between the selected signals, in the opposite direction from the state being output, that will switch the output."]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W {
        HYS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
