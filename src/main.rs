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

struct linkedList<T: Copy> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
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

impl<T: Copy> List<T> for linkedList<T> {
    fn index(&self, mut n: usize) -> T {
        let mut current: Option<&Box<Node<T>>> = self.head.as_ref();
        while n > 0 {
            current = current.unwrap().next.as_ref();
            n -= 1;
        }
        current.as_deref().unwrap().val
    }

    fn length(&self) -> usize {
        if self.head.as_ref().is_none() {
            return 0;
        }
        let mut current = self.head.as_ref();
        let mut n = 1;
        while !current.unwrap().next.as_deref().is_none() {
            current = current.unwrap().next.as_ref();
            n += 1;
        }
        n
    }

    fn new<const N: usize>(args: [T; N]) -> Self {
        let mut linked = linkedList { head: None };
        if args.len() == 0 {
            return linked;
        }
        let mut current = &mut linked.head;
        for e in args {
            *current = Some(Box::new(Node { val: e, next: None }));
            current = &mut current.as_mut().unwrap().next;
        }
        linked
    }
}
