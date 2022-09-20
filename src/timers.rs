//! This lib is aiming at generate current time as i64.
use std::time::SystemTime;
use std::fmt::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[warn(unused_assignments)]
pub struct DateTime {year:i32,month:u8,day:u8,hour:u8,minute:u8,second:u8,millis:u16,macros:u16,nanos:u16}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}-{:02?}-{:02?} {:02?}:{:02?}:{:02?}.{:03?}{:03?}{:03?}", self.year, self.month, self.day, self.hour, self.minute, self.second, self.millis, self.macros, self.nanos)
    }
}

impl DateTime {
    pub fn new(year:i32, month:u8, day:u8, hour:u8, minute:u8, second:u8, millis:u16, macros:u16, nanos:u16) -> Self {
        DateTime{year, month, day, hour, minute, second, millis, macros, nanos}
    }

    fn leap_year(_year:i32) -> bool {
        (_year%4==0 && _year%100!=0) || _year%400==0
    }

    fn month_daily(_is_leap:bool) -> Vec<i8>{
        if _is_leap {
            vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        }else{
            vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        }
    }

    pub fn time_zone(self, time_zone:i8) -> Self {
        let (mut _year,mut _month,mut _day,mut _hour) = (self.year, self.month as i8, self.day as i8, self.hour as i8);
        let _is_leap = Self::leap_year(_year);
        let _month_day = Self::month_daily(_is_leap);
        if time_zone < 0 {
            _hour += time_zone;
            let mut _minus_one = 0;
            if _hour < 0 {
                _minus_one = 1;
                _hour += 24;
            }
            _day -= _minus_one;
            if _day < 1 {
                _month -= 1;
                if _month < 1{
                    _day = _month_day[11 as usize];
                    _month = 12;
                    _year -= 1;
                } else{
                    _day = _month_day[(_month-2) as usize];
                }
            }
        } else {
            _hour += time_zone;
            let mut _add_one = 0;
            if _hour >= 24 {
                _add_one =1;
                _hour %= 24;
            }
            _day += _add_one;
            if _day > _month_day[(_month-1) as usize] {
                _day = 1;
                _month += 1;
            }
            if _month > 12 {
                _year += 1;
                _month = 1;
            }
        }
        DateTime{year: _year, month:_month as u8, day:_day as u8, hour:_hour as u8, ..self}
    }

    pub fn from(time: i64) -> Self {
        let (mut year,mut day) = (0, 0);
        let (mut month,hour,minute,second):(i64,i64,i64,i64);
        let mut seconds = time;
        if seconds >= 0 {
            second = seconds%60;
            seconds/=60;
            minute = seconds%60;
            seconds/=60;
            hour = seconds%24;
            seconds/=24;
            year += seconds/146097*400;
            seconds %= 146097;
            let mut num100year = seconds / 36524;
            if num100year == 4 {
                num100year -= 1;
                seconds -= 36524*3;
            }else{
                seconds %= 36524;
            }
            year += num100year * 100;
            year += seconds / 1461 * 4;
            seconds %= 1461;
            let mut num1year = seconds / 365;
            if num1year == 4{
                num1year -= 1;
                seconds -= 365*3;
            }else {
                seconds %= 365;
            }
            year += num1year + 1970;
            if seconds< 31{
                month = 1;
                day = seconds +1;
            }else{
                if (year%4==0 && year%100!=0) || year%400==0 {
                    seconds -= 29;
                    month = seconds*12 / 367;
                    day = seconds - month * 367 / 12;
                    if month== 0{
                        day -= 1;
                    }
                }else{
                    seconds -= 28;
                    month = seconds*12 / 367;
                    day = seconds - month * 367 / 12;
                    if month== 0{
                        day -= 2;
                    }
                }
                month += 2;
            }
        } else {
            seconds *= -1;
            let mut _add_one = 0;
            second = (60 - seconds%60)%60;
            seconds /= 60;
            if second>0 {
                seconds += 1;
                _add_one =1;
            }
            minute = (60 - seconds%60)%60;
            seconds /= 60;
            if minute > 0 {
                _add_one = 1;
                seconds += 1;
            }
            hour = (24 - seconds%24) % 24;
            seconds/=24;
            year += seconds/146097*400;
            seconds %= 146097;
            let mut num100year = seconds / 36524;
            if num100year == 4 {
                num100year -= 1;
                seconds -= 36524*3;
            }else{
                seconds %= 36524;
            }
            year += num100year * 100;
            year += seconds / 1461 * 4;
            seconds %= 1461;
            let mut num1year = seconds / 365;
            if num1year == 4{
                num1year -= 1;
                seconds -= 365*3;
            }else {
                seconds %= 365;
            }
            year += num1year + 1;
            year = 1970 - year;
            let mut _day = 0;
            let mut reverse_month_day = Self::month_daily(Self::leap_year(year as i32));
            reverse_month_day.reverse();
            month = 0;
            for i in reverse_month_day {
                if _day + (i as i64) < seconds{
                    month += 1;
                    _day +=i as i64;
                }else{
                    day = (i as i64) - (seconds - _day);
                    month = 12 - month;
                    break;
                }
            }
        }
        if year > i32::MAX.into() || year < i32::MIN.into() {
            panic!("Input time is out of bounds!");
        }
        DateTime{year:year as i32,month:month as u8,day:day as u8,hour:hour as u8,minute:minute as u8,second:second as u8,millis:0,macros:0,nanos:0}
    }

    pub fn from_millis(time: i64) -> Self {
        let mut seconds = time;
        let millis;
        if seconds > 0{
            millis = seconds%1000;
            seconds /= 1000;
        }else{
            seconds *= -1;
            millis = (1000 - seconds%1000)%1000;
            seconds /= 1000;
            if millis > 0 {
                seconds += 1;
            }
            seconds *= -1;
        }
        let datetime = Self::from(seconds);
        DateTime{millis : millis as u16, ..datetime}
    }

    pub fn from_macros(time: i64) -> Self {
        let mut seconds = time;
        let (millis, macros);
        if seconds > 0{
            macros = seconds%1000;
            seconds /= 1000;
            millis = seconds%1000;
            seconds /= 1000;
        }else{
            seconds *= -1;
            macros = (1000 - seconds%1000)%1000;
            seconds /= 1000;
            if macros > 0 {
                seconds += 1;
            }
            millis = (1000 - seconds%1000)%1000;
            seconds /= 1000;
            if millis > 0 {
                seconds += 1;
            }
            seconds *= -1;
        }
        let datetime = Self::from(seconds);
        DateTime{millis : millis as u16, macros : macros as u16, ..datetime}
    }

    pub fn from_nanos(time: i64) -> Self {
        let mut seconds = time;
        let (millis, macros, nanos);
        if seconds > 0{
            nanos = seconds %1000;
            seconds /= 1000;
            macros = seconds%1000;
            seconds /= 1000;
            millis = seconds%1000;
            seconds /= 1000;
        }else{
            seconds *= -1;
            nanos = (1000 - seconds%1000)%1000;
            seconds /= 1000;
            if nanos > 0 {
                seconds += 1;
            }
            macros = (1000 - seconds%1000)%1000;
            seconds /= 1000;
            if macros > 0 {
                seconds += 1;
            }
            millis = (1000 - seconds%1000)%1000;
            seconds /= 1000;
            if millis > 0 {
                seconds += 1;
            }
            seconds *= -1;
        }
        let datetime = Self::from(seconds);
        DateTime{millis : millis as u16, macros : macros as u16, nanos : nanos as u16, ..datetime}
    }

    pub fn to_seconds(self) -> i64 {
        self.to_nanos()/1000000000
    }

    pub fn to_millis(self) -> i64 {
        self.to_nanos()/1000000
    }

    pub fn to_macros(self) -> i64 {
        self.to_nanos()/1000        
    }

    pub fn to_nanos(self) -> i64 {
        let _is_leap = Self::leap_year(self.year);
        let mut result:i64 = (((self.day-1) as i64)*3600*24 as i64 + (self.hour as i64)*3600 + (self.minute as i64)*60 as i64 + self.second as i64).into();
        let _month_day = Self::month_daily(_is_leap);
        let mut month = 1;
        for i in _month_day {
            if month < self.month {
                month += 1;
                result += (i as i64)*3600*24;
            }
        }
        let mut year = (self.year as i64) - 1970;
        let num400year = year / 400;
        year %= 400;
        result += 146097*3600*24*num400year;
        let num100year = year / 100;
        year %= 100;
        result += 36524 * 3600*24*num100year;
        let num4year = year/4;
        year%=4;
        result += 1461*3600*24*num4year + year*365*3600*24;
        result*= 1000000000;
        result += (self.millis  as i64)*1000000 + (self.macros as i64)*1000 + self.nanos as i64;
        result
    }

    pub fn from_string(time: &str) -> Self {
        let vec1:Vec<&str> = (*time).split(&[' ', '-', '.', ':'][..]).collect();
        let size = vec1.len();
        let mut vec_i32:Vec<i32> = Vec::new();
        if size == 7 {
            for i in vec1 {
                vec_i32.push(i.parse::<i32>().unwrap());
            }
        } else if size == 8 {
            for i in vec1 {
                if i!= "" {
                    vec_i32.push(i.parse::<i32>().unwrap());
                }
            }
            vec_i32[0] *= -1;
        }else{
            panic!("The input str {} is illegal!", &time);
        }
        DateTime{year:vec_i32[0],month:vec_i32[1] as u8,day:vec_i32[2] as u8,hour:vec_i32[3] as u8,minute:vec_i32[4] as u8,second:vec_i32[5] as u8,millis:(vec_i32[6]/1_000_000) as u16,macros:((vec_i32[6]/1_000)%1_000) as u16,nanos:(vec_i32[6]%1_000) as u16}
    }
}

/// Get current nanos time.
/// ```
/// let cur = current_time_as_naons();
/// ```
pub fn current_time_as_nanos() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_nanos() as i64,
        Err(e) => panic!("Time is illegal!! Error msg {}.", e),
    }
}

/// Get current micros time.
/// ```
/// let cur = current_time_as_micros();
/// ```
pub fn current_time_as_micros() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_micros() as i64,
        Err(e) => panic!("Time is illegal!! Error msg {}.", e),
    }
}

/// Get current millis time.
/// ```
/// let cur = current_time_as_millis();
/// ```
pub fn current_time_as_millis() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_millis() as i64,
        Err(e) => panic!("Time is illegal!! Error msg {}.", e),
    }
}

/// Get current secs time.
/// ```
/// let cur = current_time_as_secs();
/// ```
pub fn current_time_as_secs() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_secs() as i64,
        Err(e) => panic!("Time is illegal!! Error msg {}.", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use std::thread::sleep;
        use std::time::Duration;
        let result = current_time_as_millis();
        dbg!("{:?}", result);
        sleep(Duration::new(1,0));
        let result = current_time_as_secs();
        dbg!("{:?}", result);
        sleep(Duration::new(1,0));
        let result = current_time_as_micros();
        dbg!("{:?}", result);
        sleep(Duration::new(1,0));
    }
}