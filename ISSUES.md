- [ ] We are not checking queue size as of now when consuming events this need to change
- [ ] we can publish a health from each pod which will enable Nebula crate to publish events when devnet is not down and health % is below some critical threshold
- [ ]  Add S3 support to fetch keypair from a secure location and we can load S3 path from env config (or something better)


- [x] Previous event tx gets settle when new events is consumed. Ideally the event should not stay in queue and should be published