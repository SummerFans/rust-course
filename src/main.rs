use std::collections::HashMap;

// * 常用集合
// ? 在堆存储，运行时可变的集合

fn main() {
    /***********************************************
     *                TODO Vector                  *
     ***********************************************/

    /*
     * 标准库提供， 可存储多个值，只能存储唯一类型，在内存中连续存放
     */
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    println!("{}", v[0]);

    // * 使用vector宏
    let mut v2 = vec![10, 20, 30];
    v2.push(40);
    println!("{}", v2[0]);

    // 🤣 “所有权”部分概念的体现 🤣
    let v2_index2 = &v2[2];

    let v2 = vec!["stirng"];

    println!("{}", v2[0]);

    println!("{}", v2_index2);

    let mut v3 = vec![1, 2, 3, 4, 5];
    let index_v3 = &v3[3];
    println!("{}", index_v3);
    v3.push(10);
    println!("{}", v3[3]);
    // ! println!("{}", index_v3); 如果这里会因为所有权问题报错，

    // TODO vector 访问
    /*
     * 使用get方式访问超出索引不会出现panic
     */
    match v2.get(0) {
        Some(val) => println!("{}", val),
        None => println!(),
    }

    println!("Hello, world!");

    // TODO 遍历 vector
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 10;
        println!("{}", i);
    }

    // TODO Vector 存放不同类型
    /*
     * 借用枚举（enum）特性在vector中存放不同类型的数据
     */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.29),
    ];

    for r in &row {
        match r {
            SpreadsheetCell::Int(num) => println!("is number, {}", num),
            SpreadsheetCell::Float(f) => println!("is Float, {}", f),
            SpreadsheetCell::Text(str) => println!("is Text, {}", str),
        }
    }

    /***********************************************
     *                TODO String                  *
     ***********************************************/
    /*
     * 基于Byte的集合，提供一些方法解析成文本，统一使用utf8编码
     * Rust核心语言层只提供了一个字符串类型：str（字符串切片）or &str
     * String类型来自标准库（不是核心库） String和&str是不同类型的
     */
    /*
     * 标准包含其他字符串类型：OsString，OsStr，CString，CStr
     * String 是可拥有所有权， Str只能借用
     */

    let mut my_is_string = String::new();
    my_is_string.push_str("summer");
    println!("{}", my_is_string);
    let mut s = "hello".to_string();
    s.push_str(" world");
    s.push('!');
    println!("{}", s);

    // todo 字符串拼接
    // 加号操作是将首字符串于后面所有字符串的引用进行相加，首字符串的所有权会移交给新声明变量，首字符串变量就会失去所有权，而后面字符串变量就不会失去所有权，可继续访问变量
    // ? 后面所有相加都是字符串的引用，但实际是字符串切片，为什么会出现这种是因为rust进行（deref coercion）解引用强项转换
    let s0 = String::from("summer, ");
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");

    // TODO format！拼接多字符串
    // s0 不会丢失所有权
    let s4 = format!("{}-{}-{}", s0, s1, s2);
    println!("{}", s4);

    // 这种模式s0会丢失所有权
    let s3 = s0 + &s1 + &s2;
    println!("{}", s3);

    // TODO String 索引访问
    // 不支持index索引访问
    let str = String::from("中文字数");
    let len = str.len();
    println!("{}", &len);
    // ? Unicode 标量值
    // ? Rust三种看待字符串方式： 字节(Bytes) 标量值(Scalar Value) 字形簇(Grapheme Clusters)

    // str.bytes() 字节方式
    // str.chars() 标量值
    // str.graphemes(true) // 需要依赖第三方库：unicode-segmentation

    // 标量值
    for b in str.chars() {
        println!("{}", b);
    }

    // [中(12,228,184) 文(173,230,150) 字(135,229,173) 数(151,230,149,176)]
    // ! 索引取之一旦取错位，就会出现panic
    println!("{}", &str[9..12]);

    /***********************************************
     *                TODO HashMap                 *
     ***********************************************/
    /*
     * 因为HashMap用的较少，没有在prelude（预导入库中）连宏也没有；
     * 而且HashMap是同构的（K或V都是同类型）
     */

    // HashMap<K,V>
    let mut scores0: HashMap<i32, String> = HashMap::new();
    scores0.insert(100, String::from("blue"));
    scores0.insert(200, String::from("yellow"));

    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 100);
    scores1.insert(String::from("Yellow"), 100);

    let teams = vec![String::from("Yellow"), String::from("Blue")];
    let intial_scores = vec![50, 10];

    // _, _无需指明是因为rust会根据具体返回的结果判断数据类型
    let scores3: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    println!("{:?}", scores3);

    // HashMap的复制与所有权转移
    let index1 = 100;
    let index2 = 200;

    let v_yellow = String::from("Yellow");
    let v_blue = String::from("Blue");

    let mut ownership_data = HashMap::new();
    ownership_data.insert(index1, v_yellow);
    ownership_data.insert(index2, v_blue);
    // ! 持有所有权会被转移所有权给HashMap，原先变量就无法访问
    // println!("{}, {}", v_yellow, v_blue);
    // ? 实现了Copy trait类型的数据，所有权依然存在，因为会复制一份到hashmap中
    println!("{}, {}", index1, index2);

    // TODO 访问 HashMap
    let od_val = match ownership_data.get(&100) {
        Some(s) => s,
        _ => "",
    };
    println!("od_val: {}", od_val);

    // 遍历HashMap
    for (k, v) in &ownership_data {
        println!("K:{}, V:{}", k, v);
    }

    // TODO 更新HaspMap
    // 1.强制更新，k值100值已存在如果使用insert会强制替换原先的值
    ownership_data.insert(100, String::from("replace 100"));

    // 2.entry, 检查是否存在不存在插入值，存在不操作
    ownership_data.entry(100).or_insert(String::from("replace 200"));
    ownership_data.entry(101).or_insert(String::from("replace 101"));

    let text = "hello world wonderful world";
    let mut map_data = HashMap::new();

    // 字符串通过 split_whitespace 空格分割遍历
    for word in text.split_whitespace() {

        // 使用entry函数判断是否存在，不存在赋值0，如果存在返回当前值
        let count = map_data.entry(word).or_insert(0);
        // 获得当前值后解引用，最后递增+1
        *count += 1;
    }

    println!("{:?}", map_data);
    
}
