#![allow(dead_code, unused_variables, unused)]
use std::{fmt::Debug, thread::sleep, time::Duration};

///https://www.youtube.com/watch?v=bnnacleqg6k&t=1674s
///https://www.youtube.com/watch?v=pwmIQzLuYl0
const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";

#[derive(Debug, Clone)]
struct Bound {
    size: usize,
    delims: (String, String),
}
#[derive(Debug, Clone)]
struct Progress<Iter> {
    iter: Iter,
    count: usize,
    bound: Option<Bound>,
}
impl<Iter> Progress<Iter> {
    fn new(iter: Iter) -> Self {
        Self {
            iter,
            count: 0,
            bound: None,
        }
    }
}

///泛型<...>不重要，就只是個佔位符
/// where T:...才是需要注意的地方，因為會限制哪些東西可以，那些東西不可以
/// 因此直接看成
/// impl Progress<T:...>{} ，
/// 代表實作一個類型方法叫 Progress...,
/// ...在實際使用的時候，compiler會依實際類型幫我們單態化(monomorphization)
/// ex: Progress*IterI32*,Progress*RangeI32*,...
/// 且這些類型都有實作 with_bound方法。
impl<Iter> Progress<Iter>
where
    Iter: ExactSizeIterator,
{
    fn with_bound(self) -> Self {
        let bound = Bound {
            size: self.iter.len(),
            delims: ("[".into(), "]".into()),
        };
        Self {
            bound: Some(bound),
            ..self
        }
    }
    /// 直接寫with delims有個問題是，
    /// 你需要使用者選擇 with_bound後才讓使用者有可以更改邊框的選項
    /// 但在這邊直接寫，使用者沒有with_bound也可以使用with_delims，
    /// 當然你也可以當作使用者選擇with_delims就代表使用者也同意使用with_bound就是了
    fn with_delims(self, delims: (impl Into<String>, impl Into<String>)) -> Self {
        let bound = Bound {
            size: self.iter.len(),
            delims: (delims.0.into(), delims.1.into()),
        };
        Self {
            bound: Some(bound),
            ..self
        }
    }
}
impl<Itr> Iterator for Progress<Itr>
where
    Itr: Iterator,
{
    type Item = Itr::Item;
    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR_SCREEN);
        match self.bound.as_ref() {
            Some(bound) => println!(
                "{}{}{}{}",
                bound.delims.0,
                "*".repeat(self.count),
                " ".repeat(bound.size - self.count),
                bound.delims.1,
            ),
            None => println!("{}", "*".repeat(self.count)),
        }
        //println!("{:?}", "*".repeat(self.count));
        self.count += 1;
        self.iter.next()
    }
}

trait ProgressExt: Sized {
    fn progress(self) -> Progress<Self>;
}
impl<Itr> ProgressExt for Itr
where
    Itr: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}
fn expensive_calculation<T>(i: &T)
where
    T: Debug,
{
    println!("{:?}", i);
    sleep(Duration::from_secs(1));
}
#[test]
fn run() {
    //let can_construct_but_not_iterator = Progress::new(1);
    //let can_not_construct_by_progress_method = 1_i32.progress();
    let source = (0..10);
    let prog = source.progress().with_delims(("[", ">"));
    for i in prog.clone().map(|i| i + 2) {
        expensive_calculation(&i);
    }
    println!("{:?}", prog);
    //assert_eq!(1, 1);
}
