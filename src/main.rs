// "Write a program that prints the numbers from 1 to 100. But for multiples of three print “Fizz”
// instead of the number and for the multiples of five print “Buzz”.
// For numbers which are multiples of both three and five print “FizzBuzz”."

fn main() {
    for x in 0..100 {
        match x {
            _ if x % 3 == 0 && x % 5 == 0 => println!("FizzBuzz"),
            _ if x % 3 == 0 => println!("Fizz"),
            _ if x % 5 == 0 => println!("Buzz"),
            number => println!("{}", number)
        }

    }
}
