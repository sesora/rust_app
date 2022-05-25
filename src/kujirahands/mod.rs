use rand::Rng;
use std::collections::HashMap;

pub fn fizz_buzz() {
    for i in 1..101 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

pub fn chapter1_59() {
    for i in 1..51 {
        if i % 3 == 0 || i % 10 == 3 || i / 10 == 3 {
            println!("ABCe");
        } else {
            println!("{}", i);
        }
    }
}

pub fn kuku() {
    for y in 1..10 {
        let s = (1..10)
            .map(|x| format!("{:3}", x * y))
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", s);
    }
}

pub fn seireki_wareki() {
    let start_year = 1926;
    for i in 0..100 {
        let target_year = start_year + i;
        if target_year > 1925 && target_year < 1989 {
            show_s_w_year(target_year, 1926, "昭和");
        } else if target_year > 1988 && target_year < 2019 {
            show_s_w_year(target_year, 1989, "平成");
        } else if target_year > 2018 {
            show_s_w_year(target_year, 2019, "令和");
        }
    }
}

fn show_s_w_year(target_year: i32, start_year: i32, wareki_str: &str) {
    let gannen = "元";
    let wa_year = target_year - start_year + 1;
    let wa_year = if wa_year == 1 {
        gannen.to_string()
    } else {
        wa_year.to_string()
    };
    println!("{}年 => {}{}年", target_year, wareki_str, wa_year);
}

pub fn fib() {
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);
    for _ in 0..30 {
        println!("{}", a + b);
        let tmp = a;
        a = b;
        b = tmp + a;
    }
}

pub fn rei1() {
    let price = 98000.0;
    let a_price = price * 0.8 + 1200.0;
    let b_price = price * 0.9;
    println!("A社:{}円, B社:{}円", a_price, b_price);
    if a_price > b_price {
        println!("A社の方が高い")
    } else {
        println!("B社の方が高い")
    }
}

pub fn coin() {
    let recipt_fee = 3950;
    for num_500 in 0..11 {
        for num_100 in 0..4 {
            for num_50 in 0..11 {
                let sum_fee = 500 * num_500 + 100 * num_100 + 50 * num_50;
                if sum_fee == recipt_fee {
                    println!(
                        "500円玉:{}枚 100円玉:{}枚 50円玉:{}枚, 合計{}円",
                        num_500, num_100, num_50, sum_fee
                    );
                }
            }
        }
    }
}

pub fn prime_number() {
    let mut primes = [0; 100];
    let mut i = 2;
    let mut count = 0;
    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1
    }
    println!("{:?}", primes);
}

fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn dice() {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let dice = rng.gen_range(1..=6);
        println!("{}", dice);
    }
}

pub fn maze() {
    const MAP_N: usize = 25;
    let mut rng = rand::thread_rng();
    let mut maze = [[0; MAP_N]; MAP_N];
    for n in 0..MAP_N {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAP_N - 1] = 1;
        maze[MAP_N - 1][n] = 1;
    }
    for y in 2..MAP_N - 2 {
        for x in 2..MAP_N - 2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            maze[y][x] = 1;
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y - 1][x] = 1,
                1 => maze[y + 1][x] = 1,
                2 => maze[y][x - 1] = 1,
                3 => maze[y][x + 1] = 1,
                _ => {}
            }
        }
    }
    let tiles = ["  ", "ZZ"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]])
        }
        println!("");
    }
}

pub fn counter() {
    const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C";
    let mut c_map = HashMap::new();
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);
    for w in V_DATA.split(",") {
        c_map.insert(w, c_map[w] + 1);
    }
    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, c_map[k]);
    }
}
