#[test]
fn iterator_demonstations() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    //sum을 호출한 이후에는 v1_iter의 사용이 허용되지 않는다. -> sum은 반복자를 소유하여 호출한다. 
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

/*
반복자에 대한 next 메서드 호출은 반복자 내부의 상태를 변경하여 반복자가 현재 시쿼스의 어디에 있는지 추적한다. 이 코드는 반복자를 소비(consume), 즉 다 써 버린다.  -> 불변 참조자
    -> next를 호출하는 메서드들을 소비 어댑터(consuming adaptor) 라고 한다. 
iter 메서드는 불변참조자에 대한 반복자를 생성 만약 소유권을 얻어서 소유한 값을 반환하고 싶으면 into_iter를 호출
가변 참조자에 대한 반복자는 iter_mut를 호출 
*/