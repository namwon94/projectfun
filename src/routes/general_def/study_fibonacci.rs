//재귀함수로 이용한 피보나치 수열
pub fn fibonacci(num: u32) -> u32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    //_의 의미 : 루프변수를 사용하지 않겠다. 여기서는 'num'의 값을 변수로 사용하지 않는다.
    for _ in 2..=num {
        c = a + b;
        a = b;
        b = c;
    }

    c
}

//동적계획법을 이용한 피보나치 수열
pub fn fibonacci_memo(num:usize, memo: &mut Vec<u64>) -> u64 {
    if num == 0 {
        return 0
    }else if num == 1 {
        return 1
    }

    if memo[num] != 0 {
        //이미 계산된 값이 있으면 반환
        return memo[num]
    }

    //이전 두 값을 재귀 호출로 계산하고 저장
    memo[num] = fibonacci_memo(num-1, memo) + fibonacci_memo(num-2, memo);
    memo[num]
}