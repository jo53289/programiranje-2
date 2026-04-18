struct Par<T> {
    prvi: T, 
    drugi: T
} 

impl<T> Par<T> {
    fn new(a: T, b: T) -> Self {
        Par {
            prvi: a,
            drugi: b,
        }
    }
    fn prvi(&self) -> &T {
        &self.prvi
    }
    fn zamenjaj(self) -> Self {
        Par {
            prvi: self.drugi,
            drugi: self.prvi,
        }
        
    }
    fn vsebuje(&self, x: &T) -> bool 
    where T: PartialEq {
        self.prvi == *x || self.drugi == *x
    }
  
}

impl<T: PartialEq> PartialEq for Par<T> {
    fn eq(&self, other: &Self) -> bool {
        self.prvi == other.prvi && self.drugi == other.drugi
    }
}

impl Display for Par<T> {
    fn 
}

fn main() {
    let x = 5
    println!("{}", x)
}
