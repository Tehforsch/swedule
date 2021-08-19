use ordered_float::OrderedFloat;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct ProcessorPriority {
    pub time: OrderedFloat<f64>,
}
