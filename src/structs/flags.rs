

pub const PF_X: usize = 0x1;
pub const PF_W: usize = 0x2;
pub const PF_R: usize = 0x4;

fn check_mask(value: u32, mask: usize) -> bool {
    let bit_is_set = (mask & value as usize) > 0;
    bit_is_set
}

pub fn flags_to_string(flags: u32) -> String {
    let mut result = vec![" ", " ", " "];

    if check_mask(flags, PF_X) {
        result[2] = "E";
    }

    if check_mask(flags, PF_W) {
        result[1] = "W";
    }

    if check_mask(flags, PF_R){
        result[0] = "R";
    }

    String::from(result.concat())
}
