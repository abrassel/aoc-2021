// const ROTATIONS: [u8; 24] = [
//     (0, 0, 0),
//     (90, 0, 0),
//     (180, 0, 0),
//     (270, 0, 0),
//     (0, 90, 0),
//     (90, 90, 0),
//     (180, 90, 0),
//     (270, 90, 0),
//     (0, 180, 0),
//     (90, 180, 0),
//     (180, 180, 0),
//     (270, 180, 0),
//     (0, 270, 0),
//     (90, 270, 0),
//     (180, 270, 0),
//     (270, 270, 0),
//     (0, 0, 90),
//     (90, 0, 90),
//     (180, 0, 90),
//     (270, 0, 90),
//     (0, 0, 270),
//     (90, 0, 270),
//     (180, 0, 270),
//     (270, 0, 270),
// ];

#[derive(Default)]
struct Rotation([u8; 3]);

impl Rotation {
    pub const fn set_dim(&mut self, dim: usize, angle: u8) {
        self[dim] = 
    }
}

const fn rotations() -> [Rotation; 24] {
    let mut rotations: [Rotation; 24] = Default::default();
    let mut index = 0;
    for dim in 0..3 {
        for axis_angle in (0..360).step_by(90) {
            rotations[index].0[dim] = axis_angle;
            if dim > 0 {
                for x_angle in (0..360).step_by(90) {
                    rotations[index].0[0] = x_angle;
                    index += 1;
                }
            } else {
                index += 1;
            }
        }
    }

    rotations
}

const ROTATIONS: [Rotation; 24] = rotations();

const fn map_rotations() -> [i64; 24] {
    todo!()
}

const CONCRETE_ROTATIONS: [i64; 24] = map_rotations();
