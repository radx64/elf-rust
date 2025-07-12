// TODO: 
// Need to implement support for both LE and BE?
pub fn to_u16(h: u8, l: u8) -> u16{
    ((h as u16) << 8) | (l as u16)
}

pub fn to_u32(hh: u8, hl: u8, lh: u8, ll: u8) -> u32{
    ((hh as u32) << 24) | ((hl as u32) << 16) | ((lh as u32) << 8) | (ll as u32)
}

pub fn to_u32_from_slice(a: &[u8; 4]) -> u32 {
    to_u32(a[3], a[2], a[1], a[0])
}

pub fn to_u64_from_slice(a: &[u8; 8]) -> u64 {
    ((to_u32(a[7], a[6], a[5], a[4]) as u64) << 32) | (to_u32(a[3], a[2], a[1], a[0]) as u64)
}

pub fn to_u64_from_slices(h: &[u8; 4], l:&[u8; 4]) -> u64 {
    ((to_u32(h[3], h[2], h[1], h[0]) as u64) << 32) | (to_u32(l[3], l[2], l[1], l[0]) as u64)
}
