fn main()
{
        println!("{}", num_chars(-10));
}

fn num_chars(number: i64) -> int
{
        let mut curr_number = number;
        let mut num_chars: int = 0;

        if curr_number < 0
        {
                num_chars = 1;
        }

        while curr_number != 0
        {
                curr_number /= 10;
                num_chars += 1;
        }

        num_chars
}

