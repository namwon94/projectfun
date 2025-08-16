pub fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    /*
    iter() : 컬렉션의 각 요소를 반환한다.
    enumerate() : iter의 각 결과값을 튜플로 감싸 반환한다. / 반환하는 튜플은 첫번째 요소가 인덱스, 두번째 요소가 해당 요소의 참조자로 이루어짐
     */
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}