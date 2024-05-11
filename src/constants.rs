/// REAL_START and REAL_END constants are the endpoints of the interval in which the image is generated
/// The image is generated between x ∈ [REAL_START, REAL_END)
pub const REAL_END: f64 = 1.5;
pub const REAL_START: f64 = -2.5;
pub const MAX_ITER: u32 = 500;
pub const REAL_WIDTH: f64 = REAL_END - REAL_START;
// pub const BAILOUT_RADIUS: u16 = 1 << 8;
pub const BAILOUT_RADIUS_SQUARE: u32 = 1 << 16;
