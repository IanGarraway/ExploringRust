fn main() {
    print_phase("Dave");

    println!("{}", gcd(20, 4));
    
}
//naming convention for Rust is snake_casing for functions
fn print_phase(phrase: &str){
    println!("Hello from the function! {}", phrase);

}

fn gcd(mut a: u64, mut b: u64) -> u64{
    while a != 0 {
        if a< b{
            let c = a;
            a = b;
            b = c;
        }
        a=a%b;
    }
    b
}
