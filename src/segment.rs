use embedded_hal::digital::{Error, OutputPin};

#[derive(Clone, Copy)]
pub struct SegmentMask(pub u8);

impl SegmentMask {
    pub const T: Self = Self(1 << 0);
    pub const TR: Self = Self(1 << 1);
    pub const BR: Self = Self(1 << 2);
    pub const B: Self = Self(1 << 3);
    pub const BL: Self = Self(1 << 4);
    pub const TL: Self = Self(1 << 5);
    pub const M: Self = Self(1 << 6);
    pub const DP: Self = Self(1 << 7);

    pub fn contains(&self, i: u8) -> bool {
        return self.0 & (1 << i) != 0;
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl core::ops::BitOr for SegmentMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SegmentMask {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0);
    }
}

pub struct Segment<P: OutputPin> {
    on: bool,
    pin: P,
}

impl<P: OutputPin> Segment<P> {
    pub fn new(pin: P) -> Self {
        Self { on: false, pin }
    }

    pub fn set_on(&mut self) -> Result<(), P::Error> {
        self.pin.set_low()?;
        self.on = true;
        Ok(())
    }

    pub fn set_off(&mut self) -> Result<(), P::Error> {
        self.pin.set_high()?;
        self.on = false;
        Ok(())
    }

    pub fn toggle(&mut self) -> Result<(), P::Error> {
        if self.on {
            self.set_off()?;
        } else {
            self.set_on()?;
        }

        Ok(())
    }
}

pub type Digit<P> = Segment<P>;
