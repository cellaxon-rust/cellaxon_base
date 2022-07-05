use std::time::Instant;


pub struct Ticker
{
    count: u128,
    time_start: Instant,
    time_next: u128,
    time_interval: u32,
}


impl Ticker
{
    pub fn empty() -> Ticker
    {
        Ticker
        {
            count: 0,
            time_start: Instant::now(),
            time_next: 100,
            time_interval: 100,
        }
    }


    pub fn new(time_interval: u32) -> Ticker
    {
        let mut ticker = Ticker::empty();

        ticker.time_next = time_interval as u128;
        ticker.time_interval = time_interval;

        ticker
    }


    pub fn set_interval(&mut self, time_interval: u32)
    {
        self.time_interval = time_interval;
    }


    pub fn check(&mut self) -> bool
    {
        let time_now = self.time_start.elapsed().as_millis();
        if time_now > self.time_next
        {
            self.count += 1;
            self.time_next = self.time_next + self.time_interval as u128;

            return true;
        }

        return false;
    }

    pub fn get_count(&self) -> u128
    {
        self.count
    }
}

