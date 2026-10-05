pub fn get_device_name() -> String {
    #[cfg(target_os = "android")]
    {
        "Android Tablet".to_string()
    }

    #[cfg(not(target_os = "android"))]
    {
        let name = match nick_name::NickName::new() {
            Ok(value) => value,
            Err(cause) => {
                tracing::error!(?cause, "failed to get device nickname name");
                return "Unknown".to_string();
            }
        };

        match name.get() {
            Ok(value) => value,
            Err(cause) => {
                tracing::error!(?cause, "failed to get device name");
                "Unknown".to_string()
            }
        }
    }
}
