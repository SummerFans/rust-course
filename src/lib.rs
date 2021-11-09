mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}
    //     fn serve_order() {}
    //     fn take_payment() {}
    // }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    // ! 如果定义与使用永远在一个文件内，可以使用相对路径，否则使用绝对路径
    front_of_house::hosting::seat_at_table();
}


// TODO super
fn serve_order() {}

mod back_of_house{
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // super 使用
        crate::serve_order(); // 也可以使用绝对路径
    }

    fn cook_order() {}
}

pub fn use_back_of_order() {
    crate::back_of_house::fix_incorrect_order();
}


// TODO pub struct and pub enum
/*
 * pub 放在 struct前：
 * - struct是公共的
 * - struct的内部字段默认是私有的  
 */

 /*
 * pub 放在 enum前：
 * - enum是公共的
 * - enum的内部字段默认是“公共”的  
 */

mod pubstruct{
    pub struct Breakfast {
        pub toast: String,          // public
        seasonal_fruit: String,     // private
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}


pub fn eat_at_restaurant_2() {
    let mut meal = pubstruct::Breakfast::summer("Summer");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("{:?}", pubstruct::Appetizer::Soup);
    // ! seasonal_fruit 私有的不允许操作
    // ! meal.seasonal_fruit = String::from("blueberies");  
}