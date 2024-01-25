# both

-   check the dockerfile to see how dummy directories are used to cache dependencies before
    actually building

# client

-   set env variables in trunk like TRUNK_PROD=true trunk build --release
-   access env variables in Yew like :
    let in_production = std::env!("TRUNK_PROD");
-   websocket address should be in the format wss://domainname.com/ws

# server

-   actix address must be changed to 0.0.0.0:port in order to serve from inside
    a docker container
-   nginx should be configured like shown in the config file in same directory
-   in order to ensure the docker container exits on panic set
    RUST_BACKTRACE=1 env to in server dockerfile and make sure the s
    erver Cargo.toml sets the behavior of :
    [profile.release]
    panic = 'abort'
