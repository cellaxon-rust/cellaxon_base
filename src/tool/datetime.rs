use chrono;


pub fn remove_symbol(string_time: &str) -> String
{
    // 파일 이름에서 공백을 '_'로 치환
    // 윈도우에서 파일 이름에 허용하지 않는 기호를 제거
    let string_time = str::replace(&string_time, " ", "_");
    let string_time = str::replace(&string_time, "-", "");
    let string_time = str::replace(&string_time, ":", "");

    string_time
}


// UTC 날짜-시간으로 문자열을 생성하고, 윈도우에서 파일 이름에 허용하지 않는 기호 제거
pub fn get_string_date_time_utc() -> String
{
    let string_time = format!("{}", chrono::offset::Utc::now());
    remove_symbol(&string_time)
}


// 로컬 날짜-시간으로 문자열을 생성하고, 윈도우에서 파일 이름에 허용하지 않는 기호 제거
pub fn get_string_date_time_local() -> String

{
    let string_time = format!("{}", chrono::offset::Local::now());
    remove_symbol(&string_time)
}

