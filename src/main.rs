fn main() {
    // TODO 枚举
    /*
     *
     */
    #[derive(Debug)]
    enum IpAddKind {
        V4,
        V6,
    }
    let four = IpAddKind::V4;
    let size = IpAddKind::V6;
    println!("Hello, world! {:?}", four);
    println!("Hello, world! {:?}", size);

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("10.0.0.10"),
    };
    println!("{:#?}", home);

    #[derive(Debug)]
    enum IpAddKindVal {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let ip_v4_addr = IpAddKindVal::V4(10, 10, 10, 10);
    let ip_v6_addr = IpAddKindVal::V6(String::from("::1"));
    println!("Hello, world! {:#?}", ip_v4_addr);
    println!("Hello, world! {:#?}", ip_v6_addr);

    println!("---[给枚举定义方法]--------------------------------");

    impl IpAddKindVal {
        fn connect(&self) -> bool {
            true
        }
    }
    if IpAddKindVal::V4(10, 10, 10, 10).connect() {
        println!("is connect");
    }

}
