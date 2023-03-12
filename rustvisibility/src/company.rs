pub fn public_api() {}
pub(super)struct Employer {
    name: String,
}

impl Employer {
    pub(super) fn new(name: String) -> Self {
        Self { name }
    }
}

pub struct Employee {
    name: String,
    age: i32,
}

impl Employee {
    pub fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
}

mod test1 {

    use super::Employer;
    fn test() {
        let emp1 = Employer::new("canada".to_string());
    }
}
