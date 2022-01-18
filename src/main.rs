use std::{thread, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn expensive_task(i: &i32) {
    thread::sleep(Duration::from_secs(1));
    println!("{i}");
}

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.bar));
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.bar),
            " ".repeat(self.bound - progress.bar),
            self.delims.1
        );
    }
}

struct Progress<Iter, Bound> {
    bar: usize,
    it: Iter,
    bound: Bound,
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: Iterator,
{
    pub fn new(it: Iter) -> Self {
        Self {
            bar: 0,
            it,
            bound: Unbounded,
        }
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{CLEAR}");
        self.bound.display(self);
        self.bar += 1;
        self.it.next()
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    fn with_bounds(self) -> Progress<Iter, Bounded> {
        Progress {
            bound: Bounded {
                bound: self.it.len(),
                delims: ('[', ']'),
            },
            it: self.it,
            bar: self.bar,
        }
    }
}
impl<Iter> Progress<Iter, Bounded>
where
    Iter: Iterator,
{
    fn with_delmis(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}


fn main() {
    let v = vec![1, 2, 3];
    for data in v.iter().progress().with_bounds().with_delmis(('<', '>')) {
        expensive_task(data)
    }
    thread::sleep(Duration::from_secs(1));
    for i in (1..).progress() {
        expensive_task(&i)
    }
}
