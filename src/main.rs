use std::{thread, time};


fn turn_on_well() {
    println!("Brunnen eingeschaltet.");
}
fn turn_off_well() {
    println!("Brunnen ausgeschaltet.");
}
fn control_well(water_level: f32) {
    if water_level > 20.0 {
        turn_on_well();
    } else {
        turn_off_well();
    }

}
fn simulate_well_control() {
    let water_level_well = read_water_level_sensor("Brunnenbecken");
    control_well(water_level_well);
}


fn read_water_level_sensor(sensor_id: &str) -> f32 {
    let water_level = rand::random::<f32>() * 100.0;
    println!("Wasserstandsessor {} gelesen: {:.2}%", sensor_id, water_level);
    water_level
}

fn control_irrigation(water_level: f32) {
    if water_level < 20.0 {
        println!("Wasserstand zu niedrieg. Bewässerung gestoppt.");

    } else {
        println!("Wasserstand ausreichend. Bewässerung läuft!");
    }
}
fn main() {
    loop {
        let water_level_cistern = read_water_level_sensor("Zisterne");

        let water_level_well = read_water_level_sensor("Brunnenbecken");


        control_irrigation(water_level_cistern);
        simulate_well_control();

        thread::sleep(time::Duration::from_secs(5));
    }
}
