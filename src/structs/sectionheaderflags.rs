pub const SHF_WRITE: usize = 0x1;
pub const SHF_ALLOC: usize = 0x2;
pub const SHF_EXECINSTR: usize = 0x4;
pub const SHF_MERGE: usize = 0x10;
pub const SHF_STRINGS: usize = 0x20;
pub const SHF_INFO_LINK: usize = 0x40;
pub const SHF_LINK_ORDER: usize = 0x80;
pub const SHF_OS_NONCONFORMING: usize = 0x100;
pub const SHF_GROUP: usize = 0x200;
pub const SHF_TLS: usize = 0x400;
pub const SHF_MASKOS: usize = 0x0FF00000;
pub const SHF_MASKPROC: usize = 0xF0000000;
pub const SHF_ORDERED: usize = 0x4000000;
pub const SHF_EXCLUDE: usize = 0x8000000;

fn check_mask(value: u64, mask: usize) -> bool {
    let bit_is_set = (mask & value as usize) > 0;
    bit_is_set
}

pub fn flags_to_string(flags: u64) -> String {
    let mut result = vec![];

    if check_mask(flags, SHF_WRITE) {
        result.push("(SHF_WRITE) Writable");
    }

    if check_mask(flags, SHF_ALLOC) {
        result.push("(SHF_ALLOC) Occupies memory during execution");
    }

    if check_mask(flags, SHF_EXECINSTR) {
        result.push("(SHF_EXECINSTR) Executable");
    }

    if check_mask(flags, SHF_MERGE) {
        result.push("(SHF_MERGE) Might be merged");
    }

    if check_mask(flags, SHF_STRINGS) {
        result.push("(SHF_STRINGS) Contains null-terminated strings");
    }

    if check_mask(flags, SHF_INFO_LINK) {
        result.push("(SHF_INFO_LINK) 'sh_info' contains SHT index");
    }

    if check_mask(flags, SHF_LINK_ORDER) {
        result.push("(SHF_LINK_ORDER) Preserve order after combining");
    }

    if check_mask(flags, SHF_OS_NONCONFORMING) {
        result.push("(SHF_OS_NONCONFORMING) Non-standard OS specific handling required");
    }

    if check_mask(flags, SHF_GROUP) {
        result.push("(SHF_GROUP) Section is member of a group");
    }

    if check_mask(flags, SHF_TLS) {
        result.push("(SHF_TLS) Section hold thread-local data");
    }

    if check_mask(flags, SHF_MASKOS) {
        result.push("(SHF_MASKOS) OS-specific");
    }

    if check_mask(flags, SHF_MASKPROC) {
        result.push("(SHF_MASKPROC) Processor-specific");
    }

    if check_mask(flags, SHF_ORDERED) {
        result.push("(SHF_ORDERED) Special ordering requirement (Solaris)");
    }

    if check_mask(flags, SHF_EXCLUDE) {
        result.push("(SHF_EXCLUDE) Section is excluded unless referenced or allocated (Solaris)");
    }

    let mut result_string = String::new();

    for (index,flag) in result.iter().enumerate() {

        let mut to_be_appended = String::from("\t") + &String::from(*flag);

        if index < result.len() -1
        {
            to_be_appended += "\n";
        }

        result_string = result_string + &to_be_appended;
    }

    result_string
    
}
