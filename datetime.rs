use std;

import std::time::tm;
import result::{result, ok, err, extensions};

iface date {
	fn timespec() -> std::time::timespec;
	fn from_timespec(ts: std::time::timespec) -> date;
	fn tm() -> std::time::tm;
	fn from_tm(tm: std::time::tm) -> date;
}

iface time {
	fn timespec() -> std::time::timespec;
	fn from_timespec(ts: std::time::timespec) -> time;
	fn tm() -> std::time::tm;
	fn from_tm(tm: std::time::tm) -> time;
}

iface date_time {
	fn timespec() -> std::time::timespec;
	fn from_timespec(ts: std::time::timespec) -> date_time;
	fn tm() -> std::time::tm;
	fn from_tm(tm: std::time::tm) -> date_time;
}

const SECS_FROM_UNIX_EPOCH: i64 = 62135596800_i64;

fn leapyear(y: i32) -> bool { y % 4_i32 == 0_i32 && (y % 100_i32 != 0_i32 || y % 400_i32 == 0_i32) }

fn month_lookup(doy: i32, ly: bool) -> i32 {
	alt ly {
		true {
			alt check doy {
				0_i32 to 30_i32 { 1_i32 }
				31_i32 to 59_i32 { 2_i32 }
				60_i32 to 90_i32 { 3_i32 }
				91_i32 to 120_i32 { 4_i32 }
				121_i32 to 151_i32 { 5_i32 }
				152_i32 to 181_i32 { 6_i32 }
				182_i32 to 212_i32 { 7_i32 }
				213_i32 to 243_i32 { 8_i32 }
				244_i32 to 273_i32 { 9_i32 }
				274_i32 to 304_i32 { 10_i32 }
				305_i32 to 334_i32 { 11_i32 }
				335_i32 to 365_i32 { 12_i32 }
			}
		}
		false {
			alt check doy {
				0_i32 to 30_i32 { 1_i32 }
				31_i32 to 58_i32 { 2_i32 }
				59_i32 to 89_i32 { 3_i32 }
				90_i32 to 119_i32 { 4_i32 }
				120_i32 to 150_i32 { 5_i32 }
				151_i32 to 180_i32 { 6_i32 }
				181_i32 to 211_i32 { 7_i32 }
				212_i32 to 242_i32 { 8_i32 }
				243_i32 to 272_i32 { 9_i32 }
				273_i32 to 303_i32 { 10_i32 }
				304_i32 to 333_i32 { 11_i32 }
				334_i32 to 364_i32 { 12_i32 }
			}
		}
	}
}

fn accume_days(m: i32, ly: bool) -> i32 {
	let xtra = (ly && m > 2_i32) as i32;
	let rv = alt check m {
		1_i32 { 0_i32 }
		2_i32 { 31_i32 }
		3_i32 { 59_i32 }
		4_i32 { 90_i32 }
		5_i32 { 120_i32 }
		6_i32 { 151_i32 }
		7_i32 { 181_i32 }
		8_i32 { 212_i32 }
		9_i32 { 243_i32 }
		10_i32 { 273_i32 }
		11_i32 { 304_i32 }
		12_i32 { 334_i32 }
	};
	rv + xtra
}

fn month_length(m: i32, ly: bool) -> i32 {
	alt check m {
		1_i32 | 3_i32 | 5_i32 | 7_i32 | 8_i32 | 10_i32 | 12_i32 { 31_i32 }
		2_i32 { alt ly { true { 29_i32 } false { 28_i32 }}}
		4_i32 | 6_i32 | 9_i32 | 11_i32 { 30_i32 }
	}
}

fn date_from_days(days: i32) -> { year: i32, mon: i32, mday: i32, yday: i32} {
	let n400 = days/146097_i32;
	let d1 = days % 146097_i32;
	let n100 = d1/36524_i32;
	let d2 = d1 % 36524_i32;
	let n4 = d2/1461_i32;
	let d3 = d2 % 1461_i32;
	let n1 = d3/365_i32;
	let xtra = if n100 == 4_i32 || n1 == 4_i32 {
		1_i32
	}
	else {
		0_i32
	};
	let y = 400_i32*n400 + 100_i32*n100 + 4_i32*n4 + n1 + 1_i32 - xtra;
	let doy = d3 % 365_i32 + 365_i32*xtra;
	let ly = leapyear(y);
	let m = month_lookup(doy, ly);
	let d = doy - accume_days(m, ly) + 1_i32;
	{ year: y, mon: m, mday: d, yday: doy}
}

fn days_from_date(y: i32, m: i32, d: i32) -> i32 {
	let ly = leapyear(y);
	let ym1 = y - 1_i32;
	365_i32*ym1 + ym1/4_i32 - ym1/100_i32 + ym1/400_i32 + accume_days(m, ly) + d - 1_i32
}

impl of date for i32 {
	//  days since 0001-01-01
	fn timespec() -> std::time::timespec {
		{ sec: self as i64*86400_i64 - SECS_FROM_UNIX_EPOCH, nsec: 0_i32 }
	}

	fn from_timespec(ts: std::time::timespec) -> date {
		(((ts.sec + SECS_FROM_UNIX_EPOCH)/86400_i64) as i32) as date
	}

	fn tm() -> std::time::tm {
		let dp = date_from_days(self);
		{ tm_sec: 0_i32,
		  tm_min: 0_i32,
		  tm_hour: 0_i32,
		  tm_mday: dp.mday,
		  tm_mon: dp.mon - 1_i32,
		  tm_year: dp.year - 1900_i32,
		  tm_wday: (self + 1_i32) % 7_i32,
		  tm_yday: dp.yday,
		  tm_isdst: 0_i32,
		  tm_gmtoff: 0_i32,
		  tm_zone: "UTC",
		  tm_nsec: 0_i32
		}
	}

	fn from_tm(tm: std::time::tm) -> date {
		days_from_date(tm.tm_year + 1900_i32, tm.tm_mon + 1_i32, tm.tm_mday) as date
	}
}

impl of time for i64 {
	//  nanosecond resolution
	fn timespec() -> std::time::timespec {
		{ sec: (self % 86400000000000_i64)/1000000000_i64, nsec: (self % 1000000000_i64) as i32 }
	}

	fn from_timespec(ts: std::time::timespec) -> time {
		((ts.sec % 86400)*1000000000_i64 + ts.nsec as i64) as time
	}

	fn tm() -> std::time::tm {
		let s = (self % 86400000000000_i64)/1000000000_i64;
		{ tm_sec: (s % 60_i64) as i32,
		  tm_min: ((s/60_i64) % 60_i64) as i32,
		  tm_hour: (s/3600_i64) as i32,
		  tm_mday: 1_i32,
		  tm_mon: 0_i32,
		  tm_year: 70_i32,
		  tm_wday: 0_i32,
		  tm_yday: 0_i32,
		  tm_isdst: 0_i32,
		  tm_gmtoff: 0_i32,
		  tm_zone: "UTC",
		  tm_nsec: (self % 1000000000_i64) as i32
		}
	}

	fn from_tm(tm: std::time::tm) -> time {
		(tm.tm_hour as i64*3600000000000_i64 + tm.tm_min as i64*60000000000_i64 + tm.tm_sec as i64*1000000000_i64 + tm.tm_nsec as i64) as time
	}
}

impl of date_time for i64 {
	//  milliseconds since 0001-01-01
	fn timespec() -> std::time::timespec {
		{ sec: self/1000_i64 - SECS_FROM_UNIX_EPOCH, nsec: ((self % 1000_i64)*1000000_i64) as i32 }
	}

	fn from_timespec(ts: std::time::timespec) -> date_time {
		((ts.sec + SECS_FROM_UNIX_EPOCH)*1000_i64 + (ts.nsec as i64)/1000000_i64) as date_time
	}

	fn tm() -> std::time::tm {
		let d = self/86400000_i64;
		let dp = date_from_days(d as i32);
		let s = (self % 86400000_i64)/1000_i64;
		{ tm_sec: (s % 60_i64) as i32,
		  tm_min: ((s/60_i64) % 60_i64) as i32,
		  tm_hour: (s/3600_i64) as i32,
		  tm_mday: dp.mday,
		  tm_mon: dp.mon - 1_i32,
		  tm_year: dp.year - 1900_i32,
		  tm_wday: ((self + 1_i64) % 7_i64) as i32,
		  tm_yday: dp.yday,
		  tm_isdst: 0_i32,
		  tm_gmtoff: 0_i32,
		  tm_zone: "UTC",
		  tm_nsec: 1000000_i64*(self % 1000_i64) as i32
		}
	}

	fn from_tm(tm: std::time::tm) -> date_time {
		let d = days_from_date(tm.tm_year + 1900_i32, tm.tm_mon + 1_i32, tm.tm_mday);
		let s = tm.tm_hour as i64*3600_i64 + tm.tm_min as i64*60_i64 + tm.tm_sec as i64;
		(d as i64*86400000_i64 + s*1000_i64 + (tm.tm_nsec as i64)/1000000_i64) as date_time
	}
}

impl of date_time for std::time::timespec {
	fn timespec() -> std::time::timespec {
		self
	}

	fn from_timespec(ts: std::time::timespec) -> date_time {
		ts as date_time
	}

	fn tm() -> std::time::tm {
		let d = (self.sec + SECS_FROM_UNIX_EPOCH)/86400_i64;
		let dp = date_from_days(d as i32);
		let s = (self.sec + SECS_FROM_UNIX_EPOCH) % 86400_i64;
		{ tm_sec: (s % 60_i64) as i32,
		  tm_min: ((s/60_i64) % 60_i64) as i32,
		  tm_hour: (s/3600_i64) as i32,
		  tm_mday: dp.mday,
		  tm_mon: dp.mon - 1_i32,
		  tm_year: dp.year - 1900_i32,
		  tm_wday: ((d + 1_i64) % 7_i64) as i32,
		  tm_yday: dp.yday,
		  tm_isdst: 0_i32,
		  tm_gmtoff: 0_i32,
		  tm_zone: "UTC",
		  tm_nsec: self.nsec
		}
	}

	fn from_tm(tm: std::time::tm) -> date_time {
		let d = days_from_date(tm.tm_year + 1900_i32, tm.tm_mon + 1_i32, tm.tm_mday) as i64;
		let s = (tm.tm_hour as i64)*3600_i64 + (tm.tm_min as i64)*60_i64 + tm.tm_sec as i64;
		{ sec: d*86400_i64 - SECS_FROM_UNIX_EPOCH + s, nsec: tm.tm_nsec } as date_time
	}
}

impl dtm for date_time {
	fn str() -> str {
		let tm = self.tm();
		#fmt("%s%s", tm.strftime("%Y-%m-%d %H:%M:%S"), if tm.tm_nsec != 0_i32 { #fmt("%09i", tm.tm_nsec as int) } else { "" })
	}

	fn from_str(ds: str) -> result<date_time, str> {
		alt std::time::strptime(ds, "%Y-%m-%d %H:%M:%S") {
			ok(tm) { ok(({ sec: 0_i64, nsec: 0_i32 } as date_time).from_tm(tm)) }
			err(es) { err(es) }
		}
	}
}


#[cfg(test)]
mod tests {
	fn test_dt_str(s: str) {
		alt ({ sec: 0_i64, nsec: 0_i32 } as date_time).from_str(s) {
			ok(dt) {
				let dts = dt.str();
				if s != dts {
					log(error, ("test_dt_str", s, dts));
					fail
				}
			}
			err(es) {
				log(error, ("test_dt_str", s, es));
				fail
			}
		}
	}

	#[test]
	fn test_dt_limits() {
		test_dt_str("2012-05-07 09:56:33");
		test_dt_str("1000-01-01 00:00:00");
		test_dt_str("9999-12-31 23:59:59");
		test_dt_str("1900-02-28 23:59:59");
		test_dt_str("1900-03-01 00:00:00");
		test_dt_str("2000-02-29 23:59:59");
		test_dt_str("2000-03-01 00:00:00");
	}

	fn test_std_time(s: str) {
		alt ({ sec: 0_i64, nsec: 0_i32 } as date_time).from_str(s) {
			ok(dt) {
				let dtm = dt.tm();
				let stm = std::time::at_utc(dt.timespec());
				if stm != dtm {
					log(error, ("test_std_time", s, dtm, stm));
					fail
				}
				let dts = dt.timespec();
				let sts = dtm.to_timespec();
				if dts != sts {
					log(error, ("test_std_time", s, dts, sts));
					fail
				}
			}
			err(es) {
				log(error, ("test_std_time", s, es));
				fail
			}
		}
	}

	#[test]
	fn test_std_limits() {
		test_std_time("2012-05-07 09:56:33");
		test_std_time("1901-12-13 20:45:52");
		test_std_time("9999-12-31 23:59:59");
		test_std_time("2100-02-28 23:59:59");
		test_std_time("2100-03-01 00:00:00");
		test_std_time("2000-02-29 23:59:59");
		test_std_time("2000-03-01 00:00:00");
	}

	#[test]
	#[should_fail]
	fn test_std_bad_low_limit() {
		test_std_time("1901-12-13 20:45:51");
	}

	#[test]
	#[should_fail]
	fn test_bad_leap() {
		test_dt_str("2100-02-29 23:59:59");
	}

	#[test]
	#[should_fail]
	fn test_std_bad_hi_limit() {
		test_std_time("10000-01-01 00:00:00");
	}

	fn test_funcs(in: i32) {
		let dt = date_from_days(in);
		if dt.mon < 1_i32 ||
		   dt.mon > 12_i32 ||
		   dt.mday < 1_i32 ||
		   dt.mday > month_length(dt.mon, leapyear(dt.year)) + 1_i32 ||
		   dt.yday < 0_i32 ||
		   dt.yday > 365_i32 {
			log(error, ("test_funcs", in, dt));
			fail
		}
		let d = days_from_date(dt.year, dt.mon, dt.mday);
		if d != in {
			log(error, ("test_funcs", in, dt, d));
			fail
		}
		log(debug, ("test_funcs", in, ((in as date).timespec() as date_time).str()));
	}

	#[test]
	fn test_all_funcs() {
		let mut i = 0_i32;
		while i < 3652060_i32 {
			test_funcs(i);
			i += 1_i32;
		}
	}
}
