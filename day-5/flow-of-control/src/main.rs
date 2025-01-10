#[allow(dead_code)]
fn main() {

    //ifnelse();
    //loops();
    //returning_loops();
    //array_destructure();
    match_keyword();
}

fn ifnelse(){
    let n=5;
    if n>0 {
        print!("n is bigger than 0");
    }else{
        print!("n is less than 0");
    }

    let bigger_n=
        if n>3 {
            println!(" and smaller than 3, so multiplied by 3");
            n*3
        }else{
            println!(" and bigger than 3, so multiplied by 2 ");
            n*2
        };
    // : is required in the end of let 
    println!("n= {} => {}",n,bigger_n);
}

fn loops(){
    // you can lable the loop to distinguish it '_label_:loop{}
    'outer:loop{
        println!("outer loop");
        'inner:loop{
            println!("inner loop");
            // to break inner loop
            //break;

            break 'outer;
            // you can select which loop to exit by calling the label of the loop
        }
       // println!("This wont get prinited");
    }
  println!("Exited the outer loop");
}

fn returning_loops(){
     let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // value after break is returned
            break counter * 2;
        }
    };

   println!("{}",result);
    //assert_eq!(result, 20);
}

fn while_loop(){
     // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}

fn for_in_loop(){
    //  for n in 1..=100 
    //  above is for loop from 1<=n<=100
    //  below is for loop from 1<n<100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn array_destructure(){
    let array = [1, -2, 6];
    println!("array: {:?}",array);
    let [a,b,c]=array;
    println!("{},{},{}",a,b,c);
}


fn match_keyword(){
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

fn guards(number:u32){
    
    match number{
        n if n%2 == 0=> println!("{n} is even.");
        _=> println!("{n} is odd.");
    }
    
}
