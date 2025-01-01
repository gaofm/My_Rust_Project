fn main() {
    //变量和数据交互的方式：移动（Move） String版本
    // 一个string由三部分构成分别是：①存放字符串内容的指针，②长度，③容量。数据2 3是被存放在栈stack中，存储的字符串具体内容1是存放在堆heap当中。
    let s1 = String::from("hello");
    //将s1复制给s2，当变量离开作用域时Rust会自动调用drop函数，释放相应的栈内存，当s1和s2同时离开作用域时会尝试释放相同的内存，从而引起：“二次释放（double free）bug”
    let s2 = s1;
    print!("s2:{}", s2);
    //Rust为了保证内存安全，不会尝试复制被分配的内存，rust会让s1失效，当s1离开作用域师Rust不会释放任何东西，当s1被赋值给其他变量时会使得s1失效，无法被调用 println!("{}",s1);//报错，提示信息为：借用了移动的值

    //浅拷贝 (Shallow Copy)
    //浅拷贝指的是只复制数据的指针或引用，而不复制实际数据本身。在 Rust 中，当你将某些类型（如 String 或 Vec）的值赋值给另一个变量时，会发生 所有权转移（move）


     
    //深拷贝 (Deep Copy)
    //深拷贝会复制实际的数据，而不仅仅是指针或引用。深拷贝会分配新的内存，并将数据复制到新地址，在rust当中使用的是clone来进行深拷贝。
    let t1: String = String::from("hello,world!!!");
    let t2 = t1.clone();
    print!("t1:{},t2:{}", t1, t2);

    //对于stake栈上的数据直接赋值就可以
    let x = 5; //x为整数类型，在编译时是固定的，能将自己的数据完整存储在stake当中，对这些值的复制操作是非常快速的。进行深拷贝和浅拷贝是没有任何区别的。
    let y = x;
    println!("y:{}", y);

    //如果一个类型实现了copy的这个trait这个接口，那么旧的变量在赋值之后还是可以继续使用的。
    //如果一个类型或者该类型的一部分实现了Drop trait这个接口，那么rust是不允许让他再去实现Copy trait了

    //拥有copy trait的类型：
    //（1）任何简单标量的组合类型都可以是Copy的
    //（2）任何需要分配内存或某种资源的不是copy的
    //拥有copy trait的类型：
    // ①所有的整数类型，例如：u32
    // ②bool类型；
    // ③char字符类型；
    // ④所有的浮点类型，例如：f64；
    // ⑤元组类型（Tuple），如果其所有字段都是copy的，（i32，i32）是，（i32，string）不是

    //所有权函数案例
    let w1 = String::from("My name is WJD");
    take_ownership(w1);

    let w2: i32 = 5;
    makes_copy(w2);

    println!("w2:{}",w2);


let g1 = gives_ownerships();

let g2 = String::from("hello");

let g3 = takes_and_gives_back(g2);

println!("{}{}",g1,g3);


let t1 = String::from("hello");
let t2 = calculate_length(&t1);
print!("t1:{}",t1);
println!("t1字符长度为:{}",t2);
//&是指针，成为借用，用来 借用 别的值，被借用的值不会发生移动，所有权不改变

// 如果修改借用内容，需增加mut，说明指针可变
let mut w1 = String::from("hello");
let w2 = calculate_length1(&mut w1);
print!("w1:{}",w1);
println!("w1字符长度为:{}",w2);



}

fn take_ownership(some_string: String) {
    println!("some_string:{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("some_number:{}", some_number);
}

 fn gives_ownerships() -> String{
    let some_string = String::from("hello");
    some_string
 }

 fn takes_and_gives_back(a_string: String)-> String{
    a_string
 }

 fn calculate_length(w: &String)-> usize {
    w.len()
}
 

 fn calculate_length1(w: &mut String)-> usize {
    w.push_str(",哈哈");
    w.len()

}
     