use core::num;


#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor{
    Red,
    Blue,
}

struct Inventory{
    shirts: Vec<ShirtColor>,
}
impl Inventory{
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue + 1,
                ShirtColor::Red => num_red + 1,
            };
        }

        if num_blue > num_red {
            ShirtColor::Blue
        }else {
            ShirtColor::Red
        }

    }
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory{
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
    };

    let up1 = Some(ShirtColor::Blue);
    let gw1 = store.giveaway(up1);
    println!("user preferred: {:?} get {:?}", up1, gw1);

    let up2 = None;
    let gw2 = store.giveaway(up2);
    println!("user preferred: {:?} get {:?}", up2, gw2);

    let mut rect_list = vec![
        Rectangle{width: 16, height: 23},
        Rectangle{width:10, height:18},
        Rectangle{width:20, height:30}
    ];

    // rect_list.sort_by_key(|r| r.width);
    rect_list.sort_by(|a, b| b.width.cmp(&a.width));
    println!("{rect_list:#?}"); 

}
