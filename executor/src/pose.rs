#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }

    pub(crate) fn forward(&mut self, offset: i32) {
        match self.heading {
            'E' => self.x += offset,
            'S' => self.y -= offset,
            'W' => self.x -= offset,
            'N' => self.y += offset,
            _ => {}
        }
    }

    pub(crate) fn turn_left(&mut self) {
        self.heading = match self.heading {
            'E' => 'N',
            'S' => 'E',
            'W' => 'S',
            'N' => 'W',
            h => h,
        };
    }

    pub(crate) fn turn_right(&mut self) {
        self.heading = match self.heading {
            'E' => 'S',
            'S' => 'W',
            'W' => 'N',
            'N' => 'E',
            h => h,
        };
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose::new(0, 0, 'N')
    }
}