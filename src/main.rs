/**
 * struct 结构体是自定义的数据类型，与其他语言中class类型但又不同
 * - 一旦申明可变，那么struct里面所有参数都可变
 */

// TODO 定义struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        username: String::from("hello"),
        email: String::from("summer@flamingo.shop"),
        active: true,
        sign_in_count: 100_00,
    };

    user1.username = String::from("summer");

    println!("username:{}", user1.username);
    println!("email:{}", user1.email);
    println!("active:{}", user1.active);
    println!("sign_in_count:{}", user1.sign_in_count);

    fn build_user(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: false,
        }
    }

    let mut user2 = build_user(
        String::from("Summer2"),
        String::from("x.xiaokang@gmail.com"),
    );

    user2.username = String::from("summer3");

    println!("username:{}", user2.username);
    println!("email:{}", user2.email);
    println!("active:{}", user2.active);
    println!("sign_in_count:{}", user2.sign_in_count);

    // TODO 更新语法
    println!("--[struct更新语法]----------------------");

    let user3 = User {
        email: String::from("summer@dollyface.shop"),
        ..user2
    };

    println!("username:{}", user3.username);
    println!("email:{}", user3.email);

    // TODO Tuple Struct
    /*
     *  想给整个tuple起名，并且让它不同于其他tuple，而且又不需要给每个元素起名
     */
    println!("--[tuple struct]----------------------");
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("R:{}, B:{}, G:{}", black.0, black.1, black.2);
    println!("X:{}, Y:{}, Z:{}", origin.0, origin.1, origin.2);

    // TODO Unit-Like Struct 空struct
    /*
     *  常用于需要在某个类型上实现trait，但该类型又不需要存储数据的情况
     */
    println!("--[unit-Like struct]----------------------");
    // * struct Ido;

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let w = 10;
    let l = 20;
    let rect = (30, 50);
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    fn area2(dim: (u32, u32)) -> u32 {
        dim.0 * dim.1
    }
    fn area3(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
    println!("{}", area(w, l));
    println!("{}", area2(rect));
    println!("{}", area3(&rect2));
    println!("{:#?}", rect2); // ? 如果上面不是借用，那么这行就会报错


    // TODO struct 方法
    /*
     * 使用 impl（implementations 实现）
     */
    println!("--[struct function]----------------------");
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, outher: &Rectangle) -> bool {
            self.width > outher.width && self.height > outher.height
        }
    }

    let rectfn = Rectangle {
        width: 30,
        height: 50,
    };
    let rectfn2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rectfn3 = Rectangle {
        width: 35,
        height: 55,
    };
    println!("{}", rectfn.area());
    println!("{}", rectfn.can_hold(&rectfn2));
    println!("{}", rectfn.can_hold(&rectfn3));


    // TODO 关联函数    
    /*
     * 关联函数通常用于构造器；例如： String::from()
     * 也可用于模块创建的命名空间
     */
    println!("--[关联函数]----------------------");

    impl Rectangle {
        fn square(size: u32) -> Rectangle{
            Rectangle{
                width: size,
                height: size,
            }
        }
    }

    // use 关联函数
    let square = Rectangle::square(100);
    println!("{:#?}", square);
}
