//! This lib is aiming at generate current time as i64.
use std::time::SystemTime;
#[derive(Debug)]
pub struct DateTime (i16,u8,u8,u8,u8,u8,u16,u16,u16);

impl Clone for DateTime {
    fn clone(&self) -> Self { todo!() }
}

impl Copy for DateTime {

}

impl DateTime {
    pub fn new(year:i16, month:u8, day:u8, hour:u8, minute:u8, second:u8, millis:u16, macros:u16, nanos:u16) -> Self {
        DateTime(year, month, day, hour, minute, second, millis, macros, nanos)
    }

    fn leap_year(_year:i16) -> bool {
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
        let (mut _year,mut _month,mut _day,mut _hour) = (self.0, self.1 as i8, self.2 as i8, self.3 as i8);
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
        DateTime(_year, _month as u8, _day as u8, _hour as u8, self.4, self.5, self.6, self.7, self.8)
    }

    pub fn from(time: i64) -> Self {
        let (mut year,mut month,mut day,mut hour,mut minute,mut second) = (0,0,0,0,0,0);
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
            let mut reverse_month_day = Self::month_daily(Self::leap_year(year as i16));
            reverse_month_day.reverse();
            month = 0;
            for i in reverse_month_day {
                if _day + (i as i64) < seconds{
                    month += 1;
                    _day +=(i as i64);
                }else{
                    day = (i as i64) - (seconds - _day);
                    month = 12 - month;
                    break;
                }
            }
        }
        DateTime(year as i16,month as u8,day as u8,hour as u8,minute as u8,second as u8,0,0,0)
    }

    pub fn from_millis(time: i64) -> Self {
        let mut seconds = time;
        let mut millis;
        if seconds > 0{
            millis = seconds%1000;
            seconds /= 1000;
        }else{
            seconds *= -1;
            millis = 1000 - seconds%1000;
            seconds /= 1000;
            if millis > 0 {
                seconds += 1;
            }
            seconds *= -1;
        }
        let mut datetime = Self::from(seconds);
        datetime.6 = millis as u16;
        datetime
    }

    pub fn from_macros(time: i64) -> Self {
        let mut seconds = time;
        let (mut millis, mut macros);
        if seconds > 0{
            macros = seconds%1000;
            seconds /= 1000;
            millis = seconds%1000;
            seconds /= 1000;
        }else{
            seconds *= -1;
            macros = 1000 - seconds%1000;
            seconds /= 1000;
            if macros > 0 {
                seconds += 1;
            }
            millis = 1000 - seconds%1000;
            seconds /= 1000;
            if millis > 0 {
                seconds += 1;
            }
            seconds *= -1;
        }
        let mut datetime = Self::from(seconds);
        datetime.6 = millis as u16;
        datetime.7 = macros as u16;
        datetime
    }

    pub fn from_nanos(time: i64) -> Self {
        let mut seconds = time;
        let (mut millis, mut macros, mut nanos);
        if seconds > 0{
            nanos = seconds %1000;
            seconds /= 1000;
            macros = seconds%1000;
            seconds /= 1000;
            millis = seconds%1000;
            seconds /= 1000;
        }else{
            seconds *= -1;
            nanos = 1000 - seconds%1000;
            seconds /= 1000;
            if nanos > 0 {
                seconds += 1;
            }
            macros = 1000 - seconds%1000;
            seconds /= 1000;
            if macros > 0 {
                seconds += 1;
            }
            millis = 1000 - seconds%1000;
            seconds /= 1000;
            if millis > 0 {
                seconds += 1;
            }
            seconds *= -1;
        }
        let mut datetime = Self::from(seconds);
        datetime.6 = millis as u16;
        datetime.7 = macros as u16;
        datetime.8 = nanos as u16;
        datetime
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
        let _is_leap = Self::leap_year(self.0);
        let mut result:i64 = (((self.2-1) as i64)*3600*24 as i64 + (self.3 as i64)*3600 + (self.4 as i64)*60 as i64 + self.5 as i64).into();
        let _month_day = Self::month_daily(_is_leap);
        let mut month = 1;
        for i in _month_day {
            if month < self.1 {
                month += 1;
                result += (i as i64)*3600*24;
            }
        }
        let mut year = (self.0 as i64) - 1970;
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
        result += (self.6  as i64)*1000000 + (self.7 as i64)*1000 + self.8 as i64;
        result
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
    println!("{:?}", SystemTime::now());
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