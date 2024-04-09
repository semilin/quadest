use bitvec::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum Mino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

pub type MinoMatrix = BitArr!(for 4, in u8, Lsb0);

pub struct MinoStates {
    size: usize,
    matrices: [[MinoMatrix; 4]; 4]
}

impl Mino {
    pub fn states(self, rotation: u8) -> &'static MinoStates {
        match self {
            I => &MinoStates {
                size: 4,
                matrices: [
                    BitArray::new::<MinoMatrix>([0b0000u8]),
                    bitarr![1, 1, 1, 1],
                    bitarr![0, 0, 0, 0],
                    bitarr![0, 0, 0, 0],
                ]
            }
        }
    }
}
