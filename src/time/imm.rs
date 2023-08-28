use super::date::Date;
use super::enums::*;

pub struct IMM {}

impl IMM {
    pub fn is_imm_date(date: Date, main_cycle: bool) -> bool {
        if date.weekday() != Weekday::Wednesday {
            return false;
        }
        if date.day() < 15 || date.day() > 21 {
            return false;
        }
        if !main_cycle {
            return true;
        }
        match date.month() {
            3 | 6 | 9 | 12 => return true,
            _ => return false,
        }
    }

    pub fn is_imm_code(in_: String, main_cycle: bool) -> bool {
        if in_.len() != 2 {
            return false;
        }
        let str1 = "0123456789";
        let loc = str1.find(&in_[1..2]);
        if loc == None {
            return false;
        }
        let str1 = if main_cycle {
            "hmzuHMZU"
        } else {
            "fghjkmnquvxzFGHJKMNQUVXZ"
        };
        let loc = str1.find(&in_[0..1]);
        return loc != None;
    }

    pub fn code(imm_date: Date) -> String {
        if !IMM::is_imm_date(imm_date, false) {
            panic!("{} is not an IMM date", imm_date);
        }
        let y = imm_date.year() % 10;
        match imm_date.month() {
            1 => return format!("F{}", y),
            2 => return format!("G{}", y),
            3 => return format!("H{}", y),
            4 => return format!("J{}", y),
            5 => return format!("K{}", y),
            6 => return format!("M{}", y),
            7 => return format!("N{}", y),
            8 => return format!("Q{}", y),
            9 => return format!("U{}", y),
            10 => return format!("V{}", y),
            11 => return format!("X{}", y),
            12 => return format!("Z{}", y),
            _ => panic!("Invalid month number"),
        }
    }

    pub fn date(imm_code: String, reference_date: Date) -> Date {
        if reference_date == Date::empty() {
            panic!("No reference date provided");
        }

        let code = imm_code.to_uppercase();
        let ms = &code[0..1];
        let m = match ms {
            "F" => 1,
            "G" => 2,
            "H" => 3,
            "J" => 4,
            "K" => 5,
            "M" => 6,
            "N" => 7,
            "Q" => 8,
            "U" => 9,
            "V" => 10,
            "X" => 11,
            "Z" => 12,
            _ => panic!("Invalid IMM month letter"),
        };
        let mut y = match code[1..2].parse::<i32>() {
            Ok(n) => n,
            Err(e) => panic!("Invalid IMM year number: {}", e),
        };
        if y == 0 && reference_date.year() <= 1909 {
            y += 10;
        }
        let reference_year = reference_date.year() % 10;
        y += reference_date.year() - reference_year;
        
        let result = IMM::next_date(Date::new(y, m, 1), false);
        if result < reference_date {
            return IMM::next_date(Date::new(y + 10, m, 1), false);
        }
        return result;
    }

    pub fn next_date(reference_date: Date, main_cycle: bool) -> Date {
        if reference_date == Date::empty() {
            panic!("No reference date provided");
        }
        let y = reference_date.year();
        let m = reference_date.month();
        let offset = if main_cycle { 3 } else { 1 };
        let skip_months = offset - (m % offset);
        let mut m = if skip_months != offset || reference_date.day() > 21 {
            skip_months + m
        } else {
            m
        };
        let mut y = y;
        if m > 12 {
            m -= 12;
            y += 1;
        }
        let result = Date::nth_weekday(3, Weekday::Wednesday, m, y);
        if result <= reference_date {
            return IMM::next_date(Date::new(y, m, 22), main_cycle);
        }
        return result;
    }

    pub fn next_date_with_code(imm_code: String, main_cycle: bool, reference_date: Date) -> Date {
        let imm_date = IMM::date(imm_code, reference_date);
        return IMM::next_date(imm_date + 1, main_cycle);
    }

    pub fn next_code(d: Date, main_cycle: bool) -> String {
        let next = IMM::next_date(d, main_cycle);
        return IMM::code(next);
    }

    pub fn next_code_with_code(imm_code: String, main_cycle: bool, reference_date: Date) -> String {
        let imm_date = IMM::date(imm_code, reference_date);
        let next = IMM::next_date(imm_date, main_cycle);
        return IMM::code(next);
    }
}

#[cfg(test)]
mod tests {
    use super::super::date::Date;
    use super::IMM;

    #[test]
    fn test_is_imm_date() {
        let d = Date::new(2023, 8, 1);
        assert_eq!(IMM::is_imm_date(d, false), false);

        let d = Date::new(2023, 8, 16);
        assert_eq!(IMM::is_imm_date(d, false), true);

        let d = Date::new(2023, 8, 16);
        assert_eq!(IMM::is_imm_date(d, true), false);

        let d = Date::new(2023, 9, 20);
        assert_eq!(IMM::is_imm_date(d, true), true);
        assert_eq!(IMM::is_imm_date(d, false), true);
    }

    #[test]
    fn test_is_imm_code() {
        let s = "F3".to_string();
        assert_eq!(IMM::is_imm_code(s, false), true);

        let s = "F3".to_string();
        assert_eq!(IMM::is_imm_code(s, true), false);

        let s = "Z3".to_string();
        assert_eq!(IMM::is_imm_code(s, true), true);

        let s = "Z3".to_string();
        assert_eq!(IMM::is_imm_code(s, false), true);
    }

    #[test]
    fn test_code() {
        let d = Date::new(2023, 8, 16);
        assert_eq!(IMM::code(d), "Q3");

        let d = Date::new(2023, 9, 20);
        assert_eq!(IMM::code(d), "U3");
    }

    #[test]
    fn test_date() {
        let d = Date::new(2023, 8, 16);
        let code1 = "Q3".to_string();
        assert_eq!(IMM::date(code1, d), d);

        let code2 = "U3".to_string();
        assert_eq!(IMM::date(code2, d), Date::new(2023, 9, 20));
    }

    #[test]
    fn test_next_date() {
        let d = Date::new(2023, 8, 16);
        assert_eq!(IMM::next_date(d, false), Date::new(2023, 9, 20));

        let d = Date::new(2023, 8, 16);
        assert_eq!(IMM::next_date(d, true), Date::new(2023, 9, 20));

        let d = Date::new(2023, 9, 20);
        assert_eq!(IMM::next_date(d, false), Date::new(2023, 10, 18));

        let d = Date::new(2023, 9, 20);
        assert_eq!(IMM::next_date(d, true), Date::new(2023, 12, 20));
    }

    #[test]
    fn test_next_date_with_code() {
        let d = Date::new(2023, 8, 16);
        let code = "Q3".to_string();
        assert_eq!(IMM::next_date_with_code(code, false, d), Date::new(2023, 9, 20));

        let d = Date::new(2023, 8, 16);
        let code = "U3".to_string();
        assert_eq!(IMM::next_date_with_code(code, false, d), Date::new(2023, 10, 18));

        let d = Date::new(2023, 8, 16);
        let code = "U3".to_string();
        assert_eq!(IMM::next_date_with_code(code, true, d), Date::new(2023, 12, 20));
    }

    #[test]
    fn test_next_code() {
        let d = Date::new(2023, 8, 16);
        assert_eq!(IMM::next_code(d, false), "U3");

        let d = Date::new(2023, 8, 16);
        assert_eq!(IMM::next_code(d, true), "U3");

        let d = Date::new(2023, 9, 20);
        assert_eq!(IMM::next_code(d, false), "V3");

        let d = Date::new(2023, 9, 20);
        assert_eq!(IMM::next_code(d, true), "Z3");
    }

    #[test]
    fn test_next_code_with_code() {
        let d = Date::new(2023, 8, 16);
        let code = "Q3".to_string();
        assert_eq!(IMM::next_code_with_code(code, false, d), "U3");

        let d = Date::new(2023, 8, 16);
        let code = "U3".to_string();
        assert_eq!(IMM::next_code_with_code(code, false, d), "V3");

        let d = Date::new(2023, 8, 16);
        let code = "U3".to_string();
        assert_eq!(IMM::next_code_with_code(code, true, d), "Z3");
    }
}
