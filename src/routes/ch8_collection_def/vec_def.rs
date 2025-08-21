
/*
타입 명시(type annotation) : 밑에 선언한 'Vec<i32>'를 참조 / 벡터는 어떠한 타입의 값이라도 저장할 수 있다. / 반면에 밑에 처럼 선언시 특정한 타입의 값을 저장할 수 있다.
    -> 대부분의 경우 초기값과 함께 벡터를 생성하고 러스트는 저장하고자 하는 타입을 유추할 수 있기 때문에 타입 명식를 할 필요가 거의 없다.
벡터는 모든 요소가 서로 붙어서 메모리에 저장된다. 그리고 새로운 요소를 벡터 끝에 추가할 경우, 현재 벡터 메로리 위치에 새로운 요소를 추가할 공간이 없다면, 
다른 넉넉한 곳에 메모리를 새로 할당하고 기존 요소를 새로 할당한 공간에 복한다. 만약 기존 요소의 참조자가 있는 경우 참조자는 해제된 메모리를 가리키게 되기 때문에, 에러가 나게 된다.
그래서 이러한 상황을 '대여 규칙'으로 막아두었다.
*/
pub fn vecs() {
    let v: Vec<i32> = Vec::new();
    //러스트는 편의를 위해 vec! 매크로를 제공한다.
    let v2 = vec![1, 2, 3];
    //push 메서드 사용가능 / 타입 명시를 붙이지 않아도 되는 이유는 러스트가 push한 값들의 타입을 통해 v3의 타입을 추론하기 때문이다.
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    //`Vec<i32>` doesn't implement `std::fmt::Display` 트레이트가 구현이 안되어 있음 Debug 트레이트만 구현되어 있다.
    println!("v : {:?}, v2 : {:?}, v3 : {:?}",v, v2, v3);

    let third: &i32 = &v2[2];
    println!("The third(&i32) element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third(option<&i32>) element is {third}"),
        None => println!("There is no third element")
    }

    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("v4 : {i}");
    }

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50;
    }
    println!("v5 : {:?}", v5);

}