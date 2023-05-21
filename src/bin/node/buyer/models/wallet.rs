use rsa::{RsaPublicKey, RsaPrivateKey, pkcs8};
use rand::{rngs::OsRng};
use rsa::pkcs8::LineEnding::CR;
use uuid::Uuid;
use rsa::pkcs1v15::{SigningKey, VerifyingKey};
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::sha2::{Digest, Sha256};
use std::marker::PhantomData;


// Wallet created per email provided
#[derive(Debug)]
pub struct Wallet
{
    // wallet local id
    pub index: Uuid,
    // wallet email owner
    pub email: String,
    // wallet private and public keys
    pub keypair: (RsaPublicKey, RsaPrivateKey),
    // total balance from wallet
    pub balance: f64,

}

impl Wallet
{
    // Create a new wallet
    pub fn new (email: String) -> Self {
        // Current wallet to be created.
        let wallet = Wallet {
            index: Uuid::new_v4(),
            email,
            keypair: Self::generate_keypair(),
            balance: 1000.0,
        };

        wallet
    }

    pub fn generate_keypair() -> (RsaPublicKey, RsaPrivateKey){
        // Generate a new 2048-bit RSA key pair
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a private key");
        // Extract the public key from the private key
        let public_key = private_key.to_public_key();

        (public_key, private_key)
    }
    // Print the key pair in PEM format
    pub fn debug_print_keys(keypair: (RsaPublicKey, RsaPrivateKey)){
        let public_key_pem = rsa::pkcs8::EncodePublicKey::to_public_key_pem(&keypair.0,CR);
        let private_key_pem = rsa::pkcs8::EncodePrivateKey::to_pkcs8_pem(&keypair.1,CR);
        println!("Private Key:\n{:#?}", private_key_pem);
        println!("Public Key:\n{:#?}", public_key_pem);
    }
    //-> SigningKey, VerifyingKey
    pub fn sign_data(data: String, priv_key: RsaPrivateKey) -> String{
        //derive the signing and verifying keys
        let signing_key = SigningKey::<Sha256>::new(priv_key);
        let verifying_key = signing_key.verifying_key();

        //sign the actual data
        let mut rng = OsRng;
        let mut data_to_sign = data.as_bytes();
        let signature = signing_key.sign_with_rng(&mut rng, data_to_sign);
        println!("{:?}", signature);

        //test verify
        verifying_key.verify(data_to_sign, &signature).expect("failed to verify");
        return signature.to_string();
    }
}