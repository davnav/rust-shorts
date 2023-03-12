
use std::fmt::Debug;
// struct Context<'a>{
//     s: &'a str
// }

// struct Parser<'a>{
//     context : &'a Context<'a>
// }

// impl <'a> Parser<'a>{

//     fn parse() -> Result<(),&'a str>{

//         // parse_context(cont)
//         Ok(())
//     }

// }

static NUM: i32 = 18;

struct Car<'a>(&'a str);

impl<'a> Car<'a>{

    fn new(s : &'a str) -> Self{
       Car(s)
        
         
    }

    fn drive<'b>(&self,s:&'b str) -> &str{
        self.0
    }
}

fn add<'a,'b>(x:&'a mut i32,y:&'b i32) -> &'b i32 
where 'a : 'b
{
   let sum = *x + *y; 
    *x = sum;
    x
    
}

fn choose_first<'a:'b,'b>(first:&'a i32,second:&'b i32) -> &'b i32{
    first
}

fn traitbound<T>(_:T) 
where T: Debug+'static
{

}


fn single_parm<'a>(x:&'a i32) -> & i32{
    &3
}

fn main() {
    println!("Hello, world!");
    

    let i = 3;
    {
        let borrow1 = &i;
    }

    {
        let borrow2 = &i;
    }

    let a = add(&mut 5,&4);

    choose_first(&5,&6);

    println!("{}",i);
}
