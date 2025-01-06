fn main(){
    let mut structExample = example{
        number: 30,
    };
    structExample.changeNumber(32);
    println!("{}", structExample.mult2())
}

struct example {
    number: i32,
}

impl example {
    fn changeNumber(&mut self, num : i32){
        self.number = num
    }

    fn mult2(&self) -> i32{
        return self.number * 2;
    }
}