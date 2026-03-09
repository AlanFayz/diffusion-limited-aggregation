pub fn unpack_rgba(c: u32) -> (f32, f32, f32) {
    let r = ((c >> 24) & 0xFF) as f32 / 255.0;
    let g = ((c >> 16) & 0xFF) as f32 / 255.0;
    let b = ((c >> 8) & 0xFF) as f32 / 255.0;
    return (r, g, b);
}

pub fn pack_rgba(r: f32, g: f32, b: f32) -> u32 {
    let r = (r.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (g.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (b.clamp(0.0, 1.0) * 255.0) as u32;
    let a = 0xFF;

    return (r << 24) | (g << 16) | (b << 8) | a;
}

pub fn lerp(s: f32, e: f32, t: f32) -> f32 {
    s + (e - s) * t
}