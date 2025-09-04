use std::ops::Deref;

/* 
재귀적 타입을 사용할때 Box<T>를 사용
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::Cons;
*/

pub fn smart_pointer() {
    let b = Box::new(5);

    println!("b = {}", b);

    //let list =  List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    let x = 5;
    //참조자 대신 Box<T>를 사용하여 작성할 수 있다. -> 참조자와의 차이점은 y에 x의 값을 가리키는 참조자가 아닌 x의 복제된 값을 가리키는 Box<T>의 인스턴스를 설정.
    let y = Box::new(x);
    let z = MyBox::new(x);

    assert_eq!(5, x);
    //y의 값을 단언하고 싶다면 *y를 사용하여 참조자를 따라가서 이 참조자가 가리키고 있는 값을 언어내여(역참조) 컴파일러가 실제 값을 비교할 수 있도록 해야한다.
    assert_eq!(5, *y);
    //assert_eq!(5, *z); -> Deref 트레이트 없이 역참조 시 에러남
    assert_eq!(5, *(z.deref()));

    let m = MyBox::new(String::from("Rust"));
    //Deref 트레이트 구현으로 &String -> &str로 역참조 강제 변환을 하기 때문에 hello(&(*m)[..])으로 코드를 작성을 안해도 된다.
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
/*
Deref 트레이트가 없으면 컴파일러는 오직 &참조자들만 역참조할 수 있다. 해당 메서드는 컴파일러가 Deref를 구현한 어떤 타입의 값에 대해 deref 메서드를 호출하여, 자신이 역참조하는 방법을 알고있는
&참조자를 가져올 수 있는 기능을 제공한다. 
*/
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}