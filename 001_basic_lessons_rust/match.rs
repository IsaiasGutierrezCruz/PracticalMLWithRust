fn main(){
    let place = "himalayas";

    let weather = match place {
        "himalayas" => "cold",
        _ => "hot"
    };

    println!("Weather is {:?}", weather);
}