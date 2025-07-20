pub const EI_MAG0: usize = 0;
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;
pub const EI_CLASS: usize = 4;
pub const EI_DATA: usize = 5;
pub const EI_VERSION: usize = 6;
pub const EI_OSABI: usize = 7;
pub const EI_ABIVERSION: usize = 8;
pub const EI_PAD: usize = 9;
pub const EI_NIDENT: usize = 16;

// Padding size is a result of whole identifier minus padding offset
pub const IDENT_PADDING_BYTES: usize = EI_NIDENT - EI_PAD;

pub const PHOFF32_START: usize = 0x1C;
pub const PHOFF32_END: usize = PHOFF32_START + 0x4;

pub const SHOFF32_START: usize = 0x20;
pub const SHOFF32_END: usize = SHOFF32_START + 0x4;

pub const FLAGS32_START: usize = 0x24;
pub const FLAGS32_END: usize = FLAGS32_START + 0x4;

pub const HEADERSIZE32_START: usize = 0x28;
pub const HEADERSIZE32_END: usize = HEADERSIZE32_START + 0x2;

pub const PHENTSIZE32_START: usize = 0x2A;
pub const PHENTSIZE32_END: usize = PHENTSIZE32_START + 0x2;

pub const PHNUM32_START: usize = 0x2C;
pub const PHNUM32_END: usize = PHNUM32_START + 0x2;

pub const SHENTSIZE32_START: usize = 0x2E;
pub const SHENTSIZE32_END: usize = SHENTSIZE32_START + 0x2;

pub const SHENTNUM32_START: usize = 0x30;
pub const SHENTNUM32_END: usize = SHENTNUM32_START + 0x2;

pub const SHSTRNDX32_START: usize = 0x32;
pub const SHSTRNDX32_END: usize = SHSTRNDX32_START + 0x2;

pub const PHOFF64_START: usize = 0x20;
pub const PHOFF64_END: usize = PHOFF64_START + 0x8;

pub const SHOFF64_START: usize = 0x28;
pub const SHOFF64_END: usize = SHOFF64_START + 0x8;

pub const FLAGS64_START: usize = 0x30;
pub const FLAGS64_END: usize = FLAGS64_START + 0x4;

pub const HEADERSIZE64_START: usize = 0x34;
pub const HEADERSIZE64_END: usize = HEADERSIZE64_START + 0x2;

pub const PHENTSIZE64_START: usize = 0x36;
pub const PHENTSIZE64_END: usize = PHENTSIZE64_START + 0x2;

pub const PHNUM64_START: usize = 0x38;
pub const PHNUM64_END: usize = PHNUM64_START + 0x2;

pub const SHENTSIZE64_START: usize = 0x3A;
pub const SHENTSIZE64_END: usize = SHENTSIZE64_START + 0x2;

pub const SHENTNUM64_START: usize = 0x3C;
pub const SHENTNUM64_END: usize = SHENTNUM64_START + 0x2;

pub const SHSTRNDX64_START: usize = 0x3E;
pub const SHSTRNDX64_END: usize = SHSTRNDX64_START + 0x2;
