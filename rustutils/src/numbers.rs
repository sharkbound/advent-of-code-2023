use paste::paste;
macro_rules! create_prime_function {
    ($int:ty, $float:ty) => {
        paste! {
            pub fn [<is_prime_ $int>](n: $int) -> bool {
                if n <= 2 { return false; }
                (2..((n as $float).sqrt() as $int)).all(|x| n % x != 0)
            }
        }
    };
}

create_prime_function!(u32, f32);
create_prime_function!(i32, f32);
create_prime_function!(u64, f64);
create_prime_function!(i64, f64);
