mod apps;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Index,

    #[at("/bridge")]
    Bridge,
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => {
            html! {
                <div id="app-index" class="container">
                    <h1>
                        { "Hello, World!" }
                    </h1>

                    <div class="link-wrapper">
                        <a href="https://github.com/Patryk27/saul">
                            { "See the project's page" }
                        </a>
                    </div>
                </div>
            }
        }

        Route::Bridge => {
            html! { <apps::bridge::App /> }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
