pub fn sq(m:i32, n:i32) {
    let mut pattern: String = String::from("");
    for _i in 0..n {

        pattern.push_str("*  ");
    }
    println!("\n");
    for _j in 0..m {
        println!("\t\t\t{}",pattern );
    }
    println!("\n");
}