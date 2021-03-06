pub trait PixelOpps<T> {
    fn substract(self, other: T) -> T;
    fn lt(self, other: T) -> bool;
}

impl PixelOpps<[u8; 3]> for [u8; 3] {
    fn substract(self, other: [u8; 3]) -> [u8; 3] {
        [
            self[0].substract(other[0]),
            self[1].substract(other[1]),
            self[2].substract(other[2]),
        ]
    }

    fn lt(self, other: [u8; 3]) -> bool {
        self[0] < other[0] && self[1] < other[1] && self[2] < other[2]
    }
}

impl PixelOpps<[u16; 3]> for [u16; 3] {
    fn substract(self, other: [u16; 3]) -> [u16; 3] {
        [
            self[0].substract(other[0]),
            self[1].substract(other[1]),
            self[2].substract(other[2]),
        ]
    }

    fn lt(self, other: [u16; 3]) -> bool {
        self[0] < other[0] && self[1] < other[1] && self[2] < other[2]
    }
}

impl PixelOpps<u8> for u8 {
    fn substract(self, other: u8) -> u8 {
        match self > other {
            true => self - other,
            false => other - self,
        }
    }

    fn lt(self, other: u8) -> bool {
        self < other
    }
}

impl PixelOpps<u16> for u16 {
    fn substract(self, other: u16) -> u16 {
        match self > other {
            true => self - other,
            false => other - self,
        }
    }

    fn lt(self, other: u16) -> bool {
        self < other
    }
}

impl PixelOpps<i32> for i32 {
    fn substract(self, other: i32) -> i32 {
        match self > other {
            true => self - other,
            false => other - self,
        }
    }

    fn lt(self, other: i32) -> bool {
        self < other
    }
}

impl PixelOpps<u32> for u32 {
    fn substract(self, other: u32) -> u32 {
        match self > other {
            true => self - other,
            false => other - self,
        }
    }

    fn lt(self, other: u32) -> bool {
        self < other
    }
}
