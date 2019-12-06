const api_url = "http://localhost:8001";

const auth_config = {
    domain:    "dev-ztmxpnax.auth0.com",
    client_id: "aBugMkF4ioYLtE02wWNo28lYxPhwx0eC",
    leeway:    300
};

// These are globals set and used by the app.
let auth0 = null;
let add_project = null;
let get_project = null;

