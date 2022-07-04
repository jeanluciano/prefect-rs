struct Setting {
    setting_type: String,
    value_callback: fn(),
}

trait Value {
    fn value(self);
    fn value_from(self, settings: Setting);
}