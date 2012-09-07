// NB: transitionary, de-mode-ing.
//#[forbid(deprecated_mode)];
#[forbid(deprecated_pattern)];

use std;

use std::time::Tm;
use std::time::Tm_;
use result::{Result, Ok, Err};

mod date {
	trait Date {
		pure fn timespec() -> std::time::Timespec;
		static pure fn from_timespec(ts: std::time::Timespec) -> self;
		pure fn tm() -> std::time::Tm;
		static pure fn from_tm(tm: &std::time::Tm) -> self;
	}
}

mod time {
	trait Time {
		pure fn timespec() -> std::time::Timespec;
		static pure fn from_timespec(ts: std::time::Timespec) -> self;
		pure fn tm() -> std::time::Tm;
		static pure fn from_tm(tm: &std::time::Tm) -> self;
	}
}

trait DateTime {
	pure fn timespec() -> std::time::Timespec;
	pure fn from_timespec(ts: std::time::Timespec) -> DateTime;
	pure fn tm() -> std::time::Tm;
	pure fn from_tm(tm: &std::time::Tm) -> DateTime;
}

trait DateStr {
	fn str() -> ~str;
	fn from_str(ds: &str) -> Result<DateTime, ~str>;
}

const SECS_FROM_UNIX_EPOCH: i64 = 62135596800;

#[inline(always)]
pure fn leapyear(y: i32) -> bool { y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) }

const month_lookup_vec: [i32]/365 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                     2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                                     3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                                     4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                                     5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                                     6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                                     7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
                                     8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
                                     9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                                     10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                                     11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
                                     12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12];

#[inline(always)]
pure fn month_lookup(doy: i32, ly: bool) -> i32 {
	let xtra = (ly && doy > 58) as i32;
	month_lookup_vec[doy - xtra]
}

const accume_days_vec: [i32]/13 = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

#[inline(always)]
pure fn accume_days(m: i32, ly: bool) -> i32 {
	let xtra = (ly && m > 2) as i32;
	accume_days_vec[m] + xtra
}

const month_length_vec: [i32]/13 = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[inline(always)]
pure fn month_length(m: i32, ly: bool) -> i32 {
	let xtra = (ly && m == 2) as i32;
	month_length_vec[m] + xtra
}

#[inline(always)]
pure fn date_from_days(days: i32) -> { year: i32, mon: i32, mday: i32, yday: i32} {
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

#[inline(always)]
pure fn days_from_date(y: i32, m: i32, d: i32) -> i32 {
	let ly = leapyear(y);
	let ym1 = y - 1;
	365*ym1 + ym1/4 - ym1/100 + ym1/400 + accume_days(m, ly) + d - 1
}

impl i32: date::Date {
	//  days since 0001-01-01
	pure fn timespec() -> std::time::Timespec {
		{ sec: self as i64*86400 - SECS_FROM_UNIX_EPOCH, nsec: 0 }
	}

	static pure fn from_timespec(ts: std::time::Timespec) -> i32 {
		((ts.sec + SECS_FROM_UNIX_EPOCH)/86400) as i32
	}

	pure fn tm() -> std::time::Tm {
		let dp = date_from_days(self);
		Tm_({ tm_sec: 0,
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
		})
	}

	static pure fn from_tm(tm: &std::time::Tm) -> i32 {
		days_from_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday)
	}
}

impl i64: time::Time {
	//  nanosecond resolution
	pure fn timespec() -> std::time::Timespec {
		{ sec: (self % 86400000000000)/1000000000, nsec: (self % 1000000000) as i32 }
	}

	static pure fn from_timespec(ts: std::time::Timespec) -> i64 {
		(ts.sec % 86400)*1000000000 + ts.nsec as i64
	}

	pure fn tm() -> std::time::Tm {
		let s = (self % 86400000000000)/1000000000;
		Tm_({ tm_sec: (s % 60) as i32,
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
		})
	}

	static pure fn from_tm(tm: &std::time::Tm) -> i64 {
		tm.tm_hour as i64*3600000000000 + tm.tm_min as i64*60000000000 + tm.tm_sec as i64*1000000000 + tm.tm_nsec as i64
	}
}

impl i64: DateTime {
	//  milliseconds since 0001-01-01
	pure fn timespec() -> std::time::Timespec {
		{ sec: self/1000 - SECS_FROM_UNIX_EPOCH, nsec: ((self % 1000)*1000000) as i32 }
	}

	pure fn from_timespec(ts: std::time::Timespec) -> DateTime {
		((ts.sec + SECS_FROM_UNIX_EPOCH)*1000 + (ts.nsec as i64)/1000000) as DateTime
	}

	pure fn tm() -> std::time::Tm {
		let d = self/86400000;
		let dp = date_from_days(d as i32);
		let s = (self % 86400000)/1000;
		Tm_({ tm_sec: (s % 60) as i32,
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
		})
	}

	pure fn from_tm(tm: &std::time::Tm) -> DateTime {
		let d = days_from_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday);
		let s = tm.tm_hour as i64*3600 + tm.tm_min as i64*60 + tm.tm_sec as i64;
		(d as i64*86400000 + s*1000 + (tm.tm_nsec as i64)/1000000) as DateTime
	}
}

impl std::time::Timespec: DateTime {
	pure fn timespec() -> std::time::Timespec {
		self
	}

	pure fn from_timespec(ts: std::time::Timespec) -> DateTime {
		ts as DateTime
	}

	pure fn tm() -> std::time::Tm {
		let d = (self.sec + SECS_FROM_UNIX_EPOCH)/86400;
		let dp = date_from_days(d as i32);
		let s = (self.sec + SECS_FROM_UNIX_EPOCH) % 86400;
		Tm_({ tm_sec: (s % 60) as i32,
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
		})
	}

	pure fn from_tm(tm: &std::time::Tm) -> DateTime {
		let d = days_from_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday) as i64;
		let s = (tm.tm_hour as i64)*3600 + (tm.tm_min as i64)*60 + tm.tm_sec as i64;
		{ sec: d*86400 - SECS_FROM_UNIX_EPOCH + s, nsec: tm.tm_nsec } as DateTime
	}
}

impl DateTime: DateStr {
	fn str() -> ~str {
		let tm = self.tm();
		fmt!("%s%s", tm.strftime("%Y-%m-%d %H:%M:%S"), if tm.tm_nsec != 0 { fmt!("%09i", tm.tm_nsec as int) } else { ~"" })
	}

	fn from_str(ds: &str) -> Result<DateTime, ~str> {
		match std::time::strptime(ds, "%Y-%m-%d %H:%M:%S") {
			Ok(ref tm) => { Ok(({ sec: 0_i64, nsec: 0_i32 } as DateTime).from_tm(tm)) }
			Err(ref es) => { Err(copy *es) }
		}
	}
}


#[cfg(test)]
mod tests {
	fn test_time(i: i64) {
		let tm = (i as time::Time).tm();
		let i2 = time::from_tm(&tm);
		if i2 != i {
			log(error, (~"test_time failed for:", i, i2, tm));
			fail
		}
		let ts = (i as time::Time).timespec();
		let i2 = time::from_timespec(ts);
		if i2 != i {
			log(error, (~"test_time failed for:", i, i2, ts));
			fail
		}
	}

	#[test]
	fn test_some_times() {
		test_time(0);
		test_time(1);
		test_time(-1);
		test_time(86399999999998);
		test_time(86399999999999);
		test_time(-86399999999999);
	}

	fn test_date(i: i32) {
		let tm = (i as date::Date).tm();
		let i2 = date::from_tm(&tm);
		if i2 != i {
			log(error, (~"test_date failed for:", i, i2, tm));
			fail
		}
		let ts = (i as date::Date).timespec();
		let i2 = date::from_timespec(ts);
		if i2 != i {
			log(error, (~"test_date failed for:", i, i2, ts));
			fail
		}
	}

	#[test]
	fn test_some_dates() {
		test_date(0);
		test_date(1);
		test_date(2147483646);
		test_date(2147483647);
	}

	fn test_dt_str(s: &str) {
		match ({ sec: 0_i64, nsec: 0_i32 } as DateTime).from_str(s) {
			Ok(dt) => {
				let dts = dt.str();
				if str::from_slice(s) != dts {
					log(error, (~"test_dt_str", str::from_slice(s), dts));
					fail
				}
			}
			Err(ref es) => {
				log(error, (~"test_dt_str", str::from_slice(s), copy *es));
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

	fn test_std_time(s: &str) {
		match ({ sec: 0_i64, nsec: 0_i32 } as DateTime).from_str(s) {
			Ok(dt) => {
				let dtm = dt.tm();
				let stm = std::time::at_utc(dt.timespec());
				if stm != dtm {
					log(error, (~"test_std_time", str::from_slice(s), dtm, stm));
					fail
				}
				let dts = dt.timespec();
				let sts = dtm.to_timespec();
				if dts != sts {
					log(error, (~"test_std_time", str::from_slice(s), dts, sts));
					fail
				}
			}
			Err(ref es) => {
				log(error, (~"test_std_time", str::from_slice(s), copy *es));
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
		if dt.mon < 1 ||
		   dt.mon > 12 ||
		   dt.mday < 1 ||
		   dt.mday > month_length(dt.mon, leapyear(dt.year)) + 1 ||
		   dt.yday < 0 ||
		   dt.yday > 365 {
			log(error, (~"test_funcs", in, dt));
			fail
		}
		let d = days_from_date(dt.year, dt.mon, dt.mday);
		if d != in {
			log(error, (~"test_funcs", in, dt, d));
			fail
		}
		log(debug, (~"test_funcs", in, ((in as date::Date).timespec() as DateTime).str()));
	}

	#[test]
	fn test_all_funcs() {
		let mplier = if os::getenv("RUST_BENCH").is_some() { 10 } else { 1 };
		let mut i = 0;
		while i < 3652060*mplier {
			test_funcs(i/mplier);
			i += 1;
		}
	}

	#[test]
	fn test_ml_perf() {
		let mplier = if os::getenv("RUST_BENCH").is_some() { 100 } else { 1 };
		let mut i = 0;
		while i < 10000000*mplier {
			let _ = month_lookup(i % 366, true);
			i += 1;
		}
	}
}
