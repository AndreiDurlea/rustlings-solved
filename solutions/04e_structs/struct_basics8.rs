struct Settings {
    volume: u8,
    brightness: u8,
    dark_mode: bool,
    language: &'static str,
}

const DEFAULT_SETTINGS: Settings = Settings {
    volume: 50,
    brightness: 70,
    dark_mode: false,
    language: "en",
};

fn main() {
    let my_settings = Settings {
        dark_mode: true,
        language: "ro",
        ..DEFAULT_SETTINGS
    };

    assert!(my_settings.dark_mode);
    assert_eq!(my_settings.language, "ro");
    assert_eq!(my_settings.volume, 50);
    assert_eq!(my_settings.brightness, 70);
}
