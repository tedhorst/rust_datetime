use std;

type date_parts = {year: int, month: int, day: int, doy: int};

type time_parts = {hour: int, minute: int, second: int, frac: int};

type date_time_parts = {date: date_funcs, time: time_funcs};

iface date_funcs {
	fn date_parts() -> date_parts;
	fn from_parts(parts: date_parts) -> date_funcs;
	fn days_since_epoch() -> int;
	fn to_str() -> str;
	fn from_str(ds: str) -> date_funcs;
	fn epcoh_date_str() -> str;
}

iface time_funcs {
	fn time_parts() -> time_parts;
	fn from_parts(parts: time_parts) -> time_funcs;
	fn to_str() -> str;
	fn from_str(ds: str) -> time_funcs;
	fn second_count() -> int;
	fn resolution() -> int;
}

iface date_time_funcs {
	fn date() -> date_funcs;
	fn time() -> time_funcs;
	fn to_str() -> str;
	fn from_str(ds: str) -> date_time_funcs;
}

fn leapyear(y: int) -> bool { y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) }

fn month_lookup(doy: int, ly: bool) -> int {
	if ly {
		alt doy {
			1 to 31 { 1 }
			32 to 60 { 2 }
			61 to 91 { 3 }
			92 to 121 { 4 }
			122 to 152 { 5 }
			153 to 182 { 6 }
			183 to 213 { 7 }
			214 to 244 { 8 }
			245 to 274 { 9 }
			275 to 305 { 10 }
			306 to 335 { 11 }
			336 to 366 { 12 }
			_ { fail }
		}
	}
	else {
		alt doy {
			1 to 31 { 1 }
			32 to 59 { 2 }
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
			_ { fail }
		}
	}
}

fn accume_days(m: int, ly: bool) -> int {
	if ly {
		alt m {
			1 { 0 }
			2 { 31 }
			3 { 60 }
			4 { 91 }
			5 { 121 }
			6 { 152 }
			7 { 182 }
			8 { 213 }
			9 { 244 }
			10 { 274 }
			11 { 305 }
			12 { 335 }
			_ { fail }
		}
	}
	else {
		alt m {
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
			_ { fail }
		}
	}
}

fn month_length(m: int, ly: bool) -> int {
	if ly {
		alt m {
			1 { 31 }
			2 { 29 }
			3 { 31 }
			4 { 30 }
			5 { 31 }
			6 { 30 }
			7 { 31 }
			8 { 31 }
			9 { 30 }
			10 { 31 }
			11 { 30 }
			12 { 31 }
			_ { fail }
		}
	}
	else {
		alt m {
			1 { 31 }
			2 { 28 }
			3 { 31 }
			4 { 30 }
			5 { 31 }
			6 { 30 }
			7 { 31 }
			8 { 31 }
			9 { 30 }
			10 { 31 }
			11 { 30 }
			12 { 31 }
			_ { fail }
		}
	}
}

impl of date_funcs for int {
	fn date_parts() -> date_parts {
		assert self >= 0 && self < 3652059;
		let n400 = self/146097;
		let d1 = self % 146097;
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
		let doy = d3 % 365 + 365*xtra + 1;
		let ly = leapyear(y);
		let m = month_lookup(doy, ly);
		let d = doy - accume_days(m, ly);
		{year: y, month: m, day: d, doy: doy}
	}

	fn from_parts(parts: date_parts) -> date_funcs {
		let y = parts.year;
		let m = parts.month;
		let d = parts.day;
		let ly = leapyear(y);
		assert y > 0 && y < 10000 && m > 0 && m < 13 && d > 0 && d <= month_length(m, ly);
		let ym1 = y - 1;
		(365*ym1 + ym1/4 - ym1/100 + ym1/400 + accume_days(m, ly) + d - 1) as date_funcs
	}

	fn to_str() -> str {
		let parts = self.date_parts();
		#fmt("%04d-%02d-%02d", parts.year, parts.month, parts.day)
	}

	fn from_str(ds: str) -> date_funcs {
		assert str::len(ds) == 10_u;
		let parts = str::split_char(ds, '-');
		assert vec::len(parts) == 3_u;
		let y = int::from_str(parts[0]);
		let m = int::from_str(parts[1]);
		let d = int::from_str(parts[2]);
		(0 as date_funcs).from_parts({year: y, month: m, day: d, doy: 0})
	}

	fn days_since_epoch() -> int {
		self as int
	}

	fn epcoh_date_str() -> str {
		"0001-01-01"
	}
}

impl of time_funcs for int {
	fn time_parts() -> time_parts {
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

	fn to_str() -> str {
		let parts = self.time_parts();
		#fmt("%02d:%02d:%02d", parts.hour, parts.minute, parts.second)
	}

	fn second_count() -> int {
		self as int
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

	fn to_str() -> str {
		#fmt("%s %s", self.date.to_str(), self.time.to_str())
	}

	fn from_str(ds: str) -> date_time_funcs {
		assert str::len(ds) == 19_u;
		let parts = str::split_char(ds, ' ');
		assert vec::len(parts) == 2_u;
		let d = (0 as date_funcs).from_str(parts[0]);
		let t = (0 as time_funcs).from_str(parts[1]);
		{date: d, time: t} as date_time_funcs
	}
}

#[test]
fn test_all_dates() {
	let i = 0;
	while i < 3652059 {
		log(debug, i);
		let parts = i.date_parts();
		log(debug, parts);
		let x2 = (i as date_funcs).from_parts(parts);
		assert x2.days_since_epoch() == i;
		i += 1;
	}
}

#[test]
fn test_date_str() {
	let x = (0 as date_funcs).from_parts({year: 1, month: 1, day: 1, doy: 1});
	assert x.to_str() == "0001-01-01";
	assert (0 as date_funcs).to_str() == "0001-01-01";
	let x = (0 as date_funcs).from_parts({year: 9999, month: 12, day: 31, doy: 1});
	assert x.to_str() == "9999-12-31";
	assert (0 as date_funcs).from_str("0001-01-01").to_str() == "0001-01-01";
	assert (0 as date_funcs).from_str("0066-01-01").to_str() == "0066-01-01";
	assert (0 as date_funcs).from_str("0077-01-01").to_str() == "0077-01-01";
	assert (0 as date_funcs).from_str("0088-01-01").to_str() == "0088-01-01";
	assert (0 as date_funcs).from_str("0099-01-01").to_str() == "0099-01-01";
	assert (0 as date_funcs).from_str("0777-01-01").to_str() == "0777-01-01";
	assert (0 as date_funcs).from_str("0888-01-01").to_str() == "0888-01-01";
	assert (0 as date_funcs).from_str("9999-12-31").to_str() == "9999-12-31";
	assert (0 as date_funcs).from_str("2000-02-29").to_str() == "2000-02-29";
}

#[test]
#[should_fail]
fn test_low_date_limit() {
	(-1).date_parts();
}

#[test]
#[should_fail]
fn test_high_date_limit() {
	3652059.date_parts();
}

#[test]
#[should_fail]
fn test_bad_date_str1() {
	(0 as date_funcs).from_str("1111-13-31").to_str();
}

#[test]
#[should_fail]
fn test_bad_date_str2() {
	(0 as date_funcs).from_str("11x1-12-31").to_str();
}

#[test]
#[should_fail]
fn test_bad_date_str3() {
	(0 as date_funcs).from_str("1111/13/31").to_str();
}

#[test]
#[should_fail]
fn test_bad_date_str4() {
	(0 as date_funcs).from_str("1111-3-31").to_str();
}

#[test]
#[should_fail]
fn test_bad_date_str5() {
	(0 as date_funcs).from_str("1111-02-31").to_str();
}

#[test]
#[should_fail]
fn test_bad_date_str6() {
	(0 as date_funcs).from_str("1900-02-29").to_str();
}

#[test]
#[should_fail]
fn test_bad_date_str7() {
	(0 as date_funcs).from_str("2100-02-29").to_str();
}

#[test]
fn test_all_times() {
	let i = 0;
	while i < 86400 {
		log(debug, i);
		let parts = i.time_parts();
		log(debug, parts);
		let x2 = (i as time_funcs).from_parts(parts);
		assert x2.second_count() == i;
		i += 1;
	}
}

#[test]
fn test_time_str() {
	let x = (0 as time_funcs).from_parts({hour: 0, minute: 0, second: 0, frac: 0});
	assert x.to_str() == "00:00:00";
	assert (0 as time_funcs).to_str() == "00:00:00";
	let x = (0 as time_funcs).from_parts({hour: 23, minute: 59, second: 59, frac: 0});
	assert x.to_str() == "23:59:59";
}

#[test]
#[should_fail]
fn test_low_time_limit() {
	(-1).time_parts();
}

#[test]
#[should_fail]
fn test_time_date_limit() {
	86400.time_parts();
}

#[test]
fn test_date_time_str() {
	let dp = {date: 0 as date_funcs, time: 0 as time_funcs};
	assert {date: 0 as date_funcs, time: 0 as time_funcs}.to_str() == "0001-01-01 00:00:00";
	assert {date: 3652058 as date_funcs, time: 86399 as time_funcs}.to_str() == "9999-12-31 23:59:59";
	assert dp.from_str("0001-01-01 00:00:00").to_str() == "0001-01-01 00:00:00";
	assert dp.from_str("9999-12-31 23:59:59").to_str() == "9999-12-31 23:59:59";
}

#[test]
#[should_fail]
fn test_bad_date_time_str1() {
	let dp = {date: 0 as date_funcs, time: 0 as time_funcs};
	dp.from_str("9999-12-31T23:59:59");
}

#[test]
#[should_fail]
fn test_bad_date_time_str2() {
	let dp = {date: 0 as date_funcs, time: 0 as time_funcs};
	dp.from_str("999-12-31 23:59:59");
}

#[test]
#[should_fail]
fn test_bad_date_time_str3() {
	let dp = {date: 0 as date_funcs, time: 0 as time_funcs};
	dp.from_str("9999-12-31 23:59:9");
}

#[test]
#[should_fail]
fn test_bad_date_time_str4() {
	let dp = {date: 0 as date_funcs, time: 0 as time_funcs};
	dp.from_str("9999-12-31 23:59:58.9");
}
