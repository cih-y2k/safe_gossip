// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use crate::error::Error;
use bincode::{deserialize, serialize};
use ed25519_dalek::{Keypair, PublicKey, Signature};
#[cfg(not(test))]
use sha3::Sha3_512;

/// Messages sent via a direct connection, wrapper of gossip protocol rpcs.
#[derive(Serialize, Debug, Deserialize)]
pub struct Message(pub Vec<u8>, pub Signature);

#[cfg(not(test))]
impl Message {
    pub fn serialise(rpc: &GossipRpc, keys: &Keypair) -> Result<Vec<u8>, Error> {
        let serialised_msg = serialize(rpc)?;
        let sig: Signature = keys.sign::<Sha3_512>(&serialised_msg);
        Ok(serialize(&Message(serialised_msg, sig))?)
    }

    pub fn deserialise(serialised_msg: &[u8], key: &PublicKey) -> Result<GossipRpc, Error> {
        let msg: Message = deserialize(serialised_msg)?;
        if key.verify::<Sha3_512>(&msg.0, &msg.1).is_ok() {
            Ok(deserialize(&msg.0)?)
        } else {
            Err(Error::SigFailure)
        }
    }
}

#[cfg(test)]
impl Message {
    pub fn serialise(rpc: &GossipRpc, _keys: &Keypair) -> Result<Vec<u8>, Error> {
        Ok(serialize(rpc)?)
    }

    pub fn deserialise(serialised_msg: &[u8], _key: &PublicKey) -> Result<GossipRpc, Error> {
        Ok(deserialize(serialised_msg)?)
    }
}

/// Gossip rpcs
#[derive(Debug, Serialize, Deserialize)]
pub enum GossipRpc {
    /// Sent from Node A to Node B to push a message and its counter.
    Push { msg: Vec<u8>, counter: u8 },
    /// Sent from Node B to Node A as a reaction to receiving a push message from A.
    Pull { msg: Vec<u8>, counter: u8 },
}
