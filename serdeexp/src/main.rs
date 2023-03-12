
use serde::{Serialize, Deserialize, ser::SerializeStruct};
// use serde_derive::{ Serialize ,Deserialize };
use serde_json::Value;
// #[derive(Serialize,Deserialize,Debug)]
struct Point{
    x:i32,
    y:i32,
}
 

impl Point{
    fn new(x:i32,y:i32) -> Self{
        Point{
            x,
            y
        }
    }
}

// #[derive(Serialize)]
// struct Point(i32,i32);

impl Serialize for Point{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        let mut strt = serializer.serialize_struct("Point",2)?;
        strt.serialize_field("x",&self.x)?;
        strt.serialize_field("y",&self.y)?;
        strt.end()

    }
}
// impl Point{
//     fn new(x:i32,y:i32) -> Self{
//         Point(x,y)
//     }
// }

fn main() {

    let p = Point::new(40,30);

    let serialized = serde_json::to_string(&p).unwrap();

    // let deserialized :Point = serde_json::from_str(&serialized).unwrap();

    // println!("{},{:?}",serialized,deserialized);
    println!("{}",serialized);
}
