use modules::prompt::{
    complete_key_exchange, create_new_key_exchange, decrypt_prompt, delete_key_exchange,
    encrypt_prompt, select_menu_action, show_public_key,
};

fn main() {
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
