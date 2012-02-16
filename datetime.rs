use std;

type date_parts = {year: u16, month: u8, day: u8, doy: u16};

type time_parts = {hour: int, minute: int, second: int, frac: int};

type date_time_parts = {date: date_funcs, time: time_funcs};

iface date_funcs {
	fn parts() -> date_parts;
	fn from_parts(parts: date_parts) -> date_funcs;
	fn days() -> u32;
	fn from_days(d: u32) -> date_funcs;
	fn str() -> str;
	fn from_str(ds: str) -> date_funcs;
	fn epcoh_date_str() -> str;
}

iface time_funcs {
	fn parts() -> time_parts;
	fn from_parts(parts: time_parts) -> time_funcs;
	fn str() -> str;
	fn from_str(ds: str) -> time_funcs;
	fn secs() -> int;
	fn from_secs(s: int) -> time_funcs;
	fn resolution() -> int;
}

iface date_time_funcs {
	fn date() -> date_funcs;
	fn time() -> time_funcs;
	fn str() -> str;
	fn from_str(ds: str) -> date_time_funcs;
}

fn leapyear(y: u16) -> bool { y % 4_u16 == 0_u16 && (y % 100_u16 != 0_u16 || y % 400_u16 == 0_u16) }

fn month_lookup(doy: u16, ly: bool) -> u8 {
	if ly {
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
	else {
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

fn accume_days(m: u8, ly: bool) -> u16 {
	if ly {
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
	else {
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

fn month_length(m: u8, ly: bool) -> u8 {
	if ly {
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
	else {
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

impl of date_funcs for u32 {
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

	fn from_parts(parts: date_parts) -> date_funcs {
		let y = parts.year;
		let m = parts.month;
		let d = parts.day;
		let ly = leapyear(y);
		assert y > 0_u16 && y < 10000_u16 && m > 0_u8 && m < 13_u8 && d > 0_u8 && d <= month_length(m, ly);
		let ym1 = y as u32 - 1_u32;
		(365_u32*ym1 + ym1/4_u32 - ym1/100_u32 + ym1/400_u32 + accume_days(m, ly) as u32 + d as u32 - 1_u32) as date_funcs
	}

	fn str() -> str {
		let parts = self.parts();
		#fmt("%04u-%02u-%02u", parts.year as uint, parts.month as uint, parts.day as uint)
	}

	fn from_str(ds: str) -> date_funcs {
		assert str::len(ds) == 10_u;
		let parts = str::split_char(ds, '-');
		assert vec::len(parts) == 3_u;
		let y = uint::from_str(parts[0]) as u16;
		let m = uint::from_str(parts[1]) as u8;
		let d = uint::from_str(parts[2]) as u8;
		(0_u32 as date_funcs).from_parts({year: y, month: m, day: d, doy: 0_u16})
	}

	fn days() -> u32 {
		self
	}

	fn from_days(d: u32) -> date_funcs {
		d as date_funcs
	}

	fn epcoh_date_str() -> str {
		"0001-01-01"
	}
}

impl of time_funcs for int {
	fn parts() -> time_parts {
		assert self >= 0 && self < 86400;
		{hour: self/3600, minute: self/60 % 60, second: self % 60, frac: 0}
	}

	fn from_parts(parts: time_parts) -> time_funcs {
		let h = parts.hour;
		let m = parts.minute;
		let s = parts.second;
		assert h >= 0 && h < 24 && m >= 0 && m < 60 && s >= 0 && s < 60;
		(3600*parts.hour + 60*parts.minute + parts.second) as time_funcs
	}

	fn str() -> str {
		let parts = self.parts();
		#fmt("%02d:%02d:%02d", parts.hour, parts.minute, parts.second)
	}

	fn secs() -> int {
		self as int
	}

	fn from_secs(s: int) -> time_funcs {
		s as time_funcs
	}

	fn from_str(ds: str) -> time_funcs {
		assert str::len(ds) == 8_u;
		let parts = str::split_char(ds, ':');
		assert vec::len(parts) == 3_u;
		let h = int::from_str(parts[0]);
		let m = int::from_str(parts[1]);
		let s = int::from_str(parts[2]);
		(0 as time_funcs).from_parts({hour: h, minute: m, second: s, frac: 0})
	}

	fn resolution() -> int {
		1
	}
}

impl of date_time_funcs for date_time_parts {
	fn date() -> date_funcs {
		self.date
	}

	fn time() -> time_funcs {
		self.time
	}

	fn str() -> str {
		#fmt("%s %s", self.date.str(), self.time.str())
	}

	fn from_str(ds: str) -> date_time_funcs {
		assert str::len(ds) == 19_u;
		let parts = str::split_char(ds, ' ');
		assert vec::len(parts) == 2_u;
		let d = (0_u32 as date_funcs).from_str(parts[0]);
		let t = (0 as time_funcs).from_str(parts[1]);
		{date: d, time: t} as date_time_funcs
	}
}

#[test]
fn test_all_dates() {
	let i = 0_u32;
	while i < 3652059_u32 {
		log(debug, i);
		let parts = i.parts();
		log(debug, parts);
		let x2 = (i as date_funcs).from_parts(parts);
		assert x2.days() == i;
		i += 1_u32;
	}
}

#[test]
fn test_date_str() {
	let x = (0_u32 as date_funcs).from_parts({year: 1_u16, month: 1_u8, day: 1_u8, doy: 1_u16});
	assert x.str() == "0001-01-01";
	assert (0_u32 as date_funcs).str() == "0001-01-01";
	let x = (0_u32 as date_funcs).from_parts({year: 9999_u16, month: 12_u8, day: 31_u8, doy: 1_u16});
	assert x.str() == "9999-12-31";
	assert (0_u32 as date_funcs).from_str("0001-01-01").str() == "0001-01-01";
	assert (0_u32 as date_funcs).from_str("0066-01-01").str() == "0066-01-01";
	assert (0_u32 as date_funcs).from_str("0077-01-01").str() == "0077-01-01";
	assert (0_u32 as date_funcs).from_str("0088-01-01").str() == "0088-01-01";
	assert (0_u32 as date_funcs).from_str("0099-01-01").str() == "0099-01-01";
	assert (0_u32 as date_funcs).from_str("0777-01-01").str() == "0777-01-01";
	assert (0_u32 as date_funcs).from_str("0888-01-01").str() == "0888-01-01";
	assert (0_u32 as date_funcs).from_str("9999-12-31").str() == "9999-12-31";
	assert (0_u32 as date_funcs).from_str("2000-02-29").str() == "2000-02-29";
}

#[test]
#[should_fail]
fn test_low_date_limit() {
	(-1 as u32).parts();
}

#[test]
#[should_fail]
fn test_high_date_limit() {
	3652059_u32.parts();
}

#[test]
#[should_fail]
fn test_bad_date_str1() {
	(0_u32 as date_funcs).from_str("1111-13-31").str();
}

#[test]
#[should_fail]
fn test_bad_date_str2() {
	(0_u32 as date_funcs).from_str("11x1-12-31").str();
}

#[test]
#[should_fail]
fn test_bad_date_str3() {
	(0_u32 as date_funcs).from_str("1111/13/31").str();
}

#[test]
#[should_fail]
fn test_bad_date_str4() {
	(0_u32 as date_funcs).from_str("1111-3-31").str();
}

#[test]
#[should_fail]
fn test_bad_date_str5() {
	(0_u32 as date_funcs).from_str("1111-02-31").str();
}

#[test]
#[should_fail]
fn test_bad_date_str6() {
	(0_u32 as date_funcs).from_str("1900-02-29").str();
}

#[test]
#[should_fail]
fn test_bad_date_str7() {
	(0_u32 as date_funcs).from_str("2100-02-29").str();
}

#[test]
fn test_all_times() {
	let i = 0;
	while i < 86400 {
		log(debug, i);
		let parts = i.parts();
		log(debug, parts);
		let x2 = (i as time_funcs).from_parts(parts);
		assert x2.secs() == i;
		i += 1;
	}
}

#[test]
fn test_time_str() {
	let x = (0 as time_funcs).from_parts({hour: 0, minute: 0, second: 0, frac: 0});
	assert x.str() == "00:00:00";
	assert (0 as time_funcs).str() == "00:00:00";
	let x = (0 as time_funcs).from_parts({hour: 23, minute: 59, second: 59, frac: 0});
	assert x.str() == "23:59:59";
}

#[test]
#[should_fail]
fn test_low_time_limit() {
	(-1).parts();
}

#[test]
#[should_fail]
fn test_time_date_limit() {
	86400.parts();
}

#[test]
fn test_date_time_str() {
	let dp = {date: 0_u32 as date_funcs, time: 0 as time_funcs};
	assert {date: 0_u32 as date_funcs, time: 0 as time_funcs}.str() == "0001-01-01 00:00:00";
	assert {date: 3652058_u32 as date_funcs, time: 86399 as time_funcs}.str() == "9999-12-31 23:59:59";
	assert dp.from_str("0001-01-01 00:00:00").str() == "0001-01-01 00:00:00";
	assert dp.from_str("9999-12-31 23:59:59").str() == "9999-12-31 23:59:59";
}

#[test]
#[should_fail]
fn test_bad_date_time_str1() {
	let dp = {date: 0_u32 as date_funcs, time: 0 as time_funcs};
	dp.from_str("9999-12-31T23:59:59");
}

#[test]
#[should_fail]
fn test_bad_date_time_str2() {
	let dp = {date: 0_u32 as date_funcs, time: 0 as time_funcs};
	dp.from_str("999-12-31 23:59:59");
}

#[test]
#[should_fail]
fn test_bad_date_time_str3() {
	let dp = {date: 0_u32 as date_funcs, time: 0 as time_funcs};
	dp.from_str("9999-12-31 23:59:9");
}

#[test]
#[should_fail]
fn test_bad_date_time_str4() {
	let dp = {date: 0_u32 as date_funcs, time: 0 as time_funcs};
	dp.from_str("9999-12-31 23:59:58.9");
}
