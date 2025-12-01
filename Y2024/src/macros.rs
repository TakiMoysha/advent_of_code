#[macro_export]
macro_rules! dataset_path {
    ($file:expr) => {
        concat!("Y2024/data/", $file)
    };
}

