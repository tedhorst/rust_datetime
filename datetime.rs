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

const SECS_FROM_UNIX_EPOCH: i64 = 62135596800;

fn leapyear(y: i32) -> bool { y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) }

fn month_lookup(doy: i32, ly: bool) -> i32 {
	alt ly {
		true {
			alt check doy {
				0 to 30 { 1 }
				31 to 59 { 2 }
				60 to 90 { 3 }
				91 to 120 { 4 }
				121 to 151 { 5 }
				152 to 181 { 6 }
				182 to 212 { 7 }
				213 to 243 { 8 }
				244 to 273 { 9 }
				274 to 304 { 10 }
				305 to 334 { 11 }
				335 to 365 { 12 }
			}
		}
		false {
			alt check doy {
				0 to 30 { 1 }
				31 to 58 { 2 }
				59 to 89 { 3 }
				90 to 119 { 4 }
				120 to 150 { 5 }
				151 to 180 { 6 }
				181 to 211 { 7 }
				212 to 242 { 8 }
				243 to 272 { 9 }
				273 to 303 { 10 }
				304 to 333 { 11 }
				334 to 364 { 12 }
			}
		}
	}
}

fn accume_days(m: i32, ly: bool) -> i32 {
	let xtra = (ly && m > 2) as i32;
	let rv = alt check m {
		1 { 0 }
		2 { 31 }
		3 { 59 }
		4 { 90 }
		5 { 120 }
		6 { 151 }
		7 { 181 }
		8 { 212 }
		9 { 243 }
		10 { 273 }
		11 { 304 }
		12 { 334 }
	};
	rv + xtra
}

fn month_length(m: i32, ly: bool) -> i32 {
	alt check m {
		1 | 3 | 5 | 7 | 8 | 10 | 12 { 31 }
		2 { alt ly { true { 29 } false { 28 }}}
		4 | 6 | 9 | 11 { 30 }
	}
}

fn date_from_days(days: i32) -> { year: i32, mon: i32, mday: i32, yday: i32} {
	let n400 = days/146097;
	let d1 = days % 146097;
	let n100 = d1/36524;
	let d2 = d1 % 36524;
	let n4 = d2/1461;
	let d3 = d2 % 1461;
	let n1 = d3/365;
	let xtra = if n100 == 4 || n1 == 4 {
		1
	}
	else {
		0
	};
	let y = 400*n400 + 100*n100 + 4*n4 + n1 + 1 - xtra;
	let doy = d3 % 365 + 365*xtra;
	let ly = leapyear(y);
	let m = month_lookup(doy, ly);
	let d = doy - accume_days(m, ly) + 1;
	{ year: y, mon: m, mday: d, yday: doy}
}

fn days_from_date(y: i32, m: i32, d: i32) -> i32 {
	let ly = leapyear(y);
	let ym1 = y - 1;
	365*ym1 + ym1/4 - ym1/100 + ym1/400 + accume_days(m, ly) + d - 1
}

impl of date for i32 {
	//  days since 0001-01-01
	fn timespec() -> std::time::timespec {
		{ sec: self as i64*86400 - SECS_FROM_UNIX_EPOCH, nsec: 0 }
	}

	fn from_timespec(ts: std::time::timespec) -> date {
		(((ts.sec + SECS_FROM_UNIX_EPOCH)/86400) as i32) as date
	}

	fn tm() -> std::time::tm {
		let dp = date_from_days(self);
		{ tm_sec: 0,
		  tm_min: 0,
		  tm_hour: 0,
		  tm_mday: dp.mday,
		  tm_mon: dp.mon - 1,
		  tm_year: dp.year - 1900,
		  tm_wday: (self + 1) % 7,
		  tm_yday: dp.yday,
		  tm_isdst: 0,
		  tm_gmtoff: 0,
		  tm_zone: ~"UTC",
		  tm_nsec: 0
		}
	}

	fn from_tm(tm: std::time::tm) -> date {
		days_from_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday) as date
	}
}

impl of time for i64 {
	//  nanosecond resolution
	fn timespec() -> std::time::timespec {
		{ sec: (self % 86400000000000)/1000000000, nsec: (self % 1000000000) as i32 }
	}

	fn from_timespec(ts: std::time::timespec) -> time {
		((ts.sec % 86400)*1000000000 + ts.nsec as i64) as time
	}

	fn tm() -> std::time::tm {
		let s = (self % 86400000000000)/1000000000;
		{ tm_sec: (s % 60) as i32,
		  tm_min: ((s/60) % 60) as i32,
		  tm_hour: (s/3600) as i32,
		  tm_mday: 1,
		  tm_mon: 0,
		  tm_year: 70,
		  tm_wday: 0,
		  tm_yday: 0,
		  tm_isdst: 0,
		  tm_gmtoff: 0,
		  tm_zone: ~"UTC",
		  tm_nsec: (self % 1000000000) as i32
		}
	}

	fn from_tm(tm: std::time::tm) -> time {
		(tm.tm_hour as i64*3600000000000 + tm.tm_min as i64*60000000000 + tm.tm_sec as i64*1000000000 + tm.tm_nsec as i64) as time
	}
}

impl of date_time for i64 {
	//  milliseconds since 0001-01-01
	fn timespec() -> std::time::timespec {
		{ sec: self/1000 - SECS_FROM_UNIX_EPOCH, nsec: ((self % 1000)*1000000) as i32 }
	}

	fn from_timespec(ts: std::time::timespec) -> date_time {
		((ts.sec + SECS_FROM_UNIX_EPOCH)*1000 + (ts.nsec as i64)/1000000) as date_time
	}

	fn tm() -> std::time::tm {
		let d = self/86400000;
		let dp = date_from_days(d as i32);
		let s = (self % 86400000)/1000;
		{ tm_sec: (s % 60) as i32,
		  tm_min: ((s/60) % 60) as i32,
		  tm_hour: (s/3600) as i32,
		  tm_mday: dp.mday,
		  tm_mon: dp.mon - 1,
		  tm_year: dp.year - 1900,
		  tm_wday: ((self + 1) % 7) as i32,
		  tm_yday: dp.yday,
		  tm_isdst: 0,
		  tm_gmtoff: 0,
		  tm_zone: ~"UTC",
		  tm_nsec: 1000000*(self % 1000) as i32
		}
	}

	fn from_tm(tm: std::time::tm) -> date_time {
		let d = days_from_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday);
		let s = tm.tm_hour as i64*3600 + tm.tm_min as i64*60 + tm.tm_sec as i64;
		(d as i64*86400000 + s*1000 + (tm.tm_nsec as i64)/1000000) as date_time
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
		let d = (self.sec + SECS_FROM_UNIX_EPOCH)/86400;
		let dp = date_from_days(d as i32);
		let s = (self.sec + SECS_FROM_UNIX_EPOCH) % 86400;
		{ tm_sec: (s % 60) as i32,
		  tm_min: ((s/60) % 60) as i32,
		  tm_hour: (s/3600) as i32,
		  tm_mday: dp.mday,
		  tm_mon: dp.mon - 1,
		  tm_year: dp.year - 1900,
		  tm_wday: ((d + 1) % 7) as i32,
		  tm_yday: dp.yday,
		  tm_isdst: 0,
		  tm_gmtoff: 0,
		  tm_zone: ~"UTC",
		  tm_nsec: self.nsec
		}
	}

	fn from_tm(tm: std::time::tm) -> date_time {
		let d = days_from_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday) as i64;
		let s = (tm.tm_hour as i64)*3600 + (tm.tm_min as i64)*60 + tm.tm_sec as i64;
		{ sec: d*86400 - SECS_FROM_UNIX_EPOCH + s, nsec: tm.tm_nsec } as date_time
	}
}

impl dtm for date_time {
	fn str() -> ~str {
		let tm = self.tm();
		#fmt("%s%s", tm.strftime(~"%Y-%m-%d %H:%M:%S"), if tm.tm_nsec != 0 { #fmt("%09i", tm.tm_nsec as int) } else { ~"" })
	}

	fn from_str(ds: ~str) -> result<date_time, ~str> {
		alt std::time::strptime(ds, ~"%Y-%m-%d %H:%M:%S") {
			ok(tm) { ok(({ sec: 0_i64, nsec: 0_i32 } as date_time).from_tm(tm)) }
			err(es) { err(copy es) }
		}
	}
}


#[cfg(test)]
mod tests {
	fn test_dt_str(s: ~str) {
		alt ({ sec: 0_i64, nsec: 0_i32 } as date_time).from_str(s) {
			ok(dt) {
				let dts = dt.str();
				if s != dts {
					log(error, ("test_dt_str", copy s, dts));
					fail
				}
			}
			err(es) {
				log(error, ("test_dt_str", copy s, copy es));
				fail
			}
		}
	}

	#[test]
	fn test_dt_limits() {
		test_dt_str(~"2012-05-07 09:56:33");
		test_dt_str(~"1000-01-01 00:00:00");
		test_dt_str(~"9999-12-31 23:59:59");
		test_dt_str(~"1900-02-28 23:59:59");
		test_dt_str(~"1900-03-01 00:00:00");
		test_dt_str(~"2000-02-29 23:59:59");
		test_dt_str(~"2000-03-01 00:00:00");
	}

	fn test_std_time(s: ~str) {
		alt ({ sec: 0_i64, nsec: 0_i32 } as date_time).from_str(s) {
			ok(dt) {
				let dtm = dt.tm();
				let stm = std::time::at_utc(dt.timespec());
				if stm != dtm {
					log(error, ("test_std_time", copy s, dtm, stm));
					fail
				}
				let dts = dt.timespec();
				let sts = dtm.to_timespec();
				if dts != sts {
					log(error, ("test_std_time", copy s, dts, sts));
					fail
				}
			}
			err(es) {
				log(error, ("test_std_time", copy s, copy es));
				fail
			}
		}
	}

	#[test]
	fn test_std_limits() {
		test_std_time(~"2012-05-07 09:56:33");
		test_std_time(~"1901-12-13 20:45:52");
		test_std_time(~"9999-12-31 23:59:59");
		test_std_time(~"2100-02-28 23:59:59");
		test_std_time(~"2100-03-01 00:00:00");
		test_std_time(~"2000-02-29 23:59:59");
		test_std_time(~"2000-03-01 00:00:00");
	}

	#[test]
	#[should_fail]
	fn test_std_bad_low_limit() {
		test_std_time(~"1901-12-13 20:45:51");
	}

	#[test]
	#[should_fail]
	fn test_bad_leap() {
		test_dt_str(~"2100-02-29 23:59:59");
	}

	#[test]
	#[should_fail]
	fn test_std_bad_hi_limit() {
		test_std_time(~"10000-01-01 00:00:00");
	}

	fn test_funcs(in: i32) {
		let dt = date_from_days(in);
		if dt.mon < 1 ||
		   dt.mon > 12 ||
		   dt.mday < 1 ||
		   dt.mday > month_length(dt.mon, leapyear(dt.year)) + 1 ||
		   dt.yday < 0 ||
		   dt.yday > 365 {
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
		let mut i = 0;
		while i < 3652060 {
			test_funcs(i);
			i += 1;
		}
	}
}
