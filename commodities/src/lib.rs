// #![cfg_attr(not(feature = "std"), no_std)]

// /// Edit this file to define custom logic or remove it if it is not needed.
// /// Learn more about FRAME and the core library of Substrate FRAME pallets:
// /// <https://docs.substrate.io/reference/frame-pallets/>
// pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// #[frame_support::pallet]
// pub mod pallet {
// 	use frame_support::{
// 		dispatch::DispatchResult,
// 		pallet_prelude::*,
// 		traits::{Currency, LockableCurrency, ReservableCurrency, tokens::Balance},
// 	};
// 	use frame_system::pallet_prelude::*;

// 	#[pallet::pallet]
// 	pub struct Pallet<T>(_);

// 	// The custom BalanceOf type.
// 	type BalanceOf<T> =
// 		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// 	/// Configure the pallet by specifying the parameters and types on which it depends.
// 	#[pallet::config]
// 	pub trait Config: frame_system::Config {
// 		/// Because this pallet emits events, it depends on the runtime's definition of an event.
// 		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

// 		type Currency: ReservableCurrency<Self::AccountId>
// 			+ LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
// 	}

// 	#[derive(PartialEq, Eq, Clone, Debug)]
// 	pub struct Contract<AccountId, Balance> {
// 		seller: AccountId,
// 		price: Balance,
// 		volume: u64,
// 		price_per_unit: Balance,
// 		finality_block: u32,
// 		product_type: u8,
// 		seller_approve: bool,
// 		buyer_approve: bool
// 	}

// 	// Store the total running contract count
// 	#[pallet::storage]
// 	#[pallet::getter(fn contract_count)]
// 	pub(super) type ContractCount<T> = StorageValue<_, u64, ValueQuery>;

// 	// Store every unique contract
// 	#[pallet::storage]
// 	#[pallet::getter(fn contracts)]
// 	pub(super) type Contracts<T: Config> =
// 		StorageMap<_, Blake2_128Concat, Contract<T::AccountId, BalanceOf<T>>, ValueQuery>;

// 	// Store bool to track whether or not a given contract has been sold
// 	#[pallet::storage]
// 	#[pallet::getter(fn sold_contracts)]
// 	pub(super) type SoldContracts<T: Config> =
// 		StorageMap<_, Blake2_128Concat, bool, u64, ValueQuery>;

// 	// Store account id of the contract buyer at the contract's unique u64
// 	#[pallet::storage]
// 	#[pallet::getter(fn contract_buyers)]
// 	pub(super) type ContractBuyers<T: Config> =
// 		StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

// 	// Pallets use events to inform users when important changes are made.
// 	// https://docs.substrate.io/main-docs/build/events-errors/
// 	#[pallet::event]
// 	#[pallet::generate_deposit(pub(super) fn deposit_event)]
// 	pub enum Event<T: Config> {
// 		ContractCreated { sender: T::AccountId, id: u64 },
// 		ContractBought { buyer: T::AccountId, id: u64, price: BalanceOf<T> },
// 		ContractFinalized { seller: T::AccountId, buyer: T::AccountId, id: u64, amount: BalanceOf<T>},
// 	}

// 	// Errors inform users that something went wrong.
// 	#[pallet::error]
// 	pub enum Error<T> {
// 		InvalidContractId,
// 		ContractNotFound,
// 		ContractAlreadyLocked,
// 		ContractNotLocked,
// 		ContractNotFinalized,
// 		InsufficientFunds,
// 		InvalidTransaction,
// 		InvalidFinalityBlock,
// 	}

// 	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// 	// These functions materialize as "extrinsics", which are often compared to transactions.
// 	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
// 	#[pallet::call]
// 	impl<T: Config> Pallet<T> {

// 		// Create a new contract with the given details.
// 		#[pallet::call_index(0)]
// 		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
// 		pub fn create_contract(origin: OriginFor<T>, price:  BalanceOf<T>, volume: u64,
// 		price_per_unit: BalanceOf<T>, finality_block: u32, product_type: u8) -> DispatchResult {
// 			ensure!(finality_block >= T::BlockNumber, Error::<T>::InvalidFinalityBlock);

// 			let sender = ensure_signed(origin)?;
// 			let id = Self::contract_count();
// 			let contract = Contract {
// 				seller: sender.clone(),
// 				price,
// 				volume,
// 				price_per_unit,
// 				finality_block,
// 				product_type,
// 				seller_approve: false,
// 				buyer_approve: false
// 			};

// 			Contracts::<T>::insert(id, contract);
// 			ContractCount::put(id + 1);

// 			Self::deposit_event(Event::ContractCreated(sender, id));

// 			Ok(())
// 		}

// 		// Buy a contract with the given ID.
// 		#[pallet::call_index(1)]
// 		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
// 		pub fn buy_contract(origin: OriginFor<T>, id: u64) -> DispatchResult {
// 			let buyer = ensure_signed(origin)?;
// 			let mut contract = Contracts::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;

// 			ensure!(!Self::sold_contracts(id), "Contract has already been sold");

// 			let final_price = contract.price_per_unit * contract.volume;

// 			ensure!(T::Currency::free_balance(&buyer) >= final_price || T::Currency::free_balance(&buyer) >=
// &contract.price, "Buyer does not have enough funds");

// 			T::Currency::transfer(&buyer, &contract.seller, &contract.price,
// frame_support::traits::ExistenceRequirement::KeepAlive)?;

// 			ContractBuyers::<T>::insert(id, buyer.clone());

// 			SoldContracts::<T>::insert(id, true);

// 			Self::deposit_event(Event::ContractBought(buyer, id, &contract.price));

// 			Ok(())
// 		}

// 		// Finalize a contract with the given ID.
// 		#[pallet::call_index(2)]
// 		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
// 		pub fn finalize_contract(origin: OriginFor<T>, id: u64) -> DispatchResult {
// 			let sender = ensure_signed(origin)?;
// 			let contract = Contracts::<T>::get(id).ok_or("Contract does not exist")?;
// 			let buyer: T::AccountId = ContractBuyers::<T>::get(id).ok_or("No buyer for this contract")?;
// 			let seller: T::AccountId = contract.seller;

// 			ensure!(contract.finality_date <= T::BlockNumber, "Contract has not yet reached finality");
// 			ensure!(sender == contract.seller || sender == buyer, "Only seller or buyer can finalize the
// contract");

// 			if seller == sender {
// 				contract.seller_approved = true;
// 			} else {
// 				contract.buyer_approved = true;
// 			}

// 			let final_price = contract.price_per_unit * contract.volume;

// 			if contract.seller_approved && contract.buyer_approved {
// 				T::Currency::transfer(&contract.seller, &buyer, final_price,
// frame_support::traits::ExistenceRequirement::KeepAlive)?;
// 				Self::deposit_event(Event::ContractFinalized(contract.seller.clone(), buyer.clone(), id,
// final_price)); 				Contracts::<T>::remove(id);
// 				ContractBuyers::<T>::remove(id);
// 			} else {
// 				Contracts::<T>::insert(id, contract);
// 			}

// 			Ok(())
// 		}
// 	}
// }

#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{Currency, LockableCurrency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	type BalanceOf<T> = 
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	
	type ContractId = u64;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: ReservableCurrency<Self::AccountId>
        	+ LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
	}

	// Store the running count of contracts
	#[pallet::storage]
	#[pallet::getter(fn contract_count)]
	pub(super) type ContractCount<T> = StorageValue<_, ContractId, ValueQuery>;

	// Store contract senders
	#[pallet::storage]
	#[pallet::getter(fn contract_sender)]
	pub(super) type ContractSender<T: Config> =
		StorageMap<_, Blake2_128Concat, ContractId, T::AccountId>;

	// Store contract senders
	#[pallet::storage]
	#[pallet::getter(fn contract_price)]
	pub(super) type ContractPrice<T: Config> =
		StorageMap<_, Blake2_128Concat, ContractId, BalanceOf<T>>;

	// Store volume of product being sold
	#[pallet::storage]
	#[pallet::getter(fn contract_volume)]
	pub(super) type ContractVolume<T: Config> =
		StorageMap<_, Blake2_128Concat, ContractId, u16>;

	// Store price per unie of product being sold for a given contract
	#[pallet::storage]
	#[pallet::getter(fn contract_price_per_unit)]
	pub(super) type ContractPricePerUnit<T: Config> =
		StorageMap<_, Blake2_128Concat, ContractId, BalanceOf<T>>;

	// Store finality block for the contract
	#[pallet::storage]
	#[pallet::getter(fn contract_finality_block)]
	pub(super) type ContractFinalityBlock<T: Config> =
		StorageMap<_, Blake2_128Concat, ContractId, u32>;
	
	// Store product type for contract
	#[pallet::storage]
	#[pallet::getter(fn contract_product_type)]
	pub(super) type ContractProductType<T: Config> =
		StorageMap<_, Blake2_128Concat, ContractId, u8>;

	// Store contract buyer
	#[pallet::storage]
	#[pallet::getter(fn contract_buyer)]
	pub(super) type ContractBuyer<T: Config> =
		StorageMap<_, Blake2_128Concat, ContractId, T::AccountId>;

	// Store whether or not contract was sold
	#[pallet::storage]
	#[pallet::getter(fn contract_sold)]
	pub(super) type ContractSold<T: Config> =
		StorageMap<_, Blake2_128Concat, ContractId, bool>;

	// Store whether or not contract was sold
	#[pallet::storage]
	#[pallet::getter(fn contract_finalized)]
	pub(super) type ContractFinalized<T: Config> =
		StorageMap<_, Blake2_128Concat, ContractId, bool>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ContractCreated { 
			id: ContractId, 
			sender: T::AccountId, 
			price: BalanceOf<T>, 
			price_per_unit: BalanceOf<T>, 
			volume: u16, 
			finality_block: u32, 
			product_type: u8
		},
		ContractPurchased { 
			id: ContractId, 
			buyer: T::AccountId, 
			contract_price: BalanceOf<T>, 
			finality_price: BalanceOf<T>, 
		},
		ContractFinalized { 
			id: ContractId, 
			buyer: T::AccountId, 
			seller: T::AccountId,
			final_price: BalanceOf<T>,
		},
		Locked{user: T::AccountId, amount: BalanceOf<T>},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		ContractNotFound,
		ContractAlreadyPurchased,
		InsufficientBalance,
		SellerSelfPurchase,
		UnauthorizedFinalizationAttempt,
		ContractAlreadyFinalized,
		FinalityBlockNotReached,
		ContractNotPurchased,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_contract(
			origin: OriginFor<T>, 
			finality_block: u32, 
			product_type: u8, 
			volume: u16,
			price_per_unit: BalanceOf<T>,
			price: BalanceOf<T>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let sender = ensure_signed(origin)?;
			let id = Self::contract_count();

			// Update storage.
			ContractCount::<T>::put(id + 1);
			ContractSender::<T>::insert(id, &sender);
			ContractPrice::<T>::insert(id, &price);
			ContractPricePerUnit::<T>::insert(id, &price_per_unit);
			ContractVolume::<T>::insert(id, &volume);
			ContractProductType::<T>::insert(id, &product_type);
			ContractFinalityBlock::<T>::insert(id, &finality_block);
			ContractSold::<T>::insert(id, false);

			// Emit an event.
			Self::deposit_event(Event::ContractCreated { 
				id, 
				sender, 
				volume, 
				finality_block, 
				product_type,
				price, 
				price_per_unit, 
			});
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Purchase a contract by transfering the amount (price) from the function caller to the seller
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn purchase_contract(origin: OriginFor<T>, id: ContractId) -> DispatchResult {
			let buyer = ensure_signed(origin)?;
			let price = ContractPrice::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;
			let ppu = ContractPricePerUnit::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;
			let volume = ContractVolume::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;
			let seller = ContractSender::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;
			
			// Ensure that the buyer has enough funds to cover the price of the contract and the final transaction
			ensure!(T::Currency::free_balance(&buyer) > price + ppu * BalanceOf::<T>::from(volume), Error::<T>::InsufficientBalance);
			// Ensure that the buyer is not the seller
			ensure!(&buyer != &seller, Error::<T>::SellerSelfPurchase);
			// Ensure that the contract has not already been bought
			ensure!(!Self::contract_sold(id).ok_or(Error::<T>::ContractNotFound)?, Error::<T>::ContractAlreadyPurchased);

			T::Currency::transfer(&buyer, &seller, price, frame_support::traits::ExistenceRequirement::KeepAlive)?;
			ContractBuyer::<T>::insert(id, &buyer);
			ContractSold::<T>::insert(id, true);


			// Emit an event.
			Self::deposit_event(Event::ContractPurchased { 
				id: id, 
				buyer: buyer, 
				contract_price: price, 
				finality_price: ppu * BalanceOf::<T>::from(volume), 
			});

			Ok(())
		}

		/// Finalize the contract (transfer funds)
		/// Can only be called by the buyer or seller of the contract after the finalization block 
		/// If the buyer calls first the final transaction is processed and the contract is marked as finalized
		/// If the seller calls first the buyer's funds are locked until they call the function as well at which point the transaction processes
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn finalize_contract(origin: OriginFor<T>, id: ContractId) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let seller = ContractSender::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;
			let buyer = ContractBuyer::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;
			let ppu = ContractPricePerUnit::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;
			let volume = ContractVolume::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;
			let contract_finalized = !Self::contract_finalized(id).ok_or(Error::<T>::ContractNotFound)?;
			let contract_sold = Self::contract_sold(id).ok_or(Error::<T>::ContractNotFound)?;
			// let finality_block = ContractFinalityBlock::<T>::get(id).ok_or(Error::<T>::ContractNotFound)?;
			// let current_block = <frame_system::Pallet<T>>::block_Number();

			// Make sure the function caller is either the buyer or the seller
			ensure!(&origin == &buyer || &origin == &seller, Error::<T>::UnauthorizedFinalizationAttempt);
			// Make sure the contract has not already been finalied
			ensure!(contract_finalized, Error::<T>::ContractAlreadyFinalized);
			// Make sure the current block is past the finality block
			// ensure!(current_block <= T::BlockNumber::from(finality_block), Error::<T>::FinalityBlockNotReached);
			// Ensure the contract has been bought
			ensure!(contract_sold, Error::<T>::ContractNotPurchased);

			if origin == buyer {
				T::Currency::transfer(&buyer, &seller, ppu * BalanceOf::<T>::from(volume), frame_support::traits::ExistenceRequirement::KeepAlive)?;
				ContractFinalized::<T>::insert(id, true);
			} else if origin == seller {

			} 

			// Emit an event.
			Self::deposit_event(Event::ContractFinalized { 
				id: id, 
				buyer: buyer, 
				seller: seller,
				final_price: ppu * BalanceOf::<T>::from(volume),
			});
			
			Ok(())
		}
	}
}
