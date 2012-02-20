use std;

type date_parts = {year: u16, month: u8, day: u8, doy: u16};

type time_parts = {hour: u8, minute: u8, second: u8, frac: u32};

type date_time_parts = {date: date, time: time};

iface date {
	fn parts() -> date_parts;
	fn from_parts(parts: date_parts) -> date;
	fn days() -> u32;
	fn from_days(d: u32) -> date;
	fn str() -> str;
	fn from_str(ds: str) -> date;
	fn epcoh_date_str() -> str;
}

iface time {
	fn parts() -> time_parts;
	fn from_parts(parts: time_parts) -> time;
	fn str() -> str;
	fn from_str(ds: str) -> time;
	fn secs() -> f64;
	fn from_secs(s: f64) -> time;
	fn millis() -> u32;
	fn from_millis(ms: u32) -> time;
	fn resolution() -> u32;
}

iface date_time {
	fn date() -> date;
	fn time() -> time;
	fn str() -> str;
	fn from_str(ds: str) -> date_time;
}

fn leapyear(y: u16) -> bool { y % 4_u16 == 0_u16 && (y % 100_u16 != 0_u16 || y % 400_u16 == 0_u16) }

fn month_lookup(doy: u16, ly: bool) -> u8 {
	alt ly {
		true {
			alt check doy {
				1_u16 to 31_u16 { 1_u8 }
				32_u16 to 60_u16 { 2_u8 }
				61_u16 to 91_u16 { 3_u8 }
				92_u16 to 121_u16 { 4_u8 }
				122_u16 to 152_u16 { 5_u8 }
				153_u16 to 182_u16 { 6_u8 }
				183_u16 to 213_u16 { 7_u8 }
				214_u16 to 244_u16 { 8_u8 }
				245_u16 to 274_u16 { 9_u8 }
				275_u16 to 305_u16 { 10_u8 }
				306_u16 to 335_u16 { 11_u8 }
				336_u16 to 366_u16 { 12_u8 }
			}
		}
		false {
			alt check doy {
				1_u16 to 31_u16 { 1_u8 }
				32_u16 to 59_u16 { 2_u8 }
				60_u16 to 90_u16 { 3_u8 }
				91_u16 to 120_u16 { 4_u8 }
				121_u16 to 151_u16 { 5_u8 }
				152_u16 to 181_u16 { 6_u8 }
				182_u16 to 212_u16 { 7_u8 }
				213_u16 to 243_u16 { 8_u8 }
				244_u16 to 273_u16 { 9_u8 }
				274_u16 to 304_u16 { 10_u8 }
				305_u16 to 334_u16 { 11_u8 }
				335_u16 to 365_u16 { 12_u8 }
			}
		}
	}
}

fn accume_days(m: u8, ly: bool) -> u16 {
	alt ly {
		true {
			alt check m {
				1_u8 { 0_u16 }
				2_u8 { 31_u16 }
				3_u8 { 60_u16 }
				4_u8 { 91_u16 }
				5_u8 { 121_u16 }
				6_u8 { 152_u16 }
				7_u8 { 182_u16 }
				8_u8 { 213_u16 }
				9_u8 { 244_u16 }
				10_u8 { 274_u16 }
				11_u8 { 305_u16 }
				12_u8 { 335_u16 }
			}
		}
		false {
			alt check m {
				1_u8 { 0_u16 }
				2_u8 { 31_u16 }
				3_u8 { 59_u16 }
				4_u8 { 90_u16 }
				5_u8 { 120_u16 }
				6_u8 { 151_u16 }
				7_u8 { 181_u16 }
				8_u8 { 212_u16 }
				9_u8 { 243_u16 }
				10_u8 { 273_u16 }
				11_u8 { 304_u16 }
				12_u8 { 334_u16 }
			}
		}
	}
}

fn month_length(m: u8, ly: bool) -> u8 {
	alt ly {
		true {
			alt check m {
				1_u8 { 31_u8 }
				2_u8 { 29_u8 }
				3_u8 { 31_u8 }
				4_u8 { 30_u8 }
				5_u8 { 31_u8 }
				6_u8 { 30_u8 }
				7_u8 { 31_u8 }
				8_u8 { 31_u8 }
				9_u8 { 30_u8 }
				10_u8 { 31_u8 }
				11_u8 { 30_u8 }
				12_u8 { 31_u8 }
			}
		}
		false {
			alt check m {
				1_u8 { 31_u8 }
				2_u8 { 28_u8 }
				3_u8 { 31_u8 }
				4_u8 { 30_u8 }
				5_u8 { 31_u8 }
				6_u8 { 30_u8 }
				7_u8 { 31_u8 }
				8_u8 { 31_u8 }
				9_u8 { 30_u8 }
				10_u8 { 31_u8 }
				11_u8 { 30_u8 }
				12_u8 { 31_u8 }
			}
		}
	}
}

impl of date for u32 {
	fn parts() -> date_parts {
		assert self >= 0_u32 && self < 3652059_u32;
		let n400 = self/146097_u32;
		let d1 = self % 146097_u32;
		let n100 = d1/36524_u32;
		let d2 = d1 % 36524_u32;
		let n4 = d2/1461_u32;
		let d3 = d2 % 1461_u32;
		let n1 = d3/365_u32;
		let xtra = if n100 == 4_u32 || n1 == 4_u32 {
			1_u32
		}
		else {
			0_u32
		};
		let y = (400_u32*n400 + 100_u32*n100 + 4_u32*n4 + n1 + 1_u32 - xtra) as u16;
		let doy = (d3 % 365_u32 + 365_u32*xtra + 1_u32) as u16;
		let ly = leapyear(y);
		let m = month_lookup(doy, ly);
		let d = doy - accume_days(m, ly);
		{year: y, month: m as u8, day: d as u8, doy: doy as u16}
	}

	fn from_parts(parts: date_parts) -> date {
		let y = parts.year;
		let m = parts.month;
		let d = parts.day;
		let ly = leapyear(y);
		assert y > 0_u16 && y < 10000_u16 && m > 0_u8 && m < 13_u8 && d > 0_u8 && d <= month_length(m, ly);
		let ym1 = y as u32 - 1_u32;
		(365_u32*ym1 + ym1/4_u32 - ym1/100_u32 + ym1/400_u32 + accume_days(m, ly) as u32 + d as u32 - 1_u32) as date
	}

	fn str() -> str {
		let parts = (self as date).parts();
		#fmt("%04u-%02u-%02u", parts.year as uint, parts.month as uint, parts.day as uint)
	}

	fn from_str(ds: str) -> date {
		assert str::len(ds) == 10_u;
		let parts = str::split_char(ds, '-');
		assert vec::len(parts) == 3_u;
		let y = uint::from_str(parts[0]) as u16;
		let m = uint::from_str(parts[1]) as u8;
		let d = uint::from_str(parts[2]) as u8;
		(0_u32 as date).from_parts({year: y, month: m, day: d, doy: 0_u16})
	}

	fn days() -> u32 {
		self
	}

	fn from_days(d: u32) -> date {
		assert d >= 0_u32 && d < 3652059_u32;
		d as date
	}

	fn epcoh_date_str() -> str {
		"0001-01-01"
	}
}

impl of time for u32 {
	fn parts() -> time_parts {
		let r = self.resolution();
		assert self >= 0_u32 && self < 86400_u32*r;
		{hour: (self/3600_u32/r) as u8, minute: (self/60_u32/r % 60_u32) as u8, second: (self/r % 60_u32) as u8, frac: self % r}
	}

	fn from_parts(parts: time_parts) -> time {
		let h = parts.hour as u32;
		let m = parts.minute as u32;
		let s = parts.second as u32;
		let f = parts.frac as u32;
		let r = self.resolution();
		assert h >= 0_u32 && h < 24_u32 && m >= 0_u32 && m < 60_u32 && s >= 0_u32 && s < 60_u32 && f >= 0_u32 && f < r;
		(r*(3600_u32*h + 60_u32*m + s) + f) as time
	}

	fn str() -> str {
		let parts = (self as time).parts();
		#fmt("%02u:%02u:%02u%s", parts.hour as uint, parts.minute as uint, parts.second as uint, if parts.frac == 0_u32 {""} else { #fmt(".%03u", parts.frac as uint) })
	}

	fn secs() -> f64 {
		self as f64/(self.resolution() as f64)
	}

	fn from_secs(s: f64) -> time {
		assert s >= 0. && s < 86400.;
		((self.resolution() as f64)*s) as u32 as time
	}

	fn millis() -> u32 {
		self
	}

	fn from_millis(ms: u32) -> time {
		assert ms >= 0_u32 && ms < 86400000_u32;
		ms as time
	}

	fn from_str(ds: str) -> time {
		let sl = str::len(ds);
		assert sl == 8_u || sl == 12_u;
		let parts = str::split_char(ds, ':');
		assert vec::len(parts) == 3_u;
		let h = uint::from_str(parts[0]) as u8;
		let m = uint::from_str(parts[1]) as u8;
		let fss = str::split_char(parts[2], '.');
		let s = uint::from_str(fss[0]) as u8;
		let f = if vec::len(fss) == 2_u {
			uint::from_str(fss[1]) as u32
		}
		else {
			0_u32
		};
		(0_u32 as time).from_parts({hour: h, minute: m, second: s, frac: f})
	}

	fn resolution() -> u32 {
		1000_u32
	}
}

impl of date_time for date_time_parts {
	fn date() -> date {
		self.date
	}

	fn time() -> time {
		self.time
	}

	fn str() -> str {
		#fmt("%s %s", self.date.str(), self.time.str())
	}

	fn from_str(ds: str) -> date_time {
		let parts = str::split_char(ds, ' ');
		assert vec::len(parts) == 2_u;
		let d = (0_u32 as date).from_str(parts[0]);
		let t = (0_u32 as time).from_str(parts[1]);
		{date: d, time: t} as date_time
	}
}

impl of date_time for u64 {
	fn date() -> date {
		((self/86400000_u64) as u32) as date
	}

	fn time() -> time {
		((self % 86400000_u64) as u32) as time
	}

	fn str() -> str {
		#fmt("%s %s", self.date().str(), self.time().str())
	}

	fn from_str(ds: str) -> date_time {
		let parts = str::split_char(ds, ' ');
		assert vec::len(parts) == 2_u;
		let d = (0_u32 as date).from_str(parts[0]);
		let t = (0_u32 as time).from_str(parts[1]);
		((d.days() as u64)*86400000_u64 + (t.millis() as u64)) as date_time
	}
}

const SECS_FROM_UNIX_EPOCH: u64 = 62135596800_u64;

impl of date_time for std::time::timeval {
	fn date() -> date {
		(((self.sec as u64 + SECS_FROM_UNIX_EPOCH)/86400_u64) as u32) as date
	}

	fn time() -> time {
		((self.sec % 86400_u32)*1000_u32 + self.usec/1000_u32) as time
	}

	fn str() -> str {
		#fmt("%s %s", self.date().str(), self.time().str())
	}

	fn from_str(ds: str) -> date_time {
		let parts = str::split_char(ds, ' ');
		assert vec::len(parts) == 2_u;
		let d = (0_u32 as date).from_str(parts[0]);
		let t = (0_u32 as time).from_str(parts[1]);
		{sec: ((d.days() as u64)*86400_u64 - SECS_FROM_UNIX_EPOCH) as u32 + t.millis()/1000_u32, usec: (t.millis() % 1000_u32)*1000_u32} as date_time
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_all_dates() {
		let i = 0_u32;
		let d = i as date;
		while i < 3652059_u32 {
			log(debug, i);
			d = i as date;
			let parts = d.parts();
			log(debug, parts);
			let x2 = d.from_parts(parts);
			assert x2.days() == i;
			i += 1_u32;
		}
		log(error, #fmt("tested %u dates, ending with: %s", i as uint, d.str()));
	}

	#[test]
	fn test_date_str() {
		assert (0_u32 as date).str() == "0001-01-01";
		assert (3652058_u32 as date).str() == "9999-12-31";
		assert (0_u32 as date).from_parts({year: 1_u16, month: 1_u8, day: 1_u8, doy: 1_u16}).str() == "0001-01-01";
		assert (0_u32 as date).from_parts({year: 9999_u16, month: 12_u8, day: 31_u8, doy: 1_u16}).str() == "9999-12-31";
		for ds in ["0001-01-01", "0001-01-02", "0001-01-31", "0001-02-28", "0001-03-01", "0001-12-31", "0066-01-01", "0077-01-01", "0088-01-01", "0099-01-01", "0777-01-01", "0888-01-01", "2000-02-29", "9999-12-31"] {
			assert (0_u32 as date).from_str(ds).str() == ds;
		}
	}

	#[test]
	#[should_fail]
	fn test_low_date_limit() {
		(-1 as u32 as date).parts();
	}

	#[test]
	#[should_fail]
	fn test_high_date_limit() {
		(3652059_u32 as date).parts();
	}

	#[test]
	#[should_fail]
	fn test_bad_date_str1() {
		(0_u32 as date).from_str("1111-13-31").str();
	}

	#[test]
	#[should_fail]
	fn test_bad_date_str2() {
		(0_u32 as date).from_str("11x1-12-31").str();
	}

	#[test]
	#[should_fail]
	fn test_bad_date_str3() {
		(0_u32 as date).from_str("1111/12/31").str();
	}

	#[test]
	#[should_fail]
	fn test_bad_date_str4() {
		(0_u32 as date).from_str("1111-3-31").str();
	}

	#[test]
	#[should_fail]
	fn test_bad_date_str5() {
		(0_u32 as date).from_str("1111-02-31").str();
	}

	#[test]
	#[should_fail]
	fn test_bad_date_str6() {
		(0_u32 as date).from_str("1900-02-29").str();
	}

	#[test]
	#[should_fail]
	fn test_bad_date_str7() {
		(0_u32 as date).from_str("2100-02-29").str();
	}

	#[test]
	fn test_all_times() {
		let rng = std::rand::mk_rng();
		let cnt = 0_u;
		let i = 0_u32;
		let t = i as time;
		while i < 86400000_u32 {
			log(debug, i);
			t = i as time;
			let parts = t.parts();
			log(debug, parts);
			let t2 = t.from_parts(parts);
			let i2 = t2.millis();
			assert i2 == i;
			i += rng.next() % 100_u32;
			cnt += 1_u;
		}
		log(error, #fmt("tested %u times, ending with: %s", cnt, t.str()));
	}

	#[test]
	fn test_time_str() {
		let x = (0_u32 as time).from_parts({hour: 0_u8, minute: 0_u8, second: 0_u8, frac: 0_u32});
		assert x.str() == "00:00:00";
		assert (0_u32 as time).str() == "00:00:00";
		assert (1_u32 as time).str() == "00:00:00.001";
		let x = (0_u32 as time).from_parts({hour: 23_u8, minute: 59_u8, second: 59_u8, frac: 999_u32});
		assert x.str() == "23:59:59.999";
	}

	#[test]
	#[should_fail]
	fn test_low_time_limit() {
		(-1 as u32 as time).parts();
	}

	#[test]
	#[should_fail]
	fn test_high_time_limit() {
		(86400000_u32 as time).parts();
	}

	#[test]
	#[should_fail]
	fn test_bad_time_str1() {
		(0_u32 as time).from_str("2100-02-28");
	}

	#[test]
	#[should_fail]
	fn test_bad_time_str2() {
		(0_u32 as time).from_str("24:22:11");
	}

	#[test]
	#[should_fail]
	fn test_bad_time_str3() {
		(0_u32 as time).from_str("20:60:11");
	}

	#[test]
	#[should_fail]
	fn test_bad_time_str4() {
		(0_u32 as time).from_str("20:22:60");
	}

	#[test]
	#[should_fail]
	fn test_bad_time_str5() {
		(0_u32 as time).from_str("20:22:11.33");
	}

	#[test]
	fn test_all_date_times() {
		let rng = std::rand::mk_rng();
		let cnt = 0_u;
		let i = 0_u64;
		let dt = i as date_time;
		while i < 315537897600000_u64 {
			dt = i as date_time;
			let d = dt.date();
			let t = dt.time();
			let i2 = ((d.days() as u64)*86400000_u64) + (t.millis() as u64);
			if i != i2 {
				log(error, (dt.str(), i, i2, d.days(), t.millis()));
				fail
			}
			i += ((rng.next() % 86400000_u32) as u64) + 86400000_u64*((rng.next() % 7_u32) as u64);
			cnt += 1_u;
		}
		log(error, #fmt("tested %u date times, ending with: %s", cnt, dt.str()));
	}

	#[test]
	fn test_now() {
		let dt = std::time::get_time() as date_time;
		log(error, dt);
		let s = dt.str();
		log(error, s);
		let dt2 = dt.from_str(s);
		log(error, dt2);
		let s2 = dt2.str();
		log(error, s2);
		assert s == s2;
	}

	#[test]
	fn test_timeval_limits() {
		let dt = {sec: u32::max_value, usec: u32::max_value} as date_time;
		log(error, dt);
		let s = dt.str();
		log(error, s);
		let dt = {sec: 0_u32, usec: 0_u32} as date_time;
		assert dt.str() == "1970-01-01 00:00:00";
	}

	#[test]
	fn test_date_time_str() {
		let dp = {date: 0_u32 as date, time: 0_u32 as time};
		assert dp.str() == "0001-01-01 00:00:00";
		assert (0_u64 as date_time).str() == "0001-01-01 00:00:00";
		assert {date: 3652058_u32 as date, time: 86399999_u32 as time}.str() == "9999-12-31 23:59:59.999";
		assert (315537897599999_u64 as date_time).str() == "9999-12-31 23:59:59.999";
		assert dp.from_str("0001-01-01 00:00:00").str() == "0001-01-01 00:00:00";
		assert dp.from_str("9999-12-31 23:59:59.999").str() == "9999-12-31 23:59:59.999";
	}

	#[test]
	#[should_fail]
	fn test_bad_date_time_str1() {
		let dp = {date: 0_u32 as date, time: 0_u32 as time};
		dp.from_str("9999-12-31T23:59:59");
	}

	#[test]
	#[should_fail]
	fn test_bad_date_time_str2() {
		let dp = {date: 0_u32 as date, time: 0_u32 as time};
		dp.from_str("999-12-31 23:59:59");
	}

	#[test]
	#[should_fail]
	fn test_bad_date_time_str3() {
		let dp = {date: 0_u32 as date, time: 0_u32 as time};
		dp.from_str("9999-12-31 23:59:9");
	}

	#[test]
	#[should_fail]
	fn test_bad_date_time_str4() {
		let dp = {date: 0_u32 as date, time: 0_u32 as time};
		dp.from_str("9999-12-31 23:59:58.9");
	}
}
