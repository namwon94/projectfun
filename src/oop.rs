//캡슐화 (encapsulation) 예제
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

/*
러스트에서 벡터의 제약사항 중 하나는 하나의 타입에 대한 요소만 저장할 수 있다. 하지만 때로는 여러 유효한 타입의 요소를 저장하게 확장하길 원하는 때가 있다.
객체지향에서 '상속'이라는 작업으로 변형을 할 수 있는데 러스트에는 상속이 없다.
그래서 해당 트레이트는 사용자들이 새로운 타입을 정의하고 확장할 수 있도록 라이브러리를 구조화하는 방법의 예제이다.
*/
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        //실제 버튼을 그리는 코드
        println!("Button drawn");
    }
}

/*
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> where T: Draw, {
    /*
    트레이트 바운드가 있는 제네릭 타입 매개변수를 사용하는 구조체를 정의하는 것과는 다르게 작동한다.
    제네릭 타입 매개변수는 한 번에 하나의 구체 타입으로만 대입될 수 있는 반면, 트레이트 객체를 사용하면 런타임에 트레이트 객체에 대해 여러 구체 타입을 넣을 수 있다.

    이렇게 되면 동일 타입의 컴포넌트의 목록을 가진 Scrren 인스턴스로 제한된다.
        -> 동일 타입의 컬렉션만 사용한다면 제네릭과 트레이트 바운드를 사용하는 것이 바람직 하다.
            -> 그 이유는 정의들은 컴파일 타임에 단형성화(monomorphize)되어 구체 타입으로 사용되기 떄문이다.
    반면 트레이트 객체를 사용하는 메서드를 이용할 경우, 하나의 Screen 인스턴스가 Box<Button> 은 물론 Box<TextField>도 담을 수 있는 Vec<T>를 보유할 수 있다.
    */
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
*/
