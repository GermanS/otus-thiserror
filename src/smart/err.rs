use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmartHomeError {
    #[error("Ошибка: комната `{0}` уже существует")]
    RoomAlreadyExists(String),

    #[error("Ошибка: устройство `{0}` уже подключено")]
    DeviceAlreadyPlugged(String),

    #[error("Ошибка: устройство `{0}` не найдено")]
    DeviceNotFound(String),

    #[error("Ошибка: устройства отсутвуют")]
    NoConnectedDevices,
}
