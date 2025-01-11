use gstd::{msg, prelude::*, ActorId};
use gstd::collections::HashMap;

#[derive(Debug, Default)]
pub struct Sowa {
    owner: ActorId,
    minter_managers: HashMap<ActorId, bool>,
    balances: HashMap<(ActorId, u128), u128>, // (account, token_id) => balance
    token_uris: HashMap<u128, String>,
}

const TOKEN_RESIDUOS: u128 = 1;
const NFT_CERTIFICADO: u128 = 2;
const SOWA: u128 = 3;
const COSTO_CERTIFICADO: u128 = 100;
const RESIDUOS_URI: &str = "https://gateway.lighthouse.storage/ipfs/bafkreih7o2nadvkegagapvqxwwobfaxhq6e4dap4l6s67bzdazsx2jhqra";

impl Sowa {
    pub fn new(owner: ActorId) -> Self {
        Sowa {
            owner,
            minter_managers: HashMap::new(),
            balances: HashMap::new(),
            token_uris: HashMap::new(),
        }
    }

    pub fn add_minter_manager(&mut self, manager: ActorId, caller: ActorId) {
        self.ensure_owner_or_minter(caller);
        self.minter_managers.insert(manager, true);
    }

    pub fn remove_minter_manager(&mut self, manager: ActorId, caller: ActorId) {
        self.ensure_owner_or_minter(caller);
        self.minter_managers.remove(&manager);
    }

    pub fn mint_residuos(&mut self, account: ActorId, amount: u128, caller: ActorId) {
        self.ensure_owner_or_minter(caller);
        self.mint(account, TOKEN_RESIDUOS, amount);
        self.set_uri(TOKEN_RESIDUOS, RESIDUOS_URI.to_string());

        let reward_amount = amount * 10;
        self.mint(account, SOWA, reward_amount);
    }

    pub fn canjear_certificado(&mut self, caller: ActorId) {
        let balance = self.balance_of(&caller, SOWA);
        assert!(balance >= COSTO_CERTIFICADO, "Fondos insuficientes");

        self.burn(caller, SOWA, COSTO_CERTIFICADO);
        self.mint(caller, NFT_CERTIFICADO, 1);
    }

    pub fn mint_certificado(&mut self, account: ActorId, token_uri: String, caller: ActorId) {
        self.ensure_owner_or_minter(caller);
        self.mint(account, NFT_CERTIFICADO, 1);
        self.set_uri(NFT_CERTIFICADO, token_uri);
    }

    fn mint(&mut self, account: ActorId, token_id: u128, amount: u128) {
        let key = (account, token_id);
        let entry = self.balances.entry(key).or_insert(0);
        *entry += amount;
    }

    fn burn(&mut self, account: ActorId, token_id: u128, amount: u128) {
        let key = (account, token_id);
        let entry = self.balances.get_mut(&key).expect("Token no existe");
        assert!(*entry >= amount, "Fondos insuficientes");
        *entry -= amount;
    }

    fn balance_of(&self, account: &ActorId, token_id: u128) -> u128 {
        *self.balances.get(&(account.clone(), token_id)).unwrap_or(&0)
    }

    fn set_uri(&mut self, token_id: u128, uri: String) {
        self.token_uris.insert(token_id, uri);
    }

    fn ensure_owner_or_minter(&self, caller: ActorId) {
        assert!(caller == self.owner || self.minter_managers.get(&caller).copied().unwrap_or(false),
                "Solo el propietario o un Minter Manager puede ejecutar esta funcion");
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let owner: ActorId = msg::source();
    let contract = Sowa::new(owner);
    gstd::debug!("Sowa initialized with owner: {:?}", owner);
    gstd::util::store(contract);
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let contract: &mut Sowa = gstd::util::load();
    // Handle incoming messages (implement deserialization & logic here)
}
