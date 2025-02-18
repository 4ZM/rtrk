use core::f32;

pub type Interpolator = fn(&[f32], f32) -> f32;

pub fn step(data: &[f32], i: f32) -> f32 {
    data[i.floor() as usize]
}

pub fn lerp(data: &[f32], i: f32) -> f32 {
    let i_a = i.floor() as usize;
    let i_b = (i_a + 1) % data.len();

    let a = data[i_a];
    let b = data[i_b];

    let w_b = i - i_a as f32;
    let w_a = 1.0 - w_b;

    a * w_a + b * w_b
}

pub fn _min(data: &[f32]) -> f32 {
    data.iter().fold(f32::INFINITY, |a, &b| a.min(b))
}

pub fn _max(data: &[f32]) -> f32 {
    data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        let test_data = vec![0.0, 2.0, 2.0];

        assert_eq!(lerp(&test_data, 0.0), 0.0);
        assert_eq!(lerp(&test_data, 1.0), 2.0);
        assert_eq!(lerp(&test_data, 2.0), 2.0);

        assert_eq!(lerp(&test_data, 0.5), 1.0);
        assert_eq!(lerp(&test_data, 0.75), 1.5);
        assert_eq!(lerp(&test_data, 1.5), 2.0);
    }

    #[test]
    fn test_step() {
        let test_data = vec![0.0, 2.0, 2.0];

        assert_eq!(step(&test_data, 0.0), 0.0);
        assert_eq!(step(&test_data, 1.0), 2.0);
        assert_eq!(step(&test_data, 2.0), 2.0);

        assert_eq!(step(&test_data, 0.5), 0.0);
        assert_eq!(step(&test_data, 0.75), 0.0);
        assert_eq!(step(&test_data, 1.5), 2.0);
    }
}
