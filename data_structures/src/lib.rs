pub struct Vec2D<T: Clone> {
    vec: Vec<T>,
    width: usize
}

impl<T: Clone> Vec2D<T> {
    pub fn new(width: usize, height: usize, val: T) -> Self {
        let mut new_vec = vec!();
        new_vec.resize(width*height, val);
        Self {
            vec: new_vec,
            width
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.vec.get(x + y * self.width)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.vec.get_mut(x + y * self.width)
    }
}

pub struct Vec2DShifted<T: Clone> {
    vec: Vec<T>,
    width: usize,
    start_x: i64,
    start_y: i64,
}

impl<T: Clone> Vec2DShifted<T> {
    pub fn new(width: usize, height: usize, start_x: i64, start_y: i64, val: T) -> Self {
        let mut new_vec = vec!();
        new_vec.resize(width*height, val);
        Self {
            vec: new_vec,
            width,
            start_x,
            start_y
        }
    }
    pub fn get(&self, x: i64, y: i64) -> Option<&T> {
        let index = self.get_index(x, y);
        if index.is_some() {    
            return self.vec.get(index.unwrap());
        }
        return None;
    }
    pub fn get_mut(&mut self, x: i64, y: i64) -> Option<&mut T> {
        let index = self.get_index(x, y);
        if index.is_some() {    
            return self.vec.get_mut(index.unwrap());
        }
        return None;
    }
    #[inline(always)]
    fn get_index(&self, x: i64, y: i64) -> Option<usize> {
        let index: i64 = x - self.start_x + (y - self.start_y) * self.width as i64;
        if index >= 0 && index < self.vec.len() as i64 {
            return Some(index as usize);
        }
        return None;
    }
}



