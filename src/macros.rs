#[macro_export]
macro_rules! parse_dec {
    ($d: expr) => {
        f64::try_from($d).expect("Error al formatear decimal")
    };
}
