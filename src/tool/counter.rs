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

