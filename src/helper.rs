pub fn slice_to_u32 (slice: &[u8], reverse: bool) -> u32
{
    if slice.len() != 4
    {
        panic!("Was given slice of more than four bytes, potentially causing an overflow - size was {}", slice.len());
    }

    if reverse
    {
        (slice[0] as u32) +
        ((slice[1] as u32) << 8) +
        ((slice[2] as u32) << 16) +
        ((slice[3] as u32) << 24)
    }
    else
    {
        (slice[3] as u32) +
        ((slice[2] as u32) << 8) +
        ((slice[1] as u32) << 16) +
        ((slice[0] as u32) << 24)
    }
}

pub fn slice_to_u64(slice: &[u8], reverse: bool) -> u64
{
    if slice.len() != 8
    {
        panic!("Was given slice of more than four bytes, potentially causing an overflow - size was {}", slice.len());
    }

    if reverse
    {
        (slice[0] as u64) +
        ((slice[1] as u64) << 8) +
        ((slice[2] as u64) << 16) +
        ((slice[3] as u64) << 24) +
        ((slice[4] as u64) << 32) +
        ((slice[5] as u64) << 40) +
        ((slice[6] as u64) << 48) +
        ((slice[7] as u64) << 56)
    }
    else
    {
        (slice[7] as u64) +
        ((slice[6] as u64) << 8) +
        ((slice[5] as u64) << 16) +
        ((slice[4] as u64) << 24) +
        ((slice[3] as u64) << 32) +
        ((slice[2] as u64) << 40) +
        ((slice[1] as u64) << 48) +
        ((slice[0] as u64) << 56)
    }
}

pub fn slice_to_u16 (slice: &[u8], reverse: bool) -> u16
{
    if slice.len() != 2
    {
        panic!("Was given slice of more than four bytes, potentially causing an overflow - size was {}", slice.len());
    }

    if reverse
    {
        (slice[0] as u16) +
        ((slice[1] as u16) << 8)
    }
    else
    {
        (slice[1] as u16) +
        ((slice[0] as u16) << 8)
    }
}