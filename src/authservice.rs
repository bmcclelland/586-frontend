#![allow(unused_imports)]

use crate::domain::*;
use yew::prelude::*;
use stdweb::{ js, serde::Serde, };
use serde::{ Serialize, Deserialize };

#[derive(Serialize,Deserialize,Debug)]
pub struct AuthUser {
    pub name: String,
    pub token: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub enum AuthState {
    Unknown,
    Yes(AuthUser),
    No,
}

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        AuthService
    }
    
    pub fn init(&mut self, msg_callback: Callback<AuthState>) {
        let callback = move |x: Serde<AuthState>| {
            msg_callback.emit(x.0); 
        };
        
        // Simplifies getting the right structure into JS.
        let default_yes = Serde(AuthState::Yes(
            AuthUser {
                name:  "".into(),
                token: "".into(),
            }
        ));

        js! {
            const callback = @{callback};

            const f = async () => {
                let payload = null;
                auth0 = await createAuth0Client(auth_config);
                const authed = await auth0.isAuthenticated();
                const query = window.location.search;

                const auth_yes = async () => {
                    payload = @{default_yes}; 
                    const user  = await auth0.getUser();
                    const token = (await auth0.getIdTokenClaims()).__raw;
                    console.log(token);
                    payload.Yes.name = user.name;
                    payload.Yes.token = token;
                };
                            
                const auth_no = async () => {
                    payload = @{Serde(AuthState::No)};
                };
                
                if (query.includes("code=") && query.includes("state=")) {
                    if (authed) {
                        await auth_yes();
                    }
                    else {
                        await auth0.handleRedirectCallback();
                        
                        if (await auth0.isAuthenticated()) {
                            await auth_yes();
                        }
                        else {
                            await auth_no();
                        }
                    }
                    
                    window.history.replaceState({}, document.title, "/");
                }
                else
                {
                    if (authed) {
                        await auth_yes();
                    }
                    else {
                        await auth_no();
                    }
                }

                callback(payload);
                callback.drop();
            };

            f();
        };
    }

    pub fn login(&mut self) {
        js! {
            if (auth0 == null) {
                return;
            }
                
            const f = async () => {
                await auth0.loginWithRedirect({
                    redirect_uri: window.location.origin
                });
            };

            f();
        };
    }

    pub fn logout(&mut self) {
        js! {
            if (auth0 == null) {
                return;
            }
                
            auth0.logout({
                returnTo: window.location.origin
            });
        };
    }
}
