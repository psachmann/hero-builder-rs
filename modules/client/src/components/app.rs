use leptos::*;

#[component]
pub fn app() -> impl IntoView {
    let counter = create_rw_signal(0);

    view! {
        <div class="container mx-auto">
            <div class="flex flex-col">
                <div class="justify-center">
                    <p class="text-4xl font-bold pt-16">{ "Hello, world from Leptos!" }</p>
                </div>
                <div class="grid grid-cols-3 grid-flow-col gap-4">
                    <div class="grid-cols-1">
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click={ move |_| counter.set(counter.get() - 1) }>
                            { "Decrement" }
                        </button>
                    </div>
                    <div class="grid-cols-1">
                        <p class="text-lg font-bold pt-16">{ counter }</p>
                    </div>
                    <div class="grid-cols-1">
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click={ move |_| counter.set(counter.get() + 1) }>
                            { "Increment" }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
