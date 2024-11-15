# Rust Server (Still working on)
-Rust Server for websites and emails addresses

## Things to Work on
1. ~~Setting up the HTTP stream correctly to use http request functions instead of the bytes~~
2. ~~Create an index.html page~~
3. Build a nameserver a DNS server
[DNS Notes](https://github.com/EmilHernvall/dnsguide) - This is to build one from scratch
[DNS Notes](https://dev.to/xfbs/writing-a-dns-server-in-rust-1gpn)
* I probably will go with the one below
[DNS Notes](https://docs.rs/hickory-server/latest/hickory_server/)
4. Using axum instead of the last method I had
[Axum Notes] (https://docs.rs/axum/latest/axum/)
3. On the index.html page have the following informaton
* Show IP Address
* Show if the server has node js installed and what version
* Show if the server has rust installed and what version
* Show if python is installed and what version
* Make the 404 page to have the information
4. ~~Get Javascript working on the site~~
4. ~~If you pick a page other than just the main address it will show the 404 page~~
5. ~~Find ways to shutdown the server without it crashing if the wrong data is being sent~~
4. See how to deploy it on the network so other computers on the same network can see it
5. Make a email server using Rust
6. Make a load balancer to be able to use another server if this one goes down
7. Have a system that can replicate the websites that are on the site and go to the other server using methods like commit
8. Going to be using SurrealDB for the email server and for the main database for the websites
9. Figure out how to setup the nameservers to be able to add them to the site
10. Make a 404 page if the client goes to an unknown page

Had some issues working with hyper and tokio but will look at it later on to see if I can change it to add it