use rustvisibility::company::{self, Employee, public_api};
fn main() {
    let emp = Employee::new("naveen".to_string(), 37);

    public_api();
}
