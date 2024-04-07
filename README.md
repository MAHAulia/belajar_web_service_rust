# Belajar Web Server With Rush x Inarust
thanks to [inarust](https://github.com/inarust/inarust) for the boilerplate 🤩

## How To Use
1. Copy .env.example to .env
    ```sh
    mv .env.example .env
    ```
2. Set your database connection and your server host to serve this web on your new .env file
    ```nano
    nano .env
    ```

    ```.env
    DB_URL=<MONGO DB CONNECTION STRING>
    DB_NAME=<DATABASE NAME>
    HOST=<HOST>:<PORT>

    ```

    example value
    ```.env
    DB_URL=mongodb://localhost:27017
    DB_NAME=mydatabase
    HOST=0.0.0.0:8080
    ```

3. Run your web server with cargo on your terminal
    ```
    cargo run
    ```

4. Test your web server
    1. test using curl from your terminal
        ### test hit base url
        ```curl
        curl --request GET \
        --url http://localhost:8080/
        ```

        ### test hit add user
        ```curl
        curl --request POST \
        --url http://localhost:8080/user \
        --header 'Content-Type: application/json' \
        --data '{
        "username": "Coba Lag",
        "email": "cobalagi@gmail.com"
        }'
        ```

        ### test hit get all user
        ```curl
        curl --request GET \
        --url http://localhost:8080/users
        ```

        ### test hit get detail user
        ```curl
        curl --request GET \
        --url http://localhost:8080/user/fbcbac32-c05b-4f63-82fd-cc82ab53e103
        ```

        ### test hit update user
        ```curl
        curl --request PUT \
        --url http://localhost:8080/user/fbcbac32-c05b-4f63-82fd-cc82ab53e103 \
        --header 'Content-Type: application/json' \
        --data '{
        "username": "Inara 2",
        "email": "inara.2@gmail.com"
        }'
        ```

        ### test hit delete user
        ```curl
        curl --request DELETE \
        --url http://localhost:8080/user/bab2d00f-27bc-425f-9f45-1f1873884329
        
        ```
    2. test using api client, im using bruno
        Lihat pada folder doc

5. Build for deployment, you can build your rust app with cargo using this command in your terminal
    ```
    cargo build
    ```

### Have A Nice Day 👋