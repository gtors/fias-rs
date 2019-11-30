
/// Классификатор адресообразующих элементов (край > область > город > район > улица) 
pub struct AddressObject {
    /// Официальное наименование
    oficial_name: String,
    /// Формальное наименование
    formal_name: String,
    /// Сокращение
    short_name: String,
}
