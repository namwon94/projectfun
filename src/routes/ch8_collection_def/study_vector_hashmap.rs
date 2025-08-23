use std::collections::HashMap;
use rand::Rng;

//정수 리스트가 주어졌을 때, 벡터를 이용하여 이 리스트의 중간 값(meduan, 정렬했을 때 가장 가운데 위치한 값) 반하하기
pub fn vec_study(num: usize) -> f64 {
    let length: usize = num;
    let mut rng = rand::thread_rng();
    let mut vlst: Vec<u32> = (0..length).map(|_| rng.gen_range(1..101)).collect();
    let meduan: f64;
    vlst.sort_unstable();
    println!("vlst : {:?}", vlst);

    if vlst.len() % 2 == 0 {
        let mid = vlst.len()/2;
        meduan = ((vlst[mid-1] + vlst[mid] ) as f64) / 2.0;
    }else {
        meduan = vlst[vlst.len()/2] as f64;
    }
    meduan
}

//최빈값(mode, 가장 많이 발생한 값; 해시맵이 여기서 도움이 된다) 반환하기.
pub fn hash_study(num: usize) {
    let length: usize = num;
    let mut rng = rand::thread_rng();
    let mut vlst: Vec<u32> = (0..length).map(|_| rng.gen_range(1..11)).collect();
    let mut map = HashMap::new();
    vlst.sort_unstable();
    println!("vlst: {:?}", vlst);

    for number in vlst {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    println!("The map is {:?}", map);

    let mut max_count = 0;
    let mut mode = 0;

    for (&number, &count) in &map {
        if count > max_count {
            max_count = count;
            mode = number;
        }
    }

    println!("The mode is {mode} and count is {max_count}");
}