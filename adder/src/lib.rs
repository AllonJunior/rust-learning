pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct  Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

// String test
pub fn greeting(name: &str) -> String{
    format!("Hello {}", name)
}

fn works_well() -> Result<(), String> {
    if 2+3 ==4 {
        Ok(())
    }else {
        Err(String::from("two plus three doesn't equal four"))
    }
}

#[cfg(test)]
mod tests {
    use std::arch::aarch64::vaba_s16;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration() {
        println!("Exloration test finished");
        assert_eq!(2+2, 4);
    }
    #[test]
    #[should_panic] // with should_panic, the function will work and pass the test
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger_Rect = Rectangle{
            width: 12,
            height:10,
        };
        let smaller_Rect = Rectangle{
            width: 10,
            height: 5,
        };
        assert!(larger_Rect.can_hold(&smaller_Rect));
    }

    #[test]
    fn greeting_contains_name(){
        let greeting = greeting("Allen");
        assert!(greeting.contains("Allen_Ren"),
                "Greeting didn't contain the name: '{}'",
                greeting
                );
    }
    #[test]
    #[ignore]
    // fn works_well() -> Result<(), String> {
    //     if 2+3 ==4 {
    //         Ok(())
    //     }else {
    //         Err(String::from("two plus three doesn't equal four"))
    //     }
    // }
    fn works_well_test() {
        let ret_value = works_well();
        assert!(ret_value.is_ok());
    }

}

