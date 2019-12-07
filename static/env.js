const api_url = "http://localhost:8001";

const auth_config = {
    domain:    "dev-ztmxpnax.auth0.com",
    client_id: "aBugMkF4ioYLtE02wWNo28lYxPhwx0eC",
    leeway:    300
};


//// Globals set by the app ////

// The auth0 client
let auth0 = null;

// Callbacks used in templates
let add_project = null;
let add_worker  = null;
let add_task    = null;
let get_project = null;
let get_worker  = null;
let get_task    = null;

