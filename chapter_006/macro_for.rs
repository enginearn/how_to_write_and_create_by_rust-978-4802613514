macro_rules! easy_for {
    (
        for $i:ident = $from:tt to $to:tt
            $body:block
    ) => {{
        for $i in $from..$to {
            $body
        }
    }};

    (
        for $i:ident = $from:tt to $to:tt step $step:tt
            $body:block
    ) => {{
        let mut $i = $from;
        while $i < $to {
            $body
            $i += $step;
        }
    }};
}

fn main() {
    let mut total = 0;
    easy_for! {
        for i = 0 to 10 {
            total += i;
        }
    }
    println!("total: {}", total);

    total = 0;

    easy_for! {
        for i = 0 to 10 step 2 {
            total += i;
        }
    }
    println!("total: {}", total);
}
