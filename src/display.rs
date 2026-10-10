use embedded_hal::digital::OutputPin;
use rp2040_hal::gpio::{DynPinId, FunctionSio, Pin, PullDown, SioOutput};

use crate::{
    numbers::{self, NUMBERS},
    segment::{Segment, SegmentMask},
};

type DynamicPin = Pin<DynPinId, FunctionSio<SioOutput>, PullDown>;

pub struct DisplayState {
    digit: u8,
    digits: [SegmentMask; 4],
    segment_pins: [Segment<DynamicPin>; 8],
    digit_pins: [Segment<DynamicPin>; 4],
    colon: bool
}

fn show<P: OutputPin>(mask: SegmentMask, segments: &mut [Segment<P>]) -> Result<(), P::Error> {
    for (i, segment) in segments.iter_mut().enumerate() {
        if mask.contains(i.try_into().unwrap()) {
            segment.set_on()?;
        } else {
            segment.set_off()?;
        }
    }

    Ok(())
}

impl DisplayState {
    pub fn new(segment_pins: [DynamicPin; 8], digit_pins: [DynamicPin; 4]) -> Self {
        Self {
            digit: 0,
            digits: [numbers::ZERO, numbers::ZERO, numbers::ZERO, numbers::ZERO],
            // digits: [numbers::ONE, numbers::TWO, numbers::THREE, numbers::FOUR],
            segment_pins: segment_pins.map(Segment::new),
            digit_pins: digit_pins.map(Segment::new),
            colon: false
        }
    }

    pub fn refresh(&mut self) {
        // 1. turn the previous digit off
        // 2. change the segments
        // 3. turn the current digit on
        // 4. return, without sleeping
        (self.digit_pins[self.digit as usize]).set_off().unwrap();

        self.digit = (self.digit + 1) % 4;

        let mut new_digits = self.digits[self.digit as usize];

        if self.digit == 1 && self.colon {
            new_digits |= SegmentMask::DP;
        }

        show(new_digits, &mut self.segment_pins).unwrap();

        (self.digit_pins[self.digit as usize]).set_on().unwrap();
    }

    pub fn change_time(&mut self, new_digits: [u8; 4], colon: bool) {
        self.colon = colon;
        self.digits = new_digits.map(|x| NUMBERS[x as usize]);
    }
}
