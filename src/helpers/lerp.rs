//linear interpolation is used to finda value at a specific percentage (t)
// between a start value (A) and and end value (B). Moving from A to B based on t
//A + (B-A)*t
pub fn lerp(a: f32, b: f32, t: f32) -> f32{
   a +(b-a) * t
}