#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

//Rectangle의 컨택스트에 함수를 정의하기 위해서, Rectangle에 대한 impl(implementation, 구현) 블록을 만드는 것으로 시작(블록 내의 모든 것은 Rectangle 타입과 연괸됨)
//impl블록 내에 구현된 모든 함수를 연관함수(associated function)라고 부른다. -> impl 뒤에 나오는 타입과 모두 연관된 함수이기 때문이다.
//각 구조체는 여러개의 impl블록을 가질 수 있다.
impl Rectangle {
    //&self는 self:&Self의 줄임말 / self는 impl블록의 대상이 되는 타입의 별칭
    /*
    &self를 선택한 이유는 기존의 &Rectangle을 사용했던 이유와 같다 -> 소유권을 가져오는 것 혹은 데이터를 쓰는 것도 아닌 데이터를 읽는 것 있기 때문이다. 
        -> 만약 메서드에서 작업 중 호출한 인스턴스를 변경하고 싶다면 첫 번째 매개변수로 '&mut self'를 사용하면 된다. (&self 라고만 작성해서 인스턴스의 소유권을 가져오는 일을 거의 없음)
        -> 이러한 기법은 self를 다른 무언가로 변환하고 그 이후에는 원본 인스턴스의 사용을 막고자 할 때 사용한다.
    */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn rectangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width:60,
        height:45,
    };

    let rect4 = Rectangle::square(4);

    println!("rect1_:? is {:?}", rect1);
    println!("rect1_:#? is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    //rect1.width 뒤에 괄호를 붙이면 러스트는 width 메서드를 의도한다는 것을 인지한다. 괄호를 사용하지 않으면 width 필드를 의미한다는 것으로 본다.
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("square pixels : {:?}", rect4);
}