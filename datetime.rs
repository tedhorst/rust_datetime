#![feature(phase)]
#[phase(syntax, link)] extern crate log;

extern crate time;

use time::{Tm, Timespec};

pub trait Date {
	fn timespec(&self) -> Timespec;
	fn from_timespec(ts: Timespec) -> Self;
	fn tm(&self) -> Tm;
	fn from_tm(tm: &Tm) -> Self;
}

pub trait Time {
	fn timespec(&self) -> Timespec;
	fn from_timespec(ts: Timespec) -> Self;
	fn tm(&self) -> Tm;
	fn from_tm(tm: &Tm) -> Self;
}

pub trait DateTime {
	fn timespec(&self) -> Timespec;
	fn from_timespec(ts: Timespec) -> Self;
	fn tm(&self) -> Tm;
	fn from_tm(tm: &Tm) -> Self;
}
/*
trait DateStr {
	fn str(&self) -> ~str;
	fn from_str(ds: &str) -> Result<Self, ~str>;
}
*/
static SECS_FROM_UNIX_EPOCH: i64 = 62135596800;

#[inline(always)]
pub fn leapyear(y: i32) -> bool { y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) }

static month_lookup_vec: [i32, ..365] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
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
pub fn month_lookup(doy: i32, ly: bool) -> i32 {
	let xtra = (ly && doy > 58) as i32;
	month_lookup_vec[doy - xtra]
}

static accume_days_vec: [i32, ..13] = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

#[inline(always)]
fn accume_days(m: i32, ly: bool) -> i32 {
	let xtra = (ly && m > 2) as i32;
	accume_days_vec[m] + xtra
}

static month_length_vec: [i32, ..13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[inline(always)]
pub fn month_length(m: i32, ly: bool) -> i32 {
	let xtra = (ly && m == 2) as i32;
	month_length_vec[m] + xtra
}

pub struct DateSpec { year: i32, mon: i32, mday: i32, yday: i32}

#[inline(always)]
pub fn date_from_days(days: i32) -> DateSpec {
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
	DateSpec { year: y, mon: m, mday: d, yday: doy}
}

#[inline(always)]
pub fn days_from_date(y: i32, m: i32, d: i32) -> i32 {
	let ly = leapyear(y);
	let ym1 = y - 1;
	365*ym1 + ym1/4 - ym1/100 + ym1/400 + accume_days(m, ly) + d - 1
}

impl Date for i32 {
	//  days since 0001-01-01
	fn timespec(&self) -> Timespec {
		Timespec { sec: *self as i64*86400 - SECS_FROM_UNIX_EPOCH, nsec: 0 }
	}

	fn from_timespec(ts: Timespec) -> i32 {
		((ts.sec + SECS_FROM_UNIX_EPOCH)/86400) as i32
	}

	fn tm(&self) -> Tm {
		let dp = date_from_days(*self);
		Tm { tm_sec: 0,
		  tm_min: 0,
		  tm_hour: 0,
		  tm_mday: dp.mday,
		  tm_mon: dp.mon - 1,
		  tm_year: dp.year - 1900,
		  tm_wday: (*self + 1) % 7,
		  tm_yday: dp.yday,
		  tm_isdst: 0,
		  tm_gmtoff: 0,
		  tm_zone: ~"UTC",
		  tm_nsec: 0
		}
	}

	fn from_tm(tm: &Tm) -> i32 {
		days_from_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday)
	}
}

impl Time for i64 {
	//  nanosecond resolution
	fn timespec(&self) -> Timespec {
		Timespec { sec: (*self % 86400000000000)/1000000000, nsec: (*self % 1000000000) as i32 }
	}

	fn from_timespec(ts: Timespec) -> i64 {
		(ts.sec % 86400)*1000000000 + ts.nsec as i64
	}

	fn tm(&self) -> Tm {
		let s = (*self % 86400000000000)/1000000000;
		Tm { tm_sec: (s % 60) as i32,
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
		  tm_nsec: (*self % 1000000000) as i32
		}
	}

	fn from_tm(tm: &Tm) -> i64 {
		tm.tm_hour as i64*3600000000000 + tm.tm_min as i64*60000000000 + tm.tm_sec as i64*1000000000 + tm.tm_nsec as i64
	}
}

impl DateTime for i64 {
	//  milliseconds since 0001-01-01
	fn timespec(&self) -> Timespec {
		Timespec { sec: *self/1000 - SECS_FROM_UNIX_EPOCH, nsec: ((*self % 1000)*1000000) as i32 }
	}

	fn from_timespec(ts: Timespec) -> i64 {
		(ts.sec + SECS_FROM_UNIX_EPOCH)*1000 + (ts.nsec as i64)/1000000
	}

	fn tm(&self) -> Tm {
		let d = *self/86400000;
		let dp = date_from_days(d as i32);
		let s = (*self % 86400000)/1000;
		Tm { tm_sec: (s % 60) as i32,
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
		  tm_nsec: 1000000*(*self % 1000) as i32
		}
	}

	fn from_tm(tm: &Tm) -> i64 {
		let d = days_from_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday);
		let s = tm.tm_hour as i64*3600 + tm.tm_min as i64*60 + tm.tm_sec as i64;
		d as i64*86400000 + s*1000 + (tm.tm_nsec as i64)/1000000
	}
}

impl DateTime for Timespec {
	fn timespec(&self) -> Timespec {
		*self
	}

	fn from_timespec(ts: Timespec) -> Timespec {
		ts
	}

	fn tm(&self) -> Tm {
		let d = ((*self).sec + SECS_FROM_UNIX_EPOCH)/86400;
		let dp = date_from_days(d as i32);
		let s = ((*self).sec + SECS_FROM_UNIX_EPOCH) % 86400;
		Tm { tm_sec: (s % 60) as i32,
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
		  tm_nsec: (*self).nsec
		}
	}

	fn from_tm(tm: &Tm) -> Timespec {
		let d = days_from_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday) as i64;
		let s = (tm.tm_hour as i64)*3600 + (tm.tm_min as i64)*60 + tm.tm_sec as i64;
		Timespec { sec: d*86400 - SECS_FROM_UNIX_EPOCH + s, nsec: tm.tm_nsec }
	}
}
/*
impl DateStr for Time {
	fn str(&self) -> ~str {
		let tm = self.tm();
		fmt!("%s%s", tm.strftime("%H:%M:%S"), if tm.tm_nsec != 0 { fmt!("%09i", tm.tm_nsec as int) } else { ~"" })
	}

	fn from_str(ds: &str) -> Result<Time, ~str> {
		match strptime(ds, "%H:%M:%S") {
			Ok(ref tm) => {
				let atime: i64 = Time::from_tm(tm);
				Ok(@atime as Time)
			}
			Err(ref es) => { Err(copy *es) }
		}
	}
}

impl DateStr for Date {
	fn str(&self) -> ~str {
		let tm = self.tm();
		tm.strftime("%Y-%m-%d")
	}

	fn from_str(ds: &str) -> Result<Date, ~str> {
		match strptime(ds, "%Y-%m-%d") {
			Ok(ref tm) => {
				let adate: i32 = Date::from_tm(tm);
				Ok(@adate as Date)
			}
			Err(ref es) => { Err(copy *es) }
		}
	}
}

impl DateStr for DateTime {
	fn str(&self) -> ~str {
		let tm = self.tm();
		fmt!("%s%s", tm.strftime("%Y-%m-%d %H:%M:%S"), if tm.tm_nsec != 0 { fmt!("%09i", tm.tm_nsec as int) } else { ~"" })
	}

	fn from_str(ds: &str) -> Result<DateTime, ~str> {
		match strptime(ds, "%Y-%m-%d %H:%M:%S") {
			Ok(ref tm) => {
				let ndt: Timespec = DateTime::from_tm(tm);
				Ok(@ndt as DateTime)
			}
			Err(ref es) => { Err(copy *es) }
		}
	}
}
*/
#[cfg(test)]
mod tests {
	extern crate test;
	use std::{os, fmt};
	use time::{Timespec, strptime};
	use super::DateTime;

	fn time_str<T: super::Time>(t: T) -> ~str {
		let tm = t.tm();
		format!("{}{}", tm.strftime("%H:%M:%S"), if tm.tm_nsec != 0 { format!("{:09i}", tm.tm_nsec as int) } else { ~"" })

	}

	fn date_str<T: super::Date>(t: T) -> ~str {
		let tm = t.tm();
		tm.strftime("%Y-%m-%d")

	}

	fn datetime_str<T: super::DateTime>(t: T) -> ~str {
		let tm = t.tm();
		format!("{}{}", tm.strftime("%Y-%m-%d %H:%M:%S"), if tm.tm_nsec != 0 { format!("{:09i}", tm.tm_nsec as int) } else { ~"" })

	}

	fn test_time<T: Eq + fmt::Show + Clone + ::Time>(i: T) {
		error!("{}, {}", i, time_str(i.clone()));
		let tm = i.tm();
		let i2: T = ::Time::from_tm(&tm);
		if i2 != i {
			fail!(format!("test_time failed for: {}, {}, {:?}", i, i2, tm))
		}
		let ts = i.timespec();
		let i2: T = ::Time::from_timespec(ts);
		if i2 != i {
			fail!(format!("test_time failed for: {}, {}, {:?}", i, i2, ts))
		}
	}

	#[test]
	fn test_some_times() {
		test_time(0_i64);
		test_time(1_i64);
		test_time(-1_i64);
		test_time(86399999999998_i64);
		test_time(86399999999999_i64);
		test_time(-86399999999999_i64);
	}

	fn test_date<T: Eq + fmt::Show + Clone + ::Date>(i: T) {
		error!("{} {}", i, date_str(i.clone()));
		let tm = i.tm();
		let i2: T = ::Date::from_tm(&tm);
		if i2 != i {
			fail!(format!("test_date failed for: {}, {}, {:?}", i, i2, tm))
		}
		let ts = i.timespec();
		let i2: T = ::Date::from_timespec(ts);
		if i2 != i {
			fail!(format!("test_date failed for: {}, {}, {:?}", i, i2, ts))
		}
	}

	#[test]
	fn test_some_dates() {
		test_date(0_i32);
		test_date(1_i32);
		test_date(2147483646_i32);
		test_date(2147483647_i32);
	}

	fn timespec_from_str(ds: &str) -> Result<Timespec, ~str> {
		match strptime(ds, "%Y-%m-%d %H:%M:%S") {
			Ok(ref tm) => {
				let ndt: Timespec = ::DateTime::from_tm(tm);
				Ok(ndt)
			}
			Err(ref es) => { Err(es.clone()) }
		}
	}

	fn test_dt_str(s: &str) {
		let tsdr = timespec_from_str(s);
		match tsdr {
			Ok(dt) => {
				let dts = datetime_str(dt);
				if s != dts {
					fail!(format!("test_dt_str: {}, {}", s, dts))
				}
			}
			Err(ref es) => {
				fail!(format!("test_dt_str: {}, {:?}", s, es))
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
		let tsr = timespec_from_str(s);
		match tsr {
			Ok(gdt) => {
				let dts = gdt.timespec();
				let dtm = dts.tm();
				let stm = ::time::at_utc(dts);
				if stm != dtm {
					fail!(format!("test_std_time: {}, {:?}, {:?}", s, dtm, stm))
				}
				let sts = dtm.to_timespec();
				if dts != sts {
					fail!(format!("test_std_time: {}, {:?}, {:?}", s, dts, sts))
				}
			}
			Err(ref es) => {
				fail!(format!("test_std_time: {}, {:?}", s, es))
			}
		}
		let ir = timespec_from_str(s);
		match ir {
			Ok(dt) => {
				let dts = dt.timespec();
				let dtm = dt.tm();
				let stm = ::time::at_utc(dts);
				if stm != dtm {
					fail!(format!("test_std_time i64: {}, {:?}, {:?}", s, dtm, stm))
				}
				let sts = dtm.to_timespec();
				if dts != sts {
					fail!(format!("test_std_time i64: {}, {:?}, {:?}", s, dts, sts))
				}
			}
			Err(ref es) => {
				fail!(format!("test_std_time i64: {}, {:?}", s, es))
			}
		}
	}

	#[test]
	fn test_std_limits() {
		test_std_time("2012-05-07 09:56:33");
		test_std_time("2012-05-08 09:56:32");
		test_std_time("2012-05-09 09:56:31");
		test_std_time("2012-05-10 09:56:30");
		test_std_time("2012-05-11 09:56:29");
		test_std_time("2012-05-12 09:56:28");
		test_std_time("2012-05-13 09:56:27");
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

	fn test_funcs(inp: i32) {
		let dt = ::date_from_days(inp);
		if dt.mon < 1 ||
		   dt.mon > 12 ||
		   dt.mday < 1 ||
		   dt.mday > ::month_length(dt.mon, ::leapyear(dt.year)) + 1 ||
		   dt.yday < 0 ||
		   dt.yday > 365 {
			fail!(format!("test_funcs:, {}, {:?}", inp, dt))
		}
		let d = ::days_from_date(dt.year, dt.mon, dt.mday);
		if d != inp {
			fail!(format!("test_funcs: {}, {:?}, {}", inp, dt, d))
		}
		debug!("test_funcs {} {}", inp, date_str(inp));
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

	#[bench]
	fn test_ml_perf(b: &mut test::BenchHarness) {
		b.iter(|| {
			let mut i = 0;
			while i <= 366 {
				let _ = ::month_lookup(i % 366, true);
				let _ = ::month_lookup(i % 365, false);
				i += 1;
			}
		})
	}
}
