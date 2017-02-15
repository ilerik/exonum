use rand::{Rng, XorShiftRng, SeedableRng};

use exonum::messages::{FromRaw, Message, RawTransaction, Error as MessageError};
use exonum::crypto::{PublicKey, SecretKey, gen_keypair};
use exonum::storage::{Error, View as StorageView};
use exonum::blockchain::{Service, Transaction};

pub const TIMESTAMPING_SERVICE: u16 = 129;
pub const TIMESTAMPING_TRANSACTION_MESSAGE_ID: u16 = 128;

message! {
    TimestampTx {
        const TYPE = TIMESTAMPING_SERVICE;
        const ID = TIMESTAMPING_TRANSACTION_MESSAGE_ID;
        const SIZE = 40;

        pub_key:        &PublicKey  [00 => 32]
        data:           &[u8]       [32 => 40]
    }
}

pub struct TimestampingService {}

pub struct TimestampingTxGenerator {
    rand: XorShiftRng,
    data_size: usize,
    public_key: PublicKey,
    secret_key: SecretKey,
}

impl TimestampingTxGenerator {
    pub fn new(data_size: usize) -> TimestampingTxGenerator {
        let rand = XorShiftRng::from_seed([192, 168, 56, 1]);
        let (public_key, secret_key) = gen_keypair();

        TimestampingTxGenerator {
            rand: rand,
            data_size: data_size,
            public_key: public_key,
            secret_key: secret_key,
        }
    }
}

impl Iterator for TimestampingTxGenerator {
    type Item = TimestampTx;

    fn next(&mut self) -> Option<TimestampTx> {
        let mut data = vec![0; self.data_size];
        self.rand.fill_bytes(&mut data);
        Some(TimestampTx::new(&self.public_key, &data, &self.secret_key))
    }
}

impl TimestampingService {
    pub fn new() -> TimestampingService {
        TimestampingService {}
    }
}

impl Transaction for TimestampTx {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, _: &StorageView) -> Result<(), Error> {
        Ok(())
    }
}

impl Service for TimestampingService {
    fn service_id(&self) -> u16 {
        TIMESTAMPING_SERVICE
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, MessageError> {
        if raw.message_type() != TIMESTAMPING_TRANSACTION_MESSAGE_ID {
            return Err(MessageError::IncorrectMessageType { message_type: raw.message_type() });
        }

        TimestampTx::from_raw(raw).map(|tx| Box::new(tx) as Box<Transaction>)
    }
}