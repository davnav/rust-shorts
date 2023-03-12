pub mod company;

mod anothermod{
    use crate::company::Employer;

    fn test2(){
        let emp1 = Employer::new("canada2".to_string());
    }
}