extern crate rand;

use rand::{thread_rng,Rng};

fn main()
{
// create a array of 7 numbers to hold the lottery row, EuroLotto allows the first 5 numbers to be between 1-50 so a i8 will do. Since i8 is the smallest number primitive in rust it will also do for the next 2 numbers that can be between 1-9
    let mut row:[i8; 7] = [0; 7];

// create a thread the random number generator runs in
    let mut rng = thread_rng();

// create the first 5 numbers in a range between 1-50, since it is not inclusive on the upper bound 51 is sent to the generator thread
    let mut number;
    for i in 0..5
    {
        'first_five: loop
        {
            number = rng.gen_range(1i8, 51i8);
            if check_unique(number, &row[0..i])
            {
                row[i] = number;
                break 'first_five;
            }
        }
    }

// the same as the previous loop only between 1-9 since those are the values the last 2 numbers in the EuroLotto lottery row allows
    for i in 5..7
    {
        'last_two: loop
        {
            number = rng.gen_range(1i8, 10i8);
            if check_unique(number, &row[5..i])
            {
                row[i] = number;
                break 'last_two;
            }
        }
    }

// printout the generated row, since a understandable formatting of the array is nice I opted to use the debug (:?) option instead of adding all those [ , , , ] symbols myself
    println!("{:?}", row);
}

fn check_unique(number: i8, slice: &[i8])
    -> bool
{
    //check stuff in for-loop
    for &value in slice
    {
        if value == number
        {return false;}
    }
    true
}
