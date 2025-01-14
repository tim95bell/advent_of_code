#[derive(Clone)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    pub r: usize,
    pub c: usize,
}

impl Coord {
    pub fn to_index(&self, width: usize) -> usize {
        width * (self.r as usize) + (self.c as usize)
    }

    pub fn zero() -> Self {
        Self { r: 0, c: 0 }
    }

    pub fn create(r: usize, c: usize) -> Self {
        Self { r, c }
    }
}

impl<T> Matrix<T> {
    pub fn create(width: usize, height: usize) -> Self {
        Self {
            data: Vec::<T>::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn create_from_string(s: &str, f: fn(u8) -> T) -> Self {
        let lines = s.lines().filter(|x| !x.is_empty());
        let width = lines.clone().next().unwrap().len();
        let height = lines.clone().count();
        let mut result = Self::create(width, height);
        for line in lines {
            let row = line.as_bytes().iter().map(|&x| f(x));
            result.data.extend(row);
        }
        result
    }

    pub fn create_from_string_with_context<C>(
        s: &str,
        context: &mut C,
        f: fn(&mut C, u8) -> T,
    ) -> Self {
        let lines = s.lines().filter(|x| !x.is_empty());
        let width = lines.clone().next().unwrap().len();
        let height = lines.clone().count();
        let mut result = Self::create(width, height);
        for line in lines {
            let row = line.as_bytes().iter().map(|&x| f(context, x));
            result.data.extend(row);
        }
        result
    }

    pub fn create_from_string_with_context_and_indices<C>(
        s: &str,
        context: &mut C,
        f: fn(&mut C, u8, usize, usize) -> T,
    ) -> Self {
        let lines = s.lines().filter(|x| !x.is_empty()).enumerate();
        let width = lines.clone().next().unwrap().1.len();
        let height = lines.clone().count();
        let mut result = Self::create(width, height);
        for (r, line) in lines {
            let row = line
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(c, &x)| f(context, x, r, c));
            result.data.extend(row);
        }
        result
    }

    pub fn get_mut_coord(&mut self, x: Coord) -> &mut T {
        self.get_mut(x.r, x.c)
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> &mut T {
        let index = r * self.width + c;
        assert!(index < self.width * self.height);
        assert!(c < self.width);
        return &mut self.data[index];
    }

    pub fn get_coord(&self, x: Coord) -> &T {
        self.get(x.r, x.c)
    }

    pub fn get(&self, r: usize, c: usize) -> &T {
        let index = r * self.width + c;
        assert!(index < self.width * self.height);
        assert!(c < self.width);
        return &self.data[index];
    }
}

impl<T> std::fmt::Debug for Matrix<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        for r in 0..self.height {
            for c in 0..self.width {
                string.push_str(&self.get(r, c).to_string());
            }
            string.push_str("\n");
        }
        f.write_str(&string)
    }
}
