#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:0width$}:{:0width$}",
            self.hours,
            self.minutes,
            width = 2
        )
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = minutes + hours * 60;

        match minutes {
            m if m >= 0 => Clock {
                hours: ((m / 60) % 24),
                minutes: (m % 60),
            },
            m if m < 0 && m % 60 == 0 => Clock {
                hours: ((24 + ((m / 60) % 24)) % 24),
                minutes: (m % 60),
            },
            m if m < 0 => Clock {
                hours: ((23 + ((m / 60) % 24)) % 24),
                minutes: ((60 + m % 60) % 60),
            },
            _ => unreachable!(),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut minutes = minutes + self.minutes + self.hours * 60;

        Clock::new(0, minutes)
    }
}
