
#[derive(PartialEq, Debug)]
struct  Shoe{
    size: u32,
    style: String,
}

fn get_shoes(shoes: Vec<Shoe>, required_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size==required_size).collect()
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn shoes_in_size(){
        let shoes = vec![
            Shoe{size: 32, style: String::from("sneaker")},
            Shoe{size: 40, style: String::from("sandal")},
            Shoe{size: 40, style: String::from("boot")},
        ];
        let shoes_in_size = get_shoes(shoes, 40);

        assert_eq!(
            shoes_in_size,
            vec![
                Shoe{size: 40, style: String::from("sandal")},
                Shoe{size: 40, style: String::from("boot")},
            ]
        );
    }
}