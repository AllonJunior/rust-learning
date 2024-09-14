fn main() {
    println!("Repetiton practice!");

    /*
    // A loop with returned value
    let mut counter = 1;
    let result = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter *2;
        }
    };
    println!("The result is {result}");
     */

    // Labeled Loop
    /*
    let mut count = 0;
    'count_loop: loop{
        println!("count: {count}");
        let mut remaining = 10;
        loop{
            println!("remaining: {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'count_loop;
            }

            remaining -= 1;
        };
        count += 1;
    };
    println!("End count: {count}");
    */
    // Using for
    // retrieve an array
    let numbers = [1, 2, 3, 4, 5, 6];
    for elm in numbers {
        println!("number: {elm}");
    };

    // repetition with a range
    for elm in 1..5 {
        println!("number in range: {elm}");
    }
    
}
