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
    // TODO: Create `my_settings` from `DEFAULT_SETTINGS`, but with dark mode
    // turned on and the language "ro". Use the struct update syntax so that
    // you don't need to write the other fields.
    let my_settings = ???;

    assert!(my_settings.dark_mode);
    assert_eq!(my_settings.language, "ro");
    assert_eq!(my_settings.volume, 50);
    assert_eq!(my_settings.brightness, 70);
}
