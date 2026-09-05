use std::collections::HashSet;

use lull_spec::enums::{CurrencyRef, InstrumentRef, OrderSide, OrderSizeRef, OrderTypeRef};
use lull_spec::types::{
    CurrencyCode, InstrumentId, Money, MoneyCurrency, MoneyValue, OrderId, OrderInstrument,
    OrderSize, OrderType, OrderTypeLimit, OrderTypeMarket, PlaceOrder, Quantity,
};

type TestCurrency = CurrencyRef<String, [u8; 3]>;
type TestOrderTypeLimit = OrderTypeLimit<i64, TestCurrency>;
type TestOrderTypeRef = OrderTypeRef<i64, TestCurrency>;
type TestPlaceOrder<OT> = PlaceOrder<String, OT, InstrumentRef<String>, OrderSizeRef<i64, i64>>;

fn usd_limit(value: i64) -> TestOrderTypeLimit {
    OrderTypeLimit::new(Money::new(
        MoneyValue::new(value),
        MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"USD"))),
    ))
}

fn usd_limit_ref(value: i64) -> TestOrderTypeRef {
    OrderTypeRef::Limit(usd_limit(value))
}

fn place_order<OT>(
    id: Option<&str>,
    side: OrderSide,
    order_type: OT,
    instrument_id: &str,
    qty: i64,
) -> TestPlaceOrder<OT> {
    PlaceOrder::new(
        id.map(|id| OrderId::new(String::from(id))),
        side,
        OrderType::new(order_type),
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from(
            instrument_id,
        )))),
        OrderSize::new(OrderSizeRef::Quantity(Quantity::new(qty))),
    )
}

#[test]
fn order_type_accepts_market() {
    let order = place_order(None, OrderSide::Buy, OrderTypeMarket::new(), "inst-1", 100);
    assert_eq!(order.r#type, OrderType::new(OrderTypeMarket::new()));
}

#[test]
fn order_type_accepts_limit() {
    let order = place_order(None, OrderSide::Buy, usd_limit(100), "inst-1", 100);
    assert_eq!(order.r#type, OrderType::new(usd_limit(100)));
}

#[test]
fn order_type_accepts_ref() {
    let order = place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100);
    assert_eq!(order.r#type, OrderType::new(usd_limit_ref(100)));
}

#[test]
fn id_may_be_absent() {
    let order = place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100);
    assert_eq!(order.id, None);
}

#[test]
fn id_may_be_present() {
    let order = place_order(
        Some("ord-1"),
        OrderSide::Buy,
        usd_limit_ref(100),
        "inst-1",
        100,
    );
    assert_eq!(order.id, Some(OrderId::new(String::from("ord-1"))));
}

#[test]
fn present_id_is_not_absent_id() {
    assert_ne!(
        place_order(
            Some("ord-1"),
            OrderSide::Buy,
            usd_limit_ref(100),
            "inst-1",
            100
        ),
        place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100)
    );
}

#[test]
fn distinct_ids_are_not_equal_place_orders() {
    assert_ne!(
        place_order(
            Some("ord-1"),
            OrderSide::Buy,
            usd_limit_ref(100),
            "inst-1",
            100
        ),
        place_order(
            Some("ord-2"),
            OrderSide::Buy,
            usd_limit_ref(100),
            "inst-1",
            100
        )
    );
}

#[test]
fn equal_fields_are_equal_place_orders() {
    assert_eq!(
        place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100),
        place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100)
    );
}

#[test]
fn distinct_sides_are_not_equal_place_orders() {
    assert_ne!(
        place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100),
        place_order(None, OrderSide::Sell, usd_limit_ref(100), "inst-1", 100)
    );
}

#[test]
fn distinct_types_are_not_equal_place_orders() {
    assert_ne!(
        place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100),
        place_order(None, OrderSide::Buy, TestOrderTypeRef::Market, "inst-1", 100)
    );
}

#[test]
fn distinct_instruments_are_not_equal_place_orders() {
    assert_ne!(
        place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100),
        place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-2", 100)
    );
}

#[test]
fn distinct_sizes_are_not_equal_place_orders() {
    assert_ne!(
        place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100),
        place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 101)
    );
}

#[test]
fn clone_preserves_equality() {
    let order = place_order(None, OrderSide::Buy, usd_limit_ref(100), "inst-1", 100);
    assert_eq!(order.clone(), order);
}

#[test]
fn equal_place_orders_hash_to_the_same_bucket() {
    let mut orders = HashSet::new();
    orders.insert(place_order(
        None,
        OrderSide::Buy,
        usd_limit_ref(100),
        "inst-1",
        100,
    ));
    orders.insert(place_order(
        None,
        OrderSide::Buy,
        usd_limit_ref(100),
        "inst-1",
        100,
    ));
    orders.insert(place_order(
        Some("ord-1"),
        OrderSide::Buy,
        usd_limit_ref(100),
        "inst-1",
        100,
    ));
    orders.insert(place_order(
        None,
        OrderSide::Sell,
        usd_limit_ref(100),
        "inst-1",
        100,
    ));
    orders.insert(place_order(
        None,
        OrderSide::Buy,
        TestOrderTypeRef::Market,
        "inst-1",
        100,
    ));
    orders.insert(place_order(
        None,
        OrderSide::Buy,
        usd_limit_ref(100),
        "inst-2",
        100,
    ));
    orders.insert(place_order(
        None,
        OrderSide::Buy,
        usd_limit_ref(100),
        "inst-1",
        101,
    ));
    assert_eq!(orders.len(), 6);
}
