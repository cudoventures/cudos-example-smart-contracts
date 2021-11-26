# Lease Management System

# UseCase:

Lease Management System is a smart contract where the owner of the contract can add different Renters. These renters can then list their properties in lease. Rentee can look into all the properties which are available and can lease any of the given properties. 5% of rent paid to the Renter goes to the Owner of the contract.

# Requirements:

## Owner of the contract

- None

## Renter Role

- Can add multiple flats or houses for lease
- Can terminate the lease. If renter accepted the rentee then renter can terminate the contract only if rentee defaults on rent.
- Can accept rentee, If request for lease is raised by rentee and then renter need to accept that request inorder to rent property to rentee
- Can reject rentee, If request for lease is raised by rentee and then renter need to reject that request inorder to make its property available for other rentee

## Rentee Role:

- Pay rent to the Renter if it is rented by the Rentee.
- Can propose to buy a property and submit security and first month’s pay in the contract.
- Once the contract is terminated Rentee is not allowed to pay rent.

# Functions:

- AddProperty(rent)

  - **Properties**
    - It is used to list the property for rent. The caller of this function will be the renter of that property.
    - If the renter is listing property first time, will register as a renter else update list with newly listed properties
    - Rent amount must be in the native currency of the chain ie.. cudos in this case.
    - Property is assigned with propertyid
    - PropertyId is auto-incremental id ie... if the contract has 100 properties (listed by different renter) then id would start from 1 to 100 and the next property id will be 101.

- RequestForLease(propertyId)
  - Properties
    - The caller of this function is Rentee who wants to rent a property and will pay rent + security in desired denomination mentioned in the contract ie.. native currency.
    - Locks rent of the first month with a security deposit which is equivalent to one month rent to the contract ie.. rentee needs to lock 2x amount of rent.
    - This rent of the first month + security is released when the Renter of the property accepts the rent.
    - If amount provided by rentee is more than one month rent + security then refund the excess rent to the rentee.
  - Technical details
    - If the denomination of the amount passed is different as mentioned inside the contract then throw an error named **InvalidDenom.**
    - If property id is not present inside the contract then throw an error **NotFound.**
    - If the amount passed to this function is less than rent + security throw an error **LessThanRent.**
- PayRent(propertyId)
  - Properties
    - It can only be done after renter accepted the rentee.
    - Can only be called by the rentee of the flat within completion of month.
    - If rentee pay the rent after 1 month then it is expired.
    - If rentee paid rent twice in the month then rentee agrement is valid for two months.
    - If amount provided by rentee is more than one month rent then refund the excess rent to the rentee.
  - Technical details
    - If the denomination of the amount passed is different as mentioned inside the contract then throw an error named **InvalidDenom.**
    - If property id is not present inside the contract then throw an error **NotFound.**
    - If the amount passed to this function is less than rent + security throw an error **LessThanRent.**
    - If Rentee of the property and caller of the function is not same then throw error **InvalidRentee.**
    - If expiration time does not exist then throw an error **ExpirationDoesNotExist.**
- AcceptLease(propertyId)
  - Properties
    - Can be called only by Renter of the property
    - The rent of the first month locked inside the contract is released to the Renter
    - Also update the expiration date with one month.
  - Technical details
    - If property id is not present inside the contract then throw an error **NotFound.**
    - If Rentee of the property is not present then error **IsNotRented.**
    - If caller is not Renter then throw error **InvalidRenter**
- RejectLease(propertyId)
  - Properties
    - It is used to reject the rentee and release the amount locked by the rentee for a given property.
    - Can be called only by Renter of the property
    - The rent of the first month+security locked inside the contract is released to the Rentee
    - Also update the expiration date with None.
    - Update rentee with None.
  - Technical details
    - If property id is not present inside the contract then throw an error **NotFound.**
    - If Rentee of the property is not present then error **IsNotRented.**
    - If caller is not Renter then throw error **InvalidRenter**
- TerminateLease(propertyId)
  - Properties
    - can be called by Renter of the property and is used to terminate the lease only if Rentee defaults on any month rent.
    - Release the security deposit to rentee.
    - Update the expiration date with **None**
    - Remove the rentee with that property id.
  - Technical details
    - If property id is not present inside the contract then throw an error **NotFound.**
    - If caller is not Renter then throw error **InvalidRenter**
    - If rent agreement is not expired then Renter can not terminate the agreement and throw error **NotExpired.**
    - If expiration date is not present then throw error **IsNotRented.**
    - If rentee is not present then throw error **InvalidRentee.**
- ShowAllAvailable()
  - Properties
    - It is use to view unrented properties
- ShowAllProperites()
  - Properties
    - It is used to view all properties.
- PropertyInfo(id)
  - Properties
    - It is to view owner, rentee and rent.

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
  - Vec
    - Vec manipulation
      - [How to find whether given value exist inside vector or not?](https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector)
  - Option
  - [Result](https://doc.rust-lang.org/std/result/)
  - String
  - Enum
  - Struct
- Package
  - [Cw0](https://docs.rs/cw0/0.10.3/cw0/)
    - [Duration](https://docs.rs/cw0/0.10.3/cw0/enum.Duration.html)
    - [Expiration](https://docs.rs/cw0/0.10.3/cw0/enum.Expiration.html)
