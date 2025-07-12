// TODO: 
// Need to implement support for both LE and BE?
pub fn to_u16(h: u8, l: u8) -> u16{
    ((h as u16) << 8) | (l as u16)
}

pub fn to_u32(hh: u8, hl: u8, lh: u8, ll: u8) -> u32{
    ((hh as u32) << 24) | ((hl as u32) << 16) | ((lh as u32) << 8) | (ll as u32)
}

pub fn to_u32_from_slice(a: &[u8; 4]) -> u32 {
    to_u32(a[0], a[1], a[2], a[3])
}

pub fn to_u64_from_slice(a: &[u8; 8]) -> u64 {
    ((to_u32(a[0], a[1], a[2], a[3]) as u64) << 32) | (to_u32(a[4], a[5], a[6], a[7]) as u64)
}

pub fn to_u64_from_slices(h: &[u8; 4], l:&[u8; 4]) -> u64 {
    ((to_u32(h[0], h[1], h[2], h[3]) as u64) << 32) | (to_u32(l[0], l[1], l[2], l[3]) as u64)
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
        let result = to_u32_from_slice(&data);
        let expected = 0x12345678 as u32;

        assert_eq!(expected, result);
    } 

    #[test]
    fn test_to_u64_from_slice(){
        let data: [u8; 8] = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        let result = to_u64_from_slice(&data);
        let expected = 0x123456789abcdef0 as u64;

        assert_eq!(expected, result);
    } 

    #[test]
    fn test_to_u64_from_slices(){
        let datah: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let datal: [u8; 4] = [0x9A, 0xBC, 0xDE, 0xF0];
        let result = to_u64_from_slices(&datah, &datal);
        let expected = 0x123456789abcdef0 as u64;

        assert_eq!(expected, result);
    } 

}
