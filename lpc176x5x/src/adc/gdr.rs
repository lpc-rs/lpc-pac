#[doc = "Reader of register GDR"]
pub type R = crate::R<u32, super::GDR>;
#[doc = "Writer for register GDR"]
pub type W = crate::W<u32, super::GDR>;
#[doc = "Register GDR `reset()`'s with value 0"]
impl crate::ResetValue for super::GDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESULT`"]
pub struct RESULT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `CHN`"]
pub type CHN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHN`"]
pub struct CHN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERRUN`"]
pub struct OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
    #[inline(always)]
    pub fn chn(&self) -> CHN_R {
        CHN_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
    #[inline(always)]
    pub fn result(&mut self) -> RESULT_W {
        RESULT_W { w: self }
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
    #[inline(always)]
    pub fn chn(&mut self) -> CHN_W {
        CHN_W { w: self }
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W { w: self }
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
}
