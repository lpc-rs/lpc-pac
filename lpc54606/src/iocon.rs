#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital I/O control for port 0 pins PIO0_0"]
    pub pio00: PIO00,
    #[doc = "0x04 - Digital I/O control for port 0 pins PIO0_1"]
    pub pio01: PIO01,
    #[doc = "0x08 - Digital I/O control for port 0 pins PIO0_2"]
    pub pio02: PIO02,
    #[doc = "0x0c - Digital I/O control for port 0 pins PIO0_3"]
    pub pio03: PIO03,
    #[doc = "0x10 - Digital I/O control for port 0 pins PIO0_4"]
    pub pio04: PIO04,
    #[doc = "0x14 - Digital I/O control for port 0 pins PIO0_5"]
    pub pio05: PIO05,
    #[doc = "0x18 - Digital I/O control for port 0 pins PIO0_6"]
    pub pio06: PIO06,
    #[doc = "0x1c - Digital I/O control for port 0 pins PIO0_7"]
    pub pio07: PIO07,
    #[doc = "0x20 - Digital I/O control for port 0 pins PIO0_8"]
    pub pio08: PIO08,
    #[doc = "0x24 - Digital I/O control for port 0 pins PIO0_9"]
    pub pio09: PIO09,
    #[doc = "0x28 - Digital I/O control for port 0 pins PIO0_10"]
    pub pio010: PIO010,
    #[doc = "0x2c - Digital I/O control for port 0 pins PIO0_11"]
    pub pio011: PIO011,
    #[doc = "0x30 - Digital I/O control for port 0 pins PIO0_12"]
    pub pio012: PIO012,
    #[doc = "0x34 - Digital I/O control for port 0 pins PIO0_13"]
    pub pio013: PIO013,
    #[doc = "0x38 - Digital I/O control for port 0 pins PIO0_14"]
    pub pio014: PIO014,
    #[doc = "0x3c - Digital I/O control for port 0 pins PIO0_15"]
    pub pio015: PIO015,
    #[doc = "0x40 - Digital I/O control for port 0 pins PIO0_16"]
    pub pio016: PIO016,
    #[doc = "0x44 - Digital I/O control for port 0 pins PIO0_17"]
    pub pio017: PIO017,
    #[doc = "0x48 - Digital I/O control for port 0 pins PIO0_18"]
    pub pio018: PIO018,
    #[doc = "0x4c - Digital I/O control for port 0 pins PIO0_19"]
    pub pio019: PIO019,
    #[doc = "0x50 - Digital I/O control for port 0 pins PIO0_20"]
    pub pio020: PIO020,
    #[doc = "0x54 - Digital I/O control for port 0 pins PIO0_21"]
    pub pio021: PIO021,
    #[doc = "0x58 - Digital I/O control for port 0 pins PIO0_22"]
    pub pio022: PIO022,
    #[doc = "0x5c - Digital I/O control for port 0 pins PIO0_23"]
    pub pio023: PIO023,
    #[doc = "0x60 - Digital I/O control for port 0 pins PIO0_24"]
    pub pio024: PIO024,
    #[doc = "0x64 - Digital I/O control for port 0 pins PIO0_25"]
    pub pio025: PIO025,
    #[doc = "0x68 - Digital I/O control for port 0 pins PIO0_26"]
    pub pio026: PIO026,
    #[doc = "0x6c - Digital I/O control for port 0 pins PIO0_27"]
    pub pio027: PIO027,
    #[doc = "0x70 - Digital I/O control for port 0 pins PIO0_28"]
    pub pio028: PIO028,
    #[doc = "0x74 - Digital I/O control for port 0 pins PIO0_29"]
    pub pio029: PIO029,
    #[doc = "0x78 - Digital I/O control for port 0 pins PIO0_30"]
    pub pio030: PIO030,
    #[doc = "0x7c - Digital I/O control for port 0 pins PIO0_31"]
    pub pio031: PIO031,
    #[doc = "0x80 - Digital I/O control for port 1 pins PIO1_0"]
    pub pio10: PIO10,
    #[doc = "0x84 - Digital I/O control for port 1 pins PIO1_1"]
    pub pio11: PIO11,
    #[doc = "0x88 - Digital I/O control for port 1 pins PIO1_2"]
    pub pio12: PIO12,
    #[doc = "0x8c - Digital I/O control for port 1 pins PIO1_3"]
    pub pio13: PIO13,
    #[doc = "0x90 - Digital I/O control for port 1 pins PIO1_4"]
    pub pio14: PIO14,
    #[doc = "0x94 - Digital I/O control for port 1 pins PIO1_5"]
    pub pio15: PIO15,
    #[doc = "0x98 - Digital I/O control for port 1 pins PIO1_6"]
    pub pio16: PIO16,
    #[doc = "0x9c - Digital I/O control for port 1 pins PIO1_7"]
    pub pio17: PIO17,
    #[doc = "0xa0 - Digital I/O control for port 1 pins PIO1_8"]
    pub pio18: PIO18,
    #[doc = "0xa4 - Digital I/O control for port 1 pins PIO1_9"]
    pub pio19: PIO19,
    #[doc = "0xa8 - Digital I/O control for port 1 pins PIO1_10"]
    pub pio110: PIO110,
    #[doc = "0xac - Digital I/O control for port 1 pins PIO1_11"]
    pub pio111: PIO111,
    #[doc = "0xb0 - Digital I/O control for port 1 pins PIO1_12"]
    pub pio112: PIO112,
    #[doc = "0xb4 - Digital I/O control for port 1 pins PIO1_13"]
    pub pio113: PIO113,
    #[doc = "0xb8 - Digital I/O control for port 1 pins PIO1_14"]
    pub pio114: PIO114,
    #[doc = "0xbc - Digital I/O control for port 1 pins PIO1_15"]
    pub pio115: PIO115,
    #[doc = "0xc0 - Digital I/O control for port 1 pins PIO1_16"]
    pub pio116: PIO116,
    #[doc = "0xc4 - Digital I/O control for port 1 pins PIO1_17"]
    pub pio117: PIO117,
    #[doc = "0xc8 - Digital I/O control for port 1 pins PIO1_18"]
    pub pio118: PIO118,
    #[doc = "0xcc - Digital I/O control for port 1 pins PIO1_19"]
    pub pio119: PIO119,
    #[doc = "0xd0 - Digital I/O control for port 1 pins PIO1_20"]
    pub pio120: PIO120,
    #[doc = "0xd4 - Digital I/O control for port 1 pins PIO1_21"]
    pub pio121: PIO121,
    #[doc = "0xd8 - Digital I/O control for port 1 pins PIO1_22"]
    pub pio122: PIO122,
    #[doc = "0xdc - Digital I/O control for port 1 pins PIO1_23"]
    pub pio123: PIO123,
    #[doc = "0xe0 - Digital I/O control for port 1 pins PIO1_24"]
    pub pio124: PIO124,
    #[doc = "0xe4 - Digital I/O control for port 1 pins PIO1_25"]
    pub pio125: PIO125,
    #[doc = "0xe8 - Digital I/O control for port 1 pins PIO1_26"]
    pub pio126: PIO126,
    #[doc = "0xec - Digital I/O control for port 1 pins PIO1_27"]
    pub pio127: PIO127,
    #[doc = "0xf0 - Digital I/O control for port 1 pins PIO1_28"]
    pub pio128: PIO128,
    #[doc = "0xf4 - Digital I/O control for port 1 pins PIO1_29"]
    pub pio129: PIO129,
    #[doc = "0xf8 - Digital I/O control for port 1 pins PIO1_30"]
    pub pio130: PIO130,
    #[doc = "0xfc - Digital I/O control for port 1 pins PIO1_31"]
    pub pio131: PIO131,
    #[doc = "0x100 - Digital I/O control for port 2 pins PIO2_0"]
    pub pio20: PIO20,
    #[doc = "0x104 - Digital I/O control for port 2 pins PIO2_1"]
    pub pio21: PIO21,
    #[doc = "0x108 - Digital I/O control for port 2 pins PIO2_2"]
    pub pio22: PIO22,
    #[doc = "0x10c - Digital I/O control for port 2 pins PIO2_3"]
    pub pio23: PIO23,
    #[doc = "0x110 - Digital I/O control for port 2 pins PIO2_4"]
    pub pio24: PIO24,
    #[doc = "0x114 - Digital I/O control for port 2 pins PIO2_5"]
    pub pio25: PIO25,
    #[doc = "0x118 - Digital I/O control for port 2 pins PIO2_6"]
    pub pio26: PIO26,
    #[doc = "0x11c - Digital I/O control for port 2 pins PIO2_7"]
    pub pio27: PIO27,
    #[doc = "0x120 - Digital I/O control for port 2 pins PIO2_8"]
    pub pio28: PIO28,
    #[doc = "0x124 - Digital I/O control for port 2 pins PIO2_9"]
    pub pio29: PIO29,
    #[doc = "0x128 - Digital I/O control for port 2 pins PIO2_10"]
    pub pio210: PIO210,
    #[doc = "0x12c - Digital I/O control for port 2 pins PIO2_11"]
    pub pio211: PIO211,
    #[doc = "0x130 - Digital I/O control for port 2 pins PIO2_12"]
    pub pio212: PIO212,
    #[doc = "0x134 - Digital I/O control for port 2 pins PIO2_13"]
    pub pio213: PIO213,
    #[doc = "0x138 - Digital I/O control for port 2 pins PIO2_14"]
    pub pio214: PIO214,
    #[doc = "0x13c - Digital I/O control for port 2 pins PIO2_15"]
    pub pio215: PIO215,
    #[doc = "0x140 - Digital I/O control for port 2 pins PIO2_16"]
    pub pio216: PIO216,
    #[doc = "0x144 - Digital I/O control for port 2 pins PIO2_17"]
    pub pio217: PIO217,
    #[doc = "0x148 - Digital I/O control for port 2 pins PIO2_18"]
    pub pio218: PIO218,
    #[doc = "0x14c - Digital I/O control for port 2 pins PIO2_19"]
    pub pio219: PIO219,
    #[doc = "0x150 - Digital I/O control for port 2 pins PIO2_20"]
    pub pio220: PIO220,
    #[doc = "0x154 - Digital I/O control for port 2 pins PIO2_21"]
    pub pio221: PIO221,
    #[doc = "0x158 - Digital I/O control for port 2 pins PIO2_22"]
    pub pio222: PIO222,
    #[doc = "0x15c - Digital I/O control for port 2 pins PIO2_23"]
    pub pio223: PIO223,
    #[doc = "0x160 - Digital I/O control for port 2 pins PIO2_24"]
    pub pio224: PIO224,
    #[doc = "0x164 - Digital I/O control for port 2 pins PIO2_25"]
    pub pio225: PIO225,
    #[doc = "0x168 - Digital I/O control for port 2 pins PIO2_26"]
    pub pio226: PIO226,
    #[doc = "0x16c - Digital I/O control for port 2 pins PIO2_27"]
    pub pio227: PIO227,
    #[doc = "0x170 - Digital I/O control for port 2 pins PIO2_28"]
    pub pio228: PIO228,
    #[doc = "0x174 - Digital I/O control for port 2 pins PIO2_29"]
    pub pio229: PIO229,
    #[doc = "0x178 - Digital I/O control for port 2 pins PIO2_30"]
    pub pio230: PIO230,
    #[doc = "0x17c - Digital I/O control for port 2 pins PIO2_31"]
    pub pio231: PIO231,
    #[doc = "0x180 - Digital I/O control for port 3 pins PIO3_0"]
    pub pio30: PIO30,
    #[doc = "0x184 - Digital I/O control for port 3 pins PIO3_1"]
    pub pio31: PIO31,
    #[doc = "0x188 - Digital I/O control for port 3 pins PIO3_2"]
    pub pio32: PIO32,
    #[doc = "0x18c - Digital I/O control for port 3 pins PIO3_3"]
    pub pio33: PIO33,
    #[doc = "0x190 - Digital I/O control for port 3 pins PIO3_4"]
    pub pio34: PIO34,
    #[doc = "0x194 - Digital I/O control for port 3 pins PIO3_5"]
    pub pio35: PIO35,
    #[doc = "0x198 - Digital I/O control for port 3 pins PIO3_6"]
    pub pio36: PIO36,
    #[doc = "0x19c - Digital I/O control for port 3 pins PIO3_7"]
    pub pio37: PIO37,
    #[doc = "0x1a0 - Digital I/O control for port 3 pins PIO3_8"]
    pub pio38: PIO38,
    #[doc = "0x1a4 - Digital I/O control for port 3 pins PIO3_9"]
    pub pio39: PIO39,
    #[doc = "0x1a8 - Digital I/O control for port 3 pins PIO3_10"]
    pub pio310: PIO310,
    #[doc = "0x1ac - Digital I/O control for port 3 pins PIO3_11"]
    pub pio311: PIO311,
    #[doc = "0x1b0 - Digital I/O control for port 3 pins PIO3_12"]
    pub pio312: PIO312,
    #[doc = "0x1b4 - Digital I/O control for port 3 pins PIO3_13"]
    pub pio313: PIO313,
    #[doc = "0x1b8 - Digital I/O control for port 3 pins PIO3_14"]
    pub pio314: PIO314,
    #[doc = "0x1bc - Digital I/O control for port 3 pins PIO3_15"]
    pub pio315: PIO315,
    #[doc = "0x1c0 - Digital I/O control for port 3 pins PIO3_16"]
    pub pio316: PIO316,
    #[doc = "0x1c4 - Digital I/O control for port 3 pins PIO3_17"]
    pub pio317: PIO317,
    #[doc = "0x1c8 - Digital I/O control for port 3 pins PIO3_18"]
    pub pio318: PIO318,
    #[doc = "0x1cc - Digital I/O control for port 3 pins PIO3_19"]
    pub pio319: PIO319,
    #[doc = "0x1d0 - Digital I/O control for port 3 pins PIO3_20"]
    pub pio320: PIO320,
    #[doc = "0x1d4 - Digital I/O control for port 3 pins PIO3_21"]
    pub pio321: PIO321,
    #[doc = "0x1d8 - Digital I/O control for port 3 pins PIO3_22"]
    pub pio322: PIO322,
    #[doc = "0x1dc - Digital I/O control for port 3 pins PIO3_23"]
    pub pio323: PIO323,
    #[doc = "0x1e0 - Digital I/O control for port 3 pins PIO3_24"]
    pub pio324: PIO324,
    #[doc = "0x1e4 - Digital I/O control for port 3 pins PIO3_25"]
    pub pio325: PIO325,
    #[doc = "0x1e8 - Digital I/O control for port 3 pins PIO3_26"]
    pub pio326: PIO326,
    #[doc = "0x1ec - Digital I/O control for port 3 pins PIO3_27"]
    pub pio327: PIO327,
    #[doc = "0x1f0 - Digital I/O control for port 3 pins PIO3_28"]
    pub pio328: PIO328,
    #[doc = "0x1f4 - Digital I/O control for port 3 pins PIO3_29"]
    pub pio329: PIO329,
    #[doc = "0x1f8 - Digital I/O control for port 3 pins PIO3_30"]
    pub pio330: PIO330,
    #[doc = "0x1fc - Digital I/O control for port 3 pins PIO3_31"]
    pub pio331: PIO331,
    #[doc = "0x200 - Digital I/O control for port 4 pins PIO4_0"]
    pub pio40: PIO40,
    #[doc = "0x204 - Digital I/O control for port 4 pins PIO4_1"]
    pub pio41: PIO41,
    #[doc = "0x208 - Digital I/O control for port 4 pins PIO4_2"]
    pub pio42: PIO42,
    #[doc = "0x20c - Digital I/O control for port 4 pins PIO4_3"]
    pub pio43: PIO43,
    #[doc = "0x210 - Digital I/O control for port 4 pins PIO4_4"]
    pub pio44: PIO44,
    #[doc = "0x214 - Digital I/O control for port 4 pins PIO4_5"]
    pub pio45: PIO45,
    #[doc = "0x218 - Digital I/O control for port 4 pins PIO4_6"]
    pub pio46: PIO46,
    #[doc = "0x21c - Digital I/O control for port 4 pins PIO4_7"]
    pub pio47: PIO47,
    #[doc = "0x220 - Digital I/O control for port 4 pins PIO4_8"]
    pub pio48: PIO48,
    #[doc = "0x224 - Digital I/O control for port 4 pins PIO4_9"]
    pub pio49: PIO49,
    #[doc = "0x228 - Digital I/O control for port 4 pins PIO4_10"]
    pub pio410: PIO410,
    #[doc = "0x22c - Digital I/O control for port 4 pins PIO4_11"]
    pub pio411: PIO411,
    #[doc = "0x230 - Digital I/O control for port 4 pins PIO4_12"]
    pub pio412: PIO412,
    #[doc = "0x234 - Digital I/O control for port 4 pins PIO4_13"]
    pub pio413: PIO413,
    #[doc = "0x238 - Digital I/O control for port 4 pins PIO4_14"]
    pub pio414: PIO414,
    #[doc = "0x23c - Digital I/O control for port 4 pins PIO4_15"]
    pub pio415: PIO415,
    #[doc = "0x240 - Digital I/O control for port 4 pins PIO4_16"]
    pub pio416: PIO416,
    #[doc = "0x244 - Digital I/O control for port 4 pins PIO4_17"]
    pub pio417: PIO417,
    #[doc = "0x248 - Digital I/O control for port 4 pins PIO4_18"]
    pub pio418: PIO418,
    #[doc = "0x24c - Digital I/O control for port 4 pins PIO4_19"]
    pub pio419: PIO419,
    #[doc = "0x250 - Digital I/O control for port 4 pins PIO4_20"]
    pub pio420: PIO420,
    #[doc = "0x254 - Digital I/O control for port 4 pins PIO4_21"]
    pub pio421: PIO421,
    #[doc = "0x258 - Digital I/O control for port 4 pins PIO4_22"]
    pub pio422: PIO422,
    #[doc = "0x25c - Digital I/O control for port 4 pins PIO4_23"]
    pub pio423: PIO423,
    #[doc = "0x260 - Digital I/O control for port 4 pins PIO4_24"]
    pub pio424: PIO424,
    #[doc = "0x264 - Digital I/O control for port 4 pins PIO4_25"]
    pub pio425: PIO425,
    #[doc = "0x268 - Digital I/O control for port 4 pins PIO4_26"]
    pub pio426: PIO426,
    #[doc = "0x26c - Digital I/O control for port 4 pins PIO4_27"]
    pub pio427: PIO427,
    #[doc = "0x270 - Digital I/O control for port 4 pins PIO4_28"]
    pub pio428: PIO428,
    #[doc = "0x274 - Digital I/O control for port 4 pins PIO4_29"]
    pub pio429: PIO429,
    #[doc = "0x278 - Digital I/O control for port 4 pins PIO4_30"]
    pub pio430: PIO430,
    #[doc = "0x27c - Digital I/O control for port 4 pins PIO4_31"]
    pub pio431: PIO431,
    #[doc = "0x280 - Digital I/O control for port 5 pins PIO5_0"]
    pub pio50: PIO50,
    #[doc = "0x284 - Digital I/O control for port 5 pins PIO5_1"]
    pub pio51: PIO51,
    #[doc = "0x288 - Digital I/O control for port 5 pins PIO5_2"]
    pub pio52: PIO52,
    #[doc = "0x28c - Digital I/O control for port 5 pins PIO5_3"]
    pub pio53: PIO53,
    #[doc = "0x290 - Digital I/O control for port 5 pins PIO5_4"]
    pub pio54: PIO54,
    #[doc = "0x294 - Digital I/O control for port 5 pins PIO5_5"]
    pub pio55: PIO55,
    #[doc = "0x298 - Digital I/O control for port 5 pins PIO5_6"]
    pub pio56: PIO56,
    #[doc = "0x29c - Digital I/O control for port 5 pins PIO5_7"]
    pub pio57: PIO57,
    #[doc = "0x2a0 - Digital I/O control for port 5 pins PIO5_8"]
    pub pio58: PIO58,
    #[doc = "0x2a4 - Digital I/O control for port 5 pins PIO5_9"]
    pub pio59: PIO59,
    #[doc = "0x2a8 - Digital I/O control for port 5 pins PIO5_10"]
    pub pio510: PIO510,
    #[doc = "0x2ac - Digital I/O control for port 5 pins PIO5_11"]
    pub pio511: PIO511,
    #[doc = "0x2b0 - Digital I/O control for port 5 pins PIO5_12"]
    pub pio512: PIO512,
    #[doc = "0x2b4 - Digital I/O control for port 5 pins PIO5_13"]
    pub pio513: PIO513,
    #[doc = "0x2b8 - Digital I/O control for port 5 pins PIO5_14"]
    pub pio514: PIO514,
    #[doc = "0x2bc - Digital I/O control for port 5 pins PIO5_15"]
    pub pio515: PIO515,
    #[doc = "0x2c0 - Digital I/O control for port 5 pins PIO5_16"]
    pub pio516: PIO516,
    #[doc = "0x2c4 - Digital I/O control for port 5 pins PIO5_17"]
    pub pio517: PIO517,
    #[doc = "0x2c8 - Digital I/O control for port 5 pins PIO5_18"]
    pub pio518: PIO518,
    #[doc = "0x2cc - Digital I/O control for port 5 pins PIO5_19"]
    pub pio519: PIO519,
    #[doc = "0x2d0 - Digital I/O control for port 5 pins PIO5_20"]
    pub pio520: PIO520,
    #[doc = "0x2d4 - Digital I/O control for port 5 pins PIO5_21"]
    pub pio521: PIO521,
    #[doc = "0x2d8 - Digital I/O control for port 5 pins PIO5_22"]
    pub pio522: PIO522,
    #[doc = "0x2dc - Digital I/O control for port 5 pins PIO5_23"]
    pub pio523: PIO523,
    #[doc = "0x2e0 - Digital I/O control for port 5 pins PIO5_24"]
    pub pio524: PIO524,
    #[doc = "0x2e4 - Digital I/O control for port 5 pins PIO5_25"]
    pub pio525: PIO525,
    #[doc = "0x2e8 - Digital I/O control for port 5 pins PIO5_26"]
    pub pio526: PIO526,
    #[doc = "0x2ec - Digital I/O control for port 5 pins PIO5_27"]
    pub pio527: PIO527,
    #[doc = "0x2f0 - Digital I/O control for port 5 pins PIO5_28"]
    pub pio528: PIO528,
    #[doc = "0x2f4 - Digital I/O control for port 5 pins PIO5_29"]
    pub pio529: PIO529,
    #[doc = "0x2f8 - Digital I/O control for port 5 pins PIO5_30"]
    pub pio530: PIO530,
    #[doc = "0x2fc - Digital I/O control for port 5 pins PIO5_31"]
    pub pio531: PIO531,
}
#[doc = "Digital I/O control for port 0 pins PIO0_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio00](pio00) module"]
pub type PIO00 = crate::Reg<u32, _PIO00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO00;
#[doc = "`read()` method returns [pio00::R](pio00::R) reader structure"]
impl crate::Readable for PIO00 {}
#[doc = "`write(|w| ..)` method takes [pio00::W](pio00::W) writer structure"]
impl crate::Writable for PIO00 {}
#[doc = "Digital I/O control for port 0 pins PIO0_0"]
pub mod pio00;
#[doc = "Digital I/O control for port 0 pins PIO0_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio01](pio01) module"]
pub type PIO01 = crate::Reg<u32, _PIO01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO01;
#[doc = "`read()` method returns [pio01::R](pio01::R) reader structure"]
impl crate::Readable for PIO01 {}
#[doc = "`write(|w| ..)` method takes [pio01::W](pio01::W) writer structure"]
impl crate::Writable for PIO01 {}
#[doc = "Digital I/O control for port 0 pins PIO0_1"]
pub mod pio01;
#[doc = "Digital I/O control for port 0 pins PIO0_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio02](pio02) module"]
pub type PIO02 = crate::Reg<u32, _PIO02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO02;
#[doc = "`read()` method returns [pio02::R](pio02::R) reader structure"]
impl crate::Readable for PIO02 {}
#[doc = "`write(|w| ..)` method takes [pio02::W](pio02::W) writer structure"]
impl crate::Writable for PIO02 {}
#[doc = "Digital I/O control for port 0 pins PIO0_2"]
pub mod pio02;
#[doc = "Digital I/O control for port 0 pins PIO0_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio03](pio03) module"]
pub type PIO03 = crate::Reg<u32, _PIO03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO03;
#[doc = "`read()` method returns [pio03::R](pio03::R) reader structure"]
impl crate::Readable for PIO03 {}
#[doc = "`write(|w| ..)` method takes [pio03::W](pio03::W) writer structure"]
impl crate::Writable for PIO03 {}
#[doc = "Digital I/O control for port 0 pins PIO0_3"]
pub mod pio03;
#[doc = "Digital I/O control for port 0 pins PIO0_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio04](pio04) module"]
pub type PIO04 = crate::Reg<u32, _PIO04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO04;
#[doc = "`read()` method returns [pio04::R](pio04::R) reader structure"]
impl crate::Readable for PIO04 {}
#[doc = "`write(|w| ..)` method takes [pio04::W](pio04::W) writer structure"]
impl crate::Writable for PIO04 {}
#[doc = "Digital I/O control for port 0 pins PIO0_4"]
pub mod pio04;
#[doc = "Digital I/O control for port 0 pins PIO0_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio05](pio05) module"]
pub type PIO05 = crate::Reg<u32, _PIO05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO05;
#[doc = "`read()` method returns [pio05::R](pio05::R) reader structure"]
impl crate::Readable for PIO05 {}
#[doc = "`write(|w| ..)` method takes [pio05::W](pio05::W) writer structure"]
impl crate::Writable for PIO05 {}
#[doc = "Digital I/O control for port 0 pins PIO0_5"]
pub mod pio05;
#[doc = "Digital I/O control for port 0 pins PIO0_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio06](pio06) module"]
pub type PIO06 = crate::Reg<u32, _PIO06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO06;
#[doc = "`read()` method returns [pio06::R](pio06::R) reader structure"]
impl crate::Readable for PIO06 {}
#[doc = "`write(|w| ..)` method takes [pio06::W](pio06::W) writer structure"]
impl crate::Writable for PIO06 {}
#[doc = "Digital I/O control for port 0 pins PIO0_6"]
pub mod pio06;
#[doc = "Digital I/O control for port 0 pins PIO0_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio07](pio07) module"]
pub type PIO07 = crate::Reg<u32, _PIO07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO07;
#[doc = "`read()` method returns [pio07::R](pio07::R) reader structure"]
impl crate::Readable for PIO07 {}
#[doc = "`write(|w| ..)` method takes [pio07::W](pio07::W) writer structure"]
impl crate::Writable for PIO07 {}
#[doc = "Digital I/O control for port 0 pins PIO0_7"]
pub mod pio07;
#[doc = "Digital I/O control for port 0 pins PIO0_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio08](pio08) module"]
pub type PIO08 = crate::Reg<u32, _PIO08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO08;
#[doc = "`read()` method returns [pio08::R](pio08::R) reader structure"]
impl crate::Readable for PIO08 {}
#[doc = "`write(|w| ..)` method takes [pio08::W](pio08::W) writer structure"]
impl crate::Writable for PIO08 {}
#[doc = "Digital I/O control for port 0 pins PIO0_8"]
pub mod pio08;
#[doc = "Digital I/O control for port 0 pins PIO0_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio09](pio09) module"]
pub type PIO09 = crate::Reg<u32, _PIO09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO09;
#[doc = "`read()` method returns [pio09::R](pio09::R) reader structure"]
impl crate::Readable for PIO09 {}
#[doc = "`write(|w| ..)` method takes [pio09::W](pio09::W) writer structure"]
impl crate::Writable for PIO09 {}
#[doc = "Digital I/O control for port 0 pins PIO0_9"]
pub mod pio09;
#[doc = "Digital I/O control for port 0 pins PIO0_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio010](pio010) module"]
pub type PIO010 = crate::Reg<u32, _PIO010>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO010;
#[doc = "`read()` method returns [pio010::R](pio010::R) reader structure"]
impl crate::Readable for PIO010 {}
#[doc = "`write(|w| ..)` method takes [pio010::W](pio010::W) writer structure"]
impl crate::Writable for PIO010 {}
#[doc = "Digital I/O control for port 0 pins PIO0_10"]
pub mod pio010;
#[doc = "Digital I/O control for port 0 pins PIO0_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio011](pio011) module"]
pub type PIO011 = crate::Reg<u32, _PIO011>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO011;
#[doc = "`read()` method returns [pio011::R](pio011::R) reader structure"]
impl crate::Readable for PIO011 {}
#[doc = "`write(|w| ..)` method takes [pio011::W](pio011::W) writer structure"]
impl crate::Writable for PIO011 {}
#[doc = "Digital I/O control for port 0 pins PIO0_11"]
pub mod pio011;
#[doc = "Digital I/O control for port 0 pins PIO0_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio012](pio012) module"]
pub type PIO012 = crate::Reg<u32, _PIO012>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO012;
#[doc = "`read()` method returns [pio012::R](pio012::R) reader structure"]
impl crate::Readable for PIO012 {}
#[doc = "`write(|w| ..)` method takes [pio012::W](pio012::W) writer structure"]
impl crate::Writable for PIO012 {}
#[doc = "Digital I/O control for port 0 pins PIO0_12"]
pub mod pio012;
#[doc = "Digital I/O control for port 0 pins PIO0_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio013](pio013) module"]
pub type PIO013 = crate::Reg<u32, _PIO013>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO013;
#[doc = "`read()` method returns [pio013::R](pio013::R) reader structure"]
impl crate::Readable for PIO013 {}
#[doc = "`write(|w| ..)` method takes [pio013::W](pio013::W) writer structure"]
impl crate::Writable for PIO013 {}
#[doc = "Digital I/O control for port 0 pins PIO0_13"]
pub mod pio013;
#[doc = "Digital I/O control for port 0 pins PIO0_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio014](pio014) module"]
pub type PIO014 = crate::Reg<u32, _PIO014>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO014;
#[doc = "`read()` method returns [pio014::R](pio014::R) reader structure"]
impl crate::Readable for PIO014 {}
#[doc = "`write(|w| ..)` method takes [pio014::W](pio014::W) writer structure"]
impl crate::Writable for PIO014 {}
#[doc = "Digital I/O control for port 0 pins PIO0_14"]
pub mod pio014;
#[doc = "Digital I/O control for port 0 pins PIO0_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio015](pio015) module"]
pub type PIO015 = crate::Reg<u32, _PIO015>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO015;
#[doc = "`read()` method returns [pio015::R](pio015::R) reader structure"]
impl crate::Readable for PIO015 {}
#[doc = "`write(|w| ..)` method takes [pio015::W](pio015::W) writer structure"]
impl crate::Writable for PIO015 {}
#[doc = "Digital I/O control for port 0 pins PIO0_15"]
pub mod pio015;
#[doc = "Digital I/O control for port 0 pins PIO0_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio016](pio016) module"]
pub type PIO016 = crate::Reg<u32, _PIO016>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO016;
#[doc = "`read()` method returns [pio016::R](pio016::R) reader structure"]
impl crate::Readable for PIO016 {}
#[doc = "`write(|w| ..)` method takes [pio016::W](pio016::W) writer structure"]
impl crate::Writable for PIO016 {}
#[doc = "Digital I/O control for port 0 pins PIO0_16"]
pub mod pio016;
#[doc = "Digital I/O control for port 0 pins PIO0_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio017](pio017) module"]
pub type PIO017 = crate::Reg<u32, _PIO017>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO017;
#[doc = "`read()` method returns [pio017::R](pio017::R) reader structure"]
impl crate::Readable for PIO017 {}
#[doc = "`write(|w| ..)` method takes [pio017::W](pio017::W) writer structure"]
impl crate::Writable for PIO017 {}
#[doc = "Digital I/O control for port 0 pins PIO0_17"]
pub mod pio017;
#[doc = "Digital I/O control for port 0 pins PIO0_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio018](pio018) module"]
pub type PIO018 = crate::Reg<u32, _PIO018>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO018;
#[doc = "`read()` method returns [pio018::R](pio018::R) reader structure"]
impl crate::Readable for PIO018 {}
#[doc = "`write(|w| ..)` method takes [pio018::W](pio018::W) writer structure"]
impl crate::Writable for PIO018 {}
#[doc = "Digital I/O control for port 0 pins PIO0_18"]
pub mod pio018;
#[doc = "Digital I/O control for port 0 pins PIO0_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio019](pio019) module"]
pub type PIO019 = crate::Reg<u32, _PIO019>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO019;
#[doc = "`read()` method returns [pio019::R](pio019::R) reader structure"]
impl crate::Readable for PIO019 {}
#[doc = "`write(|w| ..)` method takes [pio019::W](pio019::W) writer structure"]
impl crate::Writable for PIO019 {}
#[doc = "Digital I/O control for port 0 pins PIO0_19"]
pub mod pio019;
#[doc = "Digital I/O control for port 0 pins PIO0_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio020](pio020) module"]
pub type PIO020 = crate::Reg<u32, _PIO020>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO020;
#[doc = "`read()` method returns [pio020::R](pio020::R) reader structure"]
impl crate::Readable for PIO020 {}
#[doc = "`write(|w| ..)` method takes [pio020::W](pio020::W) writer structure"]
impl crate::Writable for PIO020 {}
#[doc = "Digital I/O control for port 0 pins PIO0_20"]
pub mod pio020;
#[doc = "Digital I/O control for port 0 pins PIO0_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio021](pio021) module"]
pub type PIO021 = crate::Reg<u32, _PIO021>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO021;
#[doc = "`read()` method returns [pio021::R](pio021::R) reader structure"]
impl crate::Readable for PIO021 {}
#[doc = "`write(|w| ..)` method takes [pio021::W](pio021::W) writer structure"]
impl crate::Writable for PIO021 {}
#[doc = "Digital I/O control for port 0 pins PIO0_21"]
pub mod pio021;
#[doc = "Digital I/O control for port 0 pins PIO0_22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio022](pio022) module"]
pub type PIO022 = crate::Reg<u32, _PIO022>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO022;
#[doc = "`read()` method returns [pio022::R](pio022::R) reader structure"]
impl crate::Readable for PIO022 {}
#[doc = "`write(|w| ..)` method takes [pio022::W](pio022::W) writer structure"]
impl crate::Writable for PIO022 {}
#[doc = "Digital I/O control for port 0 pins PIO0_22"]
pub mod pio022;
#[doc = "Digital I/O control for port 0 pins PIO0_23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio023](pio023) module"]
pub type PIO023 = crate::Reg<u32, _PIO023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO023;
#[doc = "`read()` method returns [pio023::R](pio023::R) reader structure"]
impl crate::Readable for PIO023 {}
#[doc = "`write(|w| ..)` method takes [pio023::W](pio023::W) writer structure"]
impl crate::Writable for PIO023 {}
#[doc = "Digital I/O control for port 0 pins PIO0_23"]
pub mod pio023;
#[doc = "Digital I/O control for port 0 pins PIO0_24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio024](pio024) module"]
pub type PIO024 = crate::Reg<u32, _PIO024>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO024;
#[doc = "`read()` method returns [pio024::R](pio024::R) reader structure"]
impl crate::Readable for PIO024 {}
#[doc = "`write(|w| ..)` method takes [pio024::W](pio024::W) writer structure"]
impl crate::Writable for PIO024 {}
#[doc = "Digital I/O control for port 0 pins PIO0_24"]
pub mod pio024;
#[doc = "Digital I/O control for port 0 pins PIO0_25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio025](pio025) module"]
pub type PIO025 = crate::Reg<u32, _PIO025>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO025;
#[doc = "`read()` method returns [pio025::R](pio025::R) reader structure"]
impl crate::Readable for PIO025 {}
#[doc = "`write(|w| ..)` method takes [pio025::W](pio025::W) writer structure"]
impl crate::Writable for PIO025 {}
#[doc = "Digital I/O control for port 0 pins PIO0_25"]
pub mod pio025;
#[doc = "Digital I/O control for port 0 pins PIO0_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio026](pio026) module"]
pub type PIO026 = crate::Reg<u32, _PIO026>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO026;
#[doc = "`read()` method returns [pio026::R](pio026::R) reader structure"]
impl crate::Readable for PIO026 {}
#[doc = "`write(|w| ..)` method takes [pio026::W](pio026::W) writer structure"]
impl crate::Writable for PIO026 {}
#[doc = "Digital I/O control for port 0 pins PIO0_26"]
pub mod pio026;
#[doc = "Digital I/O control for port 0 pins PIO0_27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio027](pio027) module"]
pub type PIO027 = crate::Reg<u32, _PIO027>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO027;
#[doc = "`read()` method returns [pio027::R](pio027::R) reader structure"]
impl crate::Readable for PIO027 {}
#[doc = "`write(|w| ..)` method takes [pio027::W](pio027::W) writer structure"]
impl crate::Writable for PIO027 {}
#[doc = "Digital I/O control for port 0 pins PIO0_27"]
pub mod pio027;
#[doc = "Digital I/O control for port 0 pins PIO0_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio028](pio028) module"]
pub type PIO028 = crate::Reg<u32, _PIO028>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO028;
#[doc = "`read()` method returns [pio028::R](pio028::R) reader structure"]
impl crate::Readable for PIO028 {}
#[doc = "`write(|w| ..)` method takes [pio028::W](pio028::W) writer structure"]
impl crate::Writable for PIO028 {}
#[doc = "Digital I/O control for port 0 pins PIO0_28"]
pub mod pio028;
#[doc = "Digital I/O control for port 0 pins PIO0_29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio029](pio029) module"]
pub type PIO029 = crate::Reg<u32, _PIO029>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO029;
#[doc = "`read()` method returns [pio029::R](pio029::R) reader structure"]
impl crate::Readable for PIO029 {}
#[doc = "`write(|w| ..)` method takes [pio029::W](pio029::W) writer structure"]
impl crate::Writable for PIO029 {}
#[doc = "Digital I/O control for port 0 pins PIO0_29"]
pub mod pio029;
#[doc = "Digital I/O control for port 0 pins PIO0_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio030](pio030) module"]
pub type PIO030 = crate::Reg<u32, _PIO030>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO030;
#[doc = "`read()` method returns [pio030::R](pio030::R) reader structure"]
impl crate::Readable for PIO030 {}
#[doc = "`write(|w| ..)` method takes [pio030::W](pio030::W) writer structure"]
impl crate::Writable for PIO030 {}
#[doc = "Digital I/O control for port 0 pins PIO0_30"]
pub mod pio030;
#[doc = "Digital I/O control for port 0 pins PIO0_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio031](pio031) module"]
pub type PIO031 = crate::Reg<u32, _PIO031>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO031;
#[doc = "`read()` method returns [pio031::R](pio031::R) reader structure"]
impl crate::Readable for PIO031 {}
#[doc = "`write(|w| ..)` method takes [pio031::W](pio031::W) writer structure"]
impl crate::Writable for PIO031 {}
#[doc = "Digital I/O control for port 0 pins PIO0_31"]
pub mod pio031;
#[doc = "Digital I/O control for port 1 pins PIO1_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio10](pio10) module"]
pub type PIO10 = crate::Reg<u32, _PIO10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO10;
#[doc = "`read()` method returns [pio10::R](pio10::R) reader structure"]
impl crate::Readable for PIO10 {}
#[doc = "`write(|w| ..)` method takes [pio10::W](pio10::W) writer structure"]
impl crate::Writable for PIO10 {}
#[doc = "Digital I/O control for port 1 pins PIO1_0"]
pub mod pio10;
#[doc = "Digital I/O control for port 1 pins PIO1_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio11](pio11) module"]
pub type PIO11 = crate::Reg<u32, _PIO11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO11;
#[doc = "`read()` method returns [pio11::R](pio11::R) reader structure"]
impl crate::Readable for PIO11 {}
#[doc = "`write(|w| ..)` method takes [pio11::W](pio11::W) writer structure"]
impl crate::Writable for PIO11 {}
#[doc = "Digital I/O control for port 1 pins PIO1_1"]
pub mod pio11;
#[doc = "Digital I/O control for port 1 pins PIO1_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio12](pio12) module"]
pub type PIO12 = crate::Reg<u32, _PIO12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO12;
#[doc = "`read()` method returns [pio12::R](pio12::R) reader structure"]
impl crate::Readable for PIO12 {}
#[doc = "`write(|w| ..)` method takes [pio12::W](pio12::W) writer structure"]
impl crate::Writable for PIO12 {}
#[doc = "Digital I/O control for port 1 pins PIO1_2"]
pub mod pio12;
#[doc = "Digital I/O control for port 1 pins PIO1_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio13](pio13) module"]
pub type PIO13 = crate::Reg<u32, _PIO13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO13;
#[doc = "`read()` method returns [pio13::R](pio13::R) reader structure"]
impl crate::Readable for PIO13 {}
#[doc = "`write(|w| ..)` method takes [pio13::W](pio13::W) writer structure"]
impl crate::Writable for PIO13 {}
#[doc = "Digital I/O control for port 1 pins PIO1_3"]
pub mod pio13;
#[doc = "Digital I/O control for port 1 pins PIO1_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio14](pio14) module"]
pub type PIO14 = crate::Reg<u32, _PIO14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO14;
#[doc = "`read()` method returns [pio14::R](pio14::R) reader structure"]
impl crate::Readable for PIO14 {}
#[doc = "`write(|w| ..)` method takes [pio14::W](pio14::W) writer structure"]
impl crate::Writable for PIO14 {}
#[doc = "Digital I/O control for port 1 pins PIO1_4"]
pub mod pio14;
#[doc = "Digital I/O control for port 1 pins PIO1_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio15](pio15) module"]
pub type PIO15 = crate::Reg<u32, _PIO15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO15;
#[doc = "`read()` method returns [pio15::R](pio15::R) reader structure"]
impl crate::Readable for PIO15 {}
#[doc = "`write(|w| ..)` method takes [pio15::W](pio15::W) writer structure"]
impl crate::Writable for PIO15 {}
#[doc = "Digital I/O control for port 1 pins PIO1_5"]
pub mod pio15;
#[doc = "Digital I/O control for port 1 pins PIO1_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio16](pio16) module"]
pub type PIO16 = crate::Reg<u32, _PIO16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO16;
#[doc = "`read()` method returns [pio16::R](pio16::R) reader structure"]
impl crate::Readable for PIO16 {}
#[doc = "`write(|w| ..)` method takes [pio16::W](pio16::W) writer structure"]
impl crate::Writable for PIO16 {}
#[doc = "Digital I/O control for port 1 pins PIO1_6"]
pub mod pio16;
#[doc = "Digital I/O control for port 1 pins PIO1_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio17](pio17) module"]
pub type PIO17 = crate::Reg<u32, _PIO17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO17;
#[doc = "`read()` method returns [pio17::R](pio17::R) reader structure"]
impl crate::Readable for PIO17 {}
#[doc = "`write(|w| ..)` method takes [pio17::W](pio17::W) writer structure"]
impl crate::Writable for PIO17 {}
#[doc = "Digital I/O control for port 1 pins PIO1_7"]
pub mod pio17;
#[doc = "Digital I/O control for port 1 pins PIO1_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio18](pio18) module"]
pub type PIO18 = crate::Reg<u32, _PIO18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO18;
#[doc = "`read()` method returns [pio18::R](pio18::R) reader structure"]
impl crate::Readable for PIO18 {}
#[doc = "`write(|w| ..)` method takes [pio18::W](pio18::W) writer structure"]
impl crate::Writable for PIO18 {}
#[doc = "Digital I/O control for port 1 pins PIO1_8"]
pub mod pio18;
#[doc = "Digital I/O control for port 1 pins PIO1_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio19](pio19) module"]
pub type PIO19 = crate::Reg<u32, _PIO19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO19;
#[doc = "`read()` method returns [pio19::R](pio19::R) reader structure"]
impl crate::Readable for PIO19 {}
#[doc = "`write(|w| ..)` method takes [pio19::W](pio19::W) writer structure"]
impl crate::Writable for PIO19 {}
#[doc = "Digital I/O control for port 1 pins PIO1_9"]
pub mod pio19;
#[doc = "Digital I/O control for port 1 pins PIO1_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio110](pio110) module"]
pub type PIO110 = crate::Reg<u32, _PIO110>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO110;
#[doc = "`read()` method returns [pio110::R](pio110::R) reader structure"]
impl crate::Readable for PIO110 {}
#[doc = "`write(|w| ..)` method takes [pio110::W](pio110::W) writer structure"]
impl crate::Writable for PIO110 {}
#[doc = "Digital I/O control for port 1 pins PIO1_10"]
pub mod pio110;
#[doc = "Digital I/O control for port 1 pins PIO1_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio111](pio111) module"]
pub type PIO111 = crate::Reg<u32, _PIO111>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO111;
#[doc = "`read()` method returns [pio111::R](pio111::R) reader structure"]
impl crate::Readable for PIO111 {}
#[doc = "`write(|w| ..)` method takes [pio111::W](pio111::W) writer structure"]
impl crate::Writable for PIO111 {}
#[doc = "Digital I/O control for port 1 pins PIO1_11"]
pub mod pio111;
#[doc = "Digital I/O control for port 1 pins PIO1_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio112](pio112) module"]
pub type PIO112 = crate::Reg<u32, _PIO112>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO112;
#[doc = "`read()` method returns [pio112::R](pio112::R) reader structure"]
impl crate::Readable for PIO112 {}
#[doc = "`write(|w| ..)` method takes [pio112::W](pio112::W) writer structure"]
impl crate::Writable for PIO112 {}
#[doc = "Digital I/O control for port 1 pins PIO1_12"]
pub mod pio112;
#[doc = "Digital I/O control for port 1 pins PIO1_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio113](pio113) module"]
pub type PIO113 = crate::Reg<u32, _PIO113>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO113;
#[doc = "`read()` method returns [pio113::R](pio113::R) reader structure"]
impl crate::Readable for PIO113 {}
#[doc = "`write(|w| ..)` method takes [pio113::W](pio113::W) writer structure"]
impl crate::Writable for PIO113 {}
#[doc = "Digital I/O control for port 1 pins PIO1_13"]
pub mod pio113;
#[doc = "Digital I/O control for port 1 pins PIO1_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio114](pio114) module"]
pub type PIO114 = crate::Reg<u32, _PIO114>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO114;
#[doc = "`read()` method returns [pio114::R](pio114::R) reader structure"]
impl crate::Readable for PIO114 {}
#[doc = "`write(|w| ..)` method takes [pio114::W](pio114::W) writer structure"]
impl crate::Writable for PIO114 {}
#[doc = "Digital I/O control for port 1 pins PIO1_14"]
pub mod pio114;
#[doc = "Digital I/O control for port 1 pins PIO1_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio115](pio115) module"]
pub type PIO115 = crate::Reg<u32, _PIO115>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO115;
#[doc = "`read()` method returns [pio115::R](pio115::R) reader structure"]
impl crate::Readable for PIO115 {}
#[doc = "`write(|w| ..)` method takes [pio115::W](pio115::W) writer structure"]
impl crate::Writable for PIO115 {}
#[doc = "Digital I/O control for port 1 pins PIO1_15"]
pub mod pio115;
#[doc = "Digital I/O control for port 1 pins PIO1_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio116](pio116) module"]
pub type PIO116 = crate::Reg<u32, _PIO116>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO116;
#[doc = "`read()` method returns [pio116::R](pio116::R) reader structure"]
impl crate::Readable for PIO116 {}
#[doc = "`write(|w| ..)` method takes [pio116::W](pio116::W) writer structure"]
impl crate::Writable for PIO116 {}
#[doc = "Digital I/O control for port 1 pins PIO1_16"]
pub mod pio116;
#[doc = "Digital I/O control for port 1 pins PIO1_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio117](pio117) module"]
pub type PIO117 = crate::Reg<u32, _PIO117>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO117;
#[doc = "`read()` method returns [pio117::R](pio117::R) reader structure"]
impl crate::Readable for PIO117 {}
#[doc = "`write(|w| ..)` method takes [pio117::W](pio117::W) writer structure"]
impl crate::Writable for PIO117 {}
#[doc = "Digital I/O control for port 1 pins PIO1_17"]
pub mod pio117;
#[doc = "Digital I/O control for port 1 pins PIO1_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio118](pio118) module"]
pub type PIO118 = crate::Reg<u32, _PIO118>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO118;
#[doc = "`read()` method returns [pio118::R](pio118::R) reader structure"]
impl crate::Readable for PIO118 {}
#[doc = "`write(|w| ..)` method takes [pio118::W](pio118::W) writer structure"]
impl crate::Writable for PIO118 {}
#[doc = "Digital I/O control for port 1 pins PIO1_18"]
pub mod pio118;
#[doc = "Digital I/O control for port 1 pins PIO1_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio119](pio119) module"]
pub type PIO119 = crate::Reg<u32, _PIO119>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO119;
#[doc = "`read()` method returns [pio119::R](pio119::R) reader structure"]
impl crate::Readable for PIO119 {}
#[doc = "`write(|w| ..)` method takes [pio119::W](pio119::W) writer structure"]
impl crate::Writable for PIO119 {}
#[doc = "Digital I/O control for port 1 pins PIO1_19"]
pub mod pio119;
#[doc = "Digital I/O control for port 1 pins PIO1_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio120](pio120) module"]
pub type PIO120 = crate::Reg<u32, _PIO120>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO120;
#[doc = "`read()` method returns [pio120::R](pio120::R) reader structure"]
impl crate::Readable for PIO120 {}
#[doc = "`write(|w| ..)` method takes [pio120::W](pio120::W) writer structure"]
impl crate::Writable for PIO120 {}
#[doc = "Digital I/O control for port 1 pins PIO1_20"]
pub mod pio120;
#[doc = "Digital I/O control for port 1 pins PIO1_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio121](pio121) module"]
pub type PIO121 = crate::Reg<u32, _PIO121>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO121;
#[doc = "`read()` method returns [pio121::R](pio121::R) reader structure"]
impl crate::Readable for PIO121 {}
#[doc = "`write(|w| ..)` method takes [pio121::W](pio121::W) writer structure"]
impl crate::Writable for PIO121 {}
#[doc = "Digital I/O control for port 1 pins PIO1_21"]
pub mod pio121;
#[doc = "Digital I/O control for port 1 pins PIO1_22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio122](pio122) module"]
pub type PIO122 = crate::Reg<u32, _PIO122>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO122;
#[doc = "`read()` method returns [pio122::R](pio122::R) reader structure"]
impl crate::Readable for PIO122 {}
#[doc = "`write(|w| ..)` method takes [pio122::W](pio122::W) writer structure"]
impl crate::Writable for PIO122 {}
#[doc = "Digital I/O control for port 1 pins PIO1_22"]
pub mod pio122;
#[doc = "Digital I/O control for port 1 pins PIO1_23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio123](pio123) module"]
pub type PIO123 = crate::Reg<u32, _PIO123>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO123;
#[doc = "`read()` method returns [pio123::R](pio123::R) reader structure"]
impl crate::Readable for PIO123 {}
#[doc = "`write(|w| ..)` method takes [pio123::W](pio123::W) writer structure"]
impl crate::Writable for PIO123 {}
#[doc = "Digital I/O control for port 1 pins PIO1_23"]
pub mod pio123;
#[doc = "Digital I/O control for port 1 pins PIO1_24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio124](pio124) module"]
pub type PIO124 = crate::Reg<u32, _PIO124>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO124;
#[doc = "`read()` method returns [pio124::R](pio124::R) reader structure"]
impl crate::Readable for PIO124 {}
#[doc = "`write(|w| ..)` method takes [pio124::W](pio124::W) writer structure"]
impl crate::Writable for PIO124 {}
#[doc = "Digital I/O control for port 1 pins PIO1_24"]
pub mod pio124;
#[doc = "Digital I/O control for port 1 pins PIO1_25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio125](pio125) module"]
pub type PIO125 = crate::Reg<u32, _PIO125>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO125;
#[doc = "`read()` method returns [pio125::R](pio125::R) reader structure"]
impl crate::Readable for PIO125 {}
#[doc = "`write(|w| ..)` method takes [pio125::W](pio125::W) writer structure"]
impl crate::Writable for PIO125 {}
#[doc = "Digital I/O control for port 1 pins PIO1_25"]
pub mod pio125;
#[doc = "Digital I/O control for port 1 pins PIO1_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio126](pio126) module"]
pub type PIO126 = crate::Reg<u32, _PIO126>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO126;
#[doc = "`read()` method returns [pio126::R](pio126::R) reader structure"]
impl crate::Readable for PIO126 {}
#[doc = "`write(|w| ..)` method takes [pio126::W](pio126::W) writer structure"]
impl crate::Writable for PIO126 {}
#[doc = "Digital I/O control for port 1 pins PIO1_26"]
pub mod pio126;
#[doc = "Digital I/O control for port 1 pins PIO1_27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio127](pio127) module"]
pub type PIO127 = crate::Reg<u32, _PIO127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO127;
#[doc = "`read()` method returns [pio127::R](pio127::R) reader structure"]
impl crate::Readable for PIO127 {}
#[doc = "`write(|w| ..)` method takes [pio127::W](pio127::W) writer structure"]
impl crate::Writable for PIO127 {}
#[doc = "Digital I/O control for port 1 pins PIO1_27"]
pub mod pio127;
#[doc = "Digital I/O control for port 1 pins PIO1_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio128](pio128) module"]
pub type PIO128 = crate::Reg<u32, _PIO128>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO128;
#[doc = "`read()` method returns [pio128::R](pio128::R) reader structure"]
impl crate::Readable for PIO128 {}
#[doc = "`write(|w| ..)` method takes [pio128::W](pio128::W) writer structure"]
impl crate::Writable for PIO128 {}
#[doc = "Digital I/O control for port 1 pins PIO1_28"]
pub mod pio128;
#[doc = "Digital I/O control for port 1 pins PIO1_29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio129](pio129) module"]
pub type PIO129 = crate::Reg<u32, _PIO129>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO129;
#[doc = "`read()` method returns [pio129::R](pio129::R) reader structure"]
impl crate::Readable for PIO129 {}
#[doc = "`write(|w| ..)` method takes [pio129::W](pio129::W) writer structure"]
impl crate::Writable for PIO129 {}
#[doc = "Digital I/O control for port 1 pins PIO1_29"]
pub mod pio129;
#[doc = "Digital I/O control for port 1 pins PIO1_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio130](pio130) module"]
pub type PIO130 = crate::Reg<u32, _PIO130>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO130;
#[doc = "`read()` method returns [pio130::R](pio130::R) reader structure"]
impl crate::Readable for PIO130 {}
#[doc = "`write(|w| ..)` method takes [pio130::W](pio130::W) writer structure"]
impl crate::Writable for PIO130 {}
#[doc = "Digital I/O control for port 1 pins PIO1_30"]
pub mod pio130;
#[doc = "Digital I/O control for port 1 pins PIO1_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio131](pio131) module"]
pub type PIO131 = crate::Reg<u32, _PIO131>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO131;
#[doc = "`read()` method returns [pio131::R](pio131::R) reader structure"]
impl crate::Readable for PIO131 {}
#[doc = "`write(|w| ..)` method takes [pio131::W](pio131::W) writer structure"]
impl crate::Writable for PIO131 {}
#[doc = "Digital I/O control for port 1 pins PIO1_31"]
pub mod pio131;
#[doc = "Digital I/O control for port 2 pins PIO2_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio20](pio20) module"]
pub type PIO20 = crate::Reg<u32, _PIO20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO20;
#[doc = "`read()` method returns [pio20::R](pio20::R) reader structure"]
impl crate::Readable for PIO20 {}
#[doc = "`write(|w| ..)` method takes [pio20::W](pio20::W) writer structure"]
impl crate::Writable for PIO20 {}
#[doc = "Digital I/O control for port 2 pins PIO2_0"]
pub mod pio20;
#[doc = "Digital I/O control for port 2 pins PIO2_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio21](pio21) module"]
pub type PIO21 = crate::Reg<u32, _PIO21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO21;
#[doc = "`read()` method returns [pio21::R](pio21::R) reader structure"]
impl crate::Readable for PIO21 {}
#[doc = "`write(|w| ..)` method takes [pio21::W](pio21::W) writer structure"]
impl crate::Writable for PIO21 {}
#[doc = "Digital I/O control for port 2 pins PIO2_1"]
pub mod pio21;
#[doc = "Digital I/O control for port 2 pins PIO2_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio22](pio22) module"]
pub type PIO22 = crate::Reg<u32, _PIO22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO22;
#[doc = "`read()` method returns [pio22::R](pio22::R) reader structure"]
impl crate::Readable for PIO22 {}
#[doc = "`write(|w| ..)` method takes [pio22::W](pio22::W) writer structure"]
impl crate::Writable for PIO22 {}
#[doc = "Digital I/O control for port 2 pins PIO2_2"]
pub mod pio22;
#[doc = "Digital I/O control for port 2 pins PIO2_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio23](pio23) module"]
pub type PIO23 = crate::Reg<u32, _PIO23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO23;
#[doc = "`read()` method returns [pio23::R](pio23::R) reader structure"]
impl crate::Readable for PIO23 {}
#[doc = "`write(|w| ..)` method takes [pio23::W](pio23::W) writer structure"]
impl crate::Writable for PIO23 {}
#[doc = "Digital I/O control for port 2 pins PIO2_3"]
pub mod pio23;
#[doc = "Digital I/O control for port 2 pins PIO2_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio24](pio24) module"]
pub type PIO24 = crate::Reg<u32, _PIO24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO24;
#[doc = "`read()` method returns [pio24::R](pio24::R) reader structure"]
impl crate::Readable for PIO24 {}
#[doc = "`write(|w| ..)` method takes [pio24::W](pio24::W) writer structure"]
impl crate::Writable for PIO24 {}
#[doc = "Digital I/O control for port 2 pins PIO2_4"]
pub mod pio24;
#[doc = "Digital I/O control for port 2 pins PIO2_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio25](pio25) module"]
pub type PIO25 = crate::Reg<u32, _PIO25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO25;
#[doc = "`read()` method returns [pio25::R](pio25::R) reader structure"]
impl crate::Readable for PIO25 {}
#[doc = "`write(|w| ..)` method takes [pio25::W](pio25::W) writer structure"]
impl crate::Writable for PIO25 {}
#[doc = "Digital I/O control for port 2 pins PIO2_5"]
pub mod pio25;
#[doc = "Digital I/O control for port 2 pins PIO2_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio26](pio26) module"]
pub type PIO26 = crate::Reg<u32, _PIO26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO26;
#[doc = "`read()` method returns [pio26::R](pio26::R) reader structure"]
impl crate::Readable for PIO26 {}
#[doc = "`write(|w| ..)` method takes [pio26::W](pio26::W) writer structure"]
impl crate::Writable for PIO26 {}
#[doc = "Digital I/O control for port 2 pins PIO2_6"]
pub mod pio26;
#[doc = "Digital I/O control for port 2 pins PIO2_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio27](pio27) module"]
pub type PIO27 = crate::Reg<u32, _PIO27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO27;
#[doc = "`read()` method returns [pio27::R](pio27::R) reader structure"]
impl crate::Readable for PIO27 {}
#[doc = "`write(|w| ..)` method takes [pio27::W](pio27::W) writer structure"]
impl crate::Writable for PIO27 {}
#[doc = "Digital I/O control for port 2 pins PIO2_7"]
pub mod pio27;
#[doc = "Digital I/O control for port 2 pins PIO2_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio28](pio28) module"]
pub type PIO28 = crate::Reg<u32, _PIO28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO28;
#[doc = "`read()` method returns [pio28::R](pio28::R) reader structure"]
impl crate::Readable for PIO28 {}
#[doc = "`write(|w| ..)` method takes [pio28::W](pio28::W) writer structure"]
impl crate::Writable for PIO28 {}
#[doc = "Digital I/O control for port 2 pins PIO2_8"]
pub mod pio28;
#[doc = "Digital I/O control for port 2 pins PIO2_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio29](pio29) module"]
pub type PIO29 = crate::Reg<u32, _PIO29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO29;
#[doc = "`read()` method returns [pio29::R](pio29::R) reader structure"]
impl crate::Readable for PIO29 {}
#[doc = "`write(|w| ..)` method takes [pio29::W](pio29::W) writer structure"]
impl crate::Writable for PIO29 {}
#[doc = "Digital I/O control for port 2 pins PIO2_9"]
pub mod pio29;
#[doc = "Digital I/O control for port 2 pins PIO2_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio210](pio210) module"]
pub type PIO210 = crate::Reg<u32, _PIO210>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO210;
#[doc = "`read()` method returns [pio210::R](pio210::R) reader structure"]
impl crate::Readable for PIO210 {}
#[doc = "`write(|w| ..)` method takes [pio210::W](pio210::W) writer structure"]
impl crate::Writable for PIO210 {}
#[doc = "Digital I/O control for port 2 pins PIO2_10"]
pub mod pio210;
#[doc = "Digital I/O control for port 2 pins PIO2_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio211](pio211) module"]
pub type PIO211 = crate::Reg<u32, _PIO211>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO211;
#[doc = "`read()` method returns [pio211::R](pio211::R) reader structure"]
impl crate::Readable for PIO211 {}
#[doc = "`write(|w| ..)` method takes [pio211::W](pio211::W) writer structure"]
impl crate::Writable for PIO211 {}
#[doc = "Digital I/O control for port 2 pins PIO2_11"]
pub mod pio211;
#[doc = "Digital I/O control for port 2 pins PIO2_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio212](pio212) module"]
pub type PIO212 = crate::Reg<u32, _PIO212>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO212;
#[doc = "`read()` method returns [pio212::R](pio212::R) reader structure"]
impl crate::Readable for PIO212 {}
#[doc = "`write(|w| ..)` method takes [pio212::W](pio212::W) writer structure"]
impl crate::Writable for PIO212 {}
#[doc = "Digital I/O control for port 2 pins PIO2_12"]
pub mod pio212;
#[doc = "Digital I/O control for port 2 pins PIO2_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio213](pio213) module"]
pub type PIO213 = crate::Reg<u32, _PIO213>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO213;
#[doc = "`read()` method returns [pio213::R](pio213::R) reader structure"]
impl crate::Readable for PIO213 {}
#[doc = "`write(|w| ..)` method takes [pio213::W](pio213::W) writer structure"]
impl crate::Writable for PIO213 {}
#[doc = "Digital I/O control for port 2 pins PIO2_13"]
pub mod pio213;
#[doc = "Digital I/O control for port 2 pins PIO2_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio214](pio214) module"]
pub type PIO214 = crate::Reg<u32, _PIO214>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO214;
#[doc = "`read()` method returns [pio214::R](pio214::R) reader structure"]
impl crate::Readable for PIO214 {}
#[doc = "`write(|w| ..)` method takes [pio214::W](pio214::W) writer structure"]
impl crate::Writable for PIO214 {}
#[doc = "Digital I/O control for port 2 pins PIO2_14"]
pub mod pio214;
#[doc = "Digital I/O control for port 2 pins PIO2_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio215](pio215) module"]
pub type PIO215 = crate::Reg<u32, _PIO215>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO215;
#[doc = "`read()` method returns [pio215::R](pio215::R) reader structure"]
impl crate::Readable for PIO215 {}
#[doc = "`write(|w| ..)` method takes [pio215::W](pio215::W) writer structure"]
impl crate::Writable for PIO215 {}
#[doc = "Digital I/O control for port 2 pins PIO2_15"]
pub mod pio215;
#[doc = "Digital I/O control for port 2 pins PIO2_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio216](pio216) module"]
pub type PIO216 = crate::Reg<u32, _PIO216>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO216;
#[doc = "`read()` method returns [pio216::R](pio216::R) reader structure"]
impl crate::Readable for PIO216 {}
#[doc = "`write(|w| ..)` method takes [pio216::W](pio216::W) writer structure"]
impl crate::Writable for PIO216 {}
#[doc = "Digital I/O control for port 2 pins PIO2_16"]
pub mod pio216;
#[doc = "Digital I/O control for port 2 pins PIO2_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio217](pio217) module"]
pub type PIO217 = crate::Reg<u32, _PIO217>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO217;
#[doc = "`read()` method returns [pio217::R](pio217::R) reader structure"]
impl crate::Readable for PIO217 {}
#[doc = "`write(|w| ..)` method takes [pio217::W](pio217::W) writer structure"]
impl crate::Writable for PIO217 {}
#[doc = "Digital I/O control for port 2 pins PIO2_17"]
pub mod pio217;
#[doc = "Digital I/O control for port 2 pins PIO2_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio218](pio218) module"]
pub type PIO218 = crate::Reg<u32, _PIO218>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO218;
#[doc = "`read()` method returns [pio218::R](pio218::R) reader structure"]
impl crate::Readable for PIO218 {}
#[doc = "`write(|w| ..)` method takes [pio218::W](pio218::W) writer structure"]
impl crate::Writable for PIO218 {}
#[doc = "Digital I/O control for port 2 pins PIO2_18"]
pub mod pio218;
#[doc = "Digital I/O control for port 2 pins PIO2_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio219](pio219) module"]
pub type PIO219 = crate::Reg<u32, _PIO219>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO219;
#[doc = "`read()` method returns [pio219::R](pio219::R) reader structure"]
impl crate::Readable for PIO219 {}
#[doc = "`write(|w| ..)` method takes [pio219::W](pio219::W) writer structure"]
impl crate::Writable for PIO219 {}
#[doc = "Digital I/O control for port 2 pins PIO2_19"]
pub mod pio219;
#[doc = "Digital I/O control for port 2 pins PIO2_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio220](pio220) module"]
pub type PIO220 = crate::Reg<u32, _PIO220>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO220;
#[doc = "`read()` method returns [pio220::R](pio220::R) reader structure"]
impl crate::Readable for PIO220 {}
#[doc = "`write(|w| ..)` method takes [pio220::W](pio220::W) writer structure"]
impl crate::Writable for PIO220 {}
#[doc = "Digital I/O control for port 2 pins PIO2_20"]
pub mod pio220;
#[doc = "Digital I/O control for port 2 pins PIO2_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio221](pio221) module"]
pub type PIO221 = crate::Reg<u32, _PIO221>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO221;
#[doc = "`read()` method returns [pio221::R](pio221::R) reader structure"]
impl crate::Readable for PIO221 {}
#[doc = "`write(|w| ..)` method takes [pio221::W](pio221::W) writer structure"]
impl crate::Writable for PIO221 {}
#[doc = "Digital I/O control for port 2 pins PIO2_21"]
pub mod pio221;
#[doc = "Digital I/O control for port 2 pins PIO2_22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio222](pio222) module"]
pub type PIO222 = crate::Reg<u32, _PIO222>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO222;
#[doc = "`read()` method returns [pio222::R](pio222::R) reader structure"]
impl crate::Readable for PIO222 {}
#[doc = "`write(|w| ..)` method takes [pio222::W](pio222::W) writer structure"]
impl crate::Writable for PIO222 {}
#[doc = "Digital I/O control for port 2 pins PIO2_22"]
pub mod pio222;
#[doc = "Digital I/O control for port 2 pins PIO2_23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio223](pio223) module"]
pub type PIO223 = crate::Reg<u32, _PIO223>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO223;
#[doc = "`read()` method returns [pio223::R](pio223::R) reader structure"]
impl crate::Readable for PIO223 {}
#[doc = "`write(|w| ..)` method takes [pio223::W](pio223::W) writer structure"]
impl crate::Writable for PIO223 {}
#[doc = "Digital I/O control for port 2 pins PIO2_23"]
pub mod pio223;
#[doc = "Digital I/O control for port 2 pins PIO2_24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio224](pio224) module"]
pub type PIO224 = crate::Reg<u32, _PIO224>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO224;
#[doc = "`read()` method returns [pio224::R](pio224::R) reader structure"]
impl crate::Readable for PIO224 {}
#[doc = "`write(|w| ..)` method takes [pio224::W](pio224::W) writer structure"]
impl crate::Writable for PIO224 {}
#[doc = "Digital I/O control for port 2 pins PIO2_24"]
pub mod pio224;
#[doc = "Digital I/O control for port 2 pins PIO2_25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio225](pio225) module"]
pub type PIO225 = crate::Reg<u32, _PIO225>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO225;
#[doc = "`read()` method returns [pio225::R](pio225::R) reader structure"]
impl crate::Readable for PIO225 {}
#[doc = "`write(|w| ..)` method takes [pio225::W](pio225::W) writer structure"]
impl crate::Writable for PIO225 {}
#[doc = "Digital I/O control for port 2 pins PIO2_25"]
pub mod pio225;
#[doc = "Digital I/O control for port 2 pins PIO2_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio226](pio226) module"]
pub type PIO226 = crate::Reg<u32, _PIO226>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO226;
#[doc = "`read()` method returns [pio226::R](pio226::R) reader structure"]
impl crate::Readable for PIO226 {}
#[doc = "`write(|w| ..)` method takes [pio226::W](pio226::W) writer structure"]
impl crate::Writable for PIO226 {}
#[doc = "Digital I/O control for port 2 pins PIO2_26"]
pub mod pio226;
#[doc = "Digital I/O control for port 2 pins PIO2_27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio227](pio227) module"]
pub type PIO227 = crate::Reg<u32, _PIO227>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO227;
#[doc = "`read()` method returns [pio227::R](pio227::R) reader structure"]
impl crate::Readable for PIO227 {}
#[doc = "`write(|w| ..)` method takes [pio227::W](pio227::W) writer structure"]
impl crate::Writable for PIO227 {}
#[doc = "Digital I/O control for port 2 pins PIO2_27"]
pub mod pio227;
#[doc = "Digital I/O control for port 2 pins PIO2_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio228](pio228) module"]
pub type PIO228 = crate::Reg<u32, _PIO228>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO228;
#[doc = "`read()` method returns [pio228::R](pio228::R) reader structure"]
impl crate::Readable for PIO228 {}
#[doc = "`write(|w| ..)` method takes [pio228::W](pio228::W) writer structure"]
impl crate::Writable for PIO228 {}
#[doc = "Digital I/O control for port 2 pins PIO2_28"]
pub mod pio228;
#[doc = "Digital I/O control for port 2 pins PIO2_29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio229](pio229) module"]
pub type PIO229 = crate::Reg<u32, _PIO229>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO229;
#[doc = "`read()` method returns [pio229::R](pio229::R) reader structure"]
impl crate::Readable for PIO229 {}
#[doc = "`write(|w| ..)` method takes [pio229::W](pio229::W) writer structure"]
impl crate::Writable for PIO229 {}
#[doc = "Digital I/O control for port 2 pins PIO2_29"]
pub mod pio229;
#[doc = "Digital I/O control for port 2 pins PIO2_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio230](pio230) module"]
pub type PIO230 = crate::Reg<u32, _PIO230>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO230;
#[doc = "`read()` method returns [pio230::R](pio230::R) reader structure"]
impl crate::Readable for PIO230 {}
#[doc = "`write(|w| ..)` method takes [pio230::W](pio230::W) writer structure"]
impl crate::Writable for PIO230 {}
#[doc = "Digital I/O control for port 2 pins PIO2_30"]
pub mod pio230;
#[doc = "Digital I/O control for port 2 pins PIO2_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio231](pio231) module"]
pub type PIO231 = crate::Reg<u32, _PIO231>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO231;
#[doc = "`read()` method returns [pio231::R](pio231::R) reader structure"]
impl crate::Readable for PIO231 {}
#[doc = "`write(|w| ..)` method takes [pio231::W](pio231::W) writer structure"]
impl crate::Writable for PIO231 {}
#[doc = "Digital I/O control for port 2 pins PIO2_31"]
pub mod pio231;
#[doc = "Digital I/O control for port 3 pins PIO3_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio30](pio30) module"]
pub type PIO30 = crate::Reg<u32, _PIO30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO30;
#[doc = "`read()` method returns [pio30::R](pio30::R) reader structure"]
impl crate::Readable for PIO30 {}
#[doc = "`write(|w| ..)` method takes [pio30::W](pio30::W) writer structure"]
impl crate::Writable for PIO30 {}
#[doc = "Digital I/O control for port 3 pins PIO3_0"]
pub mod pio30;
#[doc = "Digital I/O control for port 3 pins PIO3_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio31](pio31) module"]
pub type PIO31 = crate::Reg<u32, _PIO31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO31;
#[doc = "`read()` method returns [pio31::R](pio31::R) reader structure"]
impl crate::Readable for PIO31 {}
#[doc = "`write(|w| ..)` method takes [pio31::W](pio31::W) writer structure"]
impl crate::Writable for PIO31 {}
#[doc = "Digital I/O control for port 3 pins PIO3_1"]
pub mod pio31;
#[doc = "Digital I/O control for port 3 pins PIO3_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio32](pio32) module"]
pub type PIO32 = crate::Reg<u32, _PIO32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO32;
#[doc = "`read()` method returns [pio32::R](pio32::R) reader structure"]
impl crate::Readable for PIO32 {}
#[doc = "`write(|w| ..)` method takes [pio32::W](pio32::W) writer structure"]
impl crate::Writable for PIO32 {}
#[doc = "Digital I/O control for port 3 pins PIO3_2"]
pub mod pio32;
#[doc = "Digital I/O control for port 3 pins PIO3_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio33](pio33) module"]
pub type PIO33 = crate::Reg<u32, _PIO33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO33;
#[doc = "`read()` method returns [pio33::R](pio33::R) reader structure"]
impl crate::Readable for PIO33 {}
#[doc = "`write(|w| ..)` method takes [pio33::W](pio33::W) writer structure"]
impl crate::Writable for PIO33 {}
#[doc = "Digital I/O control for port 3 pins PIO3_3"]
pub mod pio33;
#[doc = "Digital I/O control for port 3 pins PIO3_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio34](pio34) module"]
pub type PIO34 = crate::Reg<u32, _PIO34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO34;
#[doc = "`read()` method returns [pio34::R](pio34::R) reader structure"]
impl crate::Readable for PIO34 {}
#[doc = "`write(|w| ..)` method takes [pio34::W](pio34::W) writer structure"]
impl crate::Writable for PIO34 {}
#[doc = "Digital I/O control for port 3 pins PIO3_4"]
pub mod pio34;
#[doc = "Digital I/O control for port 3 pins PIO3_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio35](pio35) module"]
pub type PIO35 = crate::Reg<u32, _PIO35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO35;
#[doc = "`read()` method returns [pio35::R](pio35::R) reader structure"]
impl crate::Readable for PIO35 {}
#[doc = "`write(|w| ..)` method takes [pio35::W](pio35::W) writer structure"]
impl crate::Writable for PIO35 {}
#[doc = "Digital I/O control for port 3 pins PIO3_5"]
pub mod pio35;
#[doc = "Digital I/O control for port 3 pins PIO3_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio36](pio36) module"]
pub type PIO36 = crate::Reg<u32, _PIO36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO36;
#[doc = "`read()` method returns [pio36::R](pio36::R) reader structure"]
impl crate::Readable for PIO36 {}
#[doc = "`write(|w| ..)` method takes [pio36::W](pio36::W) writer structure"]
impl crate::Writable for PIO36 {}
#[doc = "Digital I/O control for port 3 pins PIO3_6"]
pub mod pio36;
#[doc = "Digital I/O control for port 3 pins PIO3_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio37](pio37) module"]
pub type PIO37 = crate::Reg<u32, _PIO37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO37;
#[doc = "`read()` method returns [pio37::R](pio37::R) reader structure"]
impl crate::Readable for PIO37 {}
#[doc = "`write(|w| ..)` method takes [pio37::W](pio37::W) writer structure"]
impl crate::Writable for PIO37 {}
#[doc = "Digital I/O control for port 3 pins PIO3_7"]
pub mod pio37;
#[doc = "Digital I/O control for port 3 pins PIO3_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio38](pio38) module"]
pub type PIO38 = crate::Reg<u32, _PIO38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO38;
#[doc = "`read()` method returns [pio38::R](pio38::R) reader structure"]
impl crate::Readable for PIO38 {}
#[doc = "`write(|w| ..)` method takes [pio38::W](pio38::W) writer structure"]
impl crate::Writable for PIO38 {}
#[doc = "Digital I/O control for port 3 pins PIO3_8"]
pub mod pio38;
#[doc = "Digital I/O control for port 3 pins PIO3_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio39](pio39) module"]
pub type PIO39 = crate::Reg<u32, _PIO39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO39;
#[doc = "`read()` method returns [pio39::R](pio39::R) reader structure"]
impl crate::Readable for PIO39 {}
#[doc = "`write(|w| ..)` method takes [pio39::W](pio39::W) writer structure"]
impl crate::Writable for PIO39 {}
#[doc = "Digital I/O control for port 3 pins PIO3_9"]
pub mod pio39;
#[doc = "Digital I/O control for port 3 pins PIO3_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio310](pio310) module"]
pub type PIO310 = crate::Reg<u32, _PIO310>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO310;
#[doc = "`read()` method returns [pio310::R](pio310::R) reader structure"]
impl crate::Readable for PIO310 {}
#[doc = "`write(|w| ..)` method takes [pio310::W](pio310::W) writer structure"]
impl crate::Writable for PIO310 {}
#[doc = "Digital I/O control for port 3 pins PIO3_10"]
pub mod pio310;
#[doc = "Digital I/O control for port 3 pins PIO3_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio311](pio311) module"]
pub type PIO311 = crate::Reg<u32, _PIO311>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO311;
#[doc = "`read()` method returns [pio311::R](pio311::R) reader structure"]
impl crate::Readable for PIO311 {}
#[doc = "`write(|w| ..)` method takes [pio311::W](pio311::W) writer structure"]
impl crate::Writable for PIO311 {}
#[doc = "Digital I/O control for port 3 pins PIO3_11"]
pub mod pio311;
#[doc = "Digital I/O control for port 3 pins PIO3_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio312](pio312) module"]
pub type PIO312 = crate::Reg<u32, _PIO312>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO312;
#[doc = "`read()` method returns [pio312::R](pio312::R) reader structure"]
impl crate::Readable for PIO312 {}
#[doc = "`write(|w| ..)` method takes [pio312::W](pio312::W) writer structure"]
impl crate::Writable for PIO312 {}
#[doc = "Digital I/O control for port 3 pins PIO3_12"]
pub mod pio312;
#[doc = "Digital I/O control for port 3 pins PIO3_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio313](pio313) module"]
pub type PIO313 = crate::Reg<u32, _PIO313>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO313;
#[doc = "`read()` method returns [pio313::R](pio313::R) reader structure"]
impl crate::Readable for PIO313 {}
#[doc = "`write(|w| ..)` method takes [pio313::W](pio313::W) writer structure"]
impl crate::Writable for PIO313 {}
#[doc = "Digital I/O control for port 3 pins PIO3_13"]
pub mod pio313;
#[doc = "Digital I/O control for port 3 pins PIO3_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio314](pio314) module"]
pub type PIO314 = crate::Reg<u32, _PIO314>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO314;
#[doc = "`read()` method returns [pio314::R](pio314::R) reader structure"]
impl crate::Readable for PIO314 {}
#[doc = "`write(|w| ..)` method takes [pio314::W](pio314::W) writer structure"]
impl crate::Writable for PIO314 {}
#[doc = "Digital I/O control for port 3 pins PIO3_14"]
pub mod pio314;
#[doc = "Digital I/O control for port 3 pins PIO3_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio315](pio315) module"]
pub type PIO315 = crate::Reg<u32, _PIO315>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO315;
#[doc = "`read()` method returns [pio315::R](pio315::R) reader structure"]
impl crate::Readable for PIO315 {}
#[doc = "`write(|w| ..)` method takes [pio315::W](pio315::W) writer structure"]
impl crate::Writable for PIO315 {}
#[doc = "Digital I/O control for port 3 pins PIO3_15"]
pub mod pio315;
#[doc = "Digital I/O control for port 3 pins PIO3_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio316](pio316) module"]
pub type PIO316 = crate::Reg<u32, _PIO316>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO316;
#[doc = "`read()` method returns [pio316::R](pio316::R) reader structure"]
impl crate::Readable for PIO316 {}
#[doc = "`write(|w| ..)` method takes [pio316::W](pio316::W) writer structure"]
impl crate::Writable for PIO316 {}
#[doc = "Digital I/O control for port 3 pins PIO3_16"]
pub mod pio316;
#[doc = "Digital I/O control for port 3 pins PIO3_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio317](pio317) module"]
pub type PIO317 = crate::Reg<u32, _PIO317>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO317;
#[doc = "`read()` method returns [pio317::R](pio317::R) reader structure"]
impl crate::Readable for PIO317 {}
#[doc = "`write(|w| ..)` method takes [pio317::W](pio317::W) writer structure"]
impl crate::Writable for PIO317 {}
#[doc = "Digital I/O control for port 3 pins PIO3_17"]
pub mod pio317;
#[doc = "Digital I/O control for port 3 pins PIO3_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio318](pio318) module"]
pub type PIO318 = crate::Reg<u32, _PIO318>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO318;
#[doc = "`read()` method returns [pio318::R](pio318::R) reader structure"]
impl crate::Readable for PIO318 {}
#[doc = "`write(|w| ..)` method takes [pio318::W](pio318::W) writer structure"]
impl crate::Writable for PIO318 {}
#[doc = "Digital I/O control for port 3 pins PIO3_18"]
pub mod pio318;
#[doc = "Digital I/O control for port 3 pins PIO3_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio319](pio319) module"]
pub type PIO319 = crate::Reg<u32, _PIO319>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO319;
#[doc = "`read()` method returns [pio319::R](pio319::R) reader structure"]
impl crate::Readable for PIO319 {}
#[doc = "`write(|w| ..)` method takes [pio319::W](pio319::W) writer structure"]
impl crate::Writable for PIO319 {}
#[doc = "Digital I/O control for port 3 pins PIO3_19"]
pub mod pio319;
#[doc = "Digital I/O control for port 3 pins PIO3_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio320](pio320) module"]
pub type PIO320 = crate::Reg<u32, _PIO320>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO320;
#[doc = "`read()` method returns [pio320::R](pio320::R) reader structure"]
impl crate::Readable for PIO320 {}
#[doc = "`write(|w| ..)` method takes [pio320::W](pio320::W) writer structure"]
impl crate::Writable for PIO320 {}
#[doc = "Digital I/O control for port 3 pins PIO3_20"]
pub mod pio320;
#[doc = "Digital I/O control for port 3 pins PIO3_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio321](pio321) module"]
pub type PIO321 = crate::Reg<u32, _PIO321>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO321;
#[doc = "`read()` method returns [pio321::R](pio321::R) reader structure"]
impl crate::Readable for PIO321 {}
#[doc = "`write(|w| ..)` method takes [pio321::W](pio321::W) writer structure"]
impl crate::Writable for PIO321 {}
#[doc = "Digital I/O control for port 3 pins PIO3_21"]
pub mod pio321;
#[doc = "Digital I/O control for port 3 pins PIO3_22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio322](pio322) module"]
pub type PIO322 = crate::Reg<u32, _PIO322>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO322;
#[doc = "`read()` method returns [pio322::R](pio322::R) reader structure"]
impl crate::Readable for PIO322 {}
#[doc = "`write(|w| ..)` method takes [pio322::W](pio322::W) writer structure"]
impl crate::Writable for PIO322 {}
#[doc = "Digital I/O control for port 3 pins PIO3_22"]
pub mod pio322;
#[doc = "Digital I/O control for port 3 pins PIO3_23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio323](pio323) module"]
pub type PIO323 = crate::Reg<u32, _PIO323>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO323;
#[doc = "`read()` method returns [pio323::R](pio323::R) reader structure"]
impl crate::Readable for PIO323 {}
#[doc = "`write(|w| ..)` method takes [pio323::W](pio323::W) writer structure"]
impl crate::Writable for PIO323 {}
#[doc = "Digital I/O control for port 3 pins PIO3_23"]
pub mod pio323;
#[doc = "Digital I/O control for port 3 pins PIO3_24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio324](pio324) module"]
pub type PIO324 = crate::Reg<u32, _PIO324>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO324;
#[doc = "`read()` method returns [pio324::R](pio324::R) reader structure"]
impl crate::Readable for PIO324 {}
#[doc = "`write(|w| ..)` method takes [pio324::W](pio324::W) writer structure"]
impl crate::Writable for PIO324 {}
#[doc = "Digital I/O control for port 3 pins PIO3_24"]
pub mod pio324;
#[doc = "Digital I/O control for port 3 pins PIO3_25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio325](pio325) module"]
pub type PIO325 = crate::Reg<u32, _PIO325>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO325;
#[doc = "`read()` method returns [pio325::R](pio325::R) reader structure"]
impl crate::Readable for PIO325 {}
#[doc = "`write(|w| ..)` method takes [pio325::W](pio325::W) writer structure"]
impl crate::Writable for PIO325 {}
#[doc = "Digital I/O control for port 3 pins PIO3_25"]
pub mod pio325;
#[doc = "Digital I/O control for port 3 pins PIO3_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio326](pio326) module"]
pub type PIO326 = crate::Reg<u32, _PIO326>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO326;
#[doc = "`read()` method returns [pio326::R](pio326::R) reader structure"]
impl crate::Readable for PIO326 {}
#[doc = "`write(|w| ..)` method takes [pio326::W](pio326::W) writer structure"]
impl crate::Writable for PIO326 {}
#[doc = "Digital I/O control for port 3 pins PIO3_26"]
pub mod pio326;
#[doc = "Digital I/O control for port 3 pins PIO3_27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio327](pio327) module"]
pub type PIO327 = crate::Reg<u32, _PIO327>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO327;
#[doc = "`read()` method returns [pio327::R](pio327::R) reader structure"]
impl crate::Readable for PIO327 {}
#[doc = "`write(|w| ..)` method takes [pio327::W](pio327::W) writer structure"]
impl crate::Writable for PIO327 {}
#[doc = "Digital I/O control for port 3 pins PIO3_27"]
pub mod pio327;
#[doc = "Digital I/O control for port 3 pins PIO3_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio328](pio328) module"]
pub type PIO328 = crate::Reg<u32, _PIO328>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO328;
#[doc = "`read()` method returns [pio328::R](pio328::R) reader structure"]
impl crate::Readable for PIO328 {}
#[doc = "`write(|w| ..)` method takes [pio328::W](pio328::W) writer structure"]
impl crate::Writable for PIO328 {}
#[doc = "Digital I/O control for port 3 pins PIO3_28"]
pub mod pio328;
#[doc = "Digital I/O control for port 3 pins PIO3_29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio329](pio329) module"]
pub type PIO329 = crate::Reg<u32, _PIO329>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO329;
#[doc = "`read()` method returns [pio329::R](pio329::R) reader structure"]
impl crate::Readable for PIO329 {}
#[doc = "`write(|w| ..)` method takes [pio329::W](pio329::W) writer structure"]
impl crate::Writable for PIO329 {}
#[doc = "Digital I/O control for port 3 pins PIO3_29"]
pub mod pio329;
#[doc = "Digital I/O control for port 3 pins PIO3_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio330](pio330) module"]
pub type PIO330 = crate::Reg<u32, _PIO330>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO330;
#[doc = "`read()` method returns [pio330::R](pio330::R) reader structure"]
impl crate::Readable for PIO330 {}
#[doc = "`write(|w| ..)` method takes [pio330::W](pio330::W) writer structure"]
impl crate::Writable for PIO330 {}
#[doc = "Digital I/O control for port 3 pins PIO3_30"]
pub mod pio330;
#[doc = "Digital I/O control for port 3 pins PIO3_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio331](pio331) module"]
pub type PIO331 = crate::Reg<u32, _PIO331>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO331;
#[doc = "`read()` method returns [pio331::R](pio331::R) reader structure"]
impl crate::Readable for PIO331 {}
#[doc = "`write(|w| ..)` method takes [pio331::W](pio331::W) writer structure"]
impl crate::Writable for PIO331 {}
#[doc = "Digital I/O control for port 3 pins PIO3_31"]
pub mod pio331;
#[doc = "Digital I/O control for port 4 pins PIO4_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio40](pio40) module"]
pub type PIO40 = crate::Reg<u32, _PIO40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO40;
#[doc = "`read()` method returns [pio40::R](pio40::R) reader structure"]
impl crate::Readable for PIO40 {}
#[doc = "`write(|w| ..)` method takes [pio40::W](pio40::W) writer structure"]
impl crate::Writable for PIO40 {}
#[doc = "Digital I/O control for port 4 pins PIO4_0"]
pub mod pio40;
#[doc = "Digital I/O control for port 4 pins PIO4_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio41](pio41) module"]
pub type PIO41 = crate::Reg<u32, _PIO41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO41;
#[doc = "`read()` method returns [pio41::R](pio41::R) reader structure"]
impl crate::Readable for PIO41 {}
#[doc = "`write(|w| ..)` method takes [pio41::W](pio41::W) writer structure"]
impl crate::Writable for PIO41 {}
#[doc = "Digital I/O control for port 4 pins PIO4_1"]
pub mod pio41;
#[doc = "Digital I/O control for port 4 pins PIO4_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio42](pio42) module"]
pub type PIO42 = crate::Reg<u32, _PIO42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO42;
#[doc = "`read()` method returns [pio42::R](pio42::R) reader structure"]
impl crate::Readable for PIO42 {}
#[doc = "`write(|w| ..)` method takes [pio42::W](pio42::W) writer structure"]
impl crate::Writable for PIO42 {}
#[doc = "Digital I/O control for port 4 pins PIO4_2"]
pub mod pio42;
#[doc = "Digital I/O control for port 4 pins PIO4_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio43](pio43) module"]
pub type PIO43 = crate::Reg<u32, _PIO43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO43;
#[doc = "`read()` method returns [pio43::R](pio43::R) reader structure"]
impl crate::Readable for PIO43 {}
#[doc = "`write(|w| ..)` method takes [pio43::W](pio43::W) writer structure"]
impl crate::Writable for PIO43 {}
#[doc = "Digital I/O control for port 4 pins PIO4_3"]
pub mod pio43;
#[doc = "Digital I/O control for port 4 pins PIO4_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio44](pio44) module"]
pub type PIO44 = crate::Reg<u32, _PIO44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO44;
#[doc = "`read()` method returns [pio44::R](pio44::R) reader structure"]
impl crate::Readable for PIO44 {}
#[doc = "`write(|w| ..)` method takes [pio44::W](pio44::W) writer structure"]
impl crate::Writable for PIO44 {}
#[doc = "Digital I/O control for port 4 pins PIO4_4"]
pub mod pio44;
#[doc = "Digital I/O control for port 4 pins PIO4_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio45](pio45) module"]
pub type PIO45 = crate::Reg<u32, _PIO45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO45;
#[doc = "`read()` method returns [pio45::R](pio45::R) reader structure"]
impl crate::Readable for PIO45 {}
#[doc = "`write(|w| ..)` method takes [pio45::W](pio45::W) writer structure"]
impl crate::Writable for PIO45 {}
#[doc = "Digital I/O control for port 4 pins PIO4_5"]
pub mod pio45;
#[doc = "Digital I/O control for port 4 pins PIO4_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio46](pio46) module"]
pub type PIO46 = crate::Reg<u32, _PIO46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO46;
#[doc = "`read()` method returns [pio46::R](pio46::R) reader structure"]
impl crate::Readable for PIO46 {}
#[doc = "`write(|w| ..)` method takes [pio46::W](pio46::W) writer structure"]
impl crate::Writable for PIO46 {}
#[doc = "Digital I/O control for port 4 pins PIO4_6"]
pub mod pio46;
#[doc = "Digital I/O control for port 4 pins PIO4_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio47](pio47) module"]
pub type PIO47 = crate::Reg<u32, _PIO47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO47;
#[doc = "`read()` method returns [pio47::R](pio47::R) reader structure"]
impl crate::Readable for PIO47 {}
#[doc = "`write(|w| ..)` method takes [pio47::W](pio47::W) writer structure"]
impl crate::Writable for PIO47 {}
#[doc = "Digital I/O control for port 4 pins PIO4_7"]
pub mod pio47;
#[doc = "Digital I/O control for port 4 pins PIO4_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio48](pio48) module"]
pub type PIO48 = crate::Reg<u32, _PIO48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO48;
#[doc = "`read()` method returns [pio48::R](pio48::R) reader structure"]
impl crate::Readable for PIO48 {}
#[doc = "`write(|w| ..)` method takes [pio48::W](pio48::W) writer structure"]
impl crate::Writable for PIO48 {}
#[doc = "Digital I/O control for port 4 pins PIO4_8"]
pub mod pio48;
#[doc = "Digital I/O control for port 4 pins PIO4_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio49](pio49) module"]
pub type PIO49 = crate::Reg<u32, _PIO49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO49;
#[doc = "`read()` method returns [pio49::R](pio49::R) reader structure"]
impl crate::Readable for PIO49 {}
#[doc = "`write(|w| ..)` method takes [pio49::W](pio49::W) writer structure"]
impl crate::Writable for PIO49 {}
#[doc = "Digital I/O control for port 4 pins PIO4_9"]
pub mod pio49;
#[doc = "Digital I/O control for port 4 pins PIO4_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio410](pio410) module"]
pub type PIO410 = crate::Reg<u32, _PIO410>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO410;
#[doc = "`read()` method returns [pio410::R](pio410::R) reader structure"]
impl crate::Readable for PIO410 {}
#[doc = "`write(|w| ..)` method takes [pio410::W](pio410::W) writer structure"]
impl crate::Writable for PIO410 {}
#[doc = "Digital I/O control for port 4 pins PIO4_10"]
pub mod pio410;
#[doc = "Digital I/O control for port 4 pins PIO4_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio411](pio411) module"]
pub type PIO411 = crate::Reg<u32, _PIO411>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO411;
#[doc = "`read()` method returns [pio411::R](pio411::R) reader structure"]
impl crate::Readable for PIO411 {}
#[doc = "`write(|w| ..)` method takes [pio411::W](pio411::W) writer structure"]
impl crate::Writable for PIO411 {}
#[doc = "Digital I/O control for port 4 pins PIO4_11"]
pub mod pio411;
#[doc = "Digital I/O control for port 4 pins PIO4_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio412](pio412) module"]
pub type PIO412 = crate::Reg<u32, _PIO412>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO412;
#[doc = "`read()` method returns [pio412::R](pio412::R) reader structure"]
impl crate::Readable for PIO412 {}
#[doc = "`write(|w| ..)` method takes [pio412::W](pio412::W) writer structure"]
impl crate::Writable for PIO412 {}
#[doc = "Digital I/O control for port 4 pins PIO4_12"]
pub mod pio412;
#[doc = "Digital I/O control for port 4 pins PIO4_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio413](pio413) module"]
pub type PIO413 = crate::Reg<u32, _PIO413>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO413;
#[doc = "`read()` method returns [pio413::R](pio413::R) reader structure"]
impl crate::Readable for PIO413 {}
#[doc = "`write(|w| ..)` method takes [pio413::W](pio413::W) writer structure"]
impl crate::Writable for PIO413 {}
#[doc = "Digital I/O control for port 4 pins PIO4_13"]
pub mod pio413;
#[doc = "Digital I/O control for port 4 pins PIO4_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio414](pio414) module"]
pub type PIO414 = crate::Reg<u32, _PIO414>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO414;
#[doc = "`read()` method returns [pio414::R](pio414::R) reader structure"]
impl crate::Readable for PIO414 {}
#[doc = "`write(|w| ..)` method takes [pio414::W](pio414::W) writer structure"]
impl crate::Writable for PIO414 {}
#[doc = "Digital I/O control for port 4 pins PIO4_14"]
pub mod pio414;
#[doc = "Digital I/O control for port 4 pins PIO4_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio415](pio415) module"]
pub type PIO415 = crate::Reg<u32, _PIO415>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO415;
#[doc = "`read()` method returns [pio415::R](pio415::R) reader structure"]
impl crate::Readable for PIO415 {}
#[doc = "`write(|w| ..)` method takes [pio415::W](pio415::W) writer structure"]
impl crate::Writable for PIO415 {}
#[doc = "Digital I/O control for port 4 pins PIO4_15"]
pub mod pio415;
#[doc = "Digital I/O control for port 4 pins PIO4_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio416](pio416) module"]
pub type PIO416 = crate::Reg<u32, _PIO416>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO416;
#[doc = "`read()` method returns [pio416::R](pio416::R) reader structure"]
impl crate::Readable for PIO416 {}
#[doc = "`write(|w| ..)` method takes [pio416::W](pio416::W) writer structure"]
impl crate::Writable for PIO416 {}
#[doc = "Digital I/O control for port 4 pins PIO4_16"]
pub mod pio416;
#[doc = "Digital I/O control for port 4 pins PIO4_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio417](pio417) module"]
pub type PIO417 = crate::Reg<u32, _PIO417>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO417;
#[doc = "`read()` method returns [pio417::R](pio417::R) reader structure"]
impl crate::Readable for PIO417 {}
#[doc = "`write(|w| ..)` method takes [pio417::W](pio417::W) writer structure"]
impl crate::Writable for PIO417 {}
#[doc = "Digital I/O control for port 4 pins PIO4_17"]
pub mod pio417;
#[doc = "Digital I/O control for port 4 pins PIO4_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio418](pio418) module"]
pub type PIO418 = crate::Reg<u32, _PIO418>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO418;
#[doc = "`read()` method returns [pio418::R](pio418::R) reader structure"]
impl crate::Readable for PIO418 {}
#[doc = "`write(|w| ..)` method takes [pio418::W](pio418::W) writer structure"]
impl crate::Writable for PIO418 {}
#[doc = "Digital I/O control for port 4 pins PIO4_18"]
pub mod pio418;
#[doc = "Digital I/O control for port 4 pins PIO4_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio419](pio419) module"]
pub type PIO419 = crate::Reg<u32, _PIO419>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO419;
#[doc = "`read()` method returns [pio419::R](pio419::R) reader structure"]
impl crate::Readable for PIO419 {}
#[doc = "`write(|w| ..)` method takes [pio419::W](pio419::W) writer structure"]
impl crate::Writable for PIO419 {}
#[doc = "Digital I/O control for port 4 pins PIO4_19"]
pub mod pio419;
#[doc = "Digital I/O control for port 4 pins PIO4_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio420](pio420) module"]
pub type PIO420 = crate::Reg<u32, _PIO420>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO420;
#[doc = "`read()` method returns [pio420::R](pio420::R) reader structure"]
impl crate::Readable for PIO420 {}
#[doc = "`write(|w| ..)` method takes [pio420::W](pio420::W) writer structure"]
impl crate::Writable for PIO420 {}
#[doc = "Digital I/O control for port 4 pins PIO4_20"]
pub mod pio420;
#[doc = "Digital I/O control for port 4 pins PIO4_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio421](pio421) module"]
pub type PIO421 = crate::Reg<u32, _PIO421>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO421;
#[doc = "`read()` method returns [pio421::R](pio421::R) reader structure"]
impl crate::Readable for PIO421 {}
#[doc = "`write(|w| ..)` method takes [pio421::W](pio421::W) writer structure"]
impl crate::Writable for PIO421 {}
#[doc = "Digital I/O control for port 4 pins PIO4_21"]
pub mod pio421;
#[doc = "Digital I/O control for port 4 pins PIO4_22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio422](pio422) module"]
pub type PIO422 = crate::Reg<u32, _PIO422>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO422;
#[doc = "`read()` method returns [pio422::R](pio422::R) reader structure"]
impl crate::Readable for PIO422 {}
#[doc = "`write(|w| ..)` method takes [pio422::W](pio422::W) writer structure"]
impl crate::Writable for PIO422 {}
#[doc = "Digital I/O control for port 4 pins PIO4_22"]
pub mod pio422;
#[doc = "Digital I/O control for port 4 pins PIO4_23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio423](pio423) module"]
pub type PIO423 = crate::Reg<u32, _PIO423>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO423;
#[doc = "`read()` method returns [pio423::R](pio423::R) reader structure"]
impl crate::Readable for PIO423 {}
#[doc = "`write(|w| ..)` method takes [pio423::W](pio423::W) writer structure"]
impl crate::Writable for PIO423 {}
#[doc = "Digital I/O control for port 4 pins PIO4_23"]
pub mod pio423;
#[doc = "Digital I/O control for port 4 pins PIO4_24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio424](pio424) module"]
pub type PIO424 = crate::Reg<u32, _PIO424>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO424;
#[doc = "`read()` method returns [pio424::R](pio424::R) reader structure"]
impl crate::Readable for PIO424 {}
#[doc = "`write(|w| ..)` method takes [pio424::W](pio424::W) writer structure"]
impl crate::Writable for PIO424 {}
#[doc = "Digital I/O control for port 4 pins PIO4_24"]
pub mod pio424;
#[doc = "Digital I/O control for port 4 pins PIO4_25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio425](pio425) module"]
pub type PIO425 = crate::Reg<u32, _PIO425>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO425;
#[doc = "`read()` method returns [pio425::R](pio425::R) reader structure"]
impl crate::Readable for PIO425 {}
#[doc = "`write(|w| ..)` method takes [pio425::W](pio425::W) writer structure"]
impl crate::Writable for PIO425 {}
#[doc = "Digital I/O control for port 4 pins PIO4_25"]
pub mod pio425;
#[doc = "Digital I/O control for port 4 pins PIO4_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio426](pio426) module"]
pub type PIO426 = crate::Reg<u32, _PIO426>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO426;
#[doc = "`read()` method returns [pio426::R](pio426::R) reader structure"]
impl crate::Readable for PIO426 {}
#[doc = "`write(|w| ..)` method takes [pio426::W](pio426::W) writer structure"]
impl crate::Writable for PIO426 {}
#[doc = "Digital I/O control for port 4 pins PIO4_26"]
pub mod pio426;
#[doc = "Digital I/O control for port 4 pins PIO4_27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio427](pio427) module"]
pub type PIO427 = crate::Reg<u32, _PIO427>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO427;
#[doc = "`read()` method returns [pio427::R](pio427::R) reader structure"]
impl crate::Readable for PIO427 {}
#[doc = "`write(|w| ..)` method takes [pio427::W](pio427::W) writer structure"]
impl crate::Writable for PIO427 {}
#[doc = "Digital I/O control for port 4 pins PIO4_27"]
pub mod pio427;
#[doc = "Digital I/O control for port 4 pins PIO4_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio428](pio428) module"]
pub type PIO428 = crate::Reg<u32, _PIO428>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO428;
#[doc = "`read()` method returns [pio428::R](pio428::R) reader structure"]
impl crate::Readable for PIO428 {}
#[doc = "`write(|w| ..)` method takes [pio428::W](pio428::W) writer structure"]
impl crate::Writable for PIO428 {}
#[doc = "Digital I/O control for port 4 pins PIO4_28"]
pub mod pio428;
#[doc = "Digital I/O control for port 4 pins PIO4_29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio429](pio429) module"]
pub type PIO429 = crate::Reg<u32, _PIO429>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO429;
#[doc = "`read()` method returns [pio429::R](pio429::R) reader structure"]
impl crate::Readable for PIO429 {}
#[doc = "`write(|w| ..)` method takes [pio429::W](pio429::W) writer structure"]
impl crate::Writable for PIO429 {}
#[doc = "Digital I/O control for port 4 pins PIO4_29"]
pub mod pio429;
#[doc = "Digital I/O control for port 4 pins PIO4_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio430](pio430) module"]
pub type PIO430 = crate::Reg<u32, _PIO430>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO430;
#[doc = "`read()` method returns [pio430::R](pio430::R) reader structure"]
impl crate::Readable for PIO430 {}
#[doc = "`write(|w| ..)` method takes [pio430::W](pio430::W) writer structure"]
impl crate::Writable for PIO430 {}
#[doc = "Digital I/O control for port 4 pins PIO4_30"]
pub mod pio430;
#[doc = "Digital I/O control for port 4 pins PIO4_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio431](pio431) module"]
pub type PIO431 = crate::Reg<u32, _PIO431>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO431;
#[doc = "`read()` method returns [pio431::R](pio431::R) reader structure"]
impl crate::Readable for PIO431 {}
#[doc = "`write(|w| ..)` method takes [pio431::W](pio431::W) writer structure"]
impl crate::Writable for PIO431 {}
#[doc = "Digital I/O control for port 4 pins PIO4_31"]
pub mod pio431;
#[doc = "Digital I/O control for port 5 pins PIO5_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio50](pio50) module"]
pub type PIO50 = crate::Reg<u32, _PIO50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO50;
#[doc = "`read()` method returns [pio50::R](pio50::R) reader structure"]
impl crate::Readable for PIO50 {}
#[doc = "`write(|w| ..)` method takes [pio50::W](pio50::W) writer structure"]
impl crate::Writable for PIO50 {}
#[doc = "Digital I/O control for port 5 pins PIO5_0"]
pub mod pio50;
#[doc = "Digital I/O control for port 5 pins PIO5_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio51](pio51) module"]
pub type PIO51 = crate::Reg<u32, _PIO51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO51;
#[doc = "`read()` method returns [pio51::R](pio51::R) reader structure"]
impl crate::Readable for PIO51 {}
#[doc = "`write(|w| ..)` method takes [pio51::W](pio51::W) writer structure"]
impl crate::Writable for PIO51 {}
#[doc = "Digital I/O control for port 5 pins PIO5_1"]
pub mod pio51;
#[doc = "Digital I/O control for port 5 pins PIO5_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio52](pio52) module"]
pub type PIO52 = crate::Reg<u32, _PIO52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO52;
#[doc = "`read()` method returns [pio52::R](pio52::R) reader structure"]
impl crate::Readable for PIO52 {}
#[doc = "`write(|w| ..)` method takes [pio52::W](pio52::W) writer structure"]
impl crate::Writable for PIO52 {}
#[doc = "Digital I/O control for port 5 pins PIO5_2"]
pub mod pio52;
#[doc = "Digital I/O control for port 5 pins PIO5_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio53](pio53) module"]
pub type PIO53 = crate::Reg<u32, _PIO53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO53;
#[doc = "`read()` method returns [pio53::R](pio53::R) reader structure"]
impl crate::Readable for PIO53 {}
#[doc = "`write(|w| ..)` method takes [pio53::W](pio53::W) writer structure"]
impl crate::Writable for PIO53 {}
#[doc = "Digital I/O control for port 5 pins PIO5_3"]
pub mod pio53;
#[doc = "Digital I/O control for port 5 pins PIO5_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio54](pio54) module"]
pub type PIO54 = crate::Reg<u32, _PIO54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO54;
#[doc = "`read()` method returns [pio54::R](pio54::R) reader structure"]
impl crate::Readable for PIO54 {}
#[doc = "`write(|w| ..)` method takes [pio54::W](pio54::W) writer structure"]
impl crate::Writable for PIO54 {}
#[doc = "Digital I/O control for port 5 pins PIO5_4"]
pub mod pio54;
#[doc = "Digital I/O control for port 5 pins PIO5_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio55](pio55) module"]
pub type PIO55 = crate::Reg<u32, _PIO55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO55;
#[doc = "`read()` method returns [pio55::R](pio55::R) reader structure"]
impl crate::Readable for PIO55 {}
#[doc = "`write(|w| ..)` method takes [pio55::W](pio55::W) writer structure"]
impl crate::Writable for PIO55 {}
#[doc = "Digital I/O control for port 5 pins PIO5_5"]
pub mod pio55;
#[doc = "Digital I/O control for port 5 pins PIO5_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio56](pio56) module"]
pub type PIO56 = crate::Reg<u32, _PIO56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO56;
#[doc = "`read()` method returns [pio56::R](pio56::R) reader structure"]
impl crate::Readable for PIO56 {}
#[doc = "`write(|w| ..)` method takes [pio56::W](pio56::W) writer structure"]
impl crate::Writable for PIO56 {}
#[doc = "Digital I/O control for port 5 pins PIO5_6"]
pub mod pio56;
#[doc = "Digital I/O control for port 5 pins PIO5_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio57](pio57) module"]
pub type PIO57 = crate::Reg<u32, _PIO57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO57;
#[doc = "`read()` method returns [pio57::R](pio57::R) reader structure"]
impl crate::Readable for PIO57 {}
#[doc = "`write(|w| ..)` method takes [pio57::W](pio57::W) writer structure"]
impl crate::Writable for PIO57 {}
#[doc = "Digital I/O control for port 5 pins PIO5_7"]
pub mod pio57;
#[doc = "Digital I/O control for port 5 pins PIO5_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio58](pio58) module"]
pub type PIO58 = crate::Reg<u32, _PIO58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO58;
#[doc = "`read()` method returns [pio58::R](pio58::R) reader structure"]
impl crate::Readable for PIO58 {}
#[doc = "`write(|w| ..)` method takes [pio58::W](pio58::W) writer structure"]
impl crate::Writable for PIO58 {}
#[doc = "Digital I/O control for port 5 pins PIO5_8"]
pub mod pio58;
#[doc = "Digital I/O control for port 5 pins PIO5_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio59](pio59) module"]
pub type PIO59 = crate::Reg<u32, _PIO59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO59;
#[doc = "`read()` method returns [pio59::R](pio59::R) reader structure"]
impl crate::Readable for PIO59 {}
#[doc = "`write(|w| ..)` method takes [pio59::W](pio59::W) writer structure"]
impl crate::Writable for PIO59 {}
#[doc = "Digital I/O control for port 5 pins PIO5_9"]
pub mod pio59;
#[doc = "Digital I/O control for port 5 pins PIO5_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio510](pio510) module"]
pub type PIO510 = crate::Reg<u32, _PIO510>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO510;
#[doc = "`read()` method returns [pio510::R](pio510::R) reader structure"]
impl crate::Readable for PIO510 {}
#[doc = "`write(|w| ..)` method takes [pio510::W](pio510::W) writer structure"]
impl crate::Writable for PIO510 {}
#[doc = "Digital I/O control for port 5 pins PIO5_10"]
pub mod pio510;
#[doc = "Digital I/O control for port 5 pins PIO5_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio511](pio511) module"]
pub type PIO511 = crate::Reg<u32, _PIO511>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO511;
#[doc = "`read()` method returns [pio511::R](pio511::R) reader structure"]
impl crate::Readable for PIO511 {}
#[doc = "`write(|w| ..)` method takes [pio511::W](pio511::W) writer structure"]
impl crate::Writable for PIO511 {}
#[doc = "Digital I/O control for port 5 pins PIO5_11"]
pub mod pio511;
#[doc = "Digital I/O control for port 5 pins PIO5_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio512](pio512) module"]
pub type PIO512 = crate::Reg<u32, _PIO512>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO512;
#[doc = "`read()` method returns [pio512::R](pio512::R) reader structure"]
impl crate::Readable for PIO512 {}
#[doc = "`write(|w| ..)` method takes [pio512::W](pio512::W) writer structure"]
impl crate::Writable for PIO512 {}
#[doc = "Digital I/O control for port 5 pins PIO5_12"]
pub mod pio512;
#[doc = "Digital I/O control for port 5 pins PIO5_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio513](pio513) module"]
pub type PIO513 = crate::Reg<u32, _PIO513>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO513;
#[doc = "`read()` method returns [pio513::R](pio513::R) reader structure"]
impl crate::Readable for PIO513 {}
#[doc = "`write(|w| ..)` method takes [pio513::W](pio513::W) writer structure"]
impl crate::Writable for PIO513 {}
#[doc = "Digital I/O control for port 5 pins PIO5_13"]
pub mod pio513;
#[doc = "Digital I/O control for port 5 pins PIO5_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio514](pio514) module"]
pub type PIO514 = crate::Reg<u32, _PIO514>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO514;
#[doc = "`read()` method returns [pio514::R](pio514::R) reader structure"]
impl crate::Readable for PIO514 {}
#[doc = "`write(|w| ..)` method takes [pio514::W](pio514::W) writer structure"]
impl crate::Writable for PIO514 {}
#[doc = "Digital I/O control for port 5 pins PIO5_14"]
pub mod pio514;
#[doc = "Digital I/O control for port 5 pins PIO5_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio515](pio515) module"]
pub type PIO515 = crate::Reg<u32, _PIO515>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO515;
#[doc = "`read()` method returns [pio515::R](pio515::R) reader structure"]
impl crate::Readable for PIO515 {}
#[doc = "`write(|w| ..)` method takes [pio515::W](pio515::W) writer structure"]
impl crate::Writable for PIO515 {}
#[doc = "Digital I/O control for port 5 pins PIO5_15"]
pub mod pio515;
#[doc = "Digital I/O control for port 5 pins PIO5_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio516](pio516) module"]
pub type PIO516 = crate::Reg<u32, _PIO516>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO516;
#[doc = "`read()` method returns [pio516::R](pio516::R) reader structure"]
impl crate::Readable for PIO516 {}
#[doc = "`write(|w| ..)` method takes [pio516::W](pio516::W) writer structure"]
impl crate::Writable for PIO516 {}
#[doc = "Digital I/O control for port 5 pins PIO5_16"]
pub mod pio516;
#[doc = "Digital I/O control for port 5 pins PIO5_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio517](pio517) module"]
pub type PIO517 = crate::Reg<u32, _PIO517>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO517;
#[doc = "`read()` method returns [pio517::R](pio517::R) reader structure"]
impl crate::Readable for PIO517 {}
#[doc = "`write(|w| ..)` method takes [pio517::W](pio517::W) writer structure"]
impl crate::Writable for PIO517 {}
#[doc = "Digital I/O control for port 5 pins PIO5_17"]
pub mod pio517;
#[doc = "Digital I/O control for port 5 pins PIO5_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio518](pio518) module"]
pub type PIO518 = crate::Reg<u32, _PIO518>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO518;
#[doc = "`read()` method returns [pio518::R](pio518::R) reader structure"]
impl crate::Readable for PIO518 {}
#[doc = "`write(|w| ..)` method takes [pio518::W](pio518::W) writer structure"]
impl crate::Writable for PIO518 {}
#[doc = "Digital I/O control for port 5 pins PIO5_18"]
pub mod pio518;
#[doc = "Digital I/O control for port 5 pins PIO5_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio519](pio519) module"]
pub type PIO519 = crate::Reg<u32, _PIO519>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO519;
#[doc = "`read()` method returns [pio519::R](pio519::R) reader structure"]
impl crate::Readable for PIO519 {}
#[doc = "`write(|w| ..)` method takes [pio519::W](pio519::W) writer structure"]
impl crate::Writable for PIO519 {}
#[doc = "Digital I/O control for port 5 pins PIO5_19"]
pub mod pio519;
#[doc = "Digital I/O control for port 5 pins PIO5_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio520](pio520) module"]
pub type PIO520 = crate::Reg<u32, _PIO520>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO520;
#[doc = "`read()` method returns [pio520::R](pio520::R) reader structure"]
impl crate::Readable for PIO520 {}
#[doc = "`write(|w| ..)` method takes [pio520::W](pio520::W) writer structure"]
impl crate::Writable for PIO520 {}
#[doc = "Digital I/O control for port 5 pins PIO5_20"]
pub mod pio520;
#[doc = "Digital I/O control for port 5 pins PIO5_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio521](pio521) module"]
pub type PIO521 = crate::Reg<u32, _PIO521>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO521;
#[doc = "`read()` method returns [pio521::R](pio521::R) reader structure"]
impl crate::Readable for PIO521 {}
#[doc = "`write(|w| ..)` method takes [pio521::W](pio521::W) writer structure"]
impl crate::Writable for PIO521 {}
#[doc = "Digital I/O control for port 5 pins PIO5_21"]
pub mod pio521;
#[doc = "Digital I/O control for port 5 pins PIO5_22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio522](pio522) module"]
pub type PIO522 = crate::Reg<u32, _PIO522>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO522;
#[doc = "`read()` method returns [pio522::R](pio522::R) reader structure"]
impl crate::Readable for PIO522 {}
#[doc = "`write(|w| ..)` method takes [pio522::W](pio522::W) writer structure"]
impl crate::Writable for PIO522 {}
#[doc = "Digital I/O control for port 5 pins PIO5_22"]
pub mod pio522;
#[doc = "Digital I/O control for port 5 pins PIO5_23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio523](pio523) module"]
pub type PIO523 = crate::Reg<u32, _PIO523>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO523;
#[doc = "`read()` method returns [pio523::R](pio523::R) reader structure"]
impl crate::Readable for PIO523 {}
#[doc = "`write(|w| ..)` method takes [pio523::W](pio523::W) writer structure"]
impl crate::Writable for PIO523 {}
#[doc = "Digital I/O control for port 5 pins PIO5_23"]
pub mod pio523;
#[doc = "Digital I/O control for port 5 pins PIO5_24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio524](pio524) module"]
pub type PIO524 = crate::Reg<u32, _PIO524>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO524;
#[doc = "`read()` method returns [pio524::R](pio524::R) reader structure"]
impl crate::Readable for PIO524 {}
#[doc = "`write(|w| ..)` method takes [pio524::W](pio524::W) writer structure"]
impl crate::Writable for PIO524 {}
#[doc = "Digital I/O control for port 5 pins PIO5_24"]
pub mod pio524;
#[doc = "Digital I/O control for port 5 pins PIO5_25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio525](pio525) module"]
pub type PIO525 = crate::Reg<u32, _PIO525>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO525;
#[doc = "`read()` method returns [pio525::R](pio525::R) reader structure"]
impl crate::Readable for PIO525 {}
#[doc = "`write(|w| ..)` method takes [pio525::W](pio525::W) writer structure"]
impl crate::Writable for PIO525 {}
#[doc = "Digital I/O control for port 5 pins PIO5_25"]
pub mod pio525;
#[doc = "Digital I/O control for port 5 pins PIO5_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio526](pio526) module"]
pub type PIO526 = crate::Reg<u32, _PIO526>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO526;
#[doc = "`read()` method returns [pio526::R](pio526::R) reader structure"]
impl crate::Readable for PIO526 {}
#[doc = "`write(|w| ..)` method takes [pio526::W](pio526::W) writer structure"]
impl crate::Writable for PIO526 {}
#[doc = "Digital I/O control for port 5 pins PIO5_26"]
pub mod pio526;
#[doc = "Digital I/O control for port 5 pins PIO5_27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio527](pio527) module"]
pub type PIO527 = crate::Reg<u32, _PIO527>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO527;
#[doc = "`read()` method returns [pio527::R](pio527::R) reader structure"]
impl crate::Readable for PIO527 {}
#[doc = "`write(|w| ..)` method takes [pio527::W](pio527::W) writer structure"]
impl crate::Writable for PIO527 {}
#[doc = "Digital I/O control for port 5 pins PIO5_27"]
pub mod pio527;
#[doc = "Digital I/O control for port 5 pins PIO5_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio528](pio528) module"]
pub type PIO528 = crate::Reg<u32, _PIO528>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO528;
#[doc = "`read()` method returns [pio528::R](pio528::R) reader structure"]
impl crate::Readable for PIO528 {}
#[doc = "`write(|w| ..)` method takes [pio528::W](pio528::W) writer structure"]
impl crate::Writable for PIO528 {}
#[doc = "Digital I/O control for port 5 pins PIO5_28"]
pub mod pio528;
#[doc = "Digital I/O control for port 5 pins PIO5_29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio529](pio529) module"]
pub type PIO529 = crate::Reg<u32, _PIO529>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO529;
#[doc = "`read()` method returns [pio529::R](pio529::R) reader structure"]
impl crate::Readable for PIO529 {}
#[doc = "`write(|w| ..)` method takes [pio529::W](pio529::W) writer structure"]
impl crate::Writable for PIO529 {}
#[doc = "Digital I/O control for port 5 pins PIO5_29"]
pub mod pio529;
#[doc = "Digital I/O control for port 5 pins PIO5_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio530](pio530) module"]
pub type PIO530 = crate::Reg<u32, _PIO530>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO530;
#[doc = "`read()` method returns [pio530::R](pio530::R) reader structure"]
impl crate::Readable for PIO530 {}
#[doc = "`write(|w| ..)` method takes [pio530::W](pio530::W) writer structure"]
impl crate::Writable for PIO530 {}
#[doc = "Digital I/O control for port 5 pins PIO5_30"]
pub mod pio530;
#[doc = "Digital I/O control for port 5 pins PIO5_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio531](pio531) module"]
pub type PIO531 = crate::Reg<u32, _PIO531>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO531;
#[doc = "`read()` method returns [pio531::R](pio531::R) reader structure"]
impl crate::Readable for PIO531 {}
#[doc = "`write(|w| ..)` method takes [pio531::W](pio531::W) writer structure"]
impl crate::Writable for PIO531 {}
#[doc = "Digital I/O control for port 5 pins PIO5_31"]
pub mod pio531;
