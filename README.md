# Echo
A peer2peer chat system with end-to-end encryption.
Decentralized, uncensored and privacy-friendly application.

## Step 1 : p2p chat
- Application must be able to send and handle message throw tcp request
- Application must provide a cli to interact with the deamon

## Step 2 : encryption
- Application must provide encryption
- Cypher keys are exchange at the first connection between 2 peers

## Step 3 : proof of work
- Messages must contains a proof of work to be valid
- Application must ignore invalid proof of work messages

## Step 4 : storage
- Application must store messages on local disk
- CLI must allow user to retrive received messages for a given period of time
- Deamon must implement message pruning

## Step 5 : message propagation
- Application must propagate external messages to peers
- Application must store external messages for a given period of time
- Application must share stored external messages with new peers

