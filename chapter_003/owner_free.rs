fn main() {
    {
        let s1 = String::from("真実はワインの中にある");
        let s2 = String::from("葡萄畑と美人は手がかかる");

        {
            let s3 = s1;
            println!("s3: {}", s3);
        }
        println!("s2: {}", s2);
        // println!("s3: {}", s3);
    }
    // let s4 = s3.clone();
    // println!("s4: {}", s3);
}
