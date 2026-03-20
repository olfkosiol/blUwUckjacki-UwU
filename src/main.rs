use std::io;

fn main() {



    println!("gib den erste wert ein");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("failed to readline");
    let kartenwert1 = y.trim();
    let kartenwert1:i32 = kartenwert1.parse().unwrap();

    println!("gib den erste wert ein");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("failed to readline");
    let kartenwert2 = y.trim();
    let kartenwert2:i32 = kartenwert2.parse().unwrap();


    let x1 = kartenwert2 + kartenwert1;

    if x1 >= 16 {
        println!("hold")

    }
    else {
        println!("redraw")
    }


}
