fn main() {
    println!("Hello, world!");
}

trait List<T: Copy> {
    fn index(&self, n: usize) -> T;
    fn length(&self) -> usize;

    fn first(&self) -> T {
        self.index(0)
    }

    fn last(&self) -> T {
        self.index(self.length() - 1)
    }

    fn new<const N: usize>(args: [T; N]) -> Self;
}

struct arrayList<T: Copy> {
    data: Box<[T]>,
}

impl<T: Copy> List<T> for arrayList<T> {
    fn index(&self, n: usize) -> T {
        self.data[n]
    }

    fn length(&self) -> usize {
        self.data.len()
    }

    fn new<const N: usize>(args: [T; N]) -> Self {
        arrayList {
            data: Box::new(args),
        }
    }
}

