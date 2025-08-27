use std::cmp::PartialOrd;

pub fn gen_largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
pub fn gen_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//러스트에서 타입 이름을 지어줄 때는 대문자로 시작하는 낙타 표기법(UpperCamelCase)을 따르고, 타입 매개변수의 이름은 짧게(한 글자로만 된 경우도 있음) 짓는 것이 관례 
pub fn gen_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        /*
        item > largest에서 에러나는 이유 : 함수 본문에서 T 타입 값들에 대한 비교가 필요하므로 여기에는 값을 정렬 할 수 있는 타입에 대해서만 동작할 수 있다.
            -> 비고가 가능하도록 하기 위해, 표준 라이브러리는 임의의 타입에 대해 구현 가능한 std::cmp::PartialOrd트레이트를 제공한다.
        */
        if item > largest {
            largest = item;
        }
    }
    largest
}

//서로 다른 타입의 값들을 정의를 하고싶다면 여러 개의 제네릭 타입 매개변수를 사용해야 한다.
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

//제네릭 매개변수 중 일부가 impl에 선언되고 일부는 메서드 정의에 선언되는 경우를 보여주기 위한 예제
struct Point2<X1, Y1> {
    x: X1,
    y: Y1
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 { x: self.x, y: other.y }
    }
}

pub fn gen_method_def() {
    let p = Point {x: 5, y:10};

    println!("p.x = {}, p.y = {}", p.x(), p.y());

    let p1 = Point2{x:5, y:10.4};
    let p2 = Point2{x:"Hello", y:'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}