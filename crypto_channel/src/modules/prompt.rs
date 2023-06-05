use crate::encryption::{decrypt_message, encrypt_message, encrypted_to_str, str_to_encrypted};
use crate::key_exchange::{gen_encryption_key, gen_shared_secret, gen_static_kp, string_to_key};
use crate::key_store::{
    get_exchange_dh_public, get_exchange_dh_secret, get_exchange_encryption_key,
    get_key_exchange_names, read_storage, validate_new_exchange_name, write_storage,
};
use crate::key_store_classes::{KeyExchange, KeySignature};

use arboard::Clipboard;
use inquire::{Confirm, Select, Text};

pub fn prompt_exchange_name() -> Result<String, &'static str> {
    let name = Text::new("Please enter a name for the key pair:")
        .with_help_message(
            "This should be a meaningful identifier for the \
             person you will be exchanging keys with. For \
             instance, you can use their full name, email \
             address, or any other unique identifier.",
        )
        .with_placeholder("Bob")
        .prompt();

    match name {
        Ok(name) => Ok(name),
        Err(_) => Err("An error happened when asking for the new exchange name."),
    }
}

pub fn select_exchange() -> Result<String, &'static str> {
    let options = get_key_exchange_names();
    let ans = Select::new(
        "Please select a key exchange from the following options:",
        options,
    )
    .prompt();

    match ans {
        Ok(choice) => Ok(choice),
        Err(_) => Err("There was an error, please try again"),
    }
}

pub fn select_menu_action() -> Result<String, &'static str> {
    let options = vec![
        "Create New Key Exchange",
        "Complete Key Exchange",
        "Delete Key Exchange",
        "Send Public Key",
        "Encrypt Message",
        "Decrypt Message",
        "Exit",
    ];
    let ans = Select::new(
        "Please select an action to perform on the selected key exchange:",
        options,
    )
        .with_help_message("To encrypt and decrypt messages, you need to complete a Key Exchange. \
                            Here's how it works: \n\n \
                            1. Select the 'Complete Key Exchange' option below. \n \
                            2. Input the Public Key of the other party you wish to communicate \
                            with. This is necessary for establishing a secure channel. \n \
                            3. Inform the other party to complete the Key Exchange by sending them \
                            your Public Key. \n \n \
                            Please note: \n \
                            - Both parties must complete the Key Exchange for secure communication. \n \
                            - The Public Key is used to generate a shared key, ensuring \
                            confidentiality and integrity of the messages exchanged. \n \n \
                            Once the Key Exchange is completed successfully, you'll be able to \
                            encrypt and decrypt messages securely.")
    .prompt();

    match ans {
        Ok(choice) => Ok(choice.to_string()),
        Err(_) => Err("There was an error, please try again"),
    }
}

pub fn other_party_public_key_prompt() -> Result<String, &'static str> {
    let name =
        Text::new("Please enter the Public Key of the other party you wish to communicate with:")
            .with_help_message(
                "The Public Key of the other party is a cryptographic key used \
             to complete the Key Exchange process. It is provided by the other \
             party involved in the communication. \n \n \
             To obtain the Public Key: \n \
             1. Request the other party to generate a Public Key using the menu \
             option `Create New Key Exchange`. \n \
             2. The Public Key is represented as a series of alphanumeric \
             characters, in base64 encoding. \n \n \
             Remember to: \n \
             - Enter the Public Key accurately, without typos or omissions. \n \
             - Even a small error can prevent successful key generation and communication. \n \
             - Copy the Public Key correctly and ensure its integrity. \n \n \
             Please enter the Public Key of the other party you wish to communicate with:",
            )
            .with_placeholder("C02CYYKtZJ6qvUtux2YKw5jM/+PHs3q2iOcHN3K/EXU=")
            .prompt();

    match name {
        Ok(name) => Ok(name),
        Err(_) => Err("An error happened."),
    }
}

pub fn show_public_key() {
    let exchange = select_exchange();

    match exchange {
        Ok(exchange_name) => {
            let public_str = get_exchange_dh_public(exchange_name.clone());
            println!(
                "Your Public Key for the exchange `{}` is: {}",
                exchange_name, public_str
            );
        }
        Err(_) => println!("Failed to get exchange, it might have been deleted"),
    }
}

pub fn create_new_key_exchange() {
    let exchange_name;
    match prompt_exchange_name() {
        Ok(name) => exchange_name = name,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let is_name_valid = validate_new_exchange_name(exchange_name.clone());

    match is_name_valid {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let kp = gen_static_kp();
    let mut new_key_exchange = KeyExchange::new();
    new_key_exchange.add_your_dh_kp(kp);

    let mut curr_storage = read_storage();
    let new_result =
        curr_storage.create_exchange(&exchange_name[..], false, Some(new_key_exchange), None);

    match new_result {
        Ok(_) => println!("New Key Exchange created successfully!"),
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let write_result = write_storage(curr_storage);

    match write_result {
        Ok(_) => {
            println!(
                "Your key pair has been generated and stored \
                 securely!. You can now proceed to complete the \
                 Key Exchange by providing the Public Key of \
                 the other party. Once both parties have \
                 completed the Key Exchange, a shared secret \
                 key will be generated for secure communication. \
                 Thank you for using our application."
            );
        }
        Err(e) => println!("{}", e),
    }
}

pub fn complete_key_exchange() {
    let exchange_name;
    match select_exchange() {
        Ok(name) => exchange_name = name,
        Err(_) => {
            println!("Failed to get exchange, it might have been deleted");
            return;
        }
    }

    let dh_public;
    match other_party_public_key_prompt() {
        Ok(pub_k) => dh_public = string_to_key(pub_k),
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let dh_secret = get_exchange_dh_secret(exchange_name.clone());
    let shared_secret = gen_shared_secret(dh_public, dh_secret);
    let encryption_key = gen_encryption_key(shared_secret);

    let mut curr_storage = read_storage();
    let pair = curr_storage.exchange_map.get_mut(&exchange_name);
    let curr_exchange = &mut pair.unwrap().0;

    curr_exchange.add_other_person_public_key(dh_public);
    curr_exchange.add_shared_secret(shared_secret, false);
    curr_exchange.add_encryption_key(encryption_key, false);

    println!("{:?}", curr_storage.exchange_map);

    match write_storage(curr_storage) {
        Ok(_) => println!(
            "The Key Exchange `{}` has been completed \
             and stored securely! You can now proceed to \
             encrypt and decrypt messages using this \
             exchange.",
            exchange_name
        ),
        Err(e) => println!("{}", e),
    }
}

pub fn encrypt_prompt() {
    let ans = Confirm::new("Encrypt text from clipboard?")
        .with_default(false)
        .with_help_message(
            "The clipboard contains text that you have \
                copied. Encrypting the text will ensure its \
                confidentiality and security. If you choose \
                to encrypt the text, the program will access \
                the data from the clipboard, perform encryption, \
                and replace the contents in the clipboard with \
                the encrypted version. This allows you to securely \
                share the encrypted message with others by simply \
                pasting it. To proceed, select 'yes' and the \
                program will handle the encryption process for you.",
        )
        .prompt();

    match ans {
        Ok(true) => (),
        Ok(false) => {
            println!("The text copied on the clipboard will remain unchanged");
            return;
        }
        Err(_) => {
            println!("Error with questionnaire, try again later");
            return;
        }
    }

    let exchange_name;
    match select_exchange() {
        Ok(name) => exchange_name = name,
        Err(_) => {
            println!("Failed to get exchange, it might have been deleted");
            return;
        }
    }

    let encryption_key = get_exchange_encryption_key(exchange_name.clone());
    if encryption_key == [0u8; 32] {
        println!(
            "Error: Key Exchange was not completed for `{}`. \
             Unable to encrypt message.",
            exchange_name
        );
        return;
    }

    let mut clipboard = Clipboard::new().unwrap();
    let text;

    match clipboard.get_text() {
        Ok(txt) => {
            text = txt;
            println!("Clipboard text was: {}", text);
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let (encrypted_msg, nonce) = encrypt_message(text.as_bytes(), &encryption_key);
    let encoded_msg = encrypted_to_str(encrypted_msg, nonce);

    match clipboard.set_text(encoded_msg.clone()) {
        Ok(_) => println!(
            "The clipboard contents were encrypted successfully. \
             They should now be : \n{}. \n \n \
             The encrypted text is now available on the clipboard, \
             allowing you to securely share it with others by simply \
             pasting it. You can proceed to use the encrypted text \
             as needed, such as sending it via messaging apps or email.",
            encoded_msg
        ),
        Err(e) => println!("{}", e),
    }
}

pub fn decrypt_prompt() {
    let ans = Confirm::new("Decrypt text from clipboard?")
        .with_default(false)
        .with_help_message(
            "To decrypt an encrypted message, copy the \
            encrypted text to your clipboard. Then, proceed \
            by selecting 'yes'. The program will access the \
            encrypted text from your clipboard, perform the \
            decryption process, and display the decrypted \
            message. This allows you to securely read the \
            original content of the encrypted message. Make \
            sure you have the correct decryption key or \
            access to the necessary decryption method. Keep \
            the decrypted message confidential and avoid \
            sharing it with unauthorized individuals.",
        )
        .prompt();

    match ans {
        Ok(true) => (),
        Ok(false) => {
            println!("The text copied on the clipboard will remain unchanged");
            return;
        }
        Err(_) => {
            println!("Error with questionnaire, try again later");
            return;
        }
    }

    let exchange_name;
    match select_exchange() {
        Ok(name) => exchange_name = name,
        Err(_) => {
            println!("Failed to get exchange, it might have been deleted");
            return;
        }
    }

    let encryption_key = get_exchange_encryption_key(exchange_name.clone());
    if encryption_key == [0u8; 32] {
        println!(
            "Error: Key Exchange was not completed for `{}`. \
             Unable to decrypt message.",
            exchange_name
        );
        return;
    }

    let mut clipboard = Clipboard::new().unwrap();
    let text;

    match clipboard.get_text() {
        Ok(txt) => {
            text = txt;
            println!("Clipboard text was: {}", text);
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let (decoded_msg, decoded_nonce) = str_to_encrypted(text);
    let decrypted_msg = decrypt_message(decoded_msg, &decoded_nonce, &encryption_key);

    match std::str::from_utf8(&decrypted_msg) {
        Ok(msg) => {
            println!("The encrypted message was: \n {}", msg);
        }
        Err(e) => println!(
            "Error converting the decrypted msg to UTF-8. \
             This might have been caused by wrong encryption \
             keys. The error was: {}",
            e
        ),
    }
}

pub fn delete_key_exchange() {
    let exchange_name;
    match select_exchange() {
        Ok(name) => exchange_name = name,
        Err(_) => {
            println!("Failed to get exchange, it might have been deleted");
            return;
        }
    }

    let question = "Are you sure you want to delete the Key Exchange `";
    let question_end = "` ?`";
    let full_question = question.to_owned() + &exchange_name + question_end;
    let ans = Confirm::new(&full_question)
        .with_default(false)
        .with_help_message(
            "If you select 'yes' the Key Exchange \
             will be permanently delted from you \
             key storage.",
        )
        .prompt();

    match ans {
        Ok(true) => (),
        Ok(false) => {
            println!("Key Exchange deletion aborted");
            return;
        }
        Err(_) => {
            println!("Error with questionnaire, operation aborted");
            return;
        }
    }

    let mut curr_storage = read_storage();
    curr_storage.exchange_map.remove(&exchange_name);
    match write_storage(curr_storage) {
        Ok(_) => println!("Key Exchange `{}` was deleted successfully!", exchange_name),
        Err(e) => println!("Error: {}", e),
    }
}
