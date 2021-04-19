#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital I/O control for port 0 pins PIO0_0"]
    pub pio00: crate::Reg<pio00::PIO00_SPEC>,
    #[doc = "0x04 - Digital I/O control for port 0 pins PIO0_1"]
    pub pio01: crate::Reg<pio01::PIO01_SPEC>,
    #[doc = "0x08 - Digital I/O control for port 0 pins PIO0_2"]
    pub pio02: crate::Reg<pio02::PIO02_SPEC>,
    #[doc = "0x0c - Digital I/O control for port 0 pins PIO0_3"]
    pub pio03: crate::Reg<pio03::PIO03_SPEC>,
    #[doc = "0x10 - Digital I/O control for port 0 pins PIO0_4"]
    pub pio04: crate::Reg<pio04::PIO04_SPEC>,
    #[doc = "0x14 - Digital I/O control for port 0 pins PIO0_5"]
    pub pio05: crate::Reg<pio05::PIO05_SPEC>,
    #[doc = "0x18 - Digital I/O control for port 0 pins PIO0_6"]
    pub pio06: crate::Reg<pio06::PIO06_SPEC>,
    #[doc = "0x1c - Digital I/O control for port 0 pins PIO0_7"]
    pub pio07: crate::Reg<pio07::PIO07_SPEC>,
    #[doc = "0x20 - Digital I/O control for port 0 pins PIO0_8"]
    pub pio08: crate::Reg<pio08::PIO08_SPEC>,
    #[doc = "0x24 - Digital I/O control for port 0 pins PIO0_9"]
    pub pio09: crate::Reg<pio09::PIO09_SPEC>,
    #[doc = "0x28 - Digital I/O control for port 0 pins PIO0_10"]
    pub pio010: crate::Reg<pio010::PIO010_SPEC>,
    #[doc = "0x2c - Digital I/O control for port 0 pins PIO0_11"]
    pub pio011: crate::Reg<pio011::PIO011_SPEC>,
    #[doc = "0x30 - Digital I/O control for port 0 pins PIO0_12"]
    pub pio012: crate::Reg<pio012::PIO012_SPEC>,
    #[doc = "0x34 - Digital I/O control for port 0 pins PIO0_13"]
    pub pio013: crate::Reg<pio013::PIO013_SPEC>,
    #[doc = "0x38 - Digital I/O control for port 0 pins PIO0_14"]
    pub pio014: crate::Reg<pio014::PIO014_SPEC>,
    #[doc = "0x3c - Digital I/O control for port 0 pins PIO0_15"]
    pub pio015: crate::Reg<pio015::PIO015_SPEC>,
    #[doc = "0x40 - Digital I/O control for port 0 pins PIO0_16"]
    pub pio016: crate::Reg<pio016::PIO016_SPEC>,
    #[doc = "0x44 - Digital I/O control for port 0 pins PIO0_17"]
    pub pio017: crate::Reg<pio017::PIO017_SPEC>,
    #[doc = "0x48 - Digital I/O control for port 0 pins PIO0_18"]
    pub pio018: crate::Reg<pio018::PIO018_SPEC>,
    #[doc = "0x4c - Digital I/O control for port 0 pins PIO0_19"]
    pub pio019: crate::Reg<pio019::PIO019_SPEC>,
    #[doc = "0x50 - Digital I/O control for port 0 pins PIO0_20"]
    pub pio020: crate::Reg<pio020::PIO020_SPEC>,
    #[doc = "0x54 - Digital I/O control for port 0 pins PIO0_21"]
    pub pio021: crate::Reg<pio021::PIO021_SPEC>,
    #[doc = "0x58 - Digital I/O control for port 0 pins PIO0_22"]
    pub pio022: crate::Reg<pio022::PIO022_SPEC>,
    #[doc = "0x5c - Digital I/O control for port 0 pins PIO0_23"]
    pub pio023: crate::Reg<pio023::PIO023_SPEC>,
    #[doc = "0x60 - Digital I/O control for port 0 pins PIO0_24"]
    pub pio024: crate::Reg<pio024::PIO024_SPEC>,
    #[doc = "0x64 - Digital I/O control for port 0 pins PIO0_25"]
    pub pio025: crate::Reg<pio025::PIO025_SPEC>,
    #[doc = "0x68 - Digital I/O control for port 0 pins PIO0_26"]
    pub pio026: crate::Reg<pio026::PIO026_SPEC>,
    #[doc = "0x6c - Digital I/O control for port 0 pins PIO0_27"]
    pub pio027: crate::Reg<pio027::PIO027_SPEC>,
    #[doc = "0x70 - Digital I/O control for port 0 pins PIO0_28"]
    pub pio028: crate::Reg<pio028::PIO028_SPEC>,
    #[doc = "0x74 - Digital I/O control for port 0 pins PIO0_29"]
    pub pio029: crate::Reg<pio029::PIO029_SPEC>,
    #[doc = "0x78 - Digital I/O control for port 0 pins PIO0_30"]
    pub pio030: crate::Reg<pio030::PIO030_SPEC>,
    #[doc = "0x7c - Digital I/O control for port 0 pins PIO0_31"]
    pub pio031: crate::Reg<pio031::PIO031_SPEC>,
    #[doc = "0x80 - Digital I/O control for port 1 pins PIO1_0"]
    pub pio10: crate::Reg<pio10::PIO10_SPEC>,
    #[doc = "0x84 - Digital I/O control for port 1 pins PIO1_1"]
    pub pio11: crate::Reg<pio11::PIO11_SPEC>,
    #[doc = "0x88 - Digital I/O control for port 1 pins PIO1_2"]
    pub pio12: crate::Reg<pio12::PIO12_SPEC>,
    #[doc = "0x8c - Digital I/O control for port 1 pins PIO1_3"]
    pub pio13: crate::Reg<pio13::PIO13_SPEC>,
    #[doc = "0x90 - Digital I/O control for port 1 pins PIO1_4"]
    pub pio14: crate::Reg<pio14::PIO14_SPEC>,
    #[doc = "0x94 - Digital I/O control for port 1 pins PIO1_5"]
    pub pio15: crate::Reg<pio15::PIO15_SPEC>,
    #[doc = "0x98 - Digital I/O control for port 1 pins PIO1_6"]
    pub pio16: crate::Reg<pio16::PIO16_SPEC>,
    #[doc = "0x9c - Digital I/O control for port 1 pins PIO1_7"]
    pub pio17: crate::Reg<pio17::PIO17_SPEC>,
    #[doc = "0xa0 - Digital I/O control for port 1 pins PIO1_8"]
    pub pio18: crate::Reg<pio18::PIO18_SPEC>,
    #[doc = "0xa4 - Digital I/O control for port 1 pins PIO1_9"]
    pub pio19: crate::Reg<pio19::PIO19_SPEC>,
    #[doc = "0xa8 - Digital I/O control for port 1 pins PIO1_10"]
    pub pio110: crate::Reg<pio110::PIO110_SPEC>,
    #[doc = "0xac - Digital I/O control for port 1 pins PIO1_11"]
    pub pio111: crate::Reg<pio111::PIO111_SPEC>,
    #[doc = "0xb0 - Digital I/O control for port 1 pins PIO1_12"]
    pub pio112: crate::Reg<pio112::PIO112_SPEC>,
    #[doc = "0xb4 - Digital I/O control for port 1 pins PIO1_13"]
    pub pio113: crate::Reg<pio113::PIO113_SPEC>,
    #[doc = "0xb8 - Digital I/O control for port 1 pins PIO1_14"]
    pub pio114: crate::Reg<pio114::PIO114_SPEC>,
    #[doc = "0xbc - Digital I/O control for port 1 pins PIO1_15"]
    pub pio115: crate::Reg<pio115::PIO115_SPEC>,
    #[doc = "0xc0 - Digital I/O control for port 1 pins PIO1_16"]
    pub pio116: crate::Reg<pio116::PIO116_SPEC>,
    #[doc = "0xc4 - Digital I/O control for port 1 pins PIO1_17"]
    pub pio117: crate::Reg<pio117::PIO117_SPEC>,
    #[doc = "0xc8 - Digital I/O control for port 1 pins PIO1_18"]
    pub pio118: crate::Reg<pio118::PIO118_SPEC>,
    #[doc = "0xcc - Digital I/O control for port 1 pins PIO1_19"]
    pub pio119: crate::Reg<pio119::PIO119_SPEC>,
    #[doc = "0xd0 - Digital I/O control for port 1 pins PIO1_20"]
    pub pio120: crate::Reg<pio120::PIO120_SPEC>,
    #[doc = "0xd4 - Digital I/O control for port 1 pins PIO1_21"]
    pub pio121: crate::Reg<pio121::PIO121_SPEC>,
    #[doc = "0xd8 - Digital I/O control for port 1 pins PIO1_22"]
    pub pio122: crate::Reg<pio122::PIO122_SPEC>,
    #[doc = "0xdc - Digital I/O control for port 1 pins PIO1_23"]
    pub pio123: crate::Reg<pio123::PIO123_SPEC>,
    #[doc = "0xe0 - Digital I/O control for port 1 pins PIO1_24"]
    pub pio124: crate::Reg<pio124::PIO124_SPEC>,
    #[doc = "0xe4 - Digital I/O control for port 1 pins PIO1_25"]
    pub pio125: crate::Reg<pio125::PIO125_SPEC>,
    #[doc = "0xe8 - Digital I/O control for port 1 pins PIO1_26"]
    pub pio126: crate::Reg<pio126::PIO126_SPEC>,
    #[doc = "0xec - Digital I/O control for port 1 pins PIO1_27"]
    pub pio127: crate::Reg<pio127::PIO127_SPEC>,
    #[doc = "0xf0 - Digital I/O control for port 1 pins PIO1_28"]
    pub pio128: crate::Reg<pio128::PIO128_SPEC>,
    #[doc = "0xf4 - Digital I/O control for port 1 pins PIO1_29"]
    pub pio129: crate::Reg<pio129::PIO129_SPEC>,
    #[doc = "0xf8 - Digital I/O control for port 1 pins PIO1_30"]
    pub pio130: crate::Reg<pio130::PIO130_SPEC>,
    #[doc = "0xfc - Digital I/O control for port 1 pins PIO1_31"]
    pub pio131: crate::Reg<pio131::PIO131_SPEC>,
    #[doc = "0x100 - Digital I/O control for port 2 pins PIO2_0"]
    pub pio20: crate::Reg<pio20::PIO20_SPEC>,
    #[doc = "0x104 - Digital I/O control for port 2 pins PIO2_1"]
    pub pio21: crate::Reg<pio21::PIO21_SPEC>,
    #[doc = "0x108 - Digital I/O control for port 2 pins PIO2_2"]
    pub pio22: crate::Reg<pio22::PIO22_SPEC>,
    #[doc = "0x10c - Digital I/O control for port 2 pins PIO2_3"]
    pub pio23: crate::Reg<pio23::PIO23_SPEC>,
    #[doc = "0x110 - Digital I/O control for port 2 pins PIO2_4"]
    pub pio24: crate::Reg<pio24::PIO24_SPEC>,
    #[doc = "0x114 - Digital I/O control for port 2 pins PIO2_5"]
    pub pio25: crate::Reg<pio25::PIO25_SPEC>,
    #[doc = "0x118 - Digital I/O control for port 2 pins PIO2_6"]
    pub pio26: crate::Reg<pio26::PIO26_SPEC>,
    #[doc = "0x11c - Digital I/O control for port 2 pins PIO2_7"]
    pub pio27: crate::Reg<pio27::PIO27_SPEC>,
    #[doc = "0x120 - Digital I/O control for port 2 pins PIO2_8"]
    pub pio28: crate::Reg<pio28::PIO28_SPEC>,
    #[doc = "0x124 - Digital I/O control for port 2 pins PIO2_9"]
    pub pio29: crate::Reg<pio29::PIO29_SPEC>,
    #[doc = "0x128 - Digital I/O control for port 2 pins PIO2_10"]
    pub pio210: crate::Reg<pio210::PIO210_SPEC>,
    #[doc = "0x12c - Digital I/O control for port 2 pins PIO2_11"]
    pub pio211: crate::Reg<pio211::PIO211_SPEC>,
    #[doc = "0x130 - Digital I/O control for port 2 pins PIO2_12"]
    pub pio212: crate::Reg<pio212::PIO212_SPEC>,
    #[doc = "0x134 - Digital I/O control for port 2 pins PIO2_13"]
    pub pio213: crate::Reg<pio213::PIO213_SPEC>,
    #[doc = "0x138 - Digital I/O control for port 2 pins PIO2_14"]
    pub pio214: crate::Reg<pio214::PIO214_SPEC>,
    #[doc = "0x13c - Digital I/O control for port 2 pins PIO2_15"]
    pub pio215: crate::Reg<pio215::PIO215_SPEC>,
    #[doc = "0x140 - Digital I/O control for port 2 pins PIO2_16"]
    pub pio216: crate::Reg<pio216::PIO216_SPEC>,
    #[doc = "0x144 - Digital I/O control for port 2 pins PIO2_17"]
    pub pio217: crate::Reg<pio217::PIO217_SPEC>,
    #[doc = "0x148 - Digital I/O control for port 2 pins PIO2_18"]
    pub pio218: crate::Reg<pio218::PIO218_SPEC>,
    #[doc = "0x14c - Digital I/O control for port 2 pins PIO2_19"]
    pub pio219: crate::Reg<pio219::PIO219_SPEC>,
    #[doc = "0x150 - Digital I/O control for port 2 pins PIO2_20"]
    pub pio220: crate::Reg<pio220::PIO220_SPEC>,
    #[doc = "0x154 - Digital I/O control for port 2 pins PIO2_21"]
    pub pio221: crate::Reg<pio221::PIO221_SPEC>,
    #[doc = "0x158 - Digital I/O control for port 2 pins PIO2_22"]
    pub pio222: crate::Reg<pio222::PIO222_SPEC>,
    #[doc = "0x15c - Digital I/O control for port 2 pins PIO2_23"]
    pub pio223: crate::Reg<pio223::PIO223_SPEC>,
    #[doc = "0x160 - Digital I/O control for port 2 pins PIO2_24"]
    pub pio224: crate::Reg<pio224::PIO224_SPEC>,
    #[doc = "0x164 - Digital I/O control for port 2 pins PIO2_25"]
    pub pio225: crate::Reg<pio225::PIO225_SPEC>,
    #[doc = "0x168 - Digital I/O control for port 2 pins PIO2_26"]
    pub pio226: crate::Reg<pio226::PIO226_SPEC>,
    #[doc = "0x16c - Digital I/O control for port 2 pins PIO2_27"]
    pub pio227: crate::Reg<pio227::PIO227_SPEC>,
    #[doc = "0x170 - Digital I/O control for port 2 pins PIO2_28"]
    pub pio228: crate::Reg<pio228::PIO228_SPEC>,
    #[doc = "0x174 - Digital I/O control for port 2 pins PIO2_29"]
    pub pio229: crate::Reg<pio229::PIO229_SPEC>,
    #[doc = "0x178 - Digital I/O control for port 2 pins PIO2_30"]
    pub pio230: crate::Reg<pio230::PIO230_SPEC>,
    #[doc = "0x17c - Digital I/O control for port 2 pins PIO2_31"]
    pub pio231: crate::Reg<pio231::PIO231_SPEC>,
    #[doc = "0x180 - Digital I/O control for port 3 pins PIO3_0"]
    pub pio30: crate::Reg<pio30::PIO30_SPEC>,
    #[doc = "0x184 - Digital I/O control for port 3 pins PIO3_1"]
    pub pio31: crate::Reg<pio31::PIO31_SPEC>,
    #[doc = "0x188 - Digital I/O control for port 3 pins PIO3_2"]
    pub pio32: crate::Reg<pio32::PIO32_SPEC>,
    #[doc = "0x18c - Digital I/O control for port 3 pins PIO3_3"]
    pub pio33: crate::Reg<pio33::PIO33_SPEC>,
    #[doc = "0x190 - Digital I/O control for port 3 pins PIO3_4"]
    pub pio34: crate::Reg<pio34::PIO34_SPEC>,
    #[doc = "0x194 - Digital I/O control for port 3 pins PIO3_5"]
    pub pio35: crate::Reg<pio35::PIO35_SPEC>,
    #[doc = "0x198 - Digital I/O control for port 3 pins PIO3_6"]
    pub pio36: crate::Reg<pio36::PIO36_SPEC>,
    #[doc = "0x19c - Digital I/O control for port 3 pins PIO3_7"]
    pub pio37: crate::Reg<pio37::PIO37_SPEC>,
    #[doc = "0x1a0 - Digital I/O control for port 3 pins PIO3_8"]
    pub pio38: crate::Reg<pio38::PIO38_SPEC>,
    #[doc = "0x1a4 - Digital I/O control for port 3 pins PIO3_9"]
    pub pio39: crate::Reg<pio39::PIO39_SPEC>,
    #[doc = "0x1a8 - Digital I/O control for port 3 pins PIO3_10"]
    pub pio310: crate::Reg<pio310::PIO310_SPEC>,
    #[doc = "0x1ac - Digital I/O control for port 3 pins PIO3_11"]
    pub pio311: crate::Reg<pio311::PIO311_SPEC>,
    #[doc = "0x1b0 - Digital I/O control for port 3 pins PIO3_12"]
    pub pio312: crate::Reg<pio312::PIO312_SPEC>,
    #[doc = "0x1b4 - Digital I/O control for port 3 pins PIO3_13"]
    pub pio313: crate::Reg<pio313::PIO313_SPEC>,
    #[doc = "0x1b8 - Digital I/O control for port 3 pins PIO3_14"]
    pub pio314: crate::Reg<pio314::PIO314_SPEC>,
    #[doc = "0x1bc - Digital I/O control for port 3 pins PIO3_15"]
    pub pio315: crate::Reg<pio315::PIO315_SPEC>,
    #[doc = "0x1c0 - Digital I/O control for port 3 pins PIO3_16"]
    pub pio316: crate::Reg<pio316::PIO316_SPEC>,
    #[doc = "0x1c4 - Digital I/O control for port 3 pins PIO3_17"]
    pub pio317: crate::Reg<pio317::PIO317_SPEC>,
    #[doc = "0x1c8 - Digital I/O control for port 3 pins PIO3_18"]
    pub pio318: crate::Reg<pio318::PIO318_SPEC>,
    #[doc = "0x1cc - Digital I/O control for port 3 pins PIO3_19"]
    pub pio319: crate::Reg<pio319::PIO319_SPEC>,
    #[doc = "0x1d0 - Digital I/O control for port 3 pins PIO3_20"]
    pub pio320: crate::Reg<pio320::PIO320_SPEC>,
    #[doc = "0x1d4 - Digital I/O control for port 3 pins PIO3_21"]
    pub pio321: crate::Reg<pio321::PIO321_SPEC>,
    #[doc = "0x1d8 - Digital I/O control for port 3 pins PIO3_22"]
    pub pio322: crate::Reg<pio322::PIO322_SPEC>,
    #[doc = "0x1dc - Digital I/O control for port 3 pins PIO3_23"]
    pub pio323: crate::Reg<pio323::PIO323_SPEC>,
    #[doc = "0x1e0 - Digital I/O control for port 3 pins PIO3_24"]
    pub pio324: crate::Reg<pio324::PIO324_SPEC>,
    #[doc = "0x1e4 - Digital I/O control for port 3 pins PIO3_25"]
    pub pio325: crate::Reg<pio325::PIO325_SPEC>,
    #[doc = "0x1e8 - Digital I/O control for port 3 pins PIO3_26"]
    pub pio326: crate::Reg<pio326::PIO326_SPEC>,
    #[doc = "0x1ec - Digital I/O control for port 3 pins PIO3_27"]
    pub pio327: crate::Reg<pio327::PIO327_SPEC>,
    #[doc = "0x1f0 - Digital I/O control for port 3 pins PIO3_28"]
    pub pio328: crate::Reg<pio328::PIO328_SPEC>,
    #[doc = "0x1f4 - Digital I/O control for port 3 pins PIO3_29"]
    pub pio329: crate::Reg<pio329::PIO329_SPEC>,
    #[doc = "0x1f8 - Digital I/O control for port 3 pins PIO3_30"]
    pub pio330: crate::Reg<pio330::PIO330_SPEC>,
    #[doc = "0x1fc - Digital I/O control for port 3 pins PIO3_31"]
    pub pio331: crate::Reg<pio331::PIO331_SPEC>,
    #[doc = "0x200 - Digital I/O control for port 4 pins PIO4_0"]
    pub pio40: crate::Reg<pio40::PIO40_SPEC>,
    #[doc = "0x204 - Digital I/O control for port 4 pins PIO4_1"]
    pub pio41: crate::Reg<pio41::PIO41_SPEC>,
    #[doc = "0x208 - Digital I/O control for port 4 pins PIO4_2"]
    pub pio42: crate::Reg<pio42::PIO42_SPEC>,
    #[doc = "0x20c - Digital I/O control for port 4 pins PIO4_3"]
    pub pio43: crate::Reg<pio43::PIO43_SPEC>,
    #[doc = "0x210 - Digital I/O control for port 4 pins PIO4_4"]
    pub pio44: crate::Reg<pio44::PIO44_SPEC>,
    #[doc = "0x214 - Digital I/O control for port 4 pins PIO4_5"]
    pub pio45: crate::Reg<pio45::PIO45_SPEC>,
    #[doc = "0x218 - Digital I/O control for port 4 pins PIO4_6"]
    pub pio46: crate::Reg<pio46::PIO46_SPEC>,
    #[doc = "0x21c - Digital I/O control for port 4 pins PIO4_7"]
    pub pio47: crate::Reg<pio47::PIO47_SPEC>,
    #[doc = "0x220 - Digital I/O control for port 4 pins PIO4_8"]
    pub pio48: crate::Reg<pio48::PIO48_SPEC>,
    #[doc = "0x224 - Digital I/O control for port 4 pins PIO4_9"]
    pub pio49: crate::Reg<pio49::PIO49_SPEC>,
    #[doc = "0x228 - Digital I/O control for port 4 pins PIO4_10"]
    pub pio410: crate::Reg<pio410::PIO410_SPEC>,
    #[doc = "0x22c - Digital I/O control for port 4 pins PIO4_11"]
    pub pio411: crate::Reg<pio411::PIO411_SPEC>,
    #[doc = "0x230 - Digital I/O control for port 4 pins PIO4_12"]
    pub pio412: crate::Reg<pio412::PIO412_SPEC>,
    #[doc = "0x234 - Digital I/O control for port 4 pins PIO4_13"]
    pub pio413: crate::Reg<pio413::PIO413_SPEC>,
    #[doc = "0x238 - Digital I/O control for port 4 pins PIO4_14"]
    pub pio414: crate::Reg<pio414::PIO414_SPEC>,
    #[doc = "0x23c - Digital I/O control for port 4 pins PIO4_15"]
    pub pio415: crate::Reg<pio415::PIO415_SPEC>,
    #[doc = "0x240 - Digital I/O control for port 4 pins PIO4_16"]
    pub pio416: crate::Reg<pio416::PIO416_SPEC>,
    #[doc = "0x244 - Digital I/O control for port 4 pins PIO4_17"]
    pub pio417: crate::Reg<pio417::PIO417_SPEC>,
    #[doc = "0x248 - Digital I/O control for port 4 pins PIO4_18"]
    pub pio418: crate::Reg<pio418::PIO418_SPEC>,
    #[doc = "0x24c - Digital I/O control for port 4 pins PIO4_19"]
    pub pio419: crate::Reg<pio419::PIO419_SPEC>,
    #[doc = "0x250 - Digital I/O control for port 4 pins PIO4_20"]
    pub pio420: crate::Reg<pio420::PIO420_SPEC>,
    #[doc = "0x254 - Digital I/O control for port 4 pins PIO4_21"]
    pub pio421: crate::Reg<pio421::PIO421_SPEC>,
    #[doc = "0x258 - Digital I/O control for port 4 pins PIO4_22"]
    pub pio422: crate::Reg<pio422::PIO422_SPEC>,
    #[doc = "0x25c - Digital I/O control for port 4 pins PIO4_23"]
    pub pio423: crate::Reg<pio423::PIO423_SPEC>,
    #[doc = "0x260 - Digital I/O control for port 4 pins PIO4_24"]
    pub pio424: crate::Reg<pio424::PIO424_SPEC>,
    #[doc = "0x264 - Digital I/O control for port 4 pins PIO4_25"]
    pub pio425: crate::Reg<pio425::PIO425_SPEC>,
    #[doc = "0x268 - Digital I/O control for port 4 pins PIO4_26"]
    pub pio426: crate::Reg<pio426::PIO426_SPEC>,
    #[doc = "0x26c - Digital I/O control for port 4 pins PIO4_27"]
    pub pio427: crate::Reg<pio427::PIO427_SPEC>,
    #[doc = "0x270 - Digital I/O control for port 4 pins PIO4_28"]
    pub pio428: crate::Reg<pio428::PIO428_SPEC>,
    #[doc = "0x274 - Digital I/O control for port 4 pins PIO4_29"]
    pub pio429: crate::Reg<pio429::PIO429_SPEC>,
    #[doc = "0x278 - Digital I/O control for port 4 pins PIO4_30"]
    pub pio430: crate::Reg<pio430::PIO430_SPEC>,
    #[doc = "0x27c - Digital I/O control for port 4 pins PIO4_31"]
    pub pio431: crate::Reg<pio431::PIO431_SPEC>,
    #[doc = "0x280 - Digital I/O control for port 5 pins PIO5_0"]
    pub pio50: crate::Reg<pio50::PIO50_SPEC>,
    #[doc = "0x284 - Digital I/O control for port 5 pins PIO5_1"]
    pub pio51: crate::Reg<pio51::PIO51_SPEC>,
    #[doc = "0x288 - Digital I/O control for port 5 pins PIO5_2"]
    pub pio52: crate::Reg<pio52::PIO52_SPEC>,
    #[doc = "0x28c - Digital I/O control for port 5 pins PIO5_3"]
    pub pio53: crate::Reg<pio53::PIO53_SPEC>,
    #[doc = "0x290 - Digital I/O control for port 5 pins PIO5_4"]
    pub pio54: crate::Reg<pio54::PIO54_SPEC>,
    #[doc = "0x294 - Digital I/O control for port 5 pins PIO5_5"]
    pub pio55: crate::Reg<pio55::PIO55_SPEC>,
    #[doc = "0x298 - Digital I/O control for port 5 pins PIO5_6"]
    pub pio56: crate::Reg<pio56::PIO56_SPEC>,
    #[doc = "0x29c - Digital I/O control for port 5 pins PIO5_7"]
    pub pio57: crate::Reg<pio57::PIO57_SPEC>,
    #[doc = "0x2a0 - Digital I/O control for port 5 pins PIO5_8"]
    pub pio58: crate::Reg<pio58::PIO58_SPEC>,
    #[doc = "0x2a4 - Digital I/O control for port 5 pins PIO5_9"]
    pub pio59: crate::Reg<pio59::PIO59_SPEC>,
    #[doc = "0x2a8 - Digital I/O control for port 5 pins PIO5_10"]
    pub pio510: crate::Reg<pio510::PIO510_SPEC>,
    #[doc = "0x2ac - Digital I/O control for port 5 pins PIO5_11"]
    pub pio511: crate::Reg<pio511::PIO511_SPEC>,
    #[doc = "0x2b0 - Digital I/O control for port 5 pins PIO5_12"]
    pub pio512: crate::Reg<pio512::PIO512_SPEC>,
    #[doc = "0x2b4 - Digital I/O control for port 5 pins PIO5_13"]
    pub pio513: crate::Reg<pio513::PIO513_SPEC>,
    #[doc = "0x2b8 - Digital I/O control for port 5 pins PIO5_14"]
    pub pio514: crate::Reg<pio514::PIO514_SPEC>,
    #[doc = "0x2bc - Digital I/O control for port 5 pins PIO5_15"]
    pub pio515: crate::Reg<pio515::PIO515_SPEC>,
    #[doc = "0x2c0 - Digital I/O control for port 5 pins PIO5_16"]
    pub pio516: crate::Reg<pio516::PIO516_SPEC>,
    #[doc = "0x2c4 - Digital I/O control for port 5 pins PIO5_17"]
    pub pio517: crate::Reg<pio517::PIO517_SPEC>,
    #[doc = "0x2c8 - Digital I/O control for port 5 pins PIO5_18"]
    pub pio518: crate::Reg<pio518::PIO518_SPEC>,
    #[doc = "0x2cc - Digital I/O control for port 5 pins PIO5_19"]
    pub pio519: crate::Reg<pio519::PIO519_SPEC>,
    #[doc = "0x2d0 - Digital I/O control for port 5 pins PIO5_20"]
    pub pio520: crate::Reg<pio520::PIO520_SPEC>,
    #[doc = "0x2d4 - Digital I/O control for port 5 pins PIO5_21"]
    pub pio521: crate::Reg<pio521::PIO521_SPEC>,
    #[doc = "0x2d8 - Digital I/O control for port 5 pins PIO5_22"]
    pub pio522: crate::Reg<pio522::PIO522_SPEC>,
    #[doc = "0x2dc - Digital I/O control for port 5 pins PIO5_23"]
    pub pio523: crate::Reg<pio523::PIO523_SPEC>,
    #[doc = "0x2e0 - Digital I/O control for port 5 pins PIO5_24"]
    pub pio524: crate::Reg<pio524::PIO524_SPEC>,
    #[doc = "0x2e4 - Digital I/O control for port 5 pins PIO5_25"]
    pub pio525: crate::Reg<pio525::PIO525_SPEC>,
    #[doc = "0x2e8 - Digital I/O control for port 5 pins PIO5_26"]
    pub pio526: crate::Reg<pio526::PIO526_SPEC>,
    #[doc = "0x2ec - Digital I/O control for port 5 pins PIO5_27"]
    pub pio527: crate::Reg<pio527::PIO527_SPEC>,
    #[doc = "0x2f0 - Digital I/O control for port 5 pins PIO5_28"]
    pub pio528: crate::Reg<pio528::PIO528_SPEC>,
    #[doc = "0x2f4 - Digital I/O control for port 5 pins PIO5_29"]
    pub pio529: crate::Reg<pio529::PIO529_SPEC>,
    #[doc = "0x2f8 - Digital I/O control for port 5 pins PIO5_30"]
    pub pio530: crate::Reg<pio530::PIO530_SPEC>,
    #[doc = "0x2fc - Digital I/O control for port 5 pins PIO5_31"]
    pub pio531: crate::Reg<pio531::PIO531_SPEC>,
}
#[doc = "PIO00 register accessor: an alias for `Reg<PIO00_SPEC>`"]
pub type PIO00 = crate::Reg<pio00::PIO00_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_0"]
pub mod pio00;
#[doc = "PIO01 register accessor: an alias for `Reg<PIO01_SPEC>`"]
pub type PIO01 = crate::Reg<pio01::PIO01_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_1"]
pub mod pio01;
#[doc = "PIO02 register accessor: an alias for `Reg<PIO02_SPEC>`"]
pub type PIO02 = crate::Reg<pio02::PIO02_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_2"]
pub mod pio02;
#[doc = "PIO03 register accessor: an alias for `Reg<PIO03_SPEC>`"]
pub type PIO03 = crate::Reg<pio03::PIO03_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_3"]
pub mod pio03;
#[doc = "PIO04 register accessor: an alias for `Reg<PIO04_SPEC>`"]
pub type PIO04 = crate::Reg<pio04::PIO04_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_4"]
pub mod pio04;
#[doc = "PIO05 register accessor: an alias for `Reg<PIO05_SPEC>`"]
pub type PIO05 = crate::Reg<pio05::PIO05_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_5"]
pub mod pio05;
#[doc = "PIO06 register accessor: an alias for `Reg<PIO06_SPEC>`"]
pub type PIO06 = crate::Reg<pio06::PIO06_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_6"]
pub mod pio06;
#[doc = "PIO07 register accessor: an alias for `Reg<PIO07_SPEC>`"]
pub type PIO07 = crate::Reg<pio07::PIO07_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_7"]
pub mod pio07;
#[doc = "PIO08 register accessor: an alias for `Reg<PIO08_SPEC>`"]
pub type PIO08 = crate::Reg<pio08::PIO08_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_8"]
pub mod pio08;
#[doc = "PIO09 register accessor: an alias for `Reg<PIO09_SPEC>`"]
pub type PIO09 = crate::Reg<pio09::PIO09_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_9"]
pub mod pio09;
#[doc = "PIO010 register accessor: an alias for `Reg<PIO010_SPEC>`"]
pub type PIO010 = crate::Reg<pio010::PIO010_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_10"]
pub mod pio010;
#[doc = "PIO011 register accessor: an alias for `Reg<PIO011_SPEC>`"]
pub type PIO011 = crate::Reg<pio011::PIO011_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_11"]
pub mod pio011;
#[doc = "PIO012 register accessor: an alias for `Reg<PIO012_SPEC>`"]
pub type PIO012 = crate::Reg<pio012::PIO012_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_12"]
pub mod pio012;
#[doc = "PIO013 register accessor: an alias for `Reg<PIO013_SPEC>`"]
pub type PIO013 = crate::Reg<pio013::PIO013_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_13"]
pub mod pio013;
#[doc = "PIO014 register accessor: an alias for `Reg<PIO014_SPEC>`"]
pub type PIO014 = crate::Reg<pio014::PIO014_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_14"]
pub mod pio014;
#[doc = "PIO015 register accessor: an alias for `Reg<PIO015_SPEC>`"]
pub type PIO015 = crate::Reg<pio015::PIO015_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_15"]
pub mod pio015;
#[doc = "PIO016 register accessor: an alias for `Reg<PIO016_SPEC>`"]
pub type PIO016 = crate::Reg<pio016::PIO016_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_16"]
pub mod pio016;
#[doc = "PIO017 register accessor: an alias for `Reg<PIO017_SPEC>`"]
pub type PIO017 = crate::Reg<pio017::PIO017_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_17"]
pub mod pio017;
#[doc = "PIO018 register accessor: an alias for `Reg<PIO018_SPEC>`"]
pub type PIO018 = crate::Reg<pio018::PIO018_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_18"]
pub mod pio018;
#[doc = "PIO019 register accessor: an alias for `Reg<PIO019_SPEC>`"]
pub type PIO019 = crate::Reg<pio019::PIO019_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_19"]
pub mod pio019;
#[doc = "PIO020 register accessor: an alias for `Reg<PIO020_SPEC>`"]
pub type PIO020 = crate::Reg<pio020::PIO020_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_20"]
pub mod pio020;
#[doc = "PIO021 register accessor: an alias for `Reg<PIO021_SPEC>`"]
pub type PIO021 = crate::Reg<pio021::PIO021_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_21"]
pub mod pio021;
#[doc = "PIO022 register accessor: an alias for `Reg<PIO022_SPEC>`"]
pub type PIO022 = crate::Reg<pio022::PIO022_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_22"]
pub mod pio022;
#[doc = "PIO023 register accessor: an alias for `Reg<PIO023_SPEC>`"]
pub type PIO023 = crate::Reg<pio023::PIO023_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_23"]
pub mod pio023;
#[doc = "PIO024 register accessor: an alias for `Reg<PIO024_SPEC>`"]
pub type PIO024 = crate::Reg<pio024::PIO024_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_24"]
pub mod pio024;
#[doc = "PIO025 register accessor: an alias for `Reg<PIO025_SPEC>`"]
pub type PIO025 = crate::Reg<pio025::PIO025_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_25"]
pub mod pio025;
#[doc = "PIO026 register accessor: an alias for `Reg<PIO026_SPEC>`"]
pub type PIO026 = crate::Reg<pio026::PIO026_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_26"]
pub mod pio026;
#[doc = "PIO027 register accessor: an alias for `Reg<PIO027_SPEC>`"]
pub type PIO027 = crate::Reg<pio027::PIO027_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_27"]
pub mod pio027;
#[doc = "PIO028 register accessor: an alias for `Reg<PIO028_SPEC>`"]
pub type PIO028 = crate::Reg<pio028::PIO028_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_28"]
pub mod pio028;
#[doc = "PIO029 register accessor: an alias for `Reg<PIO029_SPEC>`"]
pub type PIO029 = crate::Reg<pio029::PIO029_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_29"]
pub mod pio029;
#[doc = "PIO030 register accessor: an alias for `Reg<PIO030_SPEC>`"]
pub type PIO030 = crate::Reg<pio030::PIO030_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_30"]
pub mod pio030;
#[doc = "PIO031 register accessor: an alias for `Reg<PIO031_SPEC>`"]
pub type PIO031 = crate::Reg<pio031::PIO031_SPEC>;
#[doc = "Digital I/O control for port 0 pins PIO0_31"]
pub mod pio031;
#[doc = "PIO10 register accessor: an alias for `Reg<PIO10_SPEC>`"]
pub type PIO10 = crate::Reg<pio10::PIO10_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_0"]
pub mod pio10;
#[doc = "PIO11 register accessor: an alias for `Reg<PIO11_SPEC>`"]
pub type PIO11 = crate::Reg<pio11::PIO11_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_1"]
pub mod pio11;
#[doc = "PIO12 register accessor: an alias for `Reg<PIO12_SPEC>`"]
pub type PIO12 = crate::Reg<pio12::PIO12_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_2"]
pub mod pio12;
#[doc = "PIO13 register accessor: an alias for `Reg<PIO13_SPEC>`"]
pub type PIO13 = crate::Reg<pio13::PIO13_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_3"]
pub mod pio13;
#[doc = "PIO14 register accessor: an alias for `Reg<PIO14_SPEC>`"]
pub type PIO14 = crate::Reg<pio14::PIO14_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_4"]
pub mod pio14;
#[doc = "PIO15 register accessor: an alias for `Reg<PIO15_SPEC>`"]
pub type PIO15 = crate::Reg<pio15::PIO15_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_5"]
pub mod pio15;
#[doc = "PIO16 register accessor: an alias for `Reg<PIO16_SPEC>`"]
pub type PIO16 = crate::Reg<pio16::PIO16_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_6"]
pub mod pio16;
#[doc = "PIO17 register accessor: an alias for `Reg<PIO17_SPEC>`"]
pub type PIO17 = crate::Reg<pio17::PIO17_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_7"]
pub mod pio17;
#[doc = "PIO18 register accessor: an alias for `Reg<PIO18_SPEC>`"]
pub type PIO18 = crate::Reg<pio18::PIO18_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_8"]
pub mod pio18;
#[doc = "PIO19 register accessor: an alias for `Reg<PIO19_SPEC>`"]
pub type PIO19 = crate::Reg<pio19::PIO19_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_9"]
pub mod pio19;
#[doc = "PIO110 register accessor: an alias for `Reg<PIO110_SPEC>`"]
pub type PIO110 = crate::Reg<pio110::PIO110_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_10"]
pub mod pio110;
#[doc = "PIO111 register accessor: an alias for `Reg<PIO111_SPEC>`"]
pub type PIO111 = crate::Reg<pio111::PIO111_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_11"]
pub mod pio111;
#[doc = "PIO112 register accessor: an alias for `Reg<PIO112_SPEC>`"]
pub type PIO112 = crate::Reg<pio112::PIO112_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_12"]
pub mod pio112;
#[doc = "PIO113 register accessor: an alias for `Reg<PIO113_SPEC>`"]
pub type PIO113 = crate::Reg<pio113::PIO113_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_13"]
pub mod pio113;
#[doc = "PIO114 register accessor: an alias for `Reg<PIO114_SPEC>`"]
pub type PIO114 = crate::Reg<pio114::PIO114_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_14"]
pub mod pio114;
#[doc = "PIO115 register accessor: an alias for `Reg<PIO115_SPEC>`"]
pub type PIO115 = crate::Reg<pio115::PIO115_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_15"]
pub mod pio115;
#[doc = "PIO116 register accessor: an alias for `Reg<PIO116_SPEC>`"]
pub type PIO116 = crate::Reg<pio116::PIO116_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_16"]
pub mod pio116;
#[doc = "PIO117 register accessor: an alias for `Reg<PIO117_SPEC>`"]
pub type PIO117 = crate::Reg<pio117::PIO117_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_17"]
pub mod pio117;
#[doc = "PIO118 register accessor: an alias for `Reg<PIO118_SPEC>`"]
pub type PIO118 = crate::Reg<pio118::PIO118_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_18"]
pub mod pio118;
#[doc = "PIO119 register accessor: an alias for `Reg<PIO119_SPEC>`"]
pub type PIO119 = crate::Reg<pio119::PIO119_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_19"]
pub mod pio119;
#[doc = "PIO120 register accessor: an alias for `Reg<PIO120_SPEC>`"]
pub type PIO120 = crate::Reg<pio120::PIO120_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_20"]
pub mod pio120;
#[doc = "PIO121 register accessor: an alias for `Reg<PIO121_SPEC>`"]
pub type PIO121 = crate::Reg<pio121::PIO121_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_21"]
pub mod pio121;
#[doc = "PIO122 register accessor: an alias for `Reg<PIO122_SPEC>`"]
pub type PIO122 = crate::Reg<pio122::PIO122_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_22"]
pub mod pio122;
#[doc = "PIO123 register accessor: an alias for `Reg<PIO123_SPEC>`"]
pub type PIO123 = crate::Reg<pio123::PIO123_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_23"]
pub mod pio123;
#[doc = "PIO124 register accessor: an alias for `Reg<PIO124_SPEC>`"]
pub type PIO124 = crate::Reg<pio124::PIO124_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_24"]
pub mod pio124;
#[doc = "PIO125 register accessor: an alias for `Reg<PIO125_SPEC>`"]
pub type PIO125 = crate::Reg<pio125::PIO125_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_25"]
pub mod pio125;
#[doc = "PIO126 register accessor: an alias for `Reg<PIO126_SPEC>`"]
pub type PIO126 = crate::Reg<pio126::PIO126_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_26"]
pub mod pio126;
#[doc = "PIO127 register accessor: an alias for `Reg<PIO127_SPEC>`"]
pub type PIO127 = crate::Reg<pio127::PIO127_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_27"]
pub mod pio127;
#[doc = "PIO128 register accessor: an alias for `Reg<PIO128_SPEC>`"]
pub type PIO128 = crate::Reg<pio128::PIO128_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_28"]
pub mod pio128;
#[doc = "PIO129 register accessor: an alias for `Reg<PIO129_SPEC>`"]
pub type PIO129 = crate::Reg<pio129::PIO129_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_29"]
pub mod pio129;
#[doc = "PIO130 register accessor: an alias for `Reg<PIO130_SPEC>`"]
pub type PIO130 = crate::Reg<pio130::PIO130_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_30"]
pub mod pio130;
#[doc = "PIO131 register accessor: an alias for `Reg<PIO131_SPEC>`"]
pub type PIO131 = crate::Reg<pio131::PIO131_SPEC>;
#[doc = "Digital I/O control for port 1 pins PIO1_31"]
pub mod pio131;
#[doc = "PIO20 register accessor: an alias for `Reg<PIO20_SPEC>`"]
pub type PIO20 = crate::Reg<pio20::PIO20_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_0"]
pub mod pio20;
#[doc = "PIO21 register accessor: an alias for `Reg<PIO21_SPEC>`"]
pub type PIO21 = crate::Reg<pio21::PIO21_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_1"]
pub mod pio21;
#[doc = "PIO22 register accessor: an alias for `Reg<PIO22_SPEC>`"]
pub type PIO22 = crate::Reg<pio22::PIO22_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_2"]
pub mod pio22;
#[doc = "PIO23 register accessor: an alias for `Reg<PIO23_SPEC>`"]
pub type PIO23 = crate::Reg<pio23::PIO23_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_3"]
pub mod pio23;
#[doc = "PIO24 register accessor: an alias for `Reg<PIO24_SPEC>`"]
pub type PIO24 = crate::Reg<pio24::PIO24_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_4"]
pub mod pio24;
#[doc = "PIO25 register accessor: an alias for `Reg<PIO25_SPEC>`"]
pub type PIO25 = crate::Reg<pio25::PIO25_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_5"]
pub mod pio25;
#[doc = "PIO26 register accessor: an alias for `Reg<PIO26_SPEC>`"]
pub type PIO26 = crate::Reg<pio26::PIO26_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_6"]
pub mod pio26;
#[doc = "PIO27 register accessor: an alias for `Reg<PIO27_SPEC>`"]
pub type PIO27 = crate::Reg<pio27::PIO27_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_7"]
pub mod pio27;
#[doc = "PIO28 register accessor: an alias for `Reg<PIO28_SPEC>`"]
pub type PIO28 = crate::Reg<pio28::PIO28_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_8"]
pub mod pio28;
#[doc = "PIO29 register accessor: an alias for `Reg<PIO29_SPEC>`"]
pub type PIO29 = crate::Reg<pio29::PIO29_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_9"]
pub mod pio29;
#[doc = "PIO210 register accessor: an alias for `Reg<PIO210_SPEC>`"]
pub type PIO210 = crate::Reg<pio210::PIO210_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_10"]
pub mod pio210;
#[doc = "PIO211 register accessor: an alias for `Reg<PIO211_SPEC>`"]
pub type PIO211 = crate::Reg<pio211::PIO211_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_11"]
pub mod pio211;
#[doc = "PIO212 register accessor: an alias for `Reg<PIO212_SPEC>`"]
pub type PIO212 = crate::Reg<pio212::PIO212_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_12"]
pub mod pio212;
#[doc = "PIO213 register accessor: an alias for `Reg<PIO213_SPEC>`"]
pub type PIO213 = crate::Reg<pio213::PIO213_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_13"]
pub mod pio213;
#[doc = "PIO214 register accessor: an alias for `Reg<PIO214_SPEC>`"]
pub type PIO214 = crate::Reg<pio214::PIO214_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_14"]
pub mod pio214;
#[doc = "PIO215 register accessor: an alias for `Reg<PIO215_SPEC>`"]
pub type PIO215 = crate::Reg<pio215::PIO215_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_15"]
pub mod pio215;
#[doc = "PIO216 register accessor: an alias for `Reg<PIO216_SPEC>`"]
pub type PIO216 = crate::Reg<pio216::PIO216_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_16"]
pub mod pio216;
#[doc = "PIO217 register accessor: an alias for `Reg<PIO217_SPEC>`"]
pub type PIO217 = crate::Reg<pio217::PIO217_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_17"]
pub mod pio217;
#[doc = "PIO218 register accessor: an alias for `Reg<PIO218_SPEC>`"]
pub type PIO218 = crate::Reg<pio218::PIO218_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_18"]
pub mod pio218;
#[doc = "PIO219 register accessor: an alias for `Reg<PIO219_SPEC>`"]
pub type PIO219 = crate::Reg<pio219::PIO219_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_19"]
pub mod pio219;
#[doc = "PIO220 register accessor: an alias for `Reg<PIO220_SPEC>`"]
pub type PIO220 = crate::Reg<pio220::PIO220_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_20"]
pub mod pio220;
#[doc = "PIO221 register accessor: an alias for `Reg<PIO221_SPEC>`"]
pub type PIO221 = crate::Reg<pio221::PIO221_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_21"]
pub mod pio221;
#[doc = "PIO222 register accessor: an alias for `Reg<PIO222_SPEC>`"]
pub type PIO222 = crate::Reg<pio222::PIO222_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_22"]
pub mod pio222;
#[doc = "PIO223 register accessor: an alias for `Reg<PIO223_SPEC>`"]
pub type PIO223 = crate::Reg<pio223::PIO223_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_23"]
pub mod pio223;
#[doc = "PIO224 register accessor: an alias for `Reg<PIO224_SPEC>`"]
pub type PIO224 = crate::Reg<pio224::PIO224_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_24"]
pub mod pio224;
#[doc = "PIO225 register accessor: an alias for `Reg<PIO225_SPEC>`"]
pub type PIO225 = crate::Reg<pio225::PIO225_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_25"]
pub mod pio225;
#[doc = "PIO226 register accessor: an alias for `Reg<PIO226_SPEC>`"]
pub type PIO226 = crate::Reg<pio226::PIO226_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_26"]
pub mod pio226;
#[doc = "PIO227 register accessor: an alias for `Reg<PIO227_SPEC>`"]
pub type PIO227 = crate::Reg<pio227::PIO227_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_27"]
pub mod pio227;
#[doc = "PIO228 register accessor: an alias for `Reg<PIO228_SPEC>`"]
pub type PIO228 = crate::Reg<pio228::PIO228_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_28"]
pub mod pio228;
#[doc = "PIO229 register accessor: an alias for `Reg<PIO229_SPEC>`"]
pub type PIO229 = crate::Reg<pio229::PIO229_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_29"]
pub mod pio229;
#[doc = "PIO230 register accessor: an alias for `Reg<PIO230_SPEC>`"]
pub type PIO230 = crate::Reg<pio230::PIO230_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_30"]
pub mod pio230;
#[doc = "PIO231 register accessor: an alias for `Reg<PIO231_SPEC>`"]
pub type PIO231 = crate::Reg<pio231::PIO231_SPEC>;
#[doc = "Digital I/O control for port 2 pins PIO2_31"]
pub mod pio231;
#[doc = "PIO30 register accessor: an alias for `Reg<PIO30_SPEC>`"]
pub type PIO30 = crate::Reg<pio30::PIO30_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_0"]
pub mod pio30;
#[doc = "PIO31 register accessor: an alias for `Reg<PIO31_SPEC>`"]
pub type PIO31 = crate::Reg<pio31::PIO31_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_1"]
pub mod pio31;
#[doc = "PIO32 register accessor: an alias for `Reg<PIO32_SPEC>`"]
pub type PIO32 = crate::Reg<pio32::PIO32_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_2"]
pub mod pio32;
#[doc = "PIO33 register accessor: an alias for `Reg<PIO33_SPEC>`"]
pub type PIO33 = crate::Reg<pio33::PIO33_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_3"]
pub mod pio33;
#[doc = "PIO34 register accessor: an alias for `Reg<PIO34_SPEC>`"]
pub type PIO34 = crate::Reg<pio34::PIO34_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_4"]
pub mod pio34;
#[doc = "PIO35 register accessor: an alias for `Reg<PIO35_SPEC>`"]
pub type PIO35 = crate::Reg<pio35::PIO35_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_5"]
pub mod pio35;
#[doc = "PIO36 register accessor: an alias for `Reg<PIO36_SPEC>`"]
pub type PIO36 = crate::Reg<pio36::PIO36_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_6"]
pub mod pio36;
#[doc = "PIO37 register accessor: an alias for `Reg<PIO37_SPEC>`"]
pub type PIO37 = crate::Reg<pio37::PIO37_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_7"]
pub mod pio37;
#[doc = "PIO38 register accessor: an alias for `Reg<PIO38_SPEC>`"]
pub type PIO38 = crate::Reg<pio38::PIO38_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_8"]
pub mod pio38;
#[doc = "PIO39 register accessor: an alias for `Reg<PIO39_SPEC>`"]
pub type PIO39 = crate::Reg<pio39::PIO39_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_9"]
pub mod pio39;
#[doc = "PIO310 register accessor: an alias for `Reg<PIO310_SPEC>`"]
pub type PIO310 = crate::Reg<pio310::PIO310_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_10"]
pub mod pio310;
#[doc = "PIO311 register accessor: an alias for `Reg<PIO311_SPEC>`"]
pub type PIO311 = crate::Reg<pio311::PIO311_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_11"]
pub mod pio311;
#[doc = "PIO312 register accessor: an alias for `Reg<PIO312_SPEC>`"]
pub type PIO312 = crate::Reg<pio312::PIO312_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_12"]
pub mod pio312;
#[doc = "PIO313 register accessor: an alias for `Reg<PIO313_SPEC>`"]
pub type PIO313 = crate::Reg<pio313::PIO313_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_13"]
pub mod pio313;
#[doc = "PIO314 register accessor: an alias for `Reg<PIO314_SPEC>`"]
pub type PIO314 = crate::Reg<pio314::PIO314_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_14"]
pub mod pio314;
#[doc = "PIO315 register accessor: an alias for `Reg<PIO315_SPEC>`"]
pub type PIO315 = crate::Reg<pio315::PIO315_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_15"]
pub mod pio315;
#[doc = "PIO316 register accessor: an alias for `Reg<PIO316_SPEC>`"]
pub type PIO316 = crate::Reg<pio316::PIO316_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_16"]
pub mod pio316;
#[doc = "PIO317 register accessor: an alias for `Reg<PIO317_SPEC>`"]
pub type PIO317 = crate::Reg<pio317::PIO317_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_17"]
pub mod pio317;
#[doc = "PIO318 register accessor: an alias for `Reg<PIO318_SPEC>`"]
pub type PIO318 = crate::Reg<pio318::PIO318_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_18"]
pub mod pio318;
#[doc = "PIO319 register accessor: an alias for `Reg<PIO319_SPEC>`"]
pub type PIO319 = crate::Reg<pio319::PIO319_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_19"]
pub mod pio319;
#[doc = "PIO320 register accessor: an alias for `Reg<PIO320_SPEC>`"]
pub type PIO320 = crate::Reg<pio320::PIO320_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_20"]
pub mod pio320;
#[doc = "PIO321 register accessor: an alias for `Reg<PIO321_SPEC>`"]
pub type PIO321 = crate::Reg<pio321::PIO321_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_21"]
pub mod pio321;
#[doc = "PIO322 register accessor: an alias for `Reg<PIO322_SPEC>`"]
pub type PIO322 = crate::Reg<pio322::PIO322_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_22"]
pub mod pio322;
#[doc = "PIO323 register accessor: an alias for `Reg<PIO323_SPEC>`"]
pub type PIO323 = crate::Reg<pio323::PIO323_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_23"]
pub mod pio323;
#[doc = "PIO324 register accessor: an alias for `Reg<PIO324_SPEC>`"]
pub type PIO324 = crate::Reg<pio324::PIO324_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_24"]
pub mod pio324;
#[doc = "PIO325 register accessor: an alias for `Reg<PIO325_SPEC>`"]
pub type PIO325 = crate::Reg<pio325::PIO325_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_25"]
pub mod pio325;
#[doc = "PIO326 register accessor: an alias for `Reg<PIO326_SPEC>`"]
pub type PIO326 = crate::Reg<pio326::PIO326_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_26"]
pub mod pio326;
#[doc = "PIO327 register accessor: an alias for `Reg<PIO327_SPEC>`"]
pub type PIO327 = crate::Reg<pio327::PIO327_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_27"]
pub mod pio327;
#[doc = "PIO328 register accessor: an alias for `Reg<PIO328_SPEC>`"]
pub type PIO328 = crate::Reg<pio328::PIO328_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_28"]
pub mod pio328;
#[doc = "PIO329 register accessor: an alias for `Reg<PIO329_SPEC>`"]
pub type PIO329 = crate::Reg<pio329::PIO329_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_29"]
pub mod pio329;
#[doc = "PIO330 register accessor: an alias for `Reg<PIO330_SPEC>`"]
pub type PIO330 = crate::Reg<pio330::PIO330_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_30"]
pub mod pio330;
#[doc = "PIO331 register accessor: an alias for `Reg<PIO331_SPEC>`"]
pub type PIO331 = crate::Reg<pio331::PIO331_SPEC>;
#[doc = "Digital I/O control for port 3 pins PIO3_31"]
pub mod pio331;
#[doc = "PIO40 register accessor: an alias for `Reg<PIO40_SPEC>`"]
pub type PIO40 = crate::Reg<pio40::PIO40_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_0"]
pub mod pio40;
#[doc = "PIO41 register accessor: an alias for `Reg<PIO41_SPEC>`"]
pub type PIO41 = crate::Reg<pio41::PIO41_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_1"]
pub mod pio41;
#[doc = "PIO42 register accessor: an alias for `Reg<PIO42_SPEC>`"]
pub type PIO42 = crate::Reg<pio42::PIO42_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_2"]
pub mod pio42;
#[doc = "PIO43 register accessor: an alias for `Reg<PIO43_SPEC>`"]
pub type PIO43 = crate::Reg<pio43::PIO43_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_3"]
pub mod pio43;
#[doc = "PIO44 register accessor: an alias for `Reg<PIO44_SPEC>`"]
pub type PIO44 = crate::Reg<pio44::PIO44_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_4"]
pub mod pio44;
#[doc = "PIO45 register accessor: an alias for `Reg<PIO45_SPEC>`"]
pub type PIO45 = crate::Reg<pio45::PIO45_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_5"]
pub mod pio45;
#[doc = "PIO46 register accessor: an alias for `Reg<PIO46_SPEC>`"]
pub type PIO46 = crate::Reg<pio46::PIO46_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_6"]
pub mod pio46;
#[doc = "PIO47 register accessor: an alias for `Reg<PIO47_SPEC>`"]
pub type PIO47 = crate::Reg<pio47::PIO47_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_7"]
pub mod pio47;
#[doc = "PIO48 register accessor: an alias for `Reg<PIO48_SPEC>`"]
pub type PIO48 = crate::Reg<pio48::PIO48_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_8"]
pub mod pio48;
#[doc = "PIO49 register accessor: an alias for `Reg<PIO49_SPEC>`"]
pub type PIO49 = crate::Reg<pio49::PIO49_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_9"]
pub mod pio49;
#[doc = "PIO410 register accessor: an alias for `Reg<PIO410_SPEC>`"]
pub type PIO410 = crate::Reg<pio410::PIO410_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_10"]
pub mod pio410;
#[doc = "PIO411 register accessor: an alias for `Reg<PIO411_SPEC>`"]
pub type PIO411 = crate::Reg<pio411::PIO411_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_11"]
pub mod pio411;
#[doc = "PIO412 register accessor: an alias for `Reg<PIO412_SPEC>`"]
pub type PIO412 = crate::Reg<pio412::PIO412_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_12"]
pub mod pio412;
#[doc = "PIO413 register accessor: an alias for `Reg<PIO413_SPEC>`"]
pub type PIO413 = crate::Reg<pio413::PIO413_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_13"]
pub mod pio413;
#[doc = "PIO414 register accessor: an alias for `Reg<PIO414_SPEC>`"]
pub type PIO414 = crate::Reg<pio414::PIO414_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_14"]
pub mod pio414;
#[doc = "PIO415 register accessor: an alias for `Reg<PIO415_SPEC>`"]
pub type PIO415 = crate::Reg<pio415::PIO415_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_15"]
pub mod pio415;
#[doc = "PIO416 register accessor: an alias for `Reg<PIO416_SPEC>`"]
pub type PIO416 = crate::Reg<pio416::PIO416_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_16"]
pub mod pio416;
#[doc = "PIO417 register accessor: an alias for `Reg<PIO417_SPEC>`"]
pub type PIO417 = crate::Reg<pio417::PIO417_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_17"]
pub mod pio417;
#[doc = "PIO418 register accessor: an alias for `Reg<PIO418_SPEC>`"]
pub type PIO418 = crate::Reg<pio418::PIO418_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_18"]
pub mod pio418;
#[doc = "PIO419 register accessor: an alias for `Reg<PIO419_SPEC>`"]
pub type PIO419 = crate::Reg<pio419::PIO419_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_19"]
pub mod pio419;
#[doc = "PIO420 register accessor: an alias for `Reg<PIO420_SPEC>`"]
pub type PIO420 = crate::Reg<pio420::PIO420_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_20"]
pub mod pio420;
#[doc = "PIO421 register accessor: an alias for `Reg<PIO421_SPEC>`"]
pub type PIO421 = crate::Reg<pio421::PIO421_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_21"]
pub mod pio421;
#[doc = "PIO422 register accessor: an alias for `Reg<PIO422_SPEC>`"]
pub type PIO422 = crate::Reg<pio422::PIO422_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_22"]
pub mod pio422;
#[doc = "PIO423 register accessor: an alias for `Reg<PIO423_SPEC>`"]
pub type PIO423 = crate::Reg<pio423::PIO423_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_23"]
pub mod pio423;
#[doc = "PIO424 register accessor: an alias for `Reg<PIO424_SPEC>`"]
pub type PIO424 = crate::Reg<pio424::PIO424_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_24"]
pub mod pio424;
#[doc = "PIO425 register accessor: an alias for `Reg<PIO425_SPEC>`"]
pub type PIO425 = crate::Reg<pio425::PIO425_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_25"]
pub mod pio425;
#[doc = "PIO426 register accessor: an alias for `Reg<PIO426_SPEC>`"]
pub type PIO426 = crate::Reg<pio426::PIO426_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_26"]
pub mod pio426;
#[doc = "PIO427 register accessor: an alias for `Reg<PIO427_SPEC>`"]
pub type PIO427 = crate::Reg<pio427::PIO427_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_27"]
pub mod pio427;
#[doc = "PIO428 register accessor: an alias for `Reg<PIO428_SPEC>`"]
pub type PIO428 = crate::Reg<pio428::PIO428_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_28"]
pub mod pio428;
#[doc = "PIO429 register accessor: an alias for `Reg<PIO429_SPEC>`"]
pub type PIO429 = crate::Reg<pio429::PIO429_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_29"]
pub mod pio429;
#[doc = "PIO430 register accessor: an alias for `Reg<PIO430_SPEC>`"]
pub type PIO430 = crate::Reg<pio430::PIO430_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_30"]
pub mod pio430;
#[doc = "PIO431 register accessor: an alias for `Reg<PIO431_SPEC>`"]
pub type PIO431 = crate::Reg<pio431::PIO431_SPEC>;
#[doc = "Digital I/O control for port 4 pins PIO4_31"]
pub mod pio431;
#[doc = "PIO50 register accessor: an alias for `Reg<PIO50_SPEC>`"]
pub type PIO50 = crate::Reg<pio50::PIO50_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_0"]
pub mod pio50;
#[doc = "PIO51 register accessor: an alias for `Reg<PIO51_SPEC>`"]
pub type PIO51 = crate::Reg<pio51::PIO51_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_1"]
pub mod pio51;
#[doc = "PIO52 register accessor: an alias for `Reg<PIO52_SPEC>`"]
pub type PIO52 = crate::Reg<pio52::PIO52_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_2"]
pub mod pio52;
#[doc = "PIO53 register accessor: an alias for `Reg<PIO53_SPEC>`"]
pub type PIO53 = crate::Reg<pio53::PIO53_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_3"]
pub mod pio53;
#[doc = "PIO54 register accessor: an alias for `Reg<PIO54_SPEC>`"]
pub type PIO54 = crate::Reg<pio54::PIO54_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_4"]
pub mod pio54;
#[doc = "PIO55 register accessor: an alias for `Reg<PIO55_SPEC>`"]
pub type PIO55 = crate::Reg<pio55::PIO55_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_5"]
pub mod pio55;
#[doc = "PIO56 register accessor: an alias for `Reg<PIO56_SPEC>`"]
pub type PIO56 = crate::Reg<pio56::PIO56_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_6"]
pub mod pio56;
#[doc = "PIO57 register accessor: an alias for `Reg<PIO57_SPEC>`"]
pub type PIO57 = crate::Reg<pio57::PIO57_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_7"]
pub mod pio57;
#[doc = "PIO58 register accessor: an alias for `Reg<PIO58_SPEC>`"]
pub type PIO58 = crate::Reg<pio58::PIO58_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_8"]
pub mod pio58;
#[doc = "PIO59 register accessor: an alias for `Reg<PIO59_SPEC>`"]
pub type PIO59 = crate::Reg<pio59::PIO59_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_9"]
pub mod pio59;
#[doc = "PIO510 register accessor: an alias for `Reg<PIO510_SPEC>`"]
pub type PIO510 = crate::Reg<pio510::PIO510_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_10"]
pub mod pio510;
#[doc = "PIO511 register accessor: an alias for `Reg<PIO511_SPEC>`"]
pub type PIO511 = crate::Reg<pio511::PIO511_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_11"]
pub mod pio511;
#[doc = "PIO512 register accessor: an alias for `Reg<PIO512_SPEC>`"]
pub type PIO512 = crate::Reg<pio512::PIO512_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_12"]
pub mod pio512;
#[doc = "PIO513 register accessor: an alias for `Reg<PIO513_SPEC>`"]
pub type PIO513 = crate::Reg<pio513::PIO513_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_13"]
pub mod pio513;
#[doc = "PIO514 register accessor: an alias for `Reg<PIO514_SPEC>`"]
pub type PIO514 = crate::Reg<pio514::PIO514_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_14"]
pub mod pio514;
#[doc = "PIO515 register accessor: an alias for `Reg<PIO515_SPEC>`"]
pub type PIO515 = crate::Reg<pio515::PIO515_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_15"]
pub mod pio515;
#[doc = "PIO516 register accessor: an alias for `Reg<PIO516_SPEC>`"]
pub type PIO516 = crate::Reg<pio516::PIO516_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_16"]
pub mod pio516;
#[doc = "PIO517 register accessor: an alias for `Reg<PIO517_SPEC>`"]
pub type PIO517 = crate::Reg<pio517::PIO517_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_17"]
pub mod pio517;
#[doc = "PIO518 register accessor: an alias for `Reg<PIO518_SPEC>`"]
pub type PIO518 = crate::Reg<pio518::PIO518_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_18"]
pub mod pio518;
#[doc = "PIO519 register accessor: an alias for `Reg<PIO519_SPEC>`"]
pub type PIO519 = crate::Reg<pio519::PIO519_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_19"]
pub mod pio519;
#[doc = "PIO520 register accessor: an alias for `Reg<PIO520_SPEC>`"]
pub type PIO520 = crate::Reg<pio520::PIO520_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_20"]
pub mod pio520;
#[doc = "PIO521 register accessor: an alias for `Reg<PIO521_SPEC>`"]
pub type PIO521 = crate::Reg<pio521::PIO521_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_21"]
pub mod pio521;
#[doc = "PIO522 register accessor: an alias for `Reg<PIO522_SPEC>`"]
pub type PIO522 = crate::Reg<pio522::PIO522_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_22"]
pub mod pio522;
#[doc = "PIO523 register accessor: an alias for `Reg<PIO523_SPEC>`"]
pub type PIO523 = crate::Reg<pio523::PIO523_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_23"]
pub mod pio523;
#[doc = "PIO524 register accessor: an alias for `Reg<PIO524_SPEC>`"]
pub type PIO524 = crate::Reg<pio524::PIO524_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_24"]
pub mod pio524;
#[doc = "PIO525 register accessor: an alias for `Reg<PIO525_SPEC>`"]
pub type PIO525 = crate::Reg<pio525::PIO525_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_25"]
pub mod pio525;
#[doc = "PIO526 register accessor: an alias for `Reg<PIO526_SPEC>`"]
pub type PIO526 = crate::Reg<pio526::PIO526_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_26"]
pub mod pio526;
#[doc = "PIO527 register accessor: an alias for `Reg<PIO527_SPEC>`"]
pub type PIO527 = crate::Reg<pio527::PIO527_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_27"]
pub mod pio527;
#[doc = "PIO528 register accessor: an alias for `Reg<PIO528_SPEC>`"]
pub type PIO528 = crate::Reg<pio528::PIO528_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_28"]
pub mod pio528;
#[doc = "PIO529 register accessor: an alias for `Reg<PIO529_SPEC>`"]
pub type PIO529 = crate::Reg<pio529::PIO529_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_29"]
pub mod pio529;
#[doc = "PIO530 register accessor: an alias for `Reg<PIO530_SPEC>`"]
pub type PIO530 = crate::Reg<pio530::PIO530_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_30"]
pub mod pio530;
#[doc = "PIO531 register accessor: an alias for `Reg<PIO531_SPEC>`"]
pub type PIO531 = crate::Reg<pio531::PIO531_SPEC>;
#[doc = "Digital I/O control for port 5 pins PIO5_31"]
pub mod pio531;
