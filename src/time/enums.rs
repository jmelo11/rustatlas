#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Frequency {
    NoFrequency = -1,
    Once = 0,
    Annual = 1,
    Semiannual = 2,
    EveryFourthMonth = 3,
    Quarterly = 4,
    Bimonthly = 6,
    Monthly = 12,
    EveryFourthWeek = 13,
    Biweekly = 26,
    Weekly = 52,
    Daily = 365,
    OtherFrequency = 999,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TimeUnit {
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]

pub enum DateGenerationRule {
    Backward,
    Forward,
    Zero,
    ThirdWednesday,
    ThirdWednesdayInclusive,
    Twentieth,
    TwentiethIMM,
    OldCDS,
    CDS,
    CDS2015,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]

pub enum BusinessDayConvention {
    Following,
    ModifiedFollowing,
    Preceding,
    ModifiedPreceding,
    Unadjusted,
    HalfMonthModifiedFollowing,
    Nearest,
}
