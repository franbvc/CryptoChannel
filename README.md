# CryptoChannel - CLI Tool for Secure Key Exchanges
## 1. Introduction

The CLI Tool for Secure Key Exchanges is designed to facilitate secure key exchanges for encrypted messaging through unsecure channels, such as messaging apps and email. Its purpose is to ensure the privacy and confidentiality of communication, particularly in light of potential government legislation that may compromise encryption.

### 1.1 Purpose and Motivation

The tool was developed in response to the global wave of governments proposing bills that aim to hold social media platforms responsible for user message exchanges involving sensitive or dangerous topics. Bills in countries like the US, UK, and Brazil raise concerns about the erosion of privacy and the potential for censorship. This tool aims to demonstrate the futility of such legislation and uphold the individual's right to privacy.

### 1.2 Compatibility

The CLI tool is currently tested and compatible with Linux and Windows systems (Powershell). Support for other operating systems may be considered in future updates.

### 1.3 Technologies Used

The tool utilizes the following technologies for secure key exchanges and message encryption:

- x25519 elliptic curve Diffie-Hellman (ECDH) for secure key exchange
- HKDF (HMAC-based Key Derivation Function) for deriving keys from the shared secret
- AES-GCM (Advanced Encryption Standard - Galois/Counter Mode) for encrypting and decrypting messages

The combination of these cryptographic algorithms ensures strong security and confidentiality of the exchanged keys and encrypted messages.

## Table of Contents

1. [Introduction](#1-introduction)
2. [How to Build from Source](#2-how-to-build-from-source)
3. [How to Use](#3-how-to-use)

Please refer to the respective sections for detailed instructions on installation, building, and usage of the CLI tool.

## 2. How to Build from Source

Before building the CLI tool from source, ensure that Rust and Cargo are installed on your system. If you don't have them installed, you can follow the instructions below:

### 2.1 Installing Rust and Cargo

To install Rust and Cargo, follow the official Rust installation guide:

[Install Rust](https://www.rust-lang.org/tools/install)

Once you have Rust and Cargo installed, you can proceed with building the CLI tool from source.

### 2.2 Building the CLI Tool

To build the CLI tool, perform the following steps:

1. Open a terminal or command prompt.

2. Clone the repository to your local machine using Git:

   ```shell
   git clone https://github.com/franbvc/CryptoChannel
   ```

3. Change into the project directory:

    ```shell
    cd CryptoChannel/crypto_channel
    ```


4. Build the project using Cargo:
    
    ```shell
    cargo build --release
    ```

5. Once the build process completes successfully, you can find the compiled
    binary in the target/release directory. You can run the CLI tool by
    executing the binary file:

    ```shell
    ./target/release/crypto_channel
    ```

## 3. How to Use

The CryptoChannel CLI tool provides a menu-based interface to perform various actions related to key exchanges and message encryption/decryption. Follow the steps below to use the tool effectively:

### 3.1 Configuration File

   The CLI tool uses a configuration file that is stored at `$HOME/.config/crypto-channel/config.json`. This file contains important settings and paths used by the tool. To modify the file in which the tool stores the keys, you can edit the `"storage_path"` field in the `config.json` file and set it to the desired path.

### 3.2 Running the Tool

   To use the CryptoChannel CLI tool, simply run the executable file. You will be presented with a menu of actions to choose from. Use the arrow keys to navigate through the menu and press Enter to select an action.

1. **Creating a New Key Exchange**

   To create a new Key Exchange, follow these steps:
   
   - Select the "Create New Key Exchange" option from the menu.
   - Enter a name for the Key Exchange.
   - The tool will generate a new key pair consisting of a private key and a corresponding public key and will save them in the storage file.
   - The public key of your Key Exchange can be obtained using the "Send Public Key" option in the menu.

2. **Key Exchange Concepts**

   Before using the tool, it's important to understand the concept of a Key Exchange. In cryptography, a Key Exchange is a process where two parties establish a shared secret key over an insecure channel. The CryptoChannel tool utilizes the Diffie-Hellman key exchange algorithm, specifically the x25519 elliptic curve variant, to securely generate shared keys.

3. **Completing a Key Exchange**

    To be able to encrypt and decrypt messages, you and the other party involved in the communication must complete a Key Exchange.
   To complete a Key Exchange, follow these steps:
   
   - Select the "Complete Key Exchange" option from the menu.
   - Enter the public key of the other party involved in the communication (they can get their public key using the "Send Public Key" option in the menu and then send it to you through an unsecure channel, such as email or messaging apps).
   - The tool will generate a shared key using your private key and the received public key. Using this shared key, an encryption key will be derived and both will be saved in the storage file.
   - Once the Key Exchange is completed, you can proceed to encrypt and decrypt messages using the generated shared key.
   - For the other party to be able to decrypt your messages, they must complete a Key Exchange with you as well, by you sending them your public key and them following the same steps as above.

4. **Encrypting Message**

   To encrypt messages using the CryptoChannel tool, follow these steps:
   
   - Select the "Encrypt Message" option from the menu.
   - Copy the text of the message you want to encrypt to the clipboard. You can do this in multiple ways:
     1. Select the text and use the keyboard shortcut for copying (e.g., Ctrl+C).
     2. Alternatively, right-click on the selected text and choose the "Copy" option from the context menu.
   - Confirm that the text is copied to the clipboard by entering "y" when prompted.
   - Choose the Key Exchange to use for encryption.
   - The tool will encrypt the message using the encryption key of the chosen Key Exchange and will put the encrypted message in the clipboard.
   - Paste the encrypted message from the clipboard to the desired location.

5. **Decrypting Message**

   To decrypt messages using the CryptoChannel tool, follow these steps:
   
   - Select the "Decrypt Message" option from the menu.
   - Copy the encrypted message to the clipboard.
   - Confirm that the encrypted message is copied to the clipboard by entering "y" when prompted.
   - Choose the Key Exchange to use for decryption.
   - The tool will decrypt the message using the encryption key of the chosen Key Exchange and will print the decrypted message to the terminal.

Make sure to familiarize yourself with the concepts and functionalities of the CryptoChannel tool to effectively utilize
