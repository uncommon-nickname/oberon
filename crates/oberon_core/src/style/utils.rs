pub(crate) fn calculate_hue(max: f32, min: f32, r: f32, g: f32, b: f32) -> f32
{
    let h = if close_eq(max, r)
    {
        60.0 * (g - b) / (max - min)
    }
    else if close_eq(max, g)
    {
        60.0 * (b - r) / (max - min) + 120.0
    }
    else if close_eq(max, b)
    {
        60.0 * (r - g) / (max - min) + 240.0
    }
    else
    {
        0.0
    };
    (h + 360.0) % 360.0
}

pub(crate) fn calculate_saturation(max: f32, min: f32, l: f32) -> f32
{
    if 0.0 < l && l <= 0.5
    {
        (max - min) / (2.0 * l)
    }
    else
    {
        (max - min) / 2.0_f32.mul_add(-l, 2.0)
    }
}

fn close_eq(a: f32, b: f32) -> bool
{
    if a == b
    {
        return true;
    }
    let max_abs = a.abs().max(b.abs());
    max_abs <= f32::EPSILON || ((a - b).abs() / max_abs) <= f32::EPSILON
}
