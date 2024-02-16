pub fn lerp(part: f64, from: f64, to: f64) -> f64 {
    from + part * (to - from)
}