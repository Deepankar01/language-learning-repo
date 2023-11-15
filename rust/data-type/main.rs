// +------------------------+--------+----------+
// | Length                 | Signed | Unsigned |
// +------------------------+--------+----------+
// | 8-bit                  | i8     | u8       |
// +------------------------+--------+----------+
// | 16-bit                 | i16    | u16      |
// +------------------------+--------+----------+
// | 32-bit                 | i32    | u32      |
// +------------------------+--------+----------+
// | 64-bit                 | i64    | u64      |
// +------------------------+--------+----------+
// | 128-bit                | i128   | u128     |
// +------------------------+--------+----------+
// | architecture-dependent | isize  | usize    |
// +------------------------+--------+----------+

// Rust has two floating-point data types for decimal values: f32 (32 bits) and f64 (64 bits).
// The default floating-point type is f64.
// On modern CPUs, the f64 type is roughly the same speed as the f32 type, but it has greater precision.

// Some languages treat their char types as 8-bit unsigned integers, which is the equivalent of the Rust u8 type.
// The char type in Rust contains unicode code points, but they don't use UTF-8 encoding.
// A char in Rust is a 21-bit integer that's padded to be 32 bits wide. The char contains the plain code point value directly.
// You can learn more about the char type in Rust in the Rust documentation.

// Classic struct with named fields
struct Student {
    name: String,
    level: u8,
    remote: bool,
}

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

// Unit struct
struct Unit;

fn main() {
    let number: u32 = 14;
    let number_64 = 4.0; // compiler infers the value to use the default type f64
    let number_32: f32 = 5.0; // type f32 specified via annotation
                              // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2, // 1+2 if not specified rust picks up the type from context and if ambigious then picks i32
        8i32 - 5,
        15 * 3
    );

    // Declare a tuple of three elements
    let tuple_e = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!(
        "Is '{}' the {}th letter of the alphabet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
    println!("{}", number);

    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student {
        name: String::from("Constance Sharma"), // Structs always needs String type cannot have &str type
        remote: true,
        level: 2,
    };
    let user_2 = Student {
        name: String::from("Dyson Tan"),
        level: 5,
        remote: false,
    };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );
    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );
}
