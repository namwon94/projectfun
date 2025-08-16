/*
열거형 표현 방식
1. 밑에 하단 처럼 정의하는 방식
2. 각 열거형 배리언트에 데이터를 직접 넣는 방식 -> 열겨형을 구조체의 일부로 사용하는 방식보다 더 간격하게 동일한 개면을 표현할 수 있다. (구조체를 사용할 필요가 없다.)
    ex. V4(String) / let home IpAddr::V4(String::from("127.0.0.1"));
3. 각 배리언트는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있다.
    ex. V4(u8, u8, u8, u8) / V6(String)
*/
#[derive(Debug)]
enum IpAddrKind {
    //열거형의 배리언트(variant)
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn enums() {
    //열거형을 정의할 떄의 식별자로 '네임스페이스'가 만들어져서, 각 배리언트 앞에 이중 콜록(::)을 붙여야 한다.
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //address는 String - String은 Display 트레이트 구현되어있음
    println!("hoem.kind : {:?}, home.address : {}, loopback.kind : {:?}, loopback.address : {}", home.kind, home.address, loopback.kind, loopback.address);
}