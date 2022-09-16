//! This lib is aiming at generate current time as i64.
use std::time::SystemTime;
#[derive(Debug)]
pub struct DateTime (i16,u8,u8,u8,u8,u8,u8,u8,u8);

impl Clone for DateTime {
    fn clone(&self) -> Self { todo!() }
}

impl Copy for DateTime {

}

impl DateTime {
    pub fn new(year:i16, month:u8, day:u8, hour:u8, minute:u8, second:u8, millis:u8, macros:u8, nanos:u8) -> Self {
        DateTime(year, month, day, hour, minute, second, millis, macros, nanos)
    }

    fn leap_year(self, _year:i16) -> bool {
        (_year%4==0 && _year%100!=0) || _year%400==0
    }

    fn month_daily(self, _is_leap:bool) -> Vec<i8>{
        if _is_leap {
            vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        }else{
            vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        }
    }

    pub fn time_zone(self, time_zone:i8) -> Self {
        let (mut _year,mut _month,mut _day,mut _hour) = (self.0, self.1 as i8, self.2 as i8, self.3 as i8);
        let _is_leap = (&self).leap_year(_year);
        let _month_day = (&self).month_daily(_is_leap);
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
            if _hour > 24 {
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

    fn positive(time:i64) -> Self {
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
        year += num1year+1;
        let mut month;
        let mut day;
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
        DateTime(year as i16, month as u8, day as u8, hour as u8, mins as u8, secs as u8, 0,0,0)
    }

    fn nagative(time:i64) -> Self {
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
            year += num1year;
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
            year += 1970;
        } else {
            seconds *= -1;
            
        }
        DateTime(year,month,day,hour,minute,second,0,0,0)
    }

    pub fn from(time: i64) -> Self {
        if time >= 0 {
            Self::positive(time)
        } else{
            Self::nagative(time)
        }
    }

    pub fn from_millis(time: i64) -> Self {
        let millis = time%1000;
        let mut time = Self::from(time/1000);
        time.6 = millis as u8;
        time
    }

    pub fn from_macros(time: i64) -> Self {
        let macros = time%1_000;
        let millis = (time/1_000)%1_000;
        let mut time = Self::from(time/1_000_000);
        time.6 = millis as u8;
        time.7 = macros as u8;
        time
    }

    pub fn from_nanos(time: i64) -> Self {
        let nanos = time%1_000;
        let macros = (time/1_000)%1_000;
        let millis = (time/1_000_000)%1_000;
        let mut time = Self::from(time/1_000_000_000);
        time.6 = millis as u8;
        time.7 = macros as u8;
        time.8 = nanos as u8;
        time
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