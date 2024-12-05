//! This file would be responsible for all the blockchain related operations in the gasless-relayer server.
use std::str::FromStr;

use crate::configs::ChainsConfig;
use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::{aliases::U48, Address, Bytes, FixedBytes, U256},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        PendingTransactionBuilder, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
    sol,
    transports::http::{Client, Http},
};
use TrustedForwarderContract::TrustedForwarderContractInstance;

/// This struct would be responsible for processing all the requests to be sent to the blockchain.
/// struct would also be resonsible for waiting for the transaction to be mined
pub struct Processor {
    /// The chain to which the requests would be sent
    pub chains_config: ChainsConfig,
    /// This is the address of the trusted forwarder contract
    pub trusted_forwarder: Address,
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TrustedForwarderContract,
    "src/contract-artifacts/TrustedForwarder.json"
);

pub struct ForwardRequestData {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub gas: U256,
    pub deadline: U48,
    pub data: Bytes,
    pub signature: Bytes,
}

impl Processor {
    pub fn new(chains_config: ChainsConfig, trusted_forwarder: Address) -> Self {
        Self {
            chains_config,
            trusted_forwarder,
        }
    }

    /// This is function would be responsible for processing a single request
    /// this would just send the request to the chain and wait for it to be mined
    /// after this has been sent, this tx_hash would be used to wait for the transaction to be mined
    /// on the monitoring thread
    ///
    /// # Arguments
    /// - `self`
    /// - `request` - The request to be processed
    /// - `chain` - The chain to which the request would be sent
    pub async fn process_request(
        &self,
        request: ForwardRequestData,
    ) -> PendingTransactionBuilder<Http<Client>, Ethereum> {
        let trusted_forwarder_contract = self.get_trusted_forwarder();
        let req = trusted_forwarder_contract.execute(request.into());
        req.send().await.unwrap()
    }

    /// This is function would be responsible for processing a batch of requests
    /// this would just send the requests to the chain and wait for it to be mined
    /// after this has been sent, this tx_hash would be used to wait for the transaction to be mined
    /// on the monitoring thread
    ///
    /// # Arguments
    /// - `self`
    /// - `requests` - The requests to be processed
    /// - `chain` - The chain to which the requests would be sent
    pub async fn process_batch_request(
        &self,
        request: Vec<ForwardRequestData>,
        refund_receiver: String,
    ) -> PendingTransactionBuilder<Http<Client>, Ethereum> {
        let trusted_forwarder_contract = self.get_trusted_forwarder();
        let request = request
            .iter()
            .map(|r| <&ForwardRequestData as Into<ERC2771Forwarder::ForwardRequestData>>::into(r))
            .collect();
        let req = trusted_forwarder_contract
            .executeBatch(request, Address::from_str(&refund_receiver).unwrap());
        req.send().await.unwrap()
    }

    /// This function would be responsible for waiting for the transaction to be mined
    /// this would be called by the monitoring thread
    ///
    /// # Arguments
    /// - `self`
    /// - `pending_tx` - The transaction hash to be monitored
    pub async fn wait_for_transaction(
        &self,
        pending_tx: PendingTransactionBuilder<Http<Client>, Ethereum>,
    ) -> FixedBytes<32> {
        // TODO: add configuration for the number of blocks to wait for
        let tx_hash = pending_tx.watch().await.unwrap();
        tx_hash
    }

    pub fn get_trusted_forwarder(
        &self,
    ) -> TrustedForwarderContractInstance<
        Http<Client>,
        FillProvider<
            JoinFill<
                JoinFill<
                    alloy::providers::Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<Http<Client>>,
            Http<Client>,
            Ethereum,
        >,
    > {
        let rand_private_key: PrivateKeySigner = self.chains_config.accounts_private_keys[0]
            .clone()
            .parse()
            .unwrap();
        let wallet = EthereumWallet::from(rand_private_key.clone());
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(self.chains_config.rpc_url.parse().unwrap());

        TrustedForwarderContract::new(self.trusted_forwarder, provider)
    }
}

/// impl from `ForwardRequestData` to `ERC2771Forwarder::ForwardRequestData`
impl From<ForwardRequestData> for ERC2771Forwarder::ForwardRequestData {
    fn from(data: ForwardRequestData) -> Self {
        Self {
            from: data.from,
            to: data.to,
            value: data.value,
            gas: data.gas,
            deadline: data.deadline,
            data: data.data,
            signature: data.signature,
        }
    }
}

/// impl from `&ForwardRequestData` to `ERC2771Forwarder::ForwardRequestData`
impl From<&ForwardRequestData> for ERC2771Forwarder::ForwardRequestData {
    fn from(data: &ForwardRequestData) -> Self {
        Self {
            from: data.from,
            to: data.to,
            value: data.value,
            gas: data.gas,
            deadline: data.deadline,
            data: data.data.clone(),
            signature: data.signature.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configs::ChainsConfig;
    use alloy::{
        network::EthereumWallet, node_bindings::Anvil, primitives::Address,
        providers::ProviderBuilder, signers::local::PrivateKeySigner,
    };
    use std::str::FromStr;

    #[tokio::test]
    async fn test_process_request() {
        let anvil = Anvil::new().block_time(1).try_spawn().unwrap();
        let http_rpc_url = anvil.endpoint();
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer.clone());
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(http_rpc_url.parse().unwrap());

        let tf = TrustedForwarderContract::deploy(provider.clone(), "TF".to_string())
            .await
            .unwrap();
        let tf_instance = TrustedForwarderContract::new(*tf.address(), provider);

        let chains_config = ChainsConfig {
            name: Some("Ethereum Dev Network".to_string()),
            rpc_url: http_rpc_url,
            chain_id: anvil.chain_id(),
            accounts_private_keys: vec![signer.to_bytes().to_string()],
            trusted_forwarder: tf_instance.address().to_string(),
        };
    }
}
