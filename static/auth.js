//let auth_config = {

//    domain: "dev-ztmxpnax.auth0.com",
//    client_id: "aBugMkF4ioYLtE02wWNo28lYxPhwx0eC"
//};
//
let auth0 = null;
//
//const auth0Login = async () => {
//    await auth0.loginWithRedirect({
//        redirect_uri: window.location.origin
//    });
//};
//
//const auth0Logout = () => {
//    auth0.logout({
//        returnTo: window.location.origin
//    });
//};
//
//const auth0IsAuthenticated = async () => {
//    return await auth0.isAuthenticated();
//}

//const updateUI = async () => {
//    const isAuthed = await auth0.isAuthenticated();
//    return await auth0.isAuthenticated();
//    document.getElementById("btn-logout").disabled = !isAuthed;
//    document.getElementById("btn-login").disabled = isAuthed;

//    if (isAuthed) {
//        const user = JSON.stringify(await auth0.getUser());
//        const token = await auth0.getTokenSilently();
//        document.getElementById("content").innerHTML = user + "/" + token;
//    } else {
//        document.getElementById("content").innerHTML = "";
//    }
//};

//window.onload = async () => {
//    console.log("window.onload");
//    auth0 = await createAuth0Client(auth_config);
//    console.log("auth0 created");
//    const query = window.location.search;
//    
//    if (query.includes("code=") && query.includes("state=")) {
//        const isAuthed = await auth0.isAuthenticated();
//
//        if (!isAuthed) {
//            await auth0.handleRedirectCallback();
//            updateUI();
//        }
//
//        window.history.replaceState({}, document.title, "/");
//    }
//};
