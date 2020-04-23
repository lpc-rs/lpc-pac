#[doc = "Reader of register TESSEICR"]
pub type R = crate::R<u32, super::TESSEICR>;
#[doc = "Writer for register TESSEICR"]
pub type W = crate::W<u32, super::TESSEICR>;
#[doc = "Register TESSEICR `reset()`'s with value 0"]
impl crate::ResetValue for super::TESSEICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `StartResourceSelection`"]
pub type STARTRESOURCESELECTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `StartResourceSelection`"]
pub struct STARTRESOURCESELECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTRESOURCESELECTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `StopResourceSelection`"]
pub type STOPRESOURCESELECTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `StopResourceSelection`"]
pub struct STOPRESOURCESELECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPRESOURCESELECTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn start_resource_selection(&self) -> STARTRESOURCESELECTION_R {
        STARTRESOURCESELECTION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn stop_resource_selection(&self) -> STOPRESOURCESELECTION_R {
        STOPRESOURCESELECTION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn start_resource_selection(&mut self) -> STARTRESOURCESELECTION_W {
        STARTRESOURCESELECTION_W { w: self }
    }
    #[doc = "Bits 16:19 - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn stop_resource_selection(&mut self) -> STOPRESOURCESELECTION_W {
        STOPRESOURCESELECTION_W { w: self }
    }
}
