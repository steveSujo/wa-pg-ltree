# Library for PostgreSQL Label Tree

The Ltree (label tree) is an extra Extensions or Module, as a result there is limited support
for it by the exisiting libraries in rust and javascript
and instead of writing two libraries with same logic
I wrote code for in rust and compiled it to wasm (Web Assembly),
so that I can have a node module with the required data struct

This code contains tree struct that is generated from the db ltree and send CRUD operaations to the db
provids methods for tree traversal
and tree modifications
