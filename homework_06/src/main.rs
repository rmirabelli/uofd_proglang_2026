fn angle_of_hour(hours: i32, minutes: i32) -> i32 {
    (hours % 12) * 30 + (minutes % 60) / 5
}

fn angle_of_minutes(minutes: i32) -> i32 {
    (minutes % 60)  * 6
}

fn angle_of_clock(hours: i32, minutes: i32) -> i32 {
    i32::abs(angle_of_hour(hours, minutes) - angle_of_minutes(minutes))
}

fn main() {
    println!("Angle of clock at 2:15 is {}", angle_of_clock(2, 15));
    println!("Angle of clock at 12:15 is {}", angle_of_clock(12, 15));
    println!("Angle of clock at 12:00 is {}", angle_of_clock(12, 0));
    println!("Angle of clock at 6:00 is {}", angle_of_clock(6, 0));
    println!("Angle of clock at 10:10 is {}", angle_of_clock(10, 10));
}
