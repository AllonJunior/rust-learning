mod front_of_house;

pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

pub fn serve_at_table(){
    serving::serve_order();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
