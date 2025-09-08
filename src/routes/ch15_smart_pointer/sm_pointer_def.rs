use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

//재귀적 타입을 사용할때 Box<T>를 사용
#[allow(unused)] 
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};


pub fn smart_pointer() {
    //Box<T> 및 Dref 트레이트 얘제
    let bb = Box::new(5);

    println!("bb = {}", bb);

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

    //Drop 트레이트 예제
    drop_def();

    //Rc<T> 예제
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    //참조 카운트를 감소시키기 위해 어떤 함수를 호출할 필요는 없다. Rc<T>값이 스코프 밖으로 벗어나면 Drop 트레이트의 구현체가 자동으로 참조 카운트를 감소시킨다.
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    rc_change_weak();
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

//smart pointer : Box<T> 사용
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

struct CustomSmartPointer {
    data: String,
}

/*
Drop 트레이트는 프렐루드에 포함되어 있으므로, 이를 스코프로 가져올 필요는 없다.
drop 함수의 본문에는 해당 타입의 인스턴스가 스코프 밖으로 벗어났을 때 실행시키고 싶은 어떠한 로직이라도 집어넣을 수 있다. 
drop을 명시적으로 호출하는 것이 허용되지 않음
    -> 러스트가 여전히 main의 끝부분에서 그 값에 대한 drop 호출을 자동으로 할 것이기 때문이다. 그래서 중복해제(double free) 에러가 될 수 있다. 
    -> 어떤 값에 대한 메모리를 강제로 일찍 하기 원할 때는 std::mem::drop 함수를 이용한다. ->.  이 함수는 프렐루드에 포함되어 있다. Drop 트레이트에 있는 drop 메서드와는 다른 함수.
*/
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn drop_def() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointers created.");
    drop(e);
    println!("CustomSmartPointer dropped befor the end of main.");
}

//순환 참조 방지하기 : Rc<T> -> Weak<T> 변환
//트리 데이터 구조 만들기 : 자식 노드를 가진 Node / 자식에서 부모로 가는 참조자 추가하기
#[derive(Debug)]
#[allow(unused)] 
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn rc_change_weak() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}