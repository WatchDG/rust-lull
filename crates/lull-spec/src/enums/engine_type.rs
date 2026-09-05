#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineType {
    Strategy,
    Execution,
    Risk,
    Accounting,
    Compliance,
}
