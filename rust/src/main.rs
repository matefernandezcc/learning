fn mostrar(vector: &[i32]) {
    print!("[");
    for (i, elemento) in vector.iter().enumerate() {
        if i < vector.len() - 1 {
            print!("{},", elemento);
        } else {
            print!("{}", elemento);
        }
    }
    println!("]");
}

fn main() {
   for i in 0..10{
    print!("{} ",i);
   }
   println!("");
}
