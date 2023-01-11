#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub coords: [[f32; 3]; 3],
}

impl Mat3 {
    pub fn new(coords: [[f32; 3]; 3]) -> Mat3 {
        Mat3 { coords }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let m = Mat3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        println!("{:?}", m);
    }
}
