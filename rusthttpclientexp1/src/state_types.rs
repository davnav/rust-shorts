use std::marker::PhantomData;


#[derive(Debug)]
struct Locked;


#[derive(Debug)]
struct Unlocked;

#[derive(Default,Debug)]
struct Wallet<T=Locked>{
    amount: i32,
    state : std::marker::PhantomData<T>
}
impl Wallet<Locked>{

    fn unlock(self,password:String) -> Wallet<Unlocked>{
        Wallet::<Unlocked> { 
            amount : Default::default(),
            state : PhantomData::<Unlocked>,
        }
    }
}

impl Wallet<Unlocked>{
    fn showamount(self) -> i32{
        self.amount
    }
    fn update_wallet(self,amount:i32) -> Self{
        Wallet { amount:amount, state: self.state }
    }
}

impl Wallet{
    fn new() -> Self{
        Wallet { amount: Default::default(), state: Default::default()  }
    }

   
}

fn main(){

    let mut wallet = Wallet::new();

    println!("{:?}",wallet);

    let mut wallet = wallet.unlock("password".to_string());
    
    println!("{:?}",wallet);

    let mut wallet = wallet.update_wallet(30);

    
    println!("{:?}",wallet.showamount());
}