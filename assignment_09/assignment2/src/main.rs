fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
fn main() {
    let mut n = 9;
    let mut sum = 0;
    let integers: [i32; 10] = [23, 18, 11, 22, 21, 27, 12, 60, 40, 504];
    for num in integers.iter() {
        if is_even(*num) == true {
            println!("Number {} is even!", num);
        } else {
            println!("Number {} is odd!", num);
        }
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }
    while (n >= 0) {
        sum = sum + integers[n];
        if n == 0 {
            break;
        }
        n = n - 1;
    }
    println!("Sum: {}", sum);
    let mut largest_number = integers[0];
    let mut i = 0;
    loop {
        i += 1;
        if i == integers.len() {
            break;
        }
        if integers[i] > largest_number {
            largest_number = integers[i];
        }
    }
    println!("Largest number: {}", largest_number);
}
