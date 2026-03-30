# Up and running

- [x] health_check endpoint
    - it tells that our api is running and ready to accept requests.
    - can be used by orchestrators to restart the container if endpoint is failing.
    - can be used to alert the admin if failing (usefull in our case of sending newsletters).
    - [x] integration test for health_check
        - [x] resort to tokio runtime for testing while using actix runtime for binary (as in latest actix docs)
        - [x] use reqwest for black box
        - [ ] extract API_BASE into env variable??
