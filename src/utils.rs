pub fn parse(input: &str) -> Option<u32> {
    input.parse().ok()
}

pub fn zigzag(n: u32, z: u32) -> u32 {
    let n = n as i32;
    let z = z as i32;
    if z == 0 {
        return 0;
    }
    (z - (n % (z * 2) - z).abs()) as u32
}
