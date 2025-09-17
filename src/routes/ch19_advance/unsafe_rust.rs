use std::slice;

/*
원시 포인터(raw pointer)는 불변 또는 가변이다. 각각 *const T, *mut T로 작성된다. 별표는 역참조 연산자가 아니라 타입 이름의 일부이다.
    -> 같은 위치에 대해 불변과 가변 포인터를 동시에 가질수 있고, 여러 개의 가변 포인터를 가질 수 있다.
    -> 유효한 메모리를 가리키는 것을 보장받지 못한다.
    -> 널이 될수 있다.
    -> 자동 메모리 정리를 구현하지 않는다.
원시 포인터를 사용하는 이유 : C코드와 상호작용
*/
pub fn raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
}

pub fn unsafe_method(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/*
extern 함수를 사용하면 외부 코드를 호출할 수 있다.
*/

/*
add 트레이트 예시
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
*/