# Lease Management System

# UseCase:

Lease Management System is a smart contract where the owner of the contract can add different Renters. These Renters can then list their properties in lease. Rentee can look into all the properties which are available and can lease any of the given properties.

# Requirements:

## Owner of the contract

- None

## Renter Role

- Can add multiple flats or houses for lease
- Can terminate the lease. If the Renter accepted the Rentee then the Renter can terminate the contract only if Rentee defaults on rent.
- Can accept Rentee, If the request for lease is raised by Rentee and then Renter need to accept that request to rent property to Rentee
- Can reject Rentee, If the request for lease is raised by Rentee and then Renter need to reject that request to make its property available for other Rentees

## Rentee Role:

- Pay rent to the Renter if it is rented by the Rentee.
- Can propose to buy a property and submit security and first month’s pay in the contract.
- Once the contract is terminated Rentee is not allowed to pay rent.

# Functions:

- AddProperty(rent)
  - Properties
    - It is used to list the property for rent. The caller of this function will be the Renter of that property.
    - If the Renter is listing property first time, will register as a Renter else update the list with newly listed properties
    - The rent amount must be in the native currency of the chain ie.. cudos in this case.
    - Property is assigned with propertyid
    - PropertyId is auto-incremental id ie... if the contract has 100 properties (listed by a different Renter) then the id would start from 1 to 100 and the next property id will be 101.
- RequestForLease(propertyId)
  - Properties
    - The caller of this function is Rentee who wants to rent a property and will pay rent + security in desired denomination mentioned in the contract ie.. native currency.
    - Locks rent of the first month with a security deposit which is equivalent to one month rent to the contract ie... Rentee needs to lock 2x amount of rent.
    - This rent of the first month + security is released when the Renter of the property accepts the rent.
    - If the amount provided by the Rentee is more than one month’s rent + security then refund the excess rent to the Rentee.
  - Technical details
    - If the denomination of the amount passed is different as mentioned inside the contract then throw an error named **InvalidDenom.**
    - If property id is not present inside the contract then throw an error **StdError::NotFound {kind: String::from("Property not found"),}**
    - If the amount passed to this function is less than rent + security throw an error **StdError::overflow.**
    - If property is already rented then throw an error **IsRented.**
    - If property is already requested by some other other rentee and is not accepted by renter then throw **RenteeExist.**
- PayRent(propertyId)
  - Properties
    - It can only be done after the Renter accepted the Rentee.
    - Can only be called by the Rentee of the flat within the completion of the month.
    - If the Rentee pays the rent after 1 month then it is expired.
    - If the Rentee paid rent twice in the month then the Rentee agreement is valid for two months.
    - If the amount provided by the Rentee is more than one month’s rent then refund the excess rent to the Rentee.
  - Technical details
    - If the denomination of the amount passed is different as mentioned inside the contract then throw an error named **InvalidDenom.**
    - If property id is not present inside the contract then throw an error **StdError::NotFound {kind: String::from("Property not found"),}.**
    - If the amount passed to this function is less than rent + security throw an error **StdError::overflow.**
    - If Rentee of the property and caller of the function is not the same then throw the error **InvalidRentee.**
    - If rentee is not present on a given property and caller pay rent to this given id then throw error **IsNotRented.**
    - If expiration time does not exist then throw an error **ExpirationDoesNotExist.**
- AcceptLease(propertyId)
  - Properties
    - Can be called only by Renter of the property
    - The rent of the first month locked inside the contract is released to the Renter
    - Also, update the expiration date by one month.
  - Technical details
    - If property id is not present inside the contract then throw an error **NotFound.**
    - If Rentee of the property is not present then error **IsNotRented.**
    - If the caller is not Renter then throw error **InvalidRenter**
- RejectLease(propertyId)
  - Properties
    - It is used to reject the Rentee and release the amount locked by the Rentee for a given property.
    - Can be called only by Renter of the property
    - The rent of the first month+security locked inside the contract is released to the Rentee
    - Also, update the expiration date with None.
    - Update Rentee with None.
  - Technical details
    - If property id is not present inside the contract then throw an error **StdError::NotFound {kind: String::from("Property not found"),}.**
    - If Rentee of the property is not present then error **IsNotRented.**
    - If the caller is not Renter then throw error **InvalidRenter**
    - If already accepted by renter and then renter trying to reject the lease then throw error **IsAcceptedByRenter.**
- TerminateLease(propertyId)
  - Properties
    - can be called by the Renter of the property and is used to terminate the lease only if Rentee defaults on any month’s rent.
    - Release the security deposit to Rentee.
    - Update the expiration date with **None**
    - Remove the Rentee with that property id.
  - Technical details
    - If property id is not present inside the contract then throw an error **StdError::NotFound {kind: String::from("Property not found"),}.**
    - If the caller is not Renter then throw error **InvalidRenter**
    - If the rental agreement is not expired then Renter can not terminate the agreement and throw the error **NotExpired.**
    - If an expiration date is not present then throw the error **IsNotRented.**
- ShowAllAvailable()
  - Properties
    - It is used to view unrented properties
- GetTotalProperties()
  - Properties
    - It is used to view total number of properties.
- PropertyInfo(id)
  - Properties
    - It is to view Renter, Rentee, and rent.
  - Technical details
    - If id is not present then throw a **StdError::NotFound {kind: String::from("Property not found"),}.**
- GetOwner
  - Properties
    - Get the address of the owner of the contract.

# Tips

- **acudos** is the denomination of cudos-public-testnet

# Guides

- [Cosmwasm-storage](https://docs.rs/cw-storage-plus/0.10.3/cw_storage_plus/)
  - [Item](https://docs.rs/cw-storage-plus/0.10.3/cw_storage_plus/struct.Item.html)
  - [Map](https://crates.io/crates/cw-storage-plus/0.5.0)
- [Cosmwasm-std](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/)
  - [entry_point](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/macro.create_entry_points.html)
  - [to_binary](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/fn.to_binary.html)
  - [Uint128](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/struct.Uint128.html)
  - [DepsMut](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/struct.DepsMut.html)
  - [Deps](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/struct.Deps.html)
  - [Env](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/struct.Env.html)
  - [MessageInfo](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/struct.MessageInfo.html)
  - [BankMsg](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/enum.BankMsg.html)
    - [Send](https://docs.rs/cosmwasm-std/0.16.0/cosmwasm_std/enum.BankMsg.html#variant.Send)
- Rust
  - [Vec](https://doc.rust-lang.org/rust-by-example/std/vec.html)
  - [Option](https://doc.rust-lang.org/std/option/)
  - [Result](https://doc.rust-lang.org/std/result/)
  - [String](https://doc.rust-lang.org/rust-by-example/std/str.html)
  - [Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
  - [Struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- Package
  - [Cw0](https://docs.rs/cw0/0.10.3/cw0/)
    - [Duration](https://docs.rs/cw0/0.10.3/cw0/enum.Duration.html)
    - [Expiration](https://docs.rs/cw0/0.10.3/cw0/enum.Expiration.html)
