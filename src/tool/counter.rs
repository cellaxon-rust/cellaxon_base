use std::time::Instant;


pub struct FramePerSecond
{
    time_start: Instant,
    time_check_next: u128,
    count_frame: i32,
    fps: i32,
}


// 초당 tick이 호출된 횟수를 카운트
impl FramePerSecond
{
    pub fn new() -> FramePerSecond
    {
        FramePerSecond
        {
            time_start: Instant::now(),
            time_check_next: Instant::now().elapsed().as_millis(),
            count_frame: 0,
            fps: 0,
        }
    }


    pub fn tick(&mut self) -> bool
    {
        self.count_frame = self.count_frame + 1;
        let time_now = self.time_start.elapsed().as_millis();

        if time_now > self.time_check_next
        {
            self.time_check_next = time_now + 1000;
            self.fps = self.count_frame;
            self.count_frame = 0;

            return true;
        }

        false
    }

    
    pub fn get_fps(&self) -> i32
    {
        self.fps
    }
}


pub struct Counter
{
    time_start: Instant,
    time_check_next_sec: u128,
    time_check_next_min: u128,
    count_total: u128,
    count_frame_sec: i32,
    count_frame_min: i32,
    fps: i32,
    fpm: i32,
}


// 초당 tick이 호출된 횟수를 카운트
impl Counter
{
    pub fn new() -> Counter
    {
        Counter
        {
            time_start: Instant::now(),
            time_check_next_sec: Instant::now().elapsed().as_millis(),
            time_check_next_min: Instant::now().elapsed().as_millis(),
            count_total: 0,
            count_frame_sec: 0,
            count_frame_min: 0,
            fps: 0,
            fpm: 0,
        }
    }


    pub fn tick(&mut self) -> (bool, bool)
    {
        let time_now = self.time_start.elapsed().as_millis();

        let mut flag_updated_sec = false;
        let mut flag_updated_min = false;

        self.count_total = self.count_total + 1;

        self.count_frame_sec = self.count_frame_sec + 1;
        if time_now > self.time_check_next_sec
        {
            self.time_check_next_sec = time_now + 1000;
            self.fps = self.count_frame_sec;
            self.count_frame_sec = 0;

            flag_updated_sec = true;
        }

        self.count_frame_min = self.count_frame_min + 1;
        if time_now > self.time_check_next_min
        {
            self.time_check_next_min = time_now + 1000 * 60;
            self.fpm = self.count_frame_min;
            self.count_frame_min = 0;

            flag_updated_min = true;
        }

        (flag_updated_sec, flag_updated_min)
    }

    
    pub fn get_fps(&self) -> i32
    {
        self.fps
    }

    
    pub fn get_fpm(&self) -> i32
    {
        self.fpm
    }
}

