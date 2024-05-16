// #[derive(Debug)]
// enum NationalHolidays {
//     GandhiJayanti,
//     RepublicDay,
//     IndependenceDay,
// }

// fn inspect(day: NationalHolidays) -> String {
//     match day {
//         NationalHolidays::GandhiJayanti => String::from("Oct 2"),
//         NationalHolidays::RepublicDay => String::from("Jan 26"),
//         NationalHolidays::IndependenceDay => String::from("Aug 15"),
//     }
// }

// fn main() {
//     let day = NationalHolidays::GandhiJayanti;
//     let date = inspect(day);
//     println!("{:?}", date);
// }

#[derive(Debug)]
enum NationalHolidays {
    GandhiJayanti,
    RepublicDay,
    IndependenceDay,
}

trait CalendarAssistant {
    fn inspect(&self) -> String;
}

impl CalendarAssistant for NationalHolidays {
    fn inspect(&self) -> String {
        match self {
            NationalHolidays::GandhiJayanti => String::from("Oct 2"),
            NationalHolidays::RepublicDay => String::from("Jan 26"),
            NationalHolidays::IndependenceDay => String::from("Aug 15"),
        }
    }
}

fn main() {
    let day = NationalHolidays::GandhiJayanti;
    let date = day.inspect();
    println!("{:?}", date);
}
