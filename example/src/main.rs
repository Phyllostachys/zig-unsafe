zig_unsafe::zig_unsafe! {
    pub export fn is_leap_year(year: u32) bool {
        if (year % 4 == 0) {
            if (year % 100 == 0) {
                if (year % 400 == 0) {
                    return true;
                }
                return false;
            }
            return true;
        }
        return false;
    }
}

// need this until we get a Zig function def syntax parser up and running
extern "C" {
    fn is_leap_year(year: u32) -> bool;
}

fn main() {
    let result = unsafe { is_leap_year(2024) };
    println!("is_leap_year(2024) = {}", result);
}
