# Substrate Proposal Assignment

This is a substrate project in which we can create/add Community having its own pot
and community members can propose projects to the Committee.
The Committee is a subset of the community members which can be added by sudo member.
If the Committee approves the proposal, funds will be allocated to the
beneficiary. If not, the proposal is rejected.
The deadline of voting is calculated as 7 days after the first vote is made on the proposal.
A proposal is composed by a title, the hash of a document, and the amount of
funds requested by the beneficiary.
There is a method to fund the pot which we can call by any community member that wants
to donate funds to the pot.

In summary, pallet having:
● A list of the community members (anyone can join a Community)
● Allow sudo to assign a community member to a Committee
● Allow any community member to send proposals
● Allow the Committee to vote on any Proposal
● Have an event that notifies about the outcome of a voting process(approved / rejected)


## Architecture
<img src="https://github.com/PankajChaudhary5/Assignment/blob/main/architecture.jpeg"/>


### Build the project

```sh
cargo build --release
```

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/node-template --dev
```
