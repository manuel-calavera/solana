use crate::cluster_info::{NodeInfo, FULLNODE_PORT_RANGE};
use crate::thin_client::ThinClient;

pub fn mk_client(r: &NodeInfo) -> ThinClient {
    let (_, transactions_socket) = solana_netutil::bind_in_range(FULLNODE_PORT_RANGE).unwrap();
    ThinClient::new(r.rpc, r.tpu, transactions_socket)
}
