#[doc = "Reader of register CFG2"]
pub type R = crate::R<u32, super::CFG2>;
#[doc = "Writer for register CFG2"]
pub type W = crate::W<u32, super::CFG2>;
#[doc = "Register CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAMELEN`"]
pub type FRAMELEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRAMELEN`"]
pub struct FRAMELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `POSITION`"]
pub type POSITION_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `POSITION`"]
pub struct POSITION_W<'a> {
    w: &'a mut W,
}
impl<'a> POSITION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in. See Frame format. 0x000 to 0x002 = not supported 0x003 = frame is 4 bits in total length 0x004 = frame is 5 bits in total length 0x1FF = frame is 512 bits in total length if FRAMELEN is an defines an odd length frame (e.g. 33 clocks) in mode 0 or 1, the extra clock appears in the right half. When MODE = 3, FRAMELEN must be larger than DATALEN in order for the WS pulse to be generated correctly."]
    #[inline(always)]
    pub fn framelen(&self) -> FRAMELEN_R {
        FRAMELEN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Data Position. Defines the location within the frame of the data for this channel pair. POSITION + DATALEN must be less than FRAMELEN. See Frame format. When MODE = 0, POSITION defines the location of data in both the left phase and right phase, starting one clock after the WS edge. In other modes, POSITION defines the location of data within the entire frame. ONECHANNEL = 1 while MODE = 0 is a special case, see the description of ONECHANNEL. The combination of DATALEN and the POSITION fields of all channel pairs must be made such that the channels do not overlap within the frame. 0x000 = data begins at bit position 0 (the first bit position) within the frame or WS phase. 0x001 = data begins at bit position 1 within the frame or WS phase. 0x002 = data begins at bit position 2 within the frame or WS phase."]
    #[inline(always)]
    pub fn position(&self) -> POSITION_R {
        POSITION_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Frame Length, minus 1 encoded, defines the number of clocks and data bits in the frames that this channel pair participates in. See Frame format. 0x000 to 0x002 = not supported 0x003 = frame is 4 bits in total length 0x004 = frame is 5 bits in total length 0x1FF = frame is 512 bits in total length if FRAMELEN is an defines an odd length frame (e.g. 33 clocks) in mode 0 or 1, the extra clock appears in the right half. When MODE = 3, FRAMELEN must be larger than DATALEN in order for the WS pulse to be generated correctly."]
    #[inline(always)]
    pub fn framelen(&mut self) -> FRAMELEN_W {
        FRAMELEN_W { w: self }
    }
    #[doc = "Bits 16:24 - Data Position. Defines the location within the frame of the data for this channel pair. POSITION + DATALEN must be less than FRAMELEN. See Frame format. When MODE = 0, POSITION defines the location of data in both the left phase and right phase, starting one clock after the WS edge. In other modes, POSITION defines the location of data within the entire frame. ONECHANNEL = 1 while MODE = 0 is a special case, see the description of ONECHANNEL. The combination of DATALEN and the POSITION fields of all channel pairs must be made such that the channels do not overlap within the frame. 0x000 = data begins at bit position 0 (the first bit position) within the frame or WS phase. 0x001 = data begins at bit position 1 within the frame or WS phase. 0x002 = data begins at bit position 2 within the frame or WS phase."]
    #[inline(always)]
    pub fn position(&mut self) -> POSITION_W {
        POSITION_W { w: self }
    }
}
