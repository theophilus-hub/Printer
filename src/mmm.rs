pub fn letter(m:i32, n:i32) {
    let mut pattern: String = String::from("");
    for _i in 0..n {
        pattern.push_str("*  ");
    }

    let mut pattern1: String = String::from("");
    for i1 in 0..n {
        if ( i1 < 2 || i1 > 13 ) || (i1 > 3 && i1 < 12 ){
            pattern1.push_str("*  ");
        }else{
            pattern1.push_str("   ");
        }
    }

    println!("\n");
    for j in 0..m {
        if j < 2 || j > 13 {
            println!("\t\t\t{}",pattern );
        }else{
            println!("\t\t\t{}",pattern1);

        }
    }
    println!("\n");
}