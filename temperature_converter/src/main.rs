use std::io;

fn main() {
    println!("Please enter temperature metric C or F:");
    let mut temperature_metric = String::new();
    io::stdin()
        .read_line(&mut temperature_metric)
        .expect("Failed to input temperature_metric");

    println!("Please enter the temperature value:");

    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to input temperature");

    let the_converted_temperature: String = convert_temp(temperature_metric, temperature);

    println!("The converted temperature is equal to {the_converted_temperature}");

    fn convert_temp(metric: String, temperature_value: String) -> String {
        if metric.trim() == "F" {
            let converted_temperature: i32 =
                (temperature_value.trim().parse::<i32>().unwrap() - 32) * 5 / 9;
            return converted_temperature.to_string() + "C";
        } else {
            let converted_temperature: i32 =
                (temperature_value.trim().parse::<i32>().unwrap() * 5 / 9) - 32;
            return converted_temperature.to_string() + "F";
        }
    }
}
