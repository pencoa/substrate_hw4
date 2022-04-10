#[allow(dead_code)]
pub enum TrafficColor {
    Red,
    Yellow,
    Green,
}

pub trait TrafficTime {
    fn get_time(&self) -> i32;
}

impl TrafficTime for TrafficColor {
    fn get_time(&self) -> i32 {
        match self {
            TrafficColor::Red => 60,
            TrafficColor::Yellow => 10,
            TrafficColor::Green => 45,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_time4red() {
        let red = TrafficColor::Red;
        assert_eq!(red.get_time(), 60);
    }

    #[test]
    fn test_traffic_time4yellow() {
        let yellow = TrafficColor::Yellow;
        assert_eq!(yellow.get_time(), 10);
    }

    #[test]
    fn test_traffic_time4green() {
        let green = TrafficColor::Green;
        assert_eq!(green.get_time(), 45);
    }
}
