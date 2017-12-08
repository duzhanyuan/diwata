## Diwata is a database interface with the goal of being usable, user-friendly with its basic and advanced functionality be discoverable by the user.

Diwata comprised of 3 major components:
1. The client side UI.
2. The orm
3. The intellisense

## The client side UI.
The client side is the part of the app which the user interacts. Due to the complexity of the system
we need a high static typing compiler that 



## Overall design infastructure
* The intelisense feature has been decoupled away from rustorm
    and is more specific to diwata than to an orm.
* Intelisense data needs to stored in a separate database (sqlite) in 
    such a way it doesn't mess around with the user database
* Diwata will be able to handle multiple database with 
    each specific configurations and are highly sensitive.
    Diwata needs a way to protect the app, so a login/password
    may be employed and synced into the cloud
* Diwata specific configuration will need to be persited into
    the sqlite database, this includes user preference for
    SQL encoding/beautifier/formatter, use of smart grids
    traversal of records, allow indirect links


