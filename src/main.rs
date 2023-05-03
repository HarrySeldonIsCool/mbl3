use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    for i in 2..50{
        let mut memo = Memo(HashMap::new());
        let a = Set::new(2..=i);
        println!("{}:{:?}", i, forsen(a, 0, &mut memo));
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Set(Vec<usize>);

struct Memo(HashMap<Set, bool>);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct OptSet(u128, Rc<Vec<u128>>);

impl OptSet{
    fn new(start: usize, end: usize) -> Self{
        let prot = ((1 << end)-1) ^ ((1 << start-1)-1);
        OptSet(
            prot,
            Rc::new([vec![0],(1..=end).map(|x| OptSet(prot, Rc::new(vec![])).pick(x).0).collect()].concat())
        )
    }
    fn pick(self, a: usize) -> Self{
        let mut krystof = self.0 & !1;
        for i in 2..=a{
                if a % i == 0 {
                    krystof &= !(1 << (i - 1));
                }
        }
        //println!("{}", krystof);
        OptSet(krystof, self.1)
    }
    fn opt_pick(&self, a: usize) -> Self{
        OptSet(self.0 & self.1[a], self.1.clone())
    }
    fn is_empty(&self) -> bool{
        self.0 == 0
    }
}

struct OptSetIter(OptSet, usize);

impl Iterator for OptSetIter{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.1 += 1;
        while self.1 < 128{
            if (self.0.0 >> (self.1-1)) & 1 != 0{
                return Some(self.1);
            }
            self.1 += 1;
        }
        None
    }
}

impl IntoIterator for OptSet{
    type IntoIter = OptSetIter;
    type Item = usize;
    fn into_iter(self) -> Self::IntoIter {
        OptSetIter(self, 0)
    }
}

impl Set{
    fn new<T: IntoIterator<Item = usize>>(a:T) -> Self{
        Set(a.into_iter().collect())
    }
    fn pick(&self, x: usize) -> Set{
        Set(self.0.clone().into_iter().filter(|a| x % a != 0).collect())
    }
}

#[derive(Debug)]
struct Future(Vec<Future>);

impl Future{
    fn predict(a: &Set) -> Self{
        if !a.0.is_empty(){
            Future(
                a.0.iter().map(|x| Future::predict(&a.pick(*x))).collect()
            )
        }
        else{
            Future(vec![])
        }
    }
    fn force(&self, turn: usize) -> bool{
        if self.0.is_empty(){
            turn % 2 == 1
        }
        else{
            if turn % 2 == 1{
                for x in self.0.iter(){
                    if !x.force(turn+1){
                        return false;
                    }
                }
                //self.0.iter().fold(true, |acc, x| acc && x.force(turn+1))
                true
            }
            else{
                for x in self.0.iter() {
                    if x.force(turn+1){
                        return true;
                    }
                }
                //self.0.iter().fold(false, |acc, x| acc || x.force(turn+1))
                false
            }
        }
    }
}

fn forsen(a: Set, turn: usize, memo: &mut Memo) -> (usize, bool){
    if a.0.is_empty(){
        (0, turn % 2 == 1)
    }
    else if let Some(x) = memo.0.get(&a){
        (0, x ^ (turn % 2 == 1))
    }
    else{
        if turn % 2 == 1{
            for x in a.0.iter(){
                if !forsen(a.pick(*x),turn+1, memo).1{
                    memo.0.insert(a.clone(), true);
                    return (*x, false);
                }
            }
            memo.0.insert(a.clone(), false);
            (0, true)
        }
        else{
            for x in a.0.iter() {
                if forsen(a.pick(*x), turn+1, memo).1{
                    memo.0.insert(a.clone(), true);
                    return (*x, true);
                }
            }
            memo.0.insert(a.clone(), false);
            (0, false)
        }
    }
}