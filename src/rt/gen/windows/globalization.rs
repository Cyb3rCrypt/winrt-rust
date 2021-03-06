use crate::prelude::*;
RT_CLASS!{static class ApplicationLanguages}
impl RtActivatable<IApplicationLanguagesStatics> for ApplicationLanguages {}
impl RtActivatable<IApplicationLanguagesStatics2> for ApplicationLanguages {}
impl ApplicationLanguages {
    #[inline] pub fn get_primary_language_override() -> Result<HString> {
        <Self as RtActivatable<IApplicationLanguagesStatics>>::get_activation_factory().get_primary_language_override()
    }
    #[inline] pub fn set_primary_language_override(value: &HStringArg) -> Result<()> {
        <Self as RtActivatable<IApplicationLanguagesStatics>>::get_activation_factory().set_primary_language_override(value)
    }
    #[inline] pub fn get_languages() -> Result<Option<foundation::collections::IVectorView<HString>>> {
        <Self as RtActivatable<IApplicationLanguagesStatics>>::get_activation_factory().get_languages()
    }
    #[inline] pub fn get_manifest_languages() -> Result<Option<foundation::collections::IVectorView<HString>>> {
        <Self as RtActivatable<IApplicationLanguagesStatics>>::get_activation_factory().get_manifest_languages()
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_languages_for_user(user: &super::system::User) -> Result<Option<foundation::collections::IVectorView<HString>>> {
        <Self as RtActivatable<IApplicationLanguagesStatics2>>::get_activation_factory().get_languages_for_user(user)
    }
}
DEFINE_CLSID!(ApplicationLanguages(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,65,112,112,108,105,99,97,116,105,111,110,76,97,110,103,117,97,103,101,115,0]) [CLSID_ApplicationLanguages]);
DEFINE_IID!(IID_IApplicationLanguagesStatics, 1974732871, 2636, 19090, 149, 101, 253, 99, 201, 95, 122, 237);
RT_INTERFACE!{static interface IApplicationLanguagesStatics(IApplicationLanguagesStaticsVtbl): IInspectable [IID_IApplicationLanguagesStatics] {
    fn get_PrimaryLanguageOverride(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PrimaryLanguageOverride(&self, value: HSTRING) -> HRESULT,
    fn get_Languages(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_ManifestLanguages(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl IApplicationLanguagesStatics {
    #[inline] pub fn get_primary_language_override(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PrimaryLanguageOverride)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_primary_language_override(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_PrimaryLanguageOverride)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_languages(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Languages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_manifest_languages(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ManifestLanguages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IApplicationLanguagesStatics2, 502324815, 1835, 19835, 143, 6, 203, 45, 180, 15, 43, 181);
RT_INTERFACE!{static interface IApplicationLanguagesStatics2(IApplicationLanguagesStatics2Vtbl): IInspectable [IID_IApplicationLanguagesStatics2] {
    #[cfg(feature="windows-system")] fn GetLanguagesForUser(&self, user: <super::system::User as RtType>::Abi, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl IApplicationLanguagesStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub fn get_languages_for_user(&self, user: &super::system::User) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetLanguagesForUser)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICalendar, 3392152093, 34521, 16635, 162, 107, 212, 78, 183, 207, 8, 234);
RT_INTERFACE!{interface ICalendar(ICalendarVtbl): IInspectable [IID_ICalendar] {
    fn Clone(&self, out: *mut <Calendar as RtType>::Abi) -> HRESULT,
    fn SetToMin(&self) -> HRESULT,
    fn SetToMax(&self) -> HRESULT,
    fn get_Languages(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_NumeralSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NumeralSystem(&self, value: HSTRING) -> HRESULT,
    fn GetCalendarSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn ChangeCalendarSystem(&self, value: HSTRING) -> HRESULT,
    fn GetClock(&self, out: *mut HSTRING) -> HRESULT,
    fn ChangeClock(&self, value: HSTRING) -> HRESULT,
    fn GetDateTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn SetDateTime(&self, value: foundation::DateTime) -> HRESULT,
    fn SetToNow(&self) -> HRESULT,
    fn get_FirstEra(&self, out: *mut i32) -> HRESULT,
    fn get_LastEra(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfEras(&self, out: *mut i32) -> HRESULT,
    fn get_Era(&self, out: *mut i32) -> HRESULT,
    fn put_Era(&self, value: i32) -> HRESULT,
    fn AddEras(&self, eras: i32) -> HRESULT,
    fn EraAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn EraAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn get_FirstYearInThisEra(&self, out: *mut i32) -> HRESULT,
    fn get_LastYearInThisEra(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfYearsInThisEra(&self, out: *mut i32) -> HRESULT,
    fn get_Year(&self, out: *mut i32) -> HRESULT,
    fn put_Year(&self, value: i32) -> HRESULT,
    fn AddYears(&self, years: i32) -> HRESULT,
    fn YearAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn YearAsTruncatedString(&self, remainingDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn YearAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_FirstMonthInThisYear(&self, out: *mut i32) -> HRESULT,
    fn get_LastMonthInThisYear(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfMonthsInThisYear(&self, out: *mut i32) -> HRESULT,
    fn get_Month(&self, out: *mut i32) -> HRESULT,
    fn put_Month(&self, value: i32) -> HRESULT,
    fn AddMonths(&self, months: i32) -> HRESULT,
    fn MonthAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn MonthAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn MonthAsFullSoloString(&self, out: *mut HSTRING) -> HRESULT,
    fn MonthAsSoloString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn MonthAsNumericString(&self, out: *mut HSTRING) -> HRESULT,
    fn MonthAsPaddedNumericString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn AddWeeks(&self, weeks: i32) -> HRESULT,
    fn get_FirstDayInThisMonth(&self, out: *mut i32) -> HRESULT,
    fn get_LastDayInThisMonth(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfDaysInThisMonth(&self, out: *mut i32) -> HRESULT,
    fn get_Day(&self, out: *mut i32) -> HRESULT,
    fn put_Day(&self, value: i32) -> HRESULT,
    fn AddDays(&self, days: i32) -> HRESULT,
    fn DayAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn DayAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_DayOfWeek(&self, out: *mut DayOfWeek) -> HRESULT,
    fn DayOfWeekAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn DayOfWeekAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn DayOfWeekAsFullSoloString(&self, out: *mut HSTRING) -> HRESULT,
    fn DayOfWeekAsSoloString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn get_FirstPeriodInThisDay(&self, out: *mut i32) -> HRESULT,
    fn get_LastPeriodInThisDay(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfPeriodsInThisDay(&self, out: *mut i32) -> HRESULT,
    fn get_Period(&self, out: *mut i32) -> HRESULT,
    fn put_Period(&self, value: i32) -> HRESULT,
    fn AddPeriods(&self, periods: i32) -> HRESULT,
    fn PeriodAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn PeriodAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT,
    fn get_FirstHourInThisPeriod(&self, out: *mut i32) -> HRESULT,
    fn get_LastHourInThisPeriod(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfHoursInThisPeriod(&self, out: *mut i32) -> HRESULT,
    fn get_Hour(&self, out: *mut i32) -> HRESULT,
    fn put_Hour(&self, value: i32) -> HRESULT,
    fn AddHours(&self, hours: i32) -> HRESULT,
    fn HourAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn HourAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_Minute(&self, out: *mut i32) -> HRESULT,
    fn put_Minute(&self, value: i32) -> HRESULT,
    fn AddMinutes(&self, minutes: i32) -> HRESULT,
    fn MinuteAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn MinuteAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_Second(&self, out: *mut i32) -> HRESULT,
    fn put_Second(&self, value: i32) -> HRESULT,
    fn AddSeconds(&self, seconds: i32) -> HRESULT,
    fn SecondAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn SecondAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn get_Nanosecond(&self, out: *mut i32) -> HRESULT,
    fn put_Nanosecond(&self, value: i32) -> HRESULT,
    fn AddNanoseconds(&self, nanoseconds: i32) -> HRESULT,
    fn NanosecondAsString(&self, out: *mut HSTRING) -> HRESULT,
    fn NanosecondAsPaddedString(&self, minDigits: i32, out: *mut HSTRING) -> HRESULT,
    fn Compare(&self, other: <Calendar as RtType>::Abi, out: *mut i32) -> HRESULT,
    fn CompareDateTime(&self, other: foundation::DateTime, out: *mut i32) -> HRESULT,
    fn CopyTo(&self, other: <Calendar as RtType>::Abi) -> HRESULT,
    fn get_FirstMinuteInThisHour(&self, out: *mut i32) -> HRESULT,
    fn get_LastMinuteInThisHour(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfMinutesInThisHour(&self, out: *mut i32) -> HRESULT,
    fn get_FirstSecondInThisMinute(&self, out: *mut i32) -> HRESULT,
    fn get_LastSecondInThisMinute(&self, out: *mut i32) -> HRESULT,
    fn get_NumberOfSecondsInThisMinute(&self, out: *mut i32) -> HRESULT,
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsDaylightSavingTime(&self, out: *mut bool) -> HRESULT
}}
impl ICalendar {
    #[inline] pub fn clone(&self) -> Result<Option<Calendar>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Clone)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Calendar::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_to_min(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().SetToMin)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_to_max(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().SetToMax)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_languages(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Languages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_numeral_system(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NumeralSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_numeral_system(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_NumeralSystem)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_calendar_system(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetCalendarSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn change_calendar_system(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ChangeCalendarSystem)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_clock(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetClock)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn change_clock(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ChangeClock)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_date_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().GetDateTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_date_time(&self, value: foundation::DateTime) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().SetDateTime)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_to_now(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().SetToNow)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_first_era(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FirstEra)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_era(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LastEra)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_eras(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NumberOfEras)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_era(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Era)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_era(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Era)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_eras(&self, eras: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddEras)(self.get_abi() as *const _ as *mut _, eras);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn era_as_full_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EraAsFullString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn era_as_string(&self, idealLength: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EraAsString)(self.get_abi() as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_first_year_in_this_era(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FirstYearInThisEra)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_year_in_this_era(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LastYearInThisEra)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_years_in_this_era(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NumberOfYearsInThisEra)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_year(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Year)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_year(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Year)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_years(&self, years: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddYears)(self.get_abi() as *const _ as *mut _, years);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn year_as_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().YearAsString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn year_as_truncated_string(&self, remainingDigits: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().YearAsTruncatedString)(self.get_abi() as *const _ as *mut _, remainingDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn year_as_padded_string(&self, minDigits: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().YearAsPaddedString)(self.get_abi() as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_first_month_in_this_year(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FirstMonthInThisYear)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_month_in_this_year(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LastMonthInThisYear)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_months_in_this_year(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NumberOfMonthsInThisYear)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_month(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Month)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_month(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Month)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_months(&self, months: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddMonths)(self.get_abi() as *const _ as *mut _, months);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn month_as_full_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().MonthAsFullString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn month_as_string(&self, idealLength: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().MonthAsString)(self.get_abi() as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn month_as_full_solo_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().MonthAsFullSoloString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn month_as_solo_string(&self, idealLength: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().MonthAsSoloString)(self.get_abi() as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn month_as_numeric_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().MonthAsNumericString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn month_as_padded_numeric_string(&self, minDigits: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().MonthAsPaddedNumericString)(self.get_abi() as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_weeks(&self, weeks: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddWeeks)(self.get_abi() as *const _ as *mut _, weeks);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_first_day_in_this_month(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FirstDayInThisMonth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_day_in_this_month(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LastDayInThisMonth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_days_in_this_month(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NumberOfDaysInThisMonth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_day(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Day)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_day(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Day)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_days(&self, days: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddDays)(self.get_abi() as *const _ as *mut _, days);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn day_as_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DayAsString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn day_as_padded_string(&self, minDigits: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DayAsPaddedString)(self.get_abi() as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_day_of_week(&self) -> Result<DayOfWeek> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_DayOfWeek)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn day_of_week_as_full_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DayOfWeekAsFullString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn day_of_week_as_string(&self, idealLength: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DayOfWeekAsString)(self.get_abi() as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn day_of_week_as_full_solo_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DayOfWeekAsFullSoloString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn day_of_week_as_solo_string(&self, idealLength: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DayOfWeekAsSoloString)(self.get_abi() as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_first_period_in_this_day(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FirstPeriodInThisDay)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_period_in_this_day(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LastPeriodInThisDay)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_periods_in_this_day(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NumberOfPeriodsInThisDay)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_period(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Period)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_period(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Period)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_periods(&self, periods: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddPeriods)(self.get_abi() as *const _ as *mut _, periods);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn period_as_full_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().PeriodAsFullString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn period_as_string(&self, idealLength: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().PeriodAsString)(self.get_abi() as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_first_hour_in_this_period(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FirstHourInThisPeriod)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_hour_in_this_period(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LastHourInThisPeriod)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_hours_in_this_period(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NumberOfHoursInThisPeriod)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_hour(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Hour)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_hour(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Hour)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_hours(&self, hours: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddHours)(self.get_abi() as *const _ as *mut _, hours);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn hour_as_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().HourAsString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn hour_as_padded_string(&self, minDigits: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().HourAsPaddedString)(self.get_abi() as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_minute(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Minute)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_minute(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Minute)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_minutes(&self, minutes: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddMinutes)(self.get_abi() as *const _ as *mut _, minutes);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn minute_as_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().MinuteAsString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn minute_as_padded_string(&self, minDigits: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().MinuteAsPaddedString)(self.get_abi() as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_second(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Second)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_second(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Second)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_seconds(&self, seconds: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddSeconds)(self.get_abi() as *const _ as *mut _, seconds);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn second_as_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SecondAsString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn second_as_padded_string(&self, minDigits: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SecondAsPaddedString)(self.get_abi() as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nanosecond(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Nanosecond)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_nanosecond(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Nanosecond)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_nanoseconds(&self, nanoseconds: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AddNanoseconds)(self.get_abi() as *const _ as *mut _, nanoseconds);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn nanosecond_as_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().NanosecondAsString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn nanosecond_as_padded_string(&self, minDigits: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().NanosecondAsPaddedString)(self.get_abi() as *const _ as *mut _, minDigits, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn compare(&self, other: &Calendar) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().Compare)(self.get_abi() as *const _ as *mut _, other.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn compare_date_time(&self, other: foundation::DateTime) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().CompareDateTime)(self.get_abi() as *const _ as *mut _, other, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn copy_to(&self, other: &Calendar) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().CopyTo)(self.get_abi() as *const _ as *mut _, other.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_first_minute_in_this_hour(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FirstMinuteInThisHour)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_minute_in_this_hour(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LastMinuteInThisHour)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_minutes_in_this_hour(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NumberOfMinutesInThisHour)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_first_second_in_this_minute(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FirstSecondInThisMinute)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_second_in_this_minute(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LastSecondInThisMinute)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_seconds_in_this_minute(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NumberOfSecondsInThisMinute)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ResolvedLanguage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_daylight_saving_time(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsDaylightSavingTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Calendar: ICalendar}
impl RtActivatable<ICalendarFactory> for Calendar {}
impl RtActivatable<ICalendarFactory2> for Calendar {}
impl RtActivatable<IActivationFactory> for Calendar {}
impl Calendar {
    #[inline] pub fn create_calendar_default_calendar_and_clock(languages: &foundation::collections::IIterable<HString>) -> Result<Calendar> {
        <Self as RtActivatable<ICalendarFactory>>::get_activation_factory().create_calendar_default_calendar_and_clock(languages)
    }
    #[inline] pub fn create_calendar(languages: &foundation::collections::IIterable<HString>, calendar: &HStringArg, clock: &HStringArg) -> Result<Calendar> {
        <Self as RtActivatable<ICalendarFactory>>::get_activation_factory().create_calendar(languages, calendar, clock)
    }
    #[inline] pub fn create_calendar_with_time_zone(languages: &foundation::collections::IIterable<HString>, calendar: &HStringArg, clock: &HStringArg, timeZoneId: &HStringArg) -> Result<Calendar> {
        <Self as RtActivatable<ICalendarFactory2>>::get_activation_factory().create_calendar_with_time_zone(languages, calendar, clock, timeZoneId)
    }
}
DEFINE_CLSID!(Calendar(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,97,108,101,110,100,97,114,0]) [CLSID_Calendar]);
DEFINE_IID!(IID_ICalendarFactory, 2213905426, 58731, 19573, 166, 110, 15, 99, 213, 119, 88, 166);
RT_INTERFACE!{static interface ICalendarFactory(ICalendarFactoryVtbl): IInspectable [IID_ICalendarFactory] {
    fn CreateCalendarDefaultCalendarAndClock(&self, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <Calendar as RtType>::Abi) -> HRESULT,
    fn CreateCalendar(&self, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, calendar: HSTRING, clock: HSTRING, out: *mut <Calendar as RtType>::Abi) -> HRESULT
}}
impl ICalendarFactory {
    #[inline] pub fn create_calendar_default_calendar_and_clock(&self, languages: &foundation::collections::IIterable<HString>) -> Result<Calendar> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCalendarDefaultCalendarAndClock)(self.get_abi() as *const _ as *mut _, languages.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Calendar::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_calendar(&self, languages: &foundation::collections::IIterable<HString>, calendar: &HStringArg, clock: &HStringArg) -> Result<Calendar> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCalendar)(self.get_abi() as *const _ as *mut _, languages.get_abi() as *const _ as *mut _, calendar.get(), clock.get(), &mut out);
        if hr == S_OK { Ok(Calendar::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICalendarFactory2, 3024828300, 51838, 17808, 158, 114, 234, 43, 236, 26, 81, 21);
RT_INTERFACE!{static interface ICalendarFactory2(ICalendarFactory2Vtbl): IInspectable [IID_ICalendarFactory2] {
    fn CreateCalendarWithTimeZone(&self, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, calendar: HSTRING, clock: HSTRING, timeZoneId: HSTRING, out: *mut <Calendar as RtType>::Abi) -> HRESULT
}}
impl ICalendarFactory2 {
    #[inline] pub fn create_calendar_with_time_zone(&self, languages: &foundation::collections::IIterable<HString>, calendar: &HStringArg, clock: &HStringArg, timeZoneId: &HStringArg) -> Result<Calendar> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCalendarWithTimeZone)(self.get_abi() as *const _ as *mut _, languages.get_abi() as *const _ as *mut _, calendar.get(), clock.get(), timeZoneId.get(), &mut out);
        if hr == S_OK { Ok(Calendar::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class CalendarIdentifiers}
impl RtActivatable<ICalendarIdentifiersStatics> for CalendarIdentifiers {}
impl RtActivatable<ICalendarIdentifiersStatics2> for CalendarIdentifiers {}
impl RtActivatable<ICalendarIdentifiersStatics3> for CalendarIdentifiers {}
impl CalendarIdentifiers {
    #[inline] pub fn get_gregorian() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_gregorian()
    }
    #[inline] pub fn get_hebrew() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_hebrew()
    }
    #[inline] pub fn get_hijri() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_hijri()
    }
    #[inline] pub fn get_japanese() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_japanese()
    }
    #[inline] pub fn get_julian() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_julian()
    }
    #[inline] pub fn get_korean() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_korean()
    }
    #[inline] pub fn get_taiwan() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_taiwan()
    }
    #[inline] pub fn get_thai() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_thai()
    }
    #[inline] pub fn get_um_al_qura() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics>>::get_activation_factory().get_um_al_qura()
    }
    #[inline] pub fn get_persian() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics2>>::get_activation_factory().get_persian()
    }
    #[inline] pub fn get_chinese_lunar() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_chinese_lunar()
    }
    #[inline] pub fn get_japanese_lunar() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_japanese_lunar()
    }
    #[inline] pub fn get_korean_lunar() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_korean_lunar()
    }
    #[inline] pub fn get_taiwan_lunar() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_taiwan_lunar()
    }
    #[inline] pub fn get_vietnamese_lunar() -> Result<HString> {
        <Self as RtActivatable<ICalendarIdentifiersStatics3>>::get_activation_factory().get_vietnamese_lunar()
    }
}
DEFINE_CLSID!(CalendarIdentifiers(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,97,108,101,110,100,97,114,73,100,101,110,116,105,102,105,101,114,115,0]) [CLSID_CalendarIdentifiers]);
DEFINE_IID!(IID_ICalendarIdentifiersStatics, 2154119016, 11442, 19487, 181, 144, 240, 245, 43, 244, 253, 26);
RT_INTERFACE!{static interface ICalendarIdentifiersStatics(ICalendarIdentifiersStaticsVtbl): IInspectable [IID_ICalendarIdentifiersStatics] {
    fn get_Gregorian(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Hebrew(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Hijri(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Japanese(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Julian(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Korean(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Taiwan(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Thai(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UmAlQura(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICalendarIdentifiersStatics {
    #[inline] pub fn get_gregorian(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Gregorian)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hebrew(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Hebrew)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hijri(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Hijri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_japanese(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Japanese)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_julian(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Julian)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_korean(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Korean)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_taiwan(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Taiwan)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_thai(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Thai)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_um_al_qura(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UmAlQura)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICalendarIdentifiersStatics2, 2113197192, 24528, 17063, 149, 181, 125, 152, 216, 35, 7, 95);
RT_INTERFACE!{static interface ICalendarIdentifiersStatics2(ICalendarIdentifiersStatics2Vtbl): IInspectable [IID_ICalendarIdentifiersStatics2] {
    fn get_Persian(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICalendarIdentifiersStatics2 {
    #[inline] pub fn get_persian(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Persian)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICalendarIdentifiersStatics3, 740447267, 8109, 16576, 147, 52, 168, 235, 144, 219, 4, 245);
RT_INTERFACE!{static interface ICalendarIdentifiersStatics3(ICalendarIdentifiersStatics3Vtbl): IInspectable [IID_ICalendarIdentifiersStatics3] {
    fn get_ChineseLunar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JapaneseLunar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KoreanLunar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TaiwanLunar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VietnameseLunar(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICalendarIdentifiersStatics3 {
    #[inline] pub fn get_chinese_lunar(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ChineseLunar)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_japanese_lunar(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_JapaneseLunar)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_korean_lunar(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KoreanLunar)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_taiwan_lunar(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TaiwanLunar)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vietnamese_lunar(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_VietnameseLunar)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class ClockIdentifiers}
impl RtActivatable<IClockIdentifiersStatics> for ClockIdentifiers {}
impl ClockIdentifiers {
    #[inline] pub fn get_twelve_hour() -> Result<HString> {
        <Self as RtActivatable<IClockIdentifiersStatics>>::get_activation_factory().get_twelve_hour()
    }
    #[inline] pub fn get_twenty_four_hour() -> Result<HString> {
        <Self as RtActivatable<IClockIdentifiersStatics>>::get_activation_factory().get_twenty_four_hour()
    }
}
DEFINE_CLSID!(ClockIdentifiers(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,108,111,99,107,73,100,101,110,116,105,102,105,101,114,115,0]) [CLSID_ClockIdentifiers]);
DEFINE_IID!(IID_IClockIdentifiersStatics, 1379403195, 4844, 20355, 188, 49, 177, 180, 55, 107, 8, 8);
RT_INTERFACE!{static interface IClockIdentifiersStatics(IClockIdentifiersStaticsVtbl): IInspectable [IID_IClockIdentifiersStatics] {
    fn get_TwelveHour(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TwentyFourHour(&self, out: *mut HSTRING) -> HRESULT
}}
impl IClockIdentifiersStatics {
    #[inline] pub fn get_twelve_hour(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TwelveHour)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_twenty_four_hour(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TwentyFourHour)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class CurrencyIdentifiers}
impl RtActivatable<ICurrencyIdentifiersStatics> for CurrencyIdentifiers {}
impl RtActivatable<ICurrencyIdentifiersStatics2> for CurrencyIdentifiers {}
impl RtActivatable<ICurrencyIdentifiersStatics3> for CurrencyIdentifiers {}
impl CurrencyIdentifiers {
    #[inline] pub fn get_aed() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_aed()
    }
    #[inline] pub fn get_afn() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_afn()
    }
    #[inline] pub fn get_all() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_all()
    }
    #[inline] pub fn get_amd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_amd()
    }
    #[inline] pub fn get_ang() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ang()
    }
    #[inline] pub fn get_aoa() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_aoa()
    }
    #[inline] pub fn get_ars() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ars()
    }
    #[inline] pub fn get_aud() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_aud()
    }
    #[inline] pub fn get_awg() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_awg()
    }
    #[inline] pub fn get_azn() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_azn()
    }
    #[inline] pub fn get_bam() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bam()
    }
    #[inline] pub fn get_bbd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bbd()
    }
    #[inline] pub fn get_bdt() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bdt()
    }
    #[inline] pub fn get_bgn() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bgn()
    }
    #[inline] pub fn get_bhd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bhd()
    }
    #[inline] pub fn get_bif() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bif()
    }
    #[inline] pub fn get_bmd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bmd()
    }
    #[inline] pub fn get_bnd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bnd()
    }
    #[inline] pub fn get_bob() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bob()
    }
    #[inline] pub fn get_brl() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_brl()
    }
    #[inline] pub fn get_bsd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bsd()
    }
    #[inline] pub fn get_btn() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_btn()
    }
    #[inline] pub fn get_bwp() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bwp()
    }
    #[inline] pub fn get_byr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_byr()
    }
    #[inline] pub fn get_bzd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_bzd()
    }
    #[inline] pub fn get_cad() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cad()
    }
    #[inline] pub fn get_cdf() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cdf()
    }
    #[inline] pub fn get_chf() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_chf()
    }
    #[inline] pub fn get_clp() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_clp()
    }
    #[inline] pub fn get_cny() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cny()
    }
    #[inline] pub fn get_cop() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cop()
    }
    #[inline] pub fn get_crc() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_crc()
    }
    #[inline] pub fn get_cup() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cup()
    }
    #[inline] pub fn get_cve() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_cve()
    }
    #[inline] pub fn get_czk() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_czk()
    }
    #[inline] pub fn get_djf() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_djf()
    }
    #[inline] pub fn get_dkk() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_dkk()
    }
    #[inline] pub fn get_dop() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_dop()
    }
    #[inline] pub fn get_dzd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_dzd()
    }
    #[inline] pub fn get_egp() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_egp()
    }
    #[inline] pub fn get_ern() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ern()
    }
    #[inline] pub fn get_etb() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_etb()
    }
    #[inline] pub fn get_eur() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_eur()
    }
    #[inline] pub fn get_fjd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_fjd()
    }
    #[inline] pub fn get_fkp() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_fkp()
    }
    #[inline] pub fn get_gbp() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gbp()
    }
    #[inline] pub fn get_gel() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gel()
    }
    #[inline] pub fn get_ghs() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ghs()
    }
    #[inline] pub fn get_gip() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gip()
    }
    #[inline] pub fn get_gmd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gmd()
    }
    #[inline] pub fn get_gnf() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gnf()
    }
    #[inline] pub fn get_gtq() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gtq()
    }
    #[inline] pub fn get_gyd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_gyd()
    }
    #[inline] pub fn get_hkd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_hkd()
    }
    #[inline] pub fn get_hnl() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_hnl()
    }
    #[inline] pub fn get_hrk() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_hrk()
    }
    #[inline] pub fn get_htg() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_htg()
    }
    #[inline] pub fn get_huf() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_huf()
    }
    #[inline] pub fn get_idr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_idr()
    }
    #[inline] pub fn get_ils() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ils()
    }
    #[inline] pub fn get_inr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_inr()
    }
    #[inline] pub fn get_iqd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_iqd()
    }
    #[inline] pub fn get_irr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_irr()
    }
    #[inline] pub fn get_isk() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_isk()
    }
    #[inline] pub fn get_jmd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_jmd()
    }
    #[inline] pub fn get_jod() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_jod()
    }
    #[inline] pub fn get_jpy() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_jpy()
    }
    #[inline] pub fn get_kes() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kes()
    }
    #[inline] pub fn get_kgs() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kgs()
    }
    #[inline] pub fn get_khr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_khr()
    }
    #[inline] pub fn get_kmf() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kmf()
    }
    #[inline] pub fn get_kpw() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kpw()
    }
    #[inline] pub fn get_krw() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_krw()
    }
    #[inline] pub fn get_kwd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kwd()
    }
    #[inline] pub fn get_kyd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kyd()
    }
    #[inline] pub fn get_kzt() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_kzt()
    }
    #[inline] pub fn get_lak() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lak()
    }
    #[inline] pub fn get_lbp() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lbp()
    }
    #[inline] pub fn get_lkr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lkr()
    }
    #[inline] pub fn get_lrd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lrd()
    }
    #[inline] pub fn get_lsl() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lsl()
    }
    #[inline] pub fn get_ltl() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ltl()
    }
    #[inline] pub fn get_lvl() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lvl()
    }
    #[inline] pub fn get_lyd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_lyd()
    }
    #[inline] pub fn get_mad() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mad()
    }
    #[inline] pub fn get_mdl() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mdl()
    }
    #[inline] pub fn get_mga() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mga()
    }
    #[inline] pub fn get_mkd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mkd()
    }
    #[inline] pub fn get_mmk() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mmk()
    }
    #[inline] pub fn get_mnt() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mnt()
    }
    #[inline] pub fn get_mop() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mop()
    }
    #[inline] pub fn get_mro() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mro()
    }
    #[inline] pub fn get_mur() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mur()
    }
    #[inline] pub fn get_mvr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mvr()
    }
    #[inline] pub fn get_mwk() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mwk()
    }
    #[inline] pub fn get_mxn() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mxn()
    }
    #[inline] pub fn get_myr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_myr()
    }
    #[inline] pub fn get_mzn() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_mzn()
    }
    #[inline] pub fn get_nad() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_nad()
    }
    #[inline] pub fn get_ngn() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ngn()
    }
    #[inline] pub fn get_nio() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_nio()
    }
    #[inline] pub fn get_nok() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_nok()
    }
    #[inline] pub fn get_npr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_npr()
    }
    #[inline] pub fn get_nzd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_nzd()
    }
    #[inline] pub fn get_omr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_omr()
    }
    #[inline] pub fn get_pab() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pab()
    }
    #[inline] pub fn get_pen() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pen()
    }
    #[inline] pub fn get_pgk() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pgk()
    }
    #[inline] pub fn get_php() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_php()
    }
    #[inline] pub fn get_pkr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pkr()
    }
    #[inline] pub fn get_pln() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pln()
    }
    #[inline] pub fn get_pyg() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_pyg()
    }
    #[inline] pub fn get_qar() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_qar()
    }
    #[inline] pub fn get_ron() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ron()
    }
    #[inline] pub fn get_rsd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_rsd()
    }
    #[inline] pub fn get_rub() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_rub()
    }
    #[inline] pub fn get_rwf() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_rwf()
    }
    #[inline] pub fn get_sar() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sar()
    }
    #[inline] pub fn get_sbd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sbd()
    }
    #[inline] pub fn get_scr() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_scr()
    }
    #[inline] pub fn get_sdg() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sdg()
    }
    #[inline] pub fn get_sek() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sek()
    }
    #[inline] pub fn get_sgd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sgd()
    }
    #[inline] pub fn get_shp() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_shp()
    }
    #[inline] pub fn get_sll() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sll()
    }
    #[inline] pub fn get_sos() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_sos()
    }
    #[inline] pub fn get_srd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_srd()
    }
    #[inline] pub fn get_std() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_std()
    }
    #[inline] pub fn get_syp() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_syp()
    }
    #[inline] pub fn get_szl() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_szl()
    }
    #[inline] pub fn get_thb() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_thb()
    }
    #[inline] pub fn get_tjs() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_tjs()
    }
    #[inline] pub fn get_tmt() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_tmt()
    }
    #[inline] pub fn get_tnd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_tnd()
    }
    #[inline] pub fn get_top() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_top()
    }
    #[inline] pub fn get_try() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_try()
    }
    #[inline] pub fn get_ttd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ttd()
    }
    #[inline] pub fn get_twd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_twd()
    }
    #[inline] pub fn get_tzs() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_tzs()
    }
    #[inline] pub fn get_uah() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_uah()
    }
    #[inline] pub fn get_ugx() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_ugx()
    }
    #[inline] pub fn get_usd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_usd()
    }
    #[inline] pub fn get_uyu() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_uyu()
    }
    #[inline] pub fn get_uzs() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_uzs()
    }
    #[inline] pub fn get_vef() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_vef()
    }
    #[inline] pub fn get_vnd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_vnd()
    }
    #[inline] pub fn get_vuv() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_vuv()
    }
    #[inline] pub fn get_wst() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_wst()
    }
    #[inline] pub fn get_xaf() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xaf()
    }
    #[inline] pub fn get_xcd() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xcd()
    }
    #[inline] pub fn get_xof() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xof()
    }
    #[inline] pub fn get_xpf() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xpf()
    }
    #[inline] pub fn get_xxx() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_xxx()
    }
    #[inline] pub fn get_yer() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_yer()
    }
    #[inline] pub fn get_zar() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_zar()
    }
    #[inline] pub fn get_zmw() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_zmw()
    }
    #[inline] pub fn get_zwl() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics>>::get_activation_factory().get_zwl()
    }
    #[inline] pub fn get_byn() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics2>>::get_activation_factory().get_byn()
    }
    #[inline] pub fn get_mru() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics3>>::get_activation_factory().get_mru()
    }
    #[inline] pub fn get_ssp() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics3>>::get_activation_factory().get_ssp()
    }
    #[inline] pub fn get_stn() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics3>>::get_activation_factory().get_stn()
    }
    #[inline] pub fn get_ves() -> Result<HString> {
        <Self as RtActivatable<ICurrencyIdentifiersStatics3>>::get_activation_factory().get_ves()
    }
}
DEFINE_CLSID!(CurrencyIdentifiers(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,117,114,114,101,110,99,121,73,100,101,110,116,105,102,105,101,114,115,0]) [CLSID_CurrencyIdentifiers]);
DEFINE_IID!(IID_ICurrencyIdentifiersStatics, 2669480219, 54662, 18707, 155, 106, 169, 189, 45, 193, 40, 116);
RT_INTERFACE!{static interface ICurrencyIdentifiersStatics(ICurrencyIdentifiersStaticsVtbl): IInspectable [IID_ICurrencyIdentifiersStatics] {
    fn get_AED(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AFN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ALL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AMD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ANG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AOA(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ARS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AUD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AWG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AZN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BAM(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BBD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BDT(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BGN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BHD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BIF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BMD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BND(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BOB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BRL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BSD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BTN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BWP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BYR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BZD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CAD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CDF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CHF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CLP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CNY(&self, out: *mut HSTRING) -> HRESULT,
    fn get_COP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CRC(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CUP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CVE(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CZK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DJF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DKK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DOP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DZD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EGP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ERN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ETB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EUR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FJD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FKP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GBP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GEL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GHS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GIP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GMD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GNF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GTQ(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GYD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HKD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HNL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HRK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HTG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HUF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IDR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ILS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_INR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IQD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IRR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ISK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JMD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JOD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JPY(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KES(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KGS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KHR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KMF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KPW(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KRW(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KWD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KYD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KZT(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LAK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LBP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LKR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LRD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LSL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LTL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LVL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LYD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MAD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MDL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MGA(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MKD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MMK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MNT(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MOP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MRO(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MUR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MVR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MWK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MXN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MYR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MZN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NAD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NGN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NIO(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NOK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NPR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NZD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_OMR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PAB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PEN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PGK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PHP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PKR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PLN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PYG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_QAR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RON(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RSD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RUB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RWF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SAR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SBD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SCR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SDG(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SEK(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SGD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SHP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SLL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SOS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SRD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_STD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SYP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SZL(&self, out: *mut HSTRING) -> HRESULT,
    fn get_THB(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TJS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TMT(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TND(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TOP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TRY(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TTD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TWD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TZS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UAH(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UGX(&self, out: *mut HSTRING) -> HRESULT,
    fn get_USD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UYU(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UZS(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VEF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VND(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VUV(&self, out: *mut HSTRING) -> HRESULT,
    fn get_WST(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XAF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XCD(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XOF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XPF(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XXX(&self, out: *mut HSTRING) -> HRESULT,
    fn get_YER(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZAR(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZMW(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZWL(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICurrencyIdentifiersStatics {
    #[inline] pub fn get_aed(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AED)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_afn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AFN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_all(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ALL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_amd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AMD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ang(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ANG)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_aoa(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AOA)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ars(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ARS)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_aud(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AUD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_awg(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AWG)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_azn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AZN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bam(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BAM)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bbd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BBD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bdt(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BDT)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bgn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BGN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bhd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BHD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bif(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BIF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bmd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BMD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bnd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BND)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bob(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BOB)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BRL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bsd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BSD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_btn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BTN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bwp(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BWP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_byr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BYR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bzd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BZD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cad(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CAD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cdf(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CDF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_chf(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CHF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_clp(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CLP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cny(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CNY)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cop(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_COP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_crc(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CRC)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cup(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CUP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cve(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CVE)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_czk(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CZK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_djf(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DJF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_dkk(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DKK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_dop(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DOP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_dzd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DZD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_egp(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EGP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ern(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ERN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_etb(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ETB)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_eur(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EUR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_fjd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FJD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_fkp(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FKP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gbp(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GBP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gel(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GEL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ghs(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GHS)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gip(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GIP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gmd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GMD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gnf(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GNF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gtq(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GTQ)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gyd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GYD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hkd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HKD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hnl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HNL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hrk(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HRK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_htg(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HTG)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_huf(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HUF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_idr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_IDR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ils(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ILS)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_inr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_INR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_iqd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_IQD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_irr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_IRR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_isk(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ISK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_jmd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_JMD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_jod(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_JOD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_jpy(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_JPY)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kes(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KES)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kgs(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KGS)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_khr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KHR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kmf(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KMF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kpw(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KPW)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_krw(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KRW)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kwd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KWD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kyd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KYD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kzt(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KZT)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lak(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LAK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lbp(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LBP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lkr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LKR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lrd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LRD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lsl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LSL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ltl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LTL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lvl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LVL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lyd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LYD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mad(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MAD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mdl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MDL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mga(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MGA)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mkd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MKD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mmk(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MMK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mnt(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MNT)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mop(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MOP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mro(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MRO)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mur(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MUR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mvr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MVR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mwk(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MWK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mxn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MXN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_myr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MYR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mzn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MZN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nad(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NAD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ngn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NGN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nio(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NIO)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nok(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NOK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_npr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NPR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nzd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NZD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_omr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_OMR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pab(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PAB)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pen(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PEN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pgk(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PGK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_php(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PHP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pkr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PKR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pln(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PLN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pyg(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PYG)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_qar(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_QAR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ron(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RON)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RSD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rub(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RUB)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rwf(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RWF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sar(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SAR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sbd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SBD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_scr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SCR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sdg(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SDG)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sek(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SEK)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sgd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SGD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_shp(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SHP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sll(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SLL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sos(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SOS)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_srd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SRD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_std(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_STD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_syp(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SYP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_szl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SZL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_thb(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_THB)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tjs(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TJS)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tmt(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TMT)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tnd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TND)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_top(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TOP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_try(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TRY)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ttd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TTD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_twd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TWD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tzs(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TZS)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_uah(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UAH)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ugx(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UGX)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_usd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_USD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_uyu(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UYU)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_uzs(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UZS)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vef(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_VEF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vnd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_VND)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vuv(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_VUV)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_wst(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_WST)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xaf(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_XAF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xcd(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_XCD)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xof(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_XOF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xpf(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_XPF)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xxx(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_XXX)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_yer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_YER)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_zar(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ZAR)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_zmw(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ZMW)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_zwl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ZWL)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICurrencyIdentifiersStatics2, 403995007, 50098, 19507, 149, 145, 152, 0, 17, 149, 13, 55);
RT_INTERFACE!{static interface ICurrencyIdentifiersStatics2(ICurrencyIdentifiersStatics2Vtbl): IInspectable [IID_ICurrencyIdentifiersStatics2] {
    fn get_BYN(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICurrencyIdentifiersStatics2 {
    #[inline] pub fn get_byn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BYN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICurrencyIdentifiersStatics3, 1337080826, 60709, 20301, 133, 127, 35, 127, 23, 72, 194, 28);
RT_INTERFACE!{static interface ICurrencyIdentifiersStatics3(ICurrencyIdentifiersStatics3Vtbl): IInspectable [IID_ICurrencyIdentifiersStatics3] {
    fn get_MRU(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SSP(&self, out: *mut HSTRING) -> HRESULT,
    fn get_STN(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VES(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICurrencyIdentifiersStatics3 {
    #[inline] pub fn get_mru(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MRU)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ssp(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SSP)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_stn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_STN)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ves(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_VES)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum DayOfWeek: i32 {
    Sunday = 0, Monday = 1, Tuesday = 2, Wednesday = 3, Thursday = 4, Friday = 5, Saturday = 6,
}}
DEFINE_IID!(IID_IGeographicRegion, 32089633, 19044, 20185, 149, 79, 158, 222, 176, 123, 217, 3);
RT_INTERFACE!{interface IGeographicRegion(IGeographicRegionVtbl): IInspectable [IID_IGeographicRegion] {
    fn get_Code(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CodeTwoLetter(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CodeThreeLetter(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CodeThreeDigit(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NativeName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CurrenciesInUse(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl IGeographicRegion {
    #[inline] pub fn get_code(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Code)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_code_two_letter(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CodeTwoLetter)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_code_three_letter(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CodeThreeLetter)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_code_three_digit(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CodeThreeDigit)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_native_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NativeName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_currencies_in_use(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CurrenciesInUse)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GeographicRegion: IGeographicRegion}
impl RtActivatable<IGeographicRegionFactory> for GeographicRegion {}
impl RtActivatable<IGeographicRegionStatics> for GeographicRegion {}
impl RtActivatable<IActivationFactory> for GeographicRegion {}
impl GeographicRegion {
    #[inline] pub fn create_geographic_region(geographicRegionCode: &HStringArg) -> Result<GeographicRegion> {
        <Self as RtActivatable<IGeographicRegionFactory>>::get_activation_factory().create_geographic_region(geographicRegionCode)
    }
    #[inline] pub fn is_supported(geographicRegionCode: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<IGeographicRegionStatics>>::get_activation_factory().is_supported(geographicRegionCode)
    }
}
DEFINE_CLSID!(GeographicRegion(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,71,101,111,103,114,97,112,104,105,99,82,101,103,105,111,110,0]) [CLSID_GeographicRegion]);
DEFINE_IID!(IID_IGeographicRegionFactory, 1396855408, 30644, 17003, 133, 159, 129, 225, 157, 81, 37, 70);
RT_INTERFACE!{static interface IGeographicRegionFactory(IGeographicRegionFactoryVtbl): IInspectable [IID_IGeographicRegionFactory] {
    fn CreateGeographicRegion(&self, geographicRegionCode: HSTRING, out: *mut <GeographicRegion as RtType>::Abi) -> HRESULT
}}
impl IGeographicRegionFactory {
    #[inline] pub fn create_geographic_region(&self, geographicRegionCode: &HStringArg) -> Result<GeographicRegion> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateGeographicRegion)(self.get_abi() as *const _ as *mut _, geographicRegionCode.get(), &mut out);
        if hr == S_OK { Ok(GeographicRegion::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGeographicRegionStatics, 702712180, 31449, 20212, 135, 153, 179, 180, 79, 173, 236, 8);
RT_INTERFACE!{static interface IGeographicRegionStatics(IGeographicRegionStaticsVtbl): IInspectable [IID_IGeographicRegionStatics] {
    fn IsSupported(&self, geographicRegionCode: HSTRING, out: *mut bool) -> HRESULT
}}
impl IGeographicRegionStatics {
    #[inline] pub fn is_supported(&self, geographicRegionCode: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().IsSupported)(self.get_abi() as *const _ as *mut _, geographicRegionCode.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IJapanesePhoneme, 795513600, 59483, 17382, 137, 125, 93, 130, 248, 98, 223, 33);
RT_INTERFACE!{interface IJapanesePhoneme(IJapanesePhonemeVtbl): IInspectable [IID_IJapanesePhoneme] {
    fn get_DisplayText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_YomiText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsPhraseStart(&self, out: *mut bool) -> HRESULT
}}
impl IJapanesePhoneme {
    #[inline] pub fn get_display_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DisplayText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_yomi_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_YomiText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_phrase_start(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsPhraseStart)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class JapanesePhoneme: IJapanesePhoneme}
RT_CLASS!{static class JapanesePhoneticAnalyzer}
impl RtActivatable<IJapanesePhoneticAnalyzerStatics> for JapanesePhoneticAnalyzer {}
impl JapanesePhoneticAnalyzer {
    #[inline] pub fn get_words(input: &HStringArg) -> Result<Option<foundation::collections::IVectorView<JapanesePhoneme>>> {
        <Self as RtActivatable<IJapanesePhoneticAnalyzerStatics>>::get_activation_factory().get_words(input)
    }
    #[inline] pub fn get_words_with_mono_ruby_option(input: &HStringArg, monoRuby: bool) -> Result<Option<foundation::collections::IVectorView<JapanesePhoneme>>> {
        <Self as RtActivatable<IJapanesePhoneticAnalyzerStatics>>::get_activation_factory().get_words_with_mono_ruby_option(input, monoRuby)
    }
}
DEFINE_CLSID!(JapanesePhoneticAnalyzer(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,74,97,112,97,110,101,115,101,80,104,111,110,101,116,105,99,65,110,97,108,121,122,101,114,0]) [CLSID_JapanesePhoneticAnalyzer]);
DEFINE_IID!(IID_IJapanesePhoneticAnalyzerStatics, 2292948624, 37854, 16818, 180, 213, 142, 219, 34, 127, 209, 194);
RT_INTERFACE!{static interface IJapanesePhoneticAnalyzerStatics(IJapanesePhoneticAnalyzerStaticsVtbl): IInspectable [IID_IJapanesePhoneticAnalyzerStatics] {
    fn GetWords(&self, input: HSTRING, out: *mut <foundation::collections::IVectorView<JapanesePhoneme> as RtType>::Abi) -> HRESULT,
    fn GetWordsWithMonoRubyOption(&self, input: HSTRING, monoRuby: bool, out: *mut <foundation::collections::IVectorView<JapanesePhoneme> as RtType>::Abi) -> HRESULT
}}
impl IJapanesePhoneticAnalyzerStatics {
    #[inline] pub fn get_words(&self, input: &HStringArg) -> Result<Option<foundation::collections::IVectorView<JapanesePhoneme>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetWords)(self.get_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_words_with_mono_ruby_option(&self, input: &HStringArg, monoRuby: bool) -> Result<Option<foundation::collections::IVectorView<JapanesePhoneme>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetWordsWithMonoRubyOption)(self.get_abi() as *const _ as *mut _, input.get(), monoRuby, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILanguage, 3933841234, 63426, 16997, 177, 189, 196, 222, 196, 228, 240, 128);
RT_INTERFACE!{interface ILanguage(ILanguageVtbl): IInspectable [IID_ILanguage] {
    fn get_LanguageTag(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NativeName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Script(&self, out: *mut HSTRING) -> HRESULT
}}
impl ILanguage {
    #[inline] pub fn get_language_tag(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LanguageTag)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_native_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NativeName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_script(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Script)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Language: ILanguage}
impl RtActivatable<ILanguageFactory> for Language {}
impl RtActivatable<ILanguageStatics> for Language {}
impl RtActivatable<ILanguageStatics2> for Language {}
impl Language {
    #[inline] pub fn create_language(languageTag: &HStringArg) -> Result<Language> {
        <Self as RtActivatable<ILanguageFactory>>::get_activation_factory().create_language(languageTag)
    }
    #[inline] pub fn is_well_formed(languageTag: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<ILanguageStatics>>::get_activation_factory().is_well_formed(languageTag)
    }
    #[inline] pub fn get_current_input_method_language_tag() -> Result<HString> {
        <Self as RtActivatable<ILanguageStatics>>::get_activation_factory().get_current_input_method_language_tag()
    }
    #[inline] pub fn try_set_input_method_language_tag(languageTag: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<ILanguageStatics2>>::get_activation_factory().try_set_input_method_language_tag(languageTag)
    }
}
DEFINE_CLSID!(Language(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,76,97,110,103,117,97,103,101,0]) [CLSID_Language]);
DEFINE_IID!(IID_ILanguage2, 1783096757, 55629, 18566, 164, 4, 165, 165, 185, 213, 180, 148);
RT_INTERFACE!{interface ILanguage2(ILanguage2Vtbl): IInspectable [IID_ILanguage2] {
    fn get_LayoutDirection(&self, out: *mut LanguageLayoutDirection) -> HRESULT
}}
impl ILanguage2 {
    #[inline] pub fn get_layout_direction(&self) -> Result<LanguageLayoutDirection> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LayoutDirection)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILanguageExtensionSubtags, 2105388869, 13965, 17252, 133, 43, 222, 201, 39, 3, 123, 133);
RT_INTERFACE!{interface ILanguageExtensionSubtags(ILanguageExtensionSubtagsVtbl): IInspectable [IID_ILanguageExtensionSubtags] {
    fn GetExtensionSubtags(&self, singleton: HSTRING, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl ILanguageExtensionSubtags {
    #[inline] pub fn get_extension_subtags(&self, singleton: &HStringArg) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetExtensionSubtags)(self.get_abi() as *const _ as *mut _, singleton.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILanguageFactory, 2600620716, 3111, 17656, 183, 146, 151, 147, 251, 102, 198, 62);
RT_INTERFACE!{static interface ILanguageFactory(ILanguageFactoryVtbl): IInspectable [IID_ILanguageFactory] {
    fn CreateLanguage(&self, languageTag: HSTRING, out: *mut <Language as RtType>::Abi) -> HRESULT
}}
impl ILanguageFactory {
    #[inline] pub fn create_language(&self, languageTag: &HStringArg) -> Result<Language> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateLanguage)(self.get_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(Language::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum LanguageLayoutDirection: i32 {
    Ltr = 0, Rtl = 1, TtbLtr = 2, TtbRtl = 3,
}}
DEFINE_IID!(IID_ILanguageStatics, 2990331223, 2149, 18132, 137, 184, 213, 155, 232, 153, 15, 13);
RT_INTERFACE!{static interface ILanguageStatics(ILanguageStaticsVtbl): IInspectable [IID_ILanguageStatics] {
    fn IsWellFormed(&self, languageTag: HSTRING, out: *mut bool) -> HRESULT,
    fn get_CurrentInputMethodLanguageTag(&self, out: *mut HSTRING) -> HRESULT
}}
impl ILanguageStatics {
    #[inline] pub fn is_well_formed(&self, languageTag: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().IsWellFormed)(self.get_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_input_method_language_tag(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CurrentInputMethodLanguageTag)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILanguageStatics2, 806985582, 37195, 19242, 157, 110, 227, 176, 226, 125, 190, 79);
RT_INTERFACE!{static interface ILanguageStatics2(ILanguageStatics2Vtbl): IInspectable [IID_ILanguageStatics2] {
    fn TrySetInputMethodLanguageTag(&self, languageTag: HSTRING, out: *mut bool) -> HRESULT
}}
impl ILanguageStatics2 {
    #[inline] pub fn try_set_input_method_language_tag(&self, languageTag: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().TrySetInputMethodLanguageTag)(self.get_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class NumeralSystemIdentifiers}
impl RtActivatable<INumeralSystemIdentifiersStatics> for NumeralSystemIdentifiers {}
impl RtActivatable<INumeralSystemIdentifiersStatics2> for NumeralSystemIdentifiers {}
impl NumeralSystemIdentifiers {
    #[inline] pub fn get_arab() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_arab()
    }
    #[inline] pub fn get_arab_ext() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_arab_ext()
    }
    #[inline] pub fn get_bali() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_bali()
    }
    #[inline] pub fn get_beng() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_beng()
    }
    #[inline] pub fn get_cham() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_cham()
    }
    #[inline] pub fn get_deva() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_deva()
    }
    #[inline] pub fn get_full_wide() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_full_wide()
    }
    #[inline] pub fn get_gujr() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_gujr()
    }
    #[inline] pub fn get_guru() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_guru()
    }
    #[inline] pub fn get_hani_dec() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_hani_dec()
    }
    #[inline] pub fn get_java() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_java()
    }
    #[inline] pub fn get_kali() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_kali()
    }
    #[inline] pub fn get_khmr() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_khmr()
    }
    #[inline] pub fn get_knda() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_knda()
    }
    #[inline] pub fn get_lana() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_lana()
    }
    #[inline] pub fn get_lana_tham() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_lana_tham()
    }
    #[inline] pub fn get_laoo() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_laoo()
    }
    #[inline] pub fn get_latn() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_latn()
    }
    #[inline] pub fn get_lepc() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_lepc()
    }
    #[inline] pub fn get_limb() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_limb()
    }
    #[inline] pub fn get_mlym() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mlym()
    }
    #[inline] pub fn get_mong() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mong()
    }
    #[inline] pub fn get_mtei() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mtei()
    }
    #[inline] pub fn get_mymr() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mymr()
    }
    #[inline] pub fn get_mymr_shan() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_mymr_shan()
    }
    #[inline] pub fn get_nkoo() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_nkoo()
    }
    #[inline] pub fn get_olck() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_olck()
    }
    #[inline] pub fn get_orya() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_orya()
    }
    #[inline] pub fn get_saur() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_saur()
    }
    #[inline] pub fn get_sund() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_sund()
    }
    #[inline] pub fn get_talu() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_talu()
    }
    #[inline] pub fn get_taml_dec() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_taml_dec()
    }
    #[inline] pub fn get_telu() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_telu()
    }
    #[inline] pub fn get_thai() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_thai()
    }
    #[inline] pub fn get_tibt() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_tibt()
    }
    #[inline] pub fn get_vaii() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics>>::get_activation_factory().get_vaii()
    }
    #[inline] pub fn get_brah() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_brah()
    }
    #[inline] pub fn get_osma() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_osma()
    }
    #[inline] pub fn get_math_bold() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_bold()
    }
    #[inline] pub fn get_math_dbl() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_dbl()
    }
    #[inline] pub fn get_math_sans() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_sans()
    }
    #[inline] pub fn get_math_sanb() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_sanb()
    }
    #[inline] pub fn get_math_mono() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_math_mono()
    }
    #[inline] pub fn get_zmth_bold() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_bold()
    }
    #[inline] pub fn get_zmth_dbl() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_dbl()
    }
    #[inline] pub fn get_zmth_sans() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_sans()
    }
    #[inline] pub fn get_zmth_sanb() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_sanb()
    }
    #[inline] pub fn get_zmth_mono() -> Result<HString> {
        <Self as RtActivatable<INumeralSystemIdentifiersStatics2>>::get_activation_factory().get_zmth_mono()
    }
}
DEFINE_CLSID!(NumeralSystemIdentifiers(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,101,114,97,108,83,121,115,116,101,109,73,100,101,110,116,105,102,105,101,114,115,0]) [CLSID_NumeralSystemIdentifiers]);
DEFINE_IID!(IID_INumeralSystemIdentifiersStatics, 2781242051, 26825, 19773, 183, 101, 151, 32, 41, 226, 29, 236);
RT_INTERFACE!{static interface INumeralSystemIdentifiersStatics(INumeralSystemIdentifiersStaticsVtbl): IInspectable [IID_INumeralSystemIdentifiersStatics] {
    fn get_Arab(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ArabExt(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Bali(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Beng(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Cham(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Deva(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FullWide(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Gujr(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Guru(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HaniDec(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Java(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kali(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Khmr(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Knda(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Lana(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanaTham(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Laoo(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Latn(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Lepc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Limb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Mlym(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Mong(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Mtei(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Mymr(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MymrShan(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Nkoo(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Olck(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Orya(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Saur(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sund(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Talu(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TamlDec(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Telu(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Thai(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Tibt(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Vaii(&self, out: *mut HSTRING) -> HRESULT
}}
impl INumeralSystemIdentifiersStatics {
    #[inline] pub fn get_arab(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Arab)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_arab_ext(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ArabExt)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bali(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Bali)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_beng(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Beng)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cham(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Cham)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deva(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Deva)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_full_wide(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FullWide)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gujr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Gujr)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_guru(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Guru)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hani_dec(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HaniDec)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_java(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Java)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kali(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Kali)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_khmr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Khmr)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_knda(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Knda)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lana(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Lana)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lana_tham(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LanaTham)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_laoo(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Laoo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_latn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Latn)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lepc(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Lepc)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_limb(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Limb)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mlym(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Mlym)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mong(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Mong)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mtei(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Mtei)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mymr(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Mymr)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mymr_shan(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MymrShan)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nkoo(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Nkoo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_olck(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Olck)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_orya(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Orya)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_saur(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Saur)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sund(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sund)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_talu(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Talu)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_taml_dec(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TamlDec)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_telu(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Telu)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_thai(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Thai)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tibt(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Tibt)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vaii(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Vaii)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INumeralSystemIdentifiersStatics2, 2130719272, 40411, 18996, 145, 4, 2, 96, 192, 145, 167, 199);
RT_INTERFACE!{static interface INumeralSystemIdentifiersStatics2(INumeralSystemIdentifiersStatics2Vtbl): IInspectable [IID_INumeralSystemIdentifiersStatics2] {
    fn get_Brah(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Osma(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathBold(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathDbl(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathSans(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathSanb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MathMono(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthBold(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthDbl(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthSans(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthSanb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ZmthMono(&self, out: *mut HSTRING) -> HRESULT
}}
impl INumeralSystemIdentifiersStatics2 {
    #[inline] pub fn get_brah(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Brah)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_osma(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Osma)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_math_bold(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MathBold)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_math_dbl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MathDbl)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_math_sans(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MathSans)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_math_sanb(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MathSanb)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_math_mono(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_MathMono)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_zmth_bold(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ZmthBold)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_zmth_dbl(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ZmthDbl)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_zmth_sans(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ZmthSans)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_zmth_sanb(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ZmthSanb)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_zmth_mono(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ZmthMono)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITimeZoneOnCalendar, 3141281253, 18127, 17175, 163, 245, 2, 98, 26, 213, 68, 120);
RT_INTERFACE!{interface ITimeZoneOnCalendar(ITimeZoneOnCalendarVtbl): IInspectable [IID_ITimeZoneOnCalendar] {
    fn GetTimeZone(&self, out: *mut HSTRING) -> HRESULT,
    fn ChangeTimeZone(&self, timeZoneId: HSTRING) -> HRESULT,
    fn TimeZoneAsFullString(&self, out: *mut HSTRING) -> HRESULT,
    fn TimeZoneAsString(&self, idealLength: i32, out: *mut HSTRING) -> HRESULT
}}
impl ITimeZoneOnCalendar {
    #[inline] pub fn get_time_zone(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetTimeZone)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn change_time_zone(&self, timeZoneId: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ChangeTimeZone)(self.get_abi() as *const _ as *mut _, timeZoneId.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn time_zone_as_full_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TimeZoneAsFullString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn time_zone_as_string(&self, idealLength: i32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TimeZoneAsString)(self.get_abi() as *const _ as *mut _, idealLength, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
pub mod collation { // Windows.Globalization.Collation
use crate::prelude::*;
DEFINE_IID!(IID_ICharacterGrouping, 4209467835, 32861, 19376, 149, 187, 193, 247, 195, 232, 235, 142);
RT_INTERFACE!{interface ICharacterGrouping(ICharacterGroupingVtbl): IInspectable [IID_ICharacterGrouping] {
    fn get_First(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Label(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICharacterGrouping {
    #[inline] pub fn get_first(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_First)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_label(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Label)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CharacterGrouping: ICharacterGrouping}
DEFINE_IID!(IID_ICharacterGroupings, 3100772981, 54479, 16469, 128, 229, 206, 22, 156, 34, 100, 150);
RT_INTERFACE!{interface ICharacterGroupings(ICharacterGroupingsVtbl): IInspectable [IID_ICharacterGroupings] {
    fn Lookup(&self, text: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl ICharacterGroupings {
    #[inline] pub fn lookup(&self, text: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Lookup)(self.get_abi() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CharacterGroupings: ICharacterGroupings}
impl RtActivatable<ICharacterGroupingsFactory> for CharacterGroupings {}
impl RtActivatable<IActivationFactory> for CharacterGroupings {}
impl CharacterGroupings {
    #[inline] pub fn create(language: &HStringArg) -> Result<CharacterGroupings> {
        <Self as RtActivatable<ICharacterGroupingsFactory>>::get_activation_factory().create(language)
    }
}
DEFINE_CLSID!(CharacterGroupings(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,67,111,108,108,97,116,105,111,110,46,67,104,97,114,97,99,116,101,114,71,114,111,117,112,105,110,103,115,0]) [CLSID_CharacterGroupings]);
DEFINE_IID!(IID_ICharacterGroupingsFactory, 2582290393, 34925, 17409, 159, 152, 105, 200, 45, 76, 47, 120);
RT_INTERFACE!{static interface ICharacterGroupingsFactory(ICharacterGroupingsFactoryVtbl): IInspectable [IID_ICharacterGroupingsFactory] {
    fn Create(&self, language: HSTRING, out: *mut <CharacterGroupings as RtType>::Abi) -> HRESULT
}}
impl ICharacterGroupingsFactory {
    #[inline] pub fn create(&self, language: &HStringArg) -> Result<CharacterGroupings> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, language.get(), &mut out);
        if hr == S_OK { Ok(CharacterGroupings::wrap_nonnull(out)) } else { err(hr) }
    }}
}
} // Windows.Globalization.Collation
pub mod datetimeformatting { // Windows.Globalization.DateTimeFormatting
use crate::prelude::*;
DEFINE_IID!(IID_IDateTimeFormatter, 2515454480, 29664, 20043, 161, 131, 61, 106, 208, 186, 53, 236);
RT_INTERFACE!{interface IDateTimeFormatter(IDateTimeFormatterVtbl): IInspectable [IID_IDateTimeFormatter] {
    fn get_Languages(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_GeographicRegion(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Calendar(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Clock(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumeralSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NumeralSystem(&self, value: HSTRING) -> HRESULT,
    fn get_Patterns(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_Template(&self, out: *mut HSTRING) -> HRESULT,
    fn Format(&self, value: foundation::DateTime, out: *mut HSTRING) -> HRESULT,
    fn get_IncludeYear(&self, out: *mut YearFormat) -> HRESULT,
    fn get_IncludeMonth(&self, out: *mut MonthFormat) -> HRESULT,
    fn get_IncludeDayOfWeek(&self, out: *mut DayOfWeekFormat) -> HRESULT,
    fn get_IncludeDay(&self, out: *mut DayFormat) -> HRESULT,
    fn get_IncludeHour(&self, out: *mut HourFormat) -> HRESULT,
    fn get_IncludeMinute(&self, out: *mut MinuteFormat) -> HRESULT,
    fn get_IncludeSecond(&self, out: *mut SecondFormat) -> HRESULT,
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ResolvedGeographicRegion(&self, out: *mut HSTRING) -> HRESULT
}}
impl IDateTimeFormatter {
    #[inline] pub fn get_languages(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Languages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_geographic_region(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GeographicRegion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_calendar(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Calendar)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_clock(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Clock)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_numeral_system(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NumeralSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_numeral_system(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_NumeralSystem)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_patterns(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Patterns)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_template(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Template)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn format(&self, value: foundation::DateTime) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Format)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_include_year(&self) -> Result<YearFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeYear)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_include_month(&self) -> Result<MonthFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeMonth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_include_day_of_week(&self) -> Result<DayOfWeekFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeDayOfWeek)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_include_day(&self) -> Result<DayFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeDay)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_include_hour(&self) -> Result<HourFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeHour)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_include_minute(&self) -> Result<MinuteFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeMinute)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_include_second(&self) -> Result<SecondFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeSecond)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ResolvedLanguage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_resolved_geographic_region(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ResolvedGeographicRegion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DateTimeFormatter: IDateTimeFormatter}
impl RtActivatable<IDateTimeFormatterFactory> for DateTimeFormatter {}
impl RtActivatable<IDateTimeFormatterStatics> for DateTimeFormatter {}
impl DateTimeFormatter {
    #[inline] pub fn create_date_time_formatter(formatTemplate: &HStringArg) -> Result<DateTimeFormatter> {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter(formatTemplate)
    }
    #[inline] pub fn create_date_time_formatter_languages(formatTemplate: &HStringArg, languages: &foundation::collections::IIterable<HString>) -> Result<DateTimeFormatter> {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_languages(formatTemplate, languages)
    }
    #[inline] pub fn create_date_time_formatter_context(formatTemplate: &HStringArg, languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg, calendar: &HStringArg, clock: &HStringArg) -> Result<DateTimeFormatter> {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_context(formatTemplate, languages, geographicRegion, calendar, clock)
    }
    #[inline] pub fn create_date_time_formatter_date(yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat) -> Result<DateTimeFormatter> {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_date(yearFormat, monthFormat, dayFormat, dayOfWeekFormat)
    }
    #[inline] pub fn create_date_time_formatter_time(hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat) -> Result<DateTimeFormatter> {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_time(hourFormat, minuteFormat, secondFormat)
    }
    #[inline] pub fn create_date_time_formatter_date_time_languages(yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: &foundation::collections::IIterable<HString>) -> Result<DateTimeFormatter> {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_date_time_languages(yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages)
    }
    #[inline] pub fn create_date_time_formatter_date_time_context(yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg, calendar: &HStringArg, clock: &HStringArg) -> Result<DateTimeFormatter> {
        <Self as RtActivatable<IDateTimeFormatterFactory>>::get_activation_factory().create_date_time_formatter_date_time_context(yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages, geographicRegion, calendar, clock)
    }
    #[inline] pub fn get_long_date() -> Result<Option<DateTimeFormatter>> {
        <Self as RtActivatable<IDateTimeFormatterStatics>>::get_activation_factory().get_long_date()
    }
    #[inline] pub fn get_long_time() -> Result<Option<DateTimeFormatter>> {
        <Self as RtActivatable<IDateTimeFormatterStatics>>::get_activation_factory().get_long_time()
    }
    #[inline] pub fn get_short_date() -> Result<Option<DateTimeFormatter>> {
        <Self as RtActivatable<IDateTimeFormatterStatics>>::get_activation_factory().get_short_date()
    }
    #[inline] pub fn get_short_time() -> Result<Option<DateTimeFormatter>> {
        <Self as RtActivatable<IDateTimeFormatterStatics>>::get_activation_factory().get_short_time()
    }
}
DEFINE_CLSID!(DateTimeFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,68,97,116,101,84,105,109,101,70,111,114,109,97,116,116,105,110,103,46,68,97,116,101,84,105,109,101,70,111,114,109,97,116,116,101,114,0]) [CLSID_DateTimeFormatter]);
DEFINE_IID!(IID_IDateTimeFormatter2, 667490950, 48554, 20432, 158, 54, 103, 29, 90, 165, 238, 3);
RT_INTERFACE!{interface IDateTimeFormatter2(IDateTimeFormatter2Vtbl): IInspectable [IID_IDateTimeFormatter2] {
    fn FormatUsingTimeZone(&self, datetime: foundation::DateTime, timeZoneId: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IDateTimeFormatter2 {
    #[inline] pub fn format_using_time_zone(&self, datetime: foundation::DateTime, timeZoneId: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatUsingTimeZone)(self.get_abi() as *const _ as *mut _, datetime, timeZoneId.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDateTimeFormatterFactory, 3968698963, 6702, 16685, 136, 21, 59, 116, 95, 177, 162, 160);
RT_INTERFACE!{static interface IDateTimeFormatterFactory(IDateTimeFormatterFactoryVtbl): IInspectable [IID_IDateTimeFormatterFactory] {
    fn CreateDateTimeFormatter(&self, formatTemplate: HSTRING, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT,
    fn CreateDateTimeFormatterLanguages(&self, formatTemplate: HSTRING, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT,
    fn CreateDateTimeFormatterContext(&self, formatTemplate: HSTRING, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, geographicRegion: HSTRING, calendar: HSTRING, clock: HSTRING, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT,
    fn CreateDateTimeFormatterDate(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT,
    fn CreateDateTimeFormatterTime(&self, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT,
    fn CreateDateTimeFormatterDateTimeLanguages(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT,
    fn CreateDateTimeFormatterDateTimeContext(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, geographicRegion: HSTRING, calendar: HSTRING, clock: HSTRING, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT
}}
impl IDateTimeFormatterFactory {
    #[inline] pub fn create_date_time_formatter(&self, formatTemplate: &HStringArg) -> Result<DateTimeFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateDateTimeFormatter)(self.get_abi() as *const _ as *mut _, formatTemplate.get(), &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_date_time_formatter_languages(&self, formatTemplate: &HStringArg, languages: &foundation::collections::IIterable<HString>) -> Result<DateTimeFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateDateTimeFormatterLanguages)(self.get_abi() as *const _ as *mut _, formatTemplate.get(), languages.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_date_time_formatter_context(&self, formatTemplate: &HStringArg, languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg, calendar: &HStringArg, clock: &HStringArg) -> Result<DateTimeFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateDateTimeFormatterContext)(self.get_abi() as *const _ as *mut _, formatTemplate.get(), languages.get_abi() as *const _ as *mut _, geographicRegion.get(), calendar.get(), clock.get(), &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_date_time_formatter_date(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat) -> Result<DateTimeFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateDateTimeFormatterDate)(self.get_abi() as *const _ as *mut _, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_date_time_formatter_time(&self, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat) -> Result<DateTimeFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateDateTimeFormatterTime)(self.get_abi() as *const _ as *mut _, hourFormat, minuteFormat, secondFormat, &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_date_time_formatter_date_time_languages(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: &foundation::collections::IIterable<HString>) -> Result<DateTimeFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateDateTimeFormatterDateTimeLanguages)(self.get_abi() as *const _ as *mut _, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_date_time_formatter_date_time_context(&self, yearFormat: YearFormat, monthFormat: MonthFormat, dayFormat: DayFormat, dayOfWeekFormat: DayOfWeekFormat, hourFormat: HourFormat, minuteFormat: MinuteFormat, secondFormat: SecondFormat, languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg, calendar: &HStringArg, clock: &HStringArg) -> Result<DateTimeFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateDateTimeFormatterDateTimeContext)(self.get_abi() as *const _ as *mut _, yearFormat, monthFormat, dayFormat, dayOfWeekFormat, hourFormat, minuteFormat, secondFormat, languages.get_abi() as *const _ as *mut _, geographicRegion.get(), calendar.get(), clock.get(), &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDateTimeFormatterStatics, 3217942464, 57164, 18990, 144, 18, 244, 125, 175, 63, 18, 18);
RT_INTERFACE!{static interface IDateTimeFormatterStatics(IDateTimeFormatterStaticsVtbl): IInspectable [IID_IDateTimeFormatterStatics] {
    fn get_LongDate(&self, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT,
    fn get_LongTime(&self, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT,
    fn get_ShortDate(&self, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT,
    fn get_ShortTime(&self, out: *mut <DateTimeFormatter as RtType>::Abi) -> HRESULT
}}
impl IDateTimeFormatterStatics {
    #[inline] pub fn get_long_date(&self) -> Result<Option<DateTimeFormatter>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LongDate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_long_time(&self) -> Result<Option<DateTimeFormatter>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LongTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_short_date(&self) -> Result<Option<DateTimeFormatter>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ShortDate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_short_time(&self) -> Result<Option<DateTimeFormatter>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ShortTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DateTimeFormatter::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum DayFormat: i32 {
    None = 0, Default = 1,
}}
RT_ENUM! { enum DayOfWeekFormat: i32 {
    None = 0, Default = 1, Abbreviated = 2, Full = 3,
}}
RT_ENUM! { enum HourFormat: i32 {
    None = 0, Default = 1,
}}
RT_ENUM! { enum MinuteFormat: i32 {
    None = 0, Default = 1,
}}
RT_ENUM! { enum MonthFormat: i32 {
    None = 0, Default = 1, Abbreviated = 2, Full = 3, Numeric = 4,
}}
RT_ENUM! { enum SecondFormat: i32 {
    None = 0, Default = 1,
}}
RT_ENUM! { enum YearFormat: i32 {
    None = 0, Default = 1, Abbreviated = 2, Full = 3,
}}
} // Windows.Globalization.DateTimeFormatting
pub mod fonts { // Windows.Globalization.Fonts
use crate::prelude::*;
DEFINE_IID!(IID_ILanguageFont, 2972605498, 46957, 17819, 190, 235, 144, 17, 81, 205, 119, 209);
RT_INTERFACE!{interface ILanguageFont(ILanguageFontVtbl): IInspectable [IID_ILanguageFont] {
    fn get_FontFamily(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_FontWeight(&self, out: *mut super::super::ui::text::FontWeight) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_FontStretch(&self, out: *mut super::super::ui::text::FontStretch) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_FontStyle(&self, out: *mut super::super::ui::text::FontStyle) -> HRESULT,
    fn get_ScaleFactor(&self, out: *mut f64) -> HRESULT
}}
impl ILanguageFont {
    #[inline] pub fn get_font_family(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FontFamily)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_font_weight(&self) -> Result<super::super::ui::text::FontWeight> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FontWeight)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_font_stretch(&self) -> Result<super::super::ui::text::FontStretch> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FontStretch)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_font_style(&self) -> Result<super::super::ui::text::FontStyle> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FontStyle)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_scale_factor(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ScaleFactor)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class LanguageFont: ILanguageFont}
DEFINE_IID!(IID_ILanguageFontGroup, 4080697283, 14940, 19178, 185, 255, 179, 159, 178, 66, 247, 246);
RT_INTERFACE!{interface ILanguageFontGroup(ILanguageFontGroupVtbl): IInspectable [IID_ILanguageFontGroup] {
    fn get_UITextFont(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_UIHeadingFont(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_UITitleFont(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_UICaptionFont(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_UINotificationHeadingFont(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_TraditionalDocumentFont(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_ModernDocumentFont(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_DocumentHeadingFont(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_FixedWidthTextFont(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_DocumentAlternate1Font(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT,
    fn get_DocumentAlternate2Font(&self, out: *mut <LanguageFont as RtType>::Abi) -> HRESULT
}}
impl ILanguageFontGroup {
    #[inline] pub fn get_ui_text_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UITextFont)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ui_heading_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UIHeadingFont)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ui_title_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UITitleFont)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ui_caption_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UICaptionFont)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ui_notification_heading_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UINotificationHeadingFont)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_traditional_document_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TraditionalDocumentFont)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_modern_document_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ModernDocumentFont)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_heading_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DocumentHeadingFont)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_fixed_width_text_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FixedWidthTextFont)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_alternate1_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DocumentAlternate1Font)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_alternate2_font(&self) -> Result<Option<LanguageFont>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DocumentAlternate2Font)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LanguageFont::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LanguageFontGroup: ILanguageFontGroup}
impl RtActivatable<ILanguageFontGroupFactory> for LanguageFontGroup {}
impl LanguageFontGroup {
    #[inline] pub fn create_language_font_group(languageTag: &HStringArg) -> Result<LanguageFontGroup> {
        <Self as RtActivatable<ILanguageFontGroupFactory>>::get_activation_factory().create_language_font_group(languageTag)
    }
}
DEFINE_CLSID!(LanguageFontGroup(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,70,111,110,116,115,46,76,97,110,103,117,97,103,101,70,111,110,116,71,114,111,117,112,0]) [CLSID_LanguageFontGroup]);
DEFINE_IID!(IID_ILanguageFontGroupFactory, 4239305831, 20087, 18887, 184, 86, 221, 233, 52, 252, 115, 91);
RT_INTERFACE!{static interface ILanguageFontGroupFactory(ILanguageFontGroupFactoryVtbl): IInspectable [IID_ILanguageFontGroupFactory] {
    fn CreateLanguageFontGroup(&self, languageTag: HSTRING, out: *mut <LanguageFontGroup as RtType>::Abi) -> HRESULT
}}
impl ILanguageFontGroupFactory {
    #[inline] pub fn create_language_font_group(&self, languageTag: &HStringArg) -> Result<LanguageFontGroup> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateLanguageFontGroup)(self.get_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(LanguageFontGroup::wrap_nonnull(out)) } else { err(hr) }
    }}
}
} // Windows.Globalization.Fonts
pub mod numberformatting { // Windows.Globalization.NumberFormatting
use crate::prelude::*;
DEFINE_IID!(IID_ICurrencyFormatter, 292752549, 19200, 16818, 179, 50, 115, 177, 42, 73, 125, 84);
RT_INTERFACE!{interface ICurrencyFormatter(ICurrencyFormatterVtbl): IInspectable [IID_ICurrencyFormatter] {
    fn get_Currency(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Currency(&self, value: HSTRING) -> HRESULT
}}
impl ICurrencyFormatter {
    #[inline] pub fn get_currency(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Currency)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_currency(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Currency)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class CurrencyFormatter: ICurrencyFormatter}
impl RtActivatable<ICurrencyFormatterFactory> for CurrencyFormatter {}
impl CurrencyFormatter {
    #[inline] pub fn create_currency_formatter_code(currencyCode: &HStringArg) -> Result<CurrencyFormatter> {
        <Self as RtActivatable<ICurrencyFormatterFactory>>::get_activation_factory().create_currency_formatter_code(currencyCode)
    }
    #[inline] pub fn create_currency_formatter_code_context(currencyCode: &HStringArg, languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<CurrencyFormatter> {
        <Self as RtActivatable<ICurrencyFormatterFactory>>::get_activation_factory().create_currency_formatter_code_context(currencyCode, languages, geographicRegion)
    }
}
DEFINE_CLSID!(CurrencyFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,67,117,114,114,101,110,99,121,70,111,114,109,97,116,116,101,114,0]) [CLSID_CurrencyFormatter]);
DEFINE_IID!(IID_ICurrencyFormatter2, 120336157, 59322, 16791, 146, 14, 36, 124, 146, 247, 222, 166);
RT_INTERFACE!{interface ICurrencyFormatter2(ICurrencyFormatter2Vtbl): IInspectable [IID_ICurrencyFormatter2] {
    fn get_Mode(&self, out: *mut CurrencyFormatterMode) -> HRESULT,
    fn put_Mode(&self, value: CurrencyFormatterMode) -> HRESULT,
    fn ApplyRoundingForCurrency(&self, roundingAlgorithm: RoundingAlgorithm) -> HRESULT
}}
impl ICurrencyFormatter2 {
    #[inline] pub fn get_mode(&self) -> Result<CurrencyFormatterMode> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Mode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_mode(&self, value: CurrencyFormatterMode) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Mode)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn apply_rounding_for_currency(&self, roundingAlgorithm: RoundingAlgorithm) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ApplyRoundingForCurrency)(self.get_abi() as *const _ as *mut _, roundingAlgorithm);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICurrencyFormatterFactory, 2261209982, 47416, 19106, 132, 176, 44, 51, 220, 91, 20, 80);
RT_INTERFACE!{static interface ICurrencyFormatterFactory(ICurrencyFormatterFactoryVtbl): IInspectable [IID_ICurrencyFormatterFactory] {
    fn CreateCurrencyFormatterCode(&self, currencyCode: HSTRING, out: *mut <CurrencyFormatter as RtType>::Abi) -> HRESULT,
    fn CreateCurrencyFormatterCodeContext(&self, currencyCode: HSTRING, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, geographicRegion: HSTRING, out: *mut <CurrencyFormatter as RtType>::Abi) -> HRESULT
}}
impl ICurrencyFormatterFactory {
    #[inline] pub fn create_currency_formatter_code(&self, currencyCode: &HStringArg) -> Result<CurrencyFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCurrencyFormatterCode)(self.get_abi() as *const _ as *mut _, currencyCode.get(), &mut out);
        if hr == S_OK { Ok(CurrencyFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_currency_formatter_code_context(&self, currencyCode: &HStringArg, languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<CurrencyFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCurrencyFormatterCodeContext)(self.get_abi() as *const _ as *mut _, currencyCode.get(), languages.get_abi() as *const _ as *mut _, geographicRegion.get(), &mut out);
        if hr == S_OK { Ok(CurrencyFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum CurrencyFormatterMode: i32 {
    UseSymbol = 0, UseCurrencyCode = 1,
}}
RT_CLASS!{class DecimalFormatter: INumberFormatter}
impl RtActivatable<IDecimalFormatterFactory> for DecimalFormatter {}
impl RtActivatable<IActivationFactory> for DecimalFormatter {}
impl DecimalFormatter {
    #[inline] pub fn create_decimal_formatter(languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<DecimalFormatter> {
        <Self as RtActivatable<IDecimalFormatterFactory>>::get_activation_factory().create_decimal_formatter(languages, geographicRegion)
    }
}
DEFINE_CLSID!(DecimalFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,68,101,99,105,109,97,108,70,111,114,109,97,116,116,101,114,0]) [CLSID_DecimalFormatter]);
DEFINE_IID!(IID_IDecimalFormatterFactory, 218205338, 58259, 18104, 184, 48, 122, 105, 200, 248, 159, 187);
RT_INTERFACE!{static interface IDecimalFormatterFactory(IDecimalFormatterFactoryVtbl): IInspectable [IID_IDecimalFormatterFactory] {
    fn CreateDecimalFormatter(&self, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, geographicRegion: HSTRING, out: *mut <DecimalFormatter as RtType>::Abi) -> HRESULT
}}
impl IDecimalFormatterFactory {
    #[inline] pub fn create_decimal_formatter(&self, languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<DecimalFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateDecimalFormatter)(self.get_abi() as *const _ as *mut _, languages.get_abi() as *const _ as *mut _, geographicRegion.get(), &mut out);
        if hr == S_OK { Ok(DecimalFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IIncrementNumberRounder, 1889947640, 26283, 16725, 157, 161, 115, 158, 70, 118, 69, 67);
RT_INTERFACE!{interface IIncrementNumberRounder(IIncrementNumberRounderVtbl): IInspectable [IID_IIncrementNumberRounder] {
    fn get_RoundingAlgorithm(&self, out: *mut RoundingAlgorithm) -> HRESULT,
    fn put_RoundingAlgorithm(&self, value: RoundingAlgorithm) -> HRESULT,
    fn get_Increment(&self, out: *mut f64) -> HRESULT,
    fn put_Increment(&self, value: f64) -> HRESULT
}}
impl IIncrementNumberRounder {
    #[inline] pub fn get_rounding_algorithm(&self) -> Result<RoundingAlgorithm> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RoundingAlgorithm)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_rounding_algorithm(&self, value: RoundingAlgorithm) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_RoundingAlgorithm)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_increment(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Increment)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_increment(&self, value: f64) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Increment)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class IncrementNumberRounder: INumberRounder}
impl RtActivatable<IActivationFactory> for IncrementNumberRounder {}
DEFINE_CLSID!(IncrementNumberRounder(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,73,110,99,114,101,109,101,110,116,78,117,109,98,101,114,82,111,117,110,100,101,114,0]) [CLSID_IncrementNumberRounder]);
DEFINE_IID!(IID_INumberFormatter, 2768272457, 30326, 19895, 134, 49, 27, 111, 242, 101, 202, 169);
RT_INTERFACE!{interface INumberFormatter(INumberFormatterVtbl): IInspectable [IID_INumberFormatter] {
    fn FormatInt(&self, value: i64, out: *mut HSTRING) -> HRESULT,
    fn FormatUInt(&self, value: u64, out: *mut HSTRING) -> HRESULT,
    fn FormatDouble(&self, value: f64, out: *mut HSTRING) -> HRESULT
}}
impl INumberFormatter {
    #[inline] pub fn format_int(&self, value: i64) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatInt)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn format_uint(&self, value: u64) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatUInt)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn format_double(&self, value: f64) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatDouble)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INumberFormatter2, 3567829488, 32976, 19213, 168, 158, 136, 44, 30, 143, 131, 16);
RT_INTERFACE!{interface INumberFormatter2(INumberFormatter2Vtbl): IInspectable [IID_INumberFormatter2] {
    fn FormatInt(&self, value: i64, out: *mut HSTRING) -> HRESULT,
    fn FormatUInt(&self, value: u64, out: *mut HSTRING) -> HRESULT,
    fn FormatDouble(&self, value: f64, out: *mut HSTRING) -> HRESULT
}}
impl INumberFormatter2 {
    #[inline] pub fn format_int(&self, value: i64) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatInt)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn format_uint(&self, value: u64) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatUInt)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn format_double(&self, value: f64) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatDouble)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INumberFormatterOptions, 2150837537, 44769, 19001, 186, 162, 7, 237, 140, 150, 218, 246);
RT_INTERFACE!{interface INumberFormatterOptions(INumberFormatterOptionsVtbl): IInspectable [IID_INumberFormatterOptions] {
    fn get_Languages(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_GeographicRegion(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IntegerDigits(&self, out: *mut i32) -> HRESULT,
    fn put_IntegerDigits(&self, value: i32) -> HRESULT,
    fn get_FractionDigits(&self, out: *mut i32) -> HRESULT,
    fn put_FractionDigits(&self, value: i32) -> HRESULT,
    fn get_IsGrouped(&self, out: *mut bool) -> HRESULT,
    fn put_IsGrouped(&self, value: bool) -> HRESULT,
    fn get_IsDecimalPointAlwaysDisplayed(&self, out: *mut bool) -> HRESULT,
    fn put_IsDecimalPointAlwaysDisplayed(&self, value: bool) -> HRESULT,
    fn get_NumeralSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NumeralSystem(&self, value: HSTRING) -> HRESULT,
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ResolvedGeographicRegion(&self, out: *mut HSTRING) -> HRESULT
}}
impl INumberFormatterOptions {
    #[inline] pub fn get_languages(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Languages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_geographic_region(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_GeographicRegion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_integer_digits(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IntegerDigits)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_integer_digits(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IntegerDigits)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_fraction_digits(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_FractionDigits)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_fraction_digits(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_FractionDigits)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_grouped(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsGrouped)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_grouped(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IsGrouped)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_decimal_point_always_displayed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsDecimalPointAlwaysDisplayed)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_decimal_point_always_displayed(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IsDecimalPointAlwaysDisplayed)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_numeral_system(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NumeralSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_numeral_system(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_NumeralSystem)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ResolvedLanguage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_resolved_geographic_region(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ResolvedGeographicRegion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INumberParser, 3865416722, 18963, 19027, 131, 161, 57, 47, 190, 76, 255, 159);
RT_INTERFACE!{interface INumberParser(INumberParserVtbl): IInspectable [IID_INumberParser] {
    fn ParseInt(&self, text: HSTRING, out: *mut <foundation::IReference<i64> as RtType>::Abi) -> HRESULT,
    fn ParseUInt(&self, text: HSTRING, out: *mut <foundation::IReference<u64> as RtType>::Abi) -> HRESULT,
    fn ParseDouble(&self, text: HSTRING, out: *mut <foundation::IReference<f64> as RtType>::Abi) -> HRESULT
}}
impl INumberParser {
    #[inline] pub fn parse_int(&self, text: &HStringArg) -> Result<Option<foundation::IReference<i64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ParseInt)(self.get_abi() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn parse_uint(&self, text: &HStringArg) -> Result<Option<foundation::IReference<u64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ParseUInt)(self.get_abi() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn parse_double(&self, text: &HStringArg) -> Result<Option<foundation::IReference<f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ParseDouble)(self.get_abi() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INumberRounder, 1416872821, 14573, 17969, 184, 12, 239, 52, 252, 72, 183, 245);
RT_INTERFACE!{interface INumberRounder(INumberRounderVtbl): IInspectable [IID_INumberRounder] {
    fn RoundInt32(&self, value: i32, out: *mut i32) -> HRESULT,
    fn RoundUInt32(&self, value: u32, out: *mut u32) -> HRESULT,
    fn RoundInt64(&self, value: i64, out: *mut i64) -> HRESULT,
    fn RoundUInt64(&self, value: u64, out: *mut u64) -> HRESULT,
    fn RoundSingle(&self, value: f32, out: *mut f32) -> HRESULT,
    fn RoundDouble(&self, value: f64, out: *mut f64) -> HRESULT
}}
impl INumberRounder {
    #[inline] pub fn round_int32(&self, value: i32) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().RoundInt32)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn round_uint32(&self, value: u32) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().RoundUInt32)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn round_int64(&self, value: i64) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().RoundInt64)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn round_uint64(&self, value: u64) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().RoundUInt64)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn round_single(&self, value: f32) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().RoundSingle)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn round_double(&self, value: f64) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().RoundDouble)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INumberRounderOption, 990413875, 25711, 20222, 141, 72, 102, 235, 46, 73, 231, 54);
RT_INTERFACE!{interface INumberRounderOption(INumberRounderOptionVtbl): IInspectable [IID_INumberRounderOption] {
    fn get_NumberRounder(&self, out: *mut <INumberRounder as RtType>::Abi) -> HRESULT,
    fn put_NumberRounder(&self, value: <INumberRounder as RtType>::Abi) -> HRESULT
}}
impl INumberRounderOption {
    #[inline] pub fn get_number_rounder(&self) -> Result<Option<INumberRounder>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NumberRounder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(INumberRounder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_number_rounder(&self, value: &INumberRounder) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_NumberRounder)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INumeralSystemTranslator, 687193132, 35875, 16948, 173, 46, 250, 90, 58, 66, 110, 155);
RT_INTERFACE!{interface INumeralSystemTranslator(INumeralSystemTranslatorVtbl): IInspectable [IID_INumeralSystemTranslator] {
    fn get_Languages(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumeralSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NumeralSystem(&self, value: HSTRING) -> HRESULT,
    fn TranslateNumerals(&self, value: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl INumeralSystemTranslator {
    #[inline] pub fn get_languages(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Languages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ResolvedLanguage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_numeral_system(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NumeralSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_numeral_system(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_NumeralSystem)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn translate_numerals(&self, value: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TranslateNumerals)(self.get_abi() as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class NumeralSystemTranslator: INumeralSystemTranslator}
impl RtActivatable<INumeralSystemTranslatorFactory> for NumeralSystemTranslator {}
impl RtActivatable<IActivationFactory> for NumeralSystemTranslator {}
impl NumeralSystemTranslator {
    #[inline] pub fn create(languages: &foundation::collections::IIterable<HString>) -> Result<NumeralSystemTranslator> {
        <Self as RtActivatable<INumeralSystemTranslatorFactory>>::get_activation_factory().create(languages)
    }
}
DEFINE_CLSID!(NumeralSystemTranslator(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,78,117,109,101,114,97,108,83,121,115,116,101,109,84,114,97,110,115,108,97,116,111,114,0]) [CLSID_NumeralSystemTranslator]);
DEFINE_IID!(IID_INumeralSystemTranslatorFactory, 2519779546, 14063, 19848, 168, 92, 111, 13, 152, 214, 32, 166);
RT_INTERFACE!{static interface INumeralSystemTranslatorFactory(INumeralSystemTranslatorFactoryVtbl): IInspectable [IID_INumeralSystemTranslatorFactory] {
    fn Create(&self, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <NumeralSystemTranslator as RtType>::Abi) -> HRESULT
}}
impl INumeralSystemTranslatorFactory {
    #[inline] pub fn create(&self, languages: &foundation::collections::IIterable<HString>) -> Result<NumeralSystemTranslator> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, languages.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(NumeralSystemTranslator::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PercentFormatter: INumberFormatter}
impl RtActivatable<IPercentFormatterFactory> for PercentFormatter {}
impl RtActivatable<IActivationFactory> for PercentFormatter {}
impl PercentFormatter {
    #[inline] pub fn create_percent_formatter(languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<PercentFormatter> {
        <Self as RtActivatable<IPercentFormatterFactory>>::get_activation_factory().create_percent_formatter(languages, geographicRegion)
    }
}
DEFINE_CLSID!(PercentFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,80,101,114,99,101,110,116,70,111,114,109,97,116,116,101,114,0]) [CLSID_PercentFormatter]);
DEFINE_IID!(IID_IPercentFormatterFactory, 3078785775, 65236, 16408, 166, 226, 224, 153, 97, 224, 55, 101);
RT_INTERFACE!{static interface IPercentFormatterFactory(IPercentFormatterFactoryVtbl): IInspectable [IID_IPercentFormatterFactory] {
    fn CreatePercentFormatter(&self, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, geographicRegion: HSTRING, out: *mut <PercentFormatter as RtType>::Abi) -> HRESULT
}}
impl IPercentFormatterFactory {
    #[inline] pub fn create_percent_formatter(&self, languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<PercentFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreatePercentFormatter)(self.get_abi() as *const _ as *mut _, languages.get_abi() as *const _ as *mut _, geographicRegion.get(), &mut out);
        if hr == S_OK { Ok(PercentFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PermilleFormatter: INumberFormatter}
impl RtActivatable<IPermilleFormatterFactory> for PermilleFormatter {}
impl RtActivatable<IActivationFactory> for PermilleFormatter {}
impl PermilleFormatter {
    #[inline] pub fn create_permille_formatter(languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<PermilleFormatter> {
        <Self as RtActivatable<IPermilleFormatterFactory>>::get_activation_factory().create_permille_formatter(languages, geographicRegion)
    }
}
DEFINE_CLSID!(PermilleFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,80,101,114,109,105,108,108,101,70,111,114,109,97,116,116,101,114,0]) [CLSID_PermilleFormatter]);
DEFINE_IID!(IID_IPermilleFormatterFactory, 725071020, 58936, 20181, 169, 152, 98, 246, 176, 106, 73, 174);
RT_INTERFACE!{static interface IPermilleFormatterFactory(IPermilleFormatterFactoryVtbl): IInspectable [IID_IPermilleFormatterFactory] {
    fn CreatePermilleFormatter(&self, languages: <foundation::collections::IIterable<HString> as RtType>::Abi, geographicRegion: HSTRING, out: *mut <PermilleFormatter as RtType>::Abi) -> HRESULT
}}
impl IPermilleFormatterFactory {
    #[inline] pub fn create_permille_formatter(&self, languages: &foundation::collections::IIterable<HString>, geographicRegion: &HStringArg) -> Result<PermilleFormatter> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreatePermilleFormatter)(self.get_abi() as *const _ as *mut _, languages.get_abi() as *const _ as *mut _, geographicRegion.get(), &mut out);
        if hr == S_OK { Ok(PermilleFormatter::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum RoundingAlgorithm: i32 {
    None = 0, RoundDown = 1, RoundUp = 2, RoundTowardsZero = 3, RoundAwayFromZero = 4, RoundHalfDown = 5, RoundHalfUp = 6, RoundHalfTowardsZero = 7, RoundHalfAwayFromZero = 8, RoundHalfToEven = 9, RoundHalfToOdd = 10,
}}
DEFINE_IID!(IID_ISignedZeroOption, 4246527281, 2620, 18884, 166, 66, 150, 161, 86, 79, 79, 48);
RT_INTERFACE!{interface ISignedZeroOption(ISignedZeroOptionVtbl): IInspectable [IID_ISignedZeroOption] {
    fn get_IsZeroSigned(&self, out: *mut bool) -> HRESULT,
    fn put_IsZeroSigned(&self, value: bool) -> HRESULT
}}
impl ISignedZeroOption {
    #[inline] pub fn get_is_zero_signed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsZeroSigned)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_zero_signed(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IsZeroSigned)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISignificantDigitsNumberRounder, 4120124362, 26182, 18707, 140, 118, 27, 25, 31, 249, 77, 253);
RT_INTERFACE!{interface ISignificantDigitsNumberRounder(ISignificantDigitsNumberRounderVtbl): IInspectable [IID_ISignificantDigitsNumberRounder] {
    fn get_RoundingAlgorithm(&self, out: *mut RoundingAlgorithm) -> HRESULT,
    fn put_RoundingAlgorithm(&self, value: RoundingAlgorithm) -> HRESULT,
    fn get_SignificantDigits(&self, out: *mut u32) -> HRESULT,
    fn put_SignificantDigits(&self, value: u32) -> HRESULT
}}
impl ISignificantDigitsNumberRounder {
    #[inline] pub fn get_rounding_algorithm(&self) -> Result<RoundingAlgorithm> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RoundingAlgorithm)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_rounding_algorithm(&self, value: RoundingAlgorithm) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_RoundingAlgorithm)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_significant_digits(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_SignificantDigits)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_significant_digits(&self, value: u32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_SignificantDigits)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SignificantDigitsNumberRounder: INumberRounder}
impl RtActivatable<IActivationFactory> for SignificantDigitsNumberRounder {}
DEFINE_CLSID!(SignificantDigitsNumberRounder(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,83,105,103,110,105,102,105,99,97,110,116,68,105,103,105,116,115,78,117,109,98,101,114,82,111,117,110,100,101,114,0]) [CLSID_SignificantDigitsNumberRounder]);
DEFINE_IID!(IID_ISignificantDigitsOption, 491650269, 11587, 20200, 187, 241, 193, 178, 106, 113, 26, 88);
RT_INTERFACE!{interface ISignificantDigitsOption(ISignificantDigitsOptionVtbl): IInspectable [IID_ISignificantDigitsOption] {
    fn get_SignificantDigits(&self, out: *mut i32) -> HRESULT,
    fn put_SignificantDigits(&self, value: i32) -> HRESULT
}}
impl ISignificantDigitsOption {
    #[inline] pub fn get_significant_digits(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_SignificantDigits)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_significant_digits(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_SignificantDigits)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
} // Windows.Globalization.NumberFormatting
pub mod phonenumberformatting { // Windows.Globalization.PhoneNumberFormatting
use crate::prelude::*;
RT_ENUM! { enum PhoneNumberFormat: i32 {
    E164 = 0, International = 1, National = 2, Rfc3966 = 3,
}}
DEFINE_IID!(IID_IPhoneNumberFormatter, 358003870, 47828, 19274, 144, 13, 68, 7, 173, 183, 201, 129);
RT_INTERFACE!{interface IPhoneNumberFormatter(IPhoneNumberFormatterVtbl): IInspectable [IID_IPhoneNumberFormatter] {
    fn Format(&self, number: <PhoneNumberInfo as RtType>::Abi, out: *mut HSTRING) -> HRESULT,
    fn FormatWithOutputFormat(&self, number: <PhoneNumberInfo as RtType>::Abi, numberFormat: PhoneNumberFormat, out: *mut HSTRING) -> HRESULT,
    fn FormatPartialString(&self, number: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn FormatString(&self, number: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn FormatStringWithLeftToRightMarkers(&self, number: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IPhoneNumberFormatter {
    #[inline] pub fn format(&self, number: &PhoneNumberInfo) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Format)(self.get_abi() as *const _ as *mut _, number.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn format_with_output_format(&self, number: &PhoneNumberInfo, numberFormat: PhoneNumberFormat) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatWithOutputFormat)(self.get_abi() as *const _ as *mut _, number.get_abi() as *const _ as *mut _, numberFormat, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn format_partial_string(&self, number: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatPartialString)(self.get_abi() as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn format_string(&self, number: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatString)(self.get_abi() as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn format_string_with_left_to_right_markers(&self, number: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FormatStringWithLeftToRightMarkers)(self.get_abi() as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PhoneNumberFormatter: IPhoneNumberFormatter}
impl RtActivatable<IPhoneNumberFormatterStatics> for PhoneNumberFormatter {}
impl RtActivatable<IActivationFactory> for PhoneNumberFormatter {}
impl PhoneNumberFormatter {
    #[inline] pub fn try_create(regionCode: &HStringArg) -> Result<Option<PhoneNumberFormatter>> {
        <Self as RtActivatable<IPhoneNumberFormatterStatics>>::get_activation_factory().try_create(regionCode)
    }
    #[inline] pub fn get_country_code_for_region(regionCode: &HStringArg) -> Result<i32> {
        <Self as RtActivatable<IPhoneNumberFormatterStatics>>::get_activation_factory().get_country_code_for_region(regionCode)
    }
    #[inline] pub fn get_national_direct_dialing_prefix_for_region(regionCode: &HStringArg, stripNonDigit: bool) -> Result<HString> {
        <Self as RtActivatable<IPhoneNumberFormatterStatics>>::get_activation_factory().get_national_direct_dialing_prefix_for_region(regionCode, stripNonDigit)
    }
    #[inline] pub fn wrap_with_left_to_right_markers(number: &HStringArg) -> Result<HString> {
        <Self as RtActivatable<IPhoneNumberFormatterStatics>>::get_activation_factory().wrap_with_left_to_right_markers(number)
    }
}
DEFINE_CLSID!(PhoneNumberFormatter(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,80,104,111,110,101,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,80,104,111,110,101,78,117,109,98,101,114,70,111,114,109,97,116,116,101,114,0]) [CLSID_PhoneNumberFormatter]);
DEFINE_IID!(IID_IPhoneNumberFormatterStatics, 1554446641, 34009, 16715, 171, 78, 160, 85, 44, 135, 134, 2);
RT_INTERFACE!{static interface IPhoneNumberFormatterStatics(IPhoneNumberFormatterStaticsVtbl): IInspectable [IID_IPhoneNumberFormatterStatics] {
    fn TryCreate(&self, regionCode: HSTRING, phoneNumber: *mut <PhoneNumberFormatter as RtType>::Abi) -> HRESULT,
    fn GetCountryCodeForRegion(&self, regionCode: HSTRING, out: *mut i32) -> HRESULT,
    fn GetNationalDirectDialingPrefixForRegion(&self, regionCode: HSTRING, stripNonDigit: bool, out: *mut HSTRING) -> HRESULT,
    fn WrapWithLeftToRightMarkers(&self, number: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IPhoneNumberFormatterStatics {
    #[inline] pub fn try_create(&self, regionCode: &HStringArg) -> Result<Option<PhoneNumberFormatter>> { unsafe { 
        let mut phoneNumber = null_mut();
        let hr = (self.get_vtbl().TryCreate)(self.get_abi() as *const _ as *mut _, regionCode.get(), &mut phoneNumber);
        if hr == S_OK { Ok(PhoneNumberFormatter::wrap(phoneNumber)) } else { err(hr) }
    }}
    #[inline] pub fn get_country_code_for_region(&self, regionCode: &HStringArg) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().GetCountryCodeForRegion)(self.get_abi() as *const _ as *mut _, regionCode.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_national_direct_dialing_prefix_for_region(&self, regionCode: &HStringArg, stripNonDigit: bool) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetNationalDirectDialingPrefixForRegion)(self.get_abi() as *const _ as *mut _, regionCode.get(), stripNonDigit, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn wrap_with_left_to_right_markers(&self, number: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().WrapWithLeftToRightMarkers)(self.get_abi() as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPhoneNumberInfo, 477947101, 51380, 20131, 154, 239, 179, 66, 226, 197, 180, 23);
RT_INTERFACE!{interface IPhoneNumberInfo(IPhoneNumberInfoVtbl): IInspectable [IID_IPhoneNumberInfo] {
    fn get_CountryCode(&self, out: *mut i32) -> HRESULT,
    fn get_PhoneNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn GetLengthOfGeographicalAreaCode(&self, out: *mut i32) -> HRESULT,
    fn GetNationalSignificantNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn GetLengthOfNationalDestinationCode(&self, out: *mut i32) -> HRESULT,
    fn PredictNumberKind(&self, out: *mut PredictedPhoneNumberKind) -> HRESULT,
    fn GetGeographicRegionCode(&self, out: *mut HSTRING) -> HRESULT,
    fn CheckNumberMatch(&self, otherNumber: <PhoneNumberInfo as RtType>::Abi, out: *mut PhoneNumberMatchResult) -> HRESULT
}}
impl IPhoneNumberInfo {
    #[inline] pub fn get_country_code(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CountryCode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_phone_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PhoneNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_length_of_geographical_area_code(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().GetLengthOfGeographicalAreaCode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_national_significant_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetNationalSignificantNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_length_of_national_destination_code(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().GetLengthOfNationalDestinationCode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn predict_number_kind(&self) -> Result<PredictedPhoneNumberKind> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().PredictNumberKind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_geographic_region_code(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetGeographicRegionCode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn check_number_match(&self, otherNumber: &PhoneNumberInfo) -> Result<PhoneNumberMatchResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().CheckNumberMatch)(self.get_abi() as *const _ as *mut _, otherNumber.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PhoneNumberInfo: IPhoneNumberInfo}
impl RtActivatable<IPhoneNumberInfoFactory> for PhoneNumberInfo {}
impl RtActivatable<IPhoneNumberInfoStatics> for PhoneNumberInfo {}
impl PhoneNumberInfo {
    #[inline] pub fn create(number: &HStringArg) -> Result<PhoneNumberInfo> {
        <Self as RtActivatable<IPhoneNumberInfoFactory>>::get_activation_factory().create(number)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<PhoneNumberInfo>, PhoneNumberParseResult)> {
        <Self as RtActivatable<IPhoneNumberInfoStatics>>::get_activation_factory().try_parse(input)
    }
    #[inline] pub fn try_parse_with_region(input: &HStringArg, regionCode: &HStringArg) -> Result<(Option<PhoneNumberInfo>, PhoneNumberParseResult)> {
        <Self as RtActivatable<IPhoneNumberInfoStatics>>::get_activation_factory().try_parse_with_region(input, regionCode)
    }
}
DEFINE_CLSID!(PhoneNumberInfo(&[87,105,110,100,111,119,115,46,71,108,111,98,97,108,105,122,97,116,105,111,110,46,80,104,111,110,101,78,117,109,98,101,114,70,111,114,109,97,116,116,105,110,103,46,80,104,111,110,101,78,117,109,98,101,114,73,110,102,111,0]) [CLSID_PhoneNumberInfo]);
DEFINE_IID!(IID_IPhoneNumberInfoFactory, 2181216612, 44458, 19711, 143, 207, 23, 231, 81, 106, 40, 255);
RT_INTERFACE!{static interface IPhoneNumberInfoFactory(IPhoneNumberInfoFactoryVtbl): IInspectable [IID_IPhoneNumberInfoFactory] {
    fn Create(&self, number: HSTRING, out: *mut <PhoneNumberInfo as RtType>::Abi) -> HRESULT
}}
impl IPhoneNumberInfoFactory {
    #[inline] pub fn create(&self, number: &HStringArg) -> Result<PhoneNumberInfo> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, number.get(), &mut out);
        if hr == S_OK { Ok(PhoneNumberInfo::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPhoneNumberInfoStatics, 1530875754, 34473, 16617, 134, 73, 109, 97, 22, 25, 40, 212);
RT_INTERFACE!{static interface IPhoneNumberInfoStatics(IPhoneNumberInfoStaticsVtbl): IInspectable [IID_IPhoneNumberInfoStatics] {
    fn TryParse(&self, input: HSTRING, phoneNumber: *mut <PhoneNumberInfo as RtType>::Abi, out: *mut PhoneNumberParseResult) -> HRESULT,
    fn TryParseWithRegion(&self, input: HSTRING, regionCode: HSTRING, phoneNumber: *mut <PhoneNumberInfo as RtType>::Abi, out: *mut PhoneNumberParseResult) -> HRESULT
}}
impl IPhoneNumberInfoStatics {
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<PhoneNumberInfo>, PhoneNumberParseResult)> { unsafe { 
        let mut phoneNumber = null_mut(); let mut out = zeroed();
        let hr = (self.get_vtbl().TryParse)(self.get_abi() as *const _ as *mut _, input.get(), &mut phoneNumber, &mut out);
        if hr == S_OK { Ok((PhoneNumberInfo::wrap(phoneNumber), out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_with_region(&self, input: &HStringArg, regionCode: &HStringArg) -> Result<(Option<PhoneNumberInfo>, PhoneNumberParseResult)> { unsafe { 
        let mut phoneNumber = null_mut(); let mut out = zeroed();
        let hr = (self.get_vtbl().TryParseWithRegion)(self.get_abi() as *const _ as *mut _, input.get(), regionCode.get(), &mut phoneNumber, &mut out);
        if hr == S_OK { Ok((PhoneNumberInfo::wrap(phoneNumber), out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum PhoneNumberMatchResult: i32 {
    NoMatch = 0, ShortNationalSignificantNumberMatch = 1, NationalSignificantNumberMatch = 2, ExactMatch = 3,
}}
RT_ENUM! { enum PhoneNumberParseResult: i32 {
    Valid = 0, NotANumber = 1, InvalidCountryCode = 2, TooShort = 3, TooLong = 4,
}}
RT_ENUM! { enum PredictedPhoneNumberKind: i32 {
    FixedLine = 0, Mobile = 1, FixedLineOrMobile = 2, TollFree = 3, PremiumRate = 4, SharedCost = 5, Voip = 6, PersonalNumber = 7, Pager = 8, UniversalAccountNumber = 9, Voicemail = 10, Unknown = 11,
}}
} // Windows.Globalization.PhoneNumberFormatting
