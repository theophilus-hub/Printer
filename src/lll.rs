//for printing the L in a 15 x 15 box
pub fn letter(m:i32, n:i32) {

// for the border of the box
    let mut pattern: String = String::from("");
    for _i in 0..n {
        pattern.push_str("*  ");
    }

 // for the vertical part of L
    let mut pattern1: String = String::from("");
    for i1 in 0..n {
        if i1 > 1 && i1 < 4 {
            pattern1.push_str("   ");
        }else{
            pattern1.push_str("*  ");
        }
    }

// for the horizontal part of L
let mut pattern2: String = String::from("");
    for i2 in 0..n {
        if i2 > 1 && i2 < 11 {
            pattern2.push_str("   ");
        }else{
            pattern2.push_str("*  ");
        }
    }

    
    println!("\n");
    for j in 0..m {
        if j < 2 || j > 13{
            println!("\t\t\t{}",pattern );
        }else if j < 12 {
            println!("\t\t\t{}",pattern1 );
        }else{
            println!("\t\t\t{}",pattern2 );
        }
    }
    println!("\n");
}