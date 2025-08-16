use std::io;
use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

//화씨, 섭씨 온도 변환
pub fn chg_f_c() {
    println!("Which one do you want to change? Celsius or Fahrenheit");
    println!("Please input the word that you want to change");

    const D32: Decimal = dec!(32.0);
    const D5: Decimal = dec!(5.0);
    const D9: Decimal = dec!(9.0);
    let mut tem = String::new();

    io::stdin().read_line(&mut tem).expect("Failed to read line");

    if tem.trim() == "Celsius" {
        let mut cel = String::new();
        let fah: Decimal;

        loop {
            cel.clear();
            println!("Please input Celsius");

            io::stdin().read_line(&mut cel).expect("Failed to read line");

            let cel: Decimal = match Decimal::from_str(cel.trim()) {
                Ok(num) => num,
                Err(_) => {
                    println!("you have to input number. : {cel}"); 
                    continue
                }
            };
            fah = (cel*D9/D5+D32).round_dp(1);
            break;
        }
        
        println!("The Fahrenheit is {fah}°F");
    }else if tem.trim() == "Fahrenheit" {
        let mut fah = String::new();
        let cel: Decimal;

        loop {
            fah.clear();
            println!("Please input Fahrenheit");

            io::stdin().read_line(&mut fah).expect("Failed to read line");

            let fah: Decimal = match Decimal::from_str(fah.trim()) {
                Ok(num) => num,
                Err(_) => {
                    println!("you have to input number.");
                    continue
                }
            };
            cel = ((fah-D32)*D5/D9).round_dp(1);
            break;
        }

        println!("The Celsius is {cel}°C");

    }else{
        println!("You input the wrong word.")
    }

}
