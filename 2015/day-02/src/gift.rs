use itertools::Itertools;

#[derive(Debug)]
pub struct Gift {
    l: u32,
    w: u32,
    h: u32,
}

impl Gift {
    pub fn new(l: u32, w: u32, h: u32) -> Self {
        Gift { l: l, w: w, h: h }
    }

    pub fn smallest_side(&self) -> u32 {
        let sides = vec![self.l, self.w, self.h];
        let smallest = sides
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| a * b)
            .min()
            .unwrap();
        smallest
    }

    pub fn surface_area(&self) -> u32 {
        2 * (self.l * self.w) + 2 * (self.w * self.h) + 2 * (self.h * self.l)
    }

    pub fn smallest_circum(&self) -> u32 {
        let sides = vec![self.l, self.w, self.h];
        let smallest_sides = sides
            .iter()
            .sorted()
            .take(2)
            .collect_tuple::<(_, _)>()
            .unwrap();
        2 * (smallest_sides.0) + 2 * (smallest_sides.1)
    }

    pub fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}
