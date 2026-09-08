mod candlestick_datetime;
mod candlestick_id;
mod candlestick_instrument;
mod candlestick_interval;
mod candlestick_price;
mod candlestick_volume;

use std::collections::HashSet;

use lull_spec::enums::{CurrencyRef, InstrumentRef};
use lull_spec::types::{
    Candlestick, CandlestickDateTime, CandlestickId, CandlestickInstrument, CandlestickInterval,
    CandlestickPrice, CandlestickVolume, CurrencyCode, DateTime, InstrumentId, Money,
    MoneyCurrency, MoneyValue, Quantity,
};

type TestPrice = Money<i64, CurrencyRef<String, [u8; 3]>>;
type TestVolume = Quantity<i64>;
type TestCandlestick =
    Candlestick<String, InstrumentRef<String>, i32, DateTime<i64>, TestPrice, TestVolume>;

fn usd(value: i64) -> CandlestickPrice<TestPrice> {
    CandlestickPrice::new(Money::new(
        MoneyValue::new(value),
        MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"USD"))),
    ))
}

fn candlestick_volume(inner: i64) -> CandlestickVolume<TestVolume> {
    CandlestickVolume::new(Quantity::new(inner))
}

fn candlestick(id: &str, instrument_id: &str, interval: i32, datetime: i64) -> TestCandlestick {
    candlestick_ohlc(id, instrument_id, interval, datetime, 100, 110, 90, 105)
}

fn candlestick_ohlc(
    id: &str,
    instrument_id: &str,
    interval: i32,
    datetime: i64,
    open: i64,
    high: i64,
    low: i64,
    close: i64,
) -> TestCandlestick {
    candlestick_full(
        id,
        instrument_id,
        interval,
        datetime,
        open,
        high,
        low,
        Some(close),
        None,
    )
}

fn candlestick_full(
    id: &str,
    instrument_id: &str,
    interval: i32,
    datetime: i64,
    open: i64,
    high: i64,
    low: i64,
    close: Option<i64>,
    volume: Option<i64>,
) -> TestCandlestick {
    Candlestick::new(
        CandlestickId::new(String::from(id)),
        CandlestickInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from(
            instrument_id,
        )))),
        CandlestickInterval::new(interval),
        CandlestickDateTime::new(DateTime::new(datetime)),
        usd(open),
        usd(high),
        usd(low),
        close.map(usd),
        volume.map(candlestick_volume),
    )
}

#[test]
fn equal_fields_are_equal_candlesticks() {
    assert_eq!(
        candlestick("cs-1", "inst-1", 300, 1_700_000_000),
        candlestick("cs-1", "inst-1", 300, 1_700_000_000)
    );
}

#[test]
fn distinct_ids_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick("cs-1", "inst-1", 300, 1_700_000_000),
        candlestick("cs-2", "inst-1", 300, 1_700_000_000)
    );
}

#[test]
fn distinct_instruments_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick("cs-1", "inst-1", 300, 1_700_000_000),
        candlestick("cs-1", "inst-2", 300, 1_700_000_000)
    );
}

#[test]
fn distinct_intervals_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick("cs-1", "inst-1", 300, 1_700_000_000),
        candlestick("cs-1", "inst-1", 60, 1_700_000_000)
    );
}

#[test]
fn distinct_datetimes_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick("cs-1", "inst-1", 300, 1_700_000_000),
        candlestick("cs-1", "inst-1", 300, 1_700_000_300)
    );
}

#[test]
fn distinct_open_prices_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick_ohlc("cs-1", "inst-1", 300, 1_700_000_000, 100, 110, 90, 105),
        candlestick_ohlc("cs-1", "inst-1", 300, 1_700_000_000, 101, 110, 90, 105)
    );
}

#[test]
fn distinct_high_prices_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick_ohlc("cs-1", "inst-1", 300, 1_700_000_000, 100, 110, 90, 105),
        candlestick_ohlc("cs-1", "inst-1", 300, 1_700_000_000, 100, 111, 90, 105)
    );
}

#[test]
fn distinct_low_prices_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick_ohlc("cs-1", "inst-1", 300, 1_700_000_000, 100, 110, 90, 105),
        candlestick_ohlc("cs-1", "inst-1", 300, 1_700_000_000, 100, 110, 91, 105)
    );
}

#[test]
fn distinct_close_prices_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick_ohlc("cs-1", "inst-1", 300, 1_700_000_000, 100, 110, 90, 105),
        candlestick_ohlc("cs-1", "inst-1", 300, 1_700_000_000, 100, 110, 90, 106)
    );
}

#[test]
fn present_close_price_is_not_absent_close_price() {
    assert_ne!(
        candlestick("cs-1", "inst-1", 300, 1_700_000_000),
        candlestick_full(
            "cs-1",
            "inst-1",
            300,
            1_700_000_000,
            100,
            110,
            90,
            None,
            None
        )
    );
}

#[test]
fn present_volume_is_not_absent_volume() {
    assert_ne!(
        candlestick_full(
            "cs-1",
            "inst-1",
            300,
            1_700_000_000,
            100,
            110,
            90,
            Some(105),
            Some(1_000)
        ),
        candlestick("cs-1", "inst-1", 300, 1_700_000_000)
    );
}

#[test]
fn distinct_volumes_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick_full(
            "cs-1",
            "inst-1",
            300,
            1_700_000_000,
            100,
            110,
            90,
            Some(105),
            Some(1_000)
        ),
        candlestick_full(
            "cs-1",
            "inst-1",
            300,
            1_700_000_000,
            100,
            110,
            90,
            Some(105),
            Some(2_000)
        )
    );
}

#[test]
fn clone_preserves_equality() {
    let candlestick = candlestick("cs-1", "inst-1", 300, 1_700_000_000);
    assert_eq!(candlestick.clone(), candlestick);
}

#[test]
fn equal_candlesticks_hash_to_the_same_bucket() {
    let mut candlesticks = HashSet::new();
    candlesticks.insert(candlestick("cs-1", "inst-1", 300, 1_700_000_000));
    candlesticks.insert(candlestick("cs-1", "inst-1", 300, 1_700_000_000));
    candlesticks.insert(candlestick("cs-2", "inst-1", 300, 1_700_000_000));
    candlesticks.insert(candlestick("cs-1", "inst-2", 300, 1_700_000_000));
    candlesticks.insert(candlestick("cs-1", "inst-1", 60, 1_700_000_000));
    candlesticks.insert(candlestick("cs-1", "inst-1", 300, 1_700_000_300));
    candlesticks.insert(candlestick_ohlc(
        "cs-1",
        "inst-1",
        300,
        1_700_000_000,
        101,
        110,
        90,
        105,
    ));
    candlesticks.insert(candlestick_ohlc(
        "cs-1",
        "inst-1",
        300,
        1_700_000_000,
        100,
        111,
        90,
        105,
    ));
    candlesticks.insert(candlestick_ohlc(
        "cs-1",
        "inst-1",
        300,
        1_700_000_000,
        100,
        110,
        91,
        105,
    ));
    candlesticks.insert(candlestick_ohlc(
        "cs-1",
        "inst-1",
        300,
        1_700_000_000,
        100,
        110,
        90,
        106,
    ));
    candlesticks.insert(candlestick_full(
        "cs-1",
        "inst-1",
        300,
        1_700_000_000,
        100,
        110,
        90,
        None,
        None,
    ));
    candlesticks.insert(candlestick_full(
        "cs-1",
        "inst-1",
        300,
        1_700_000_000,
        100,
        110,
        90,
        Some(105),
        Some(1_000),
    ));
    candlesticks.insert(candlestick_full(
        "cs-1",
        "inst-1",
        300,
        1_700_000_000,
        100,
        110,
        90,
        Some(105),
        Some(2_000),
    ));
    assert_eq!(candlesticks.len(), 12);
}
