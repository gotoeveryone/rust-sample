trait Sensor {
    fn read(&self) -> f32;
}

struct LightSensor {
    value: u32,
}

struct TemperaureSensor {
    value: f32,
}

impl Sensor for LightSensor {
    fn read(&self) -> f32 {
        return self.value as f32;
    }
}

impl Sensor for TemperaureSensor {
    fn read(&self) -> f32 {
        return self.value;
    }
}

fn print_sensor_value<S: Sensor>(s: S) {
    println!("value = {}", s.read());
}

pub fn print_sensor_info() {
    let s1 = LightSensor { value: 32 };
    print_sensor_value(s1);
    let s2 = TemperaureSensor { value: 32.5 };
    print_sensor_value(s2);
}
