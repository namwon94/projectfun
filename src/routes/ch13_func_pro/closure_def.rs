use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

/*
self.most_stocked : 클로저 표현식 / 클로저가 매개변수를 갖는 다면 || 사이에 매개변수가 나올 것이다. 
*/
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        }else {
            ShirtColor::Blue
        }
    }
}

pub fn closure() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    //불변참조자 빌려오기
    let list = vec![1, 2, 3];
    println!("Before defining closure : {:?}", list);

    let only_borrows = || println!("From closure : {:?}", list);

    println!("Before calling closure : {:?}", list);
    only_borrows();
    println!("After calling closure : {:?}", list);

    //가변참조자 빌려오기
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure : {:?}", list2);

    let mut borrows_mutably = || list2.push(7);

    //클로저 정의와 호출 사이에는 출력을 위한 불변 대여가 허용되지 않는데, 이는 가변 대여가 있을 떄는  다른 대여가 허용되지 않기 때문이다.
    //println!("Before defining closure : {:?}", list2);
    borrows_mutably();
    println!("After calling closure : {:?}", list2);

    //클로저의 본문에서 사용하고 있는 값의 소유권이 필요하진 않더라도 클로저가 소유권을 갖도록 만들 때 'move' 키워드 사용
    let list3 = vec![1, 2, 3];
    println!("Before defining closure : {:?}", list3);

    thread::spawn(move || println!("From thread : {:?}", list3)).join().unwrap();
}