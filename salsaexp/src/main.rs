use std::sync::Arc;


#[salsa::query_group(HelloWorldStorage)]
trait HelloWorld{

    #[salsa::input]
    fn input_string(&self,key:i32) -> Arc<String>;

    fn length(&self,key:i32) -> usize;

}

fn length(db:&dyn HelloWorld,x:i32) -> usize{
    let input_string = db.input_string(x);
    input_string.len()
}

#[salsa::database(HelloWorldStorage)]
#[derive(Default)]
struct DatabaseStruct{
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DatabaseStruct{}



fn main() {

    let mut db = DatabaseStruct::default();

    let mut value = Arc::new("Hello".to_string());
    db.set_input_string(1, value);

    value = Arc::new("hel".to_string()); 
    db.set_input_string(2, value);
    // value.insert_str(0, "H");
    println!("{}",db.length(1));
    println!("{}",db.length(2));
}
