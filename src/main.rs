
fn main() {
    let test11 = vec![1,2,3,4,5];
    let test12 = vec![1.1,2.2,3.3,4.4,5.5];
    println!("#1");
    println!("测试1：[1,2,3,4,5]总和");
    println!("{}", Buffer::buffer(test11).sum());
    println!("测试1：[1.1,2.2,3.3,4.4,5.5]总和");
    println!("{}", Buffer::buffer(test12).sum());
    println!("#2");
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    print!("Compare {} with {} : ",s1,s2);
    println!("{}", if comparestring(s1.as_str(), s2.as_str()) {"Bigger."} else {"Smaller."});
    println!("#3");
    let x:Vec<char> = vec!['a','b','c','d','e'];
    print!("Previous: ");
    for i in x.clone(){
        print!("{} ", i);
    }
    println!();
    print!("Now: ");
    for i in pluselements(x.clone()){
        print!("{} ", i);
    }println!();

}
struct Buffer<T>{
    mem: Vec<T>
}

impl <T:std::ops::Add<Output = T> + Copy> Buffer<T> {
    fn buffer(mem: Vec<T>)->Self{
        Buffer::<T> { mem }
    }
    fn sum(&self) -> T{
        let mut sum = self.mem[0];
        for i in 1..self.mem.len()-1{
            sum = sum + self.mem[i];
        }
        sum
    }
}

/*
#比较字符串的字典序，true为x的字典序大于y。
*/
fn comparestring(x: &str, y: &str) -> bool{
    let mut a = x.to_string();
    let mut b = y.to_string();
    let len = if a.len() > b.len() {b.len()} else { a.len()};
    let mut i = 0;
    while i < len{
        let aa = a.remove(0);
        let bb = b.remove(0);
        if aa > bb {
            break;
        }else if aa == bb {
            i +=1;
        }else {
            return false;
        }
    }
    true
}

/*
第三个测试，先转u8再转char，
*/
fn pluselements (items: Vec<char>) -> Vec<char>{
    let tmp = items.iter().map(|c| *c as u8+1).collect::<Vec<_>>();
    let ret = tmp.iter().map(|b| *b as char).collect::<Vec<_>>();
    ret
}