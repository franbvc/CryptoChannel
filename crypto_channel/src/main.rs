use modules::prompt::{
    complete_key_exchange, create_new_key_exchange, decrypt_prompt, delete_key_exchange,
    encrypt_prompt, select_menu_action, show_public_key,
};

use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

fn main() {
    inquire::set_global_render_config(get_render_config());

    loop {
        let action = select_menu_action().unwrap();

        match action.as_str() {
            "Create New Key Exchange" => create_new_key_exchange(),
            "Complete Key Exchange" => complete_key_exchange(),
            "Delete Key Exchange" => delete_key_exchange(),
            "Send Public Key" => show_public_key(),
            "Encrypt Message" => encrypt_prompt(),
            "Decrypt Message" => decrypt_prompt(),
            _ => return,
        }
    }
}

fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();
    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightGreen);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkMagenta);

    render_config
}
