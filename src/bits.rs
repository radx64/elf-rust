#![allow(dead_code)]

pub fn to_u16(h: u8, l: u8) -> u16{
    ((h as u16) << 8) | (l as u16)
}

fn to_u16_from_slice_be(a: &[u8; 2]) -> u16 {
    to_u16(a[0], a[1])
}

fn to_u16_from_slice_le(a: &[u8; 2]) -> u16 {
    to_u16(a[1], a[0])
}

pub fn to_u16_from_slice(a: &[u8; 2], is_little_endian: bool) -> u16 {
    match is_little_endian {
        true => to_u16_from_slice_le(a),
        false => to_u16_from_slice_be(a),
    }
}

pub fn to_u32(hh: u8, hl: u8, lh: u8, ll: u8) -> u32{
    ((hh as u32) << 24) | ((hl as u32) << 16) | ((lh as u32) << 8) | (ll as u32)
}

fn to_u32_from_slice_be(a: &[u8; 4]) -> u32 {
    to_u32(a[0], a[1], a[2], a[3])
}

fn to_u32_from_slice_le(a: &[u8; 4]) -> u32 {
    to_u32(a[3], a[2], a[1], a[0])
}

pub fn to_u32_from_slice(a: &[u8; 4], is_little_endian: bool) -> u32 {
    match is_little_endian {
        true => to_u32_from_slice_le(a),
        false => to_u32_from_slice_be(a),
    }
}

fn to_u64_from_slice_be(a: &[u8; 8]) -> u64 {
    ((to_u32(a[0], a[1], a[2], a[3]) as u64) << 32) | (to_u32(a[4], a[5], a[6], a[7]) as u64)
}

fn to_u64_from_slice_le(a: &[u8; 8]) -> u64 {
    ((to_u32(a[7], a[6], a[5], a[4]) as u64) << 32) | (to_u32(a[3], a[2], a[1], a[0]) as u64)
}

pub fn to_u64_from_slice(a: &[u8; 8], is_little_endian: bool) -> u64 {
    match is_little_endian {
        true => to_u64_from_slice_le(a),
        false => to_u64_from_slice_be(a),
    }
}

fn to_u64_from_slices_be(h: &[u8; 4], l:&[u8; 4]) -> u64 {
    ((to_u32(h[0], h[1], h[2], h[3]) as u64) << 32) | (to_u32(l[0], l[1], l[2], l[3]) as u64)
}

fn to_u64_from_slices_le(h: &[u8; 4], l:&[u8; 4]) -> u64 {
    ((to_u32(h[3], h[2], h[1], h[0]) as u64) << 32) |  (to_u32(l[3], l[2], l[1], l[0]) as u64) 
}

pub fn to_u64_from_slices(h: &[u8; 4], l:&[u8; 4], is_little_endian: bool) -> u64 {
    match is_little_endian {
        true => to_u64_from_slices_le(h, l),
        false => to_u64_from_slices_be(h, l),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u16(){
        let result = to_u16(0x12, 0x34);
        let expected = 0x1234 as u16;

        assert_eq!(expected, result);
    }

    #[test]
    fn test_to_u32(){
        let result = to_u32(0x12, 0x34, 0x56, 0x78);
        let expected = 0x12345678 as u32;

        assert_eq!(expected, result);
    } 

    #[test]
    fn test_to_u32_from_slice(){
        let data:[u8;4] = [0x12, 0x34, 0x56, 0x78];
        let is_little_endian = false;
        let result = to_u32_from_slice(&data, is_little_endian);
        let expected = 0x12345678 as u32;
        assert_eq!(expected, result);

        let data:[u8;4] = [0x78, 0x56, 0x34, 0x12];
        let is_little_endian = true;
        let result = to_u32_from_slice(&data, is_little_endian);
        let expected = 0x12345678 as u32;
        assert_eq!(expected, result);
    } 

    #[test]
    fn test_to_u64_from_slice(){
        let data: [u8; 8] = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        let is_little_endian = false;
        let result = to_u64_from_slice(&data, is_little_endian);
        let expected = 0x123456789abcdef0 as u64;
        
        assert_eq!(expected, result);

        let data: [u8; 8] = [0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12];
        let is_little_endian = true;
        let result = to_u64_from_slice(&data, is_little_endian);
        let expected = 0x123456789abcdef0 as u64;

        assert_eq!(expected, result);
    } 

    #[test]
    fn test_to_u64_from_slices(){
        let datah: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let datal: [u8; 4] = [0x9A, 0xBC, 0xDE, 0xF0];
        let is_little_endian = false;
        let result = to_u64_from_slices(&datah, &datal, is_little_endian);
        let expected = 0x123456789abcdef0 as u64;

        assert_eq!(expected, result);

        let datah: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let datal: [u8; 4] = [0x9A, 0xBC, 0xDE, 0xF0];
        let is_little_endian = false;
        let result = to_u64_from_slices(&datah, &datal, is_little_endian);
        let expected = 0x123456789abcdef0 as u64;

        assert_eq!(expected, result);
    } 

}
