fn main() {
    println!("Number of bits which are equal to one: {}", count_bits(10));
    // println!("The unreversed binary value is: {:?}", binary);
    // println!("The number of bits which equal 1: {}", count);
}


fn count_bits(mut n: i64) -> u32 {
    let mut count = 0;

    let mut binary = vec![];
    while n > 0 {
        binary.push(n % 2);
        n = n / 2;
    }
    
    for num in binary.iter() {
        if *num == 1 {
            count = count + 1;
        }
    }
    return count;     
}