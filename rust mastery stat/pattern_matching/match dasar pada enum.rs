enum TrafficLight {
    Red, Yellow, Green
}

fn action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "stop",
        TrafficLight::Yellow => "careful",
        TrafficLight::Green => "go",
    }
}

fn main(){
    println!("{}", action(TrafficLight::Green));
    println!("{}", action(TrafficLight::Yellow));
    println!("{}", action(TrafficLight::Red));
}